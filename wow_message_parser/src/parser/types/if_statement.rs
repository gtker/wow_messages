use std::collections::HashSet;

use crate::error_printer::non_matching_if_statement_variables;
use crate::file_info::FileInfo;
use crate::parser::types::parsed::parsed_if_statement::Condition;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::rust_printer::field_name_to_rust_name;
use crate::DefinerType;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) enum DefinerUsage {
    NotInIf,
    InIf,
}

#[derive(Debug, Clone)]
pub(crate) struct IfStatement {
    variable_name: String,
    equation: Equation,
    members: Vec<StructMember>,
    else_ifs: Vec<IfStatement>,
    else_statement_members: Vec<StructMember>,
    original_ty: Type,
    separate_if_statement: bool,
    flag_else_payload_name: Option<String>,
}

impl Eq for IfStatement {}

impl PartialEq for IfStatement {
    fn eq(&self, other: &Self) -> bool {
        self.members.first().unwrap() == other.members.first().unwrap()
    }
}

impl IfStatement {
    pub(crate) fn new(
        variable_name: String,
        equation: Equation,
        members: Vec<StructMember>,
        else_ifs: Vec<IfStatement>,
        else_statement_members: Vec<StructMember>,
        original_ty: Type,
        separate_if_statement: bool,
    ) -> Self {
        let is_enum = equation.definer_type() == DefinerType::Enum;
        Self {
            variable_name,
            equation,
            members,
            else_ifs,
            else_statement_members,
            original_ty,
            separate_if_statement: separate_if_statement && is_enum,
            flag_else_payload_name: None,
        }
    }

    pub(crate) fn flag_get_enumerator(&self) -> String {
        match self.equation() {
            Equation::BitwiseAnd { values: value } => value[0].clone(),
            Equation::Equals { .. } | Equation::NotEquals { .. } => {
                unreachable!("flag_get_enumerator was not flag")
            }
        }
    }

    pub(crate) fn flag_get_enumerator_rust_name(&self) -> String {
        field_name_to_rust_name(&self.flag_get_enumerator())
    }

    pub(crate) fn has_optional_members(&self) -> bool {
        fn contains_optional(members: &[StructMember]) -> bool {
            members.iter().any(|member| match member {
                StructMember::Definition(_) => false,
                StructMember::IfStatement(statement) => statement.has_optional_members(),
                StructMember::OptionalStatement(_) => true,
            })
        }

        contains_optional(self.members())
            || self.else_ifs().iter().any(Self::has_optional_members)
            || contains_optional(self.else_members())
    }

    pub(crate) fn flag_else_payload_name(&self) -> Option<String> {
        self.flag_else_payload_name.clone()
    }

    fn flag_else_payload_name_candidate(&self) -> Option<String> {
        if !matches!(self.equation(), Equation::BitwiseAnd { .. })
            || !self.else_ifs().is_empty()
            || self.else_members().is_empty()
        {
            return None;
        }

        let definitions = self.all_definitions();
        if definitions.is_empty() {
            return None;
        }

        let suffix = definitions
            .iter()
            .map(|definition| definition.name())
            .collect::<Vec<_>>()
            .join("_");

        Some(format!("{}_{}_payload", self.variable_name(), suffix))
    }

    fn legacy_flag_condition_local_name(&self) -> Option<String> {
        if self.flag_else_payload_name_candidate().is_some() {
            return None;
        }

        match self.equation() {
            Equation::BitwiseAnd { .. } => Some(format!(
                "{}_{}",
                self.variable_name(),
                self.flag_get_enumerator().to_lowercase()
            )),
            Equation::Equals { .. } | Equation::NotEquals { .. } => None,
        }
    }

    fn assign_flag_else_payload_name(&mut self, used_names: &mut HashSet<String>) {
        if let Some(base_name) = self.flag_else_payload_name_candidate() {
            // The reader also binds this payload to `{name}_if`.
            let mut name = base_name.clone();
            let mut suffix = 1;
            while used_names.contains(&name) || used_names.contains(&format!("{name}_if")) {
                name = format!("{base_name}_{suffix}");
                suffix += 1;
            }
            used_names.insert(format!("{name}_if"));
            used_names.insert(name.clone());
            self.flag_else_payload_name = Some(name);
        }

        assign_member_payload_names(&mut self.members, used_names);
        for else_if in &mut self.else_ifs {
            else_if.assign_flag_else_payload_name(used_names);
        }
        assign_member_payload_names(&mut self.else_statement_members, used_names);
    }

    fn collect_used_names(&self, names: &mut HashSet<String>) {
        if let Some(name) = self.legacy_flag_condition_local_name() {
            names.insert(name);
        }
        collect_member_used_names(&self.members, names);
        for else_if in &self.else_ifs {
            else_if.collect_used_names(names);
        }
        collect_member_used_names(&self.else_statement_members, names);
    }

    pub(crate) fn is_elseif_flag(&self) -> bool {
        match self.equation() {
            Equation::BitwiseAnd { .. } => !self.else_ifs().is_empty(),
            Equation::Equals { .. } | Equation::NotEquals { .. } => false,
        }
    }

    pub(crate) fn members(&self) -> &[StructMember] {
        &self.members
    }

    pub(crate) fn else_members(&self) -> &[StructMember] {
        &self.else_statement_members
    }

    pub(crate) fn original_ty(&self) -> &Type {
        &self.original_ty
    }

    pub(crate) fn definer_type(&self) -> DefinerType {
        self.equation.definer_type()
    }

    pub(crate) fn else_ifs(&self) -> &[IfStatement] {
        &self.else_ifs
    }

    pub(crate) fn all_members(&self) -> impl Iterator<Item = &StructMember> {
        let else_ifs = self.else_ifs.iter().flat_map(|a| a.members());
        self.members()
            .iter()
            .chain(else_ifs)
            .chain(&self.else_statement_members)
    }

    pub(crate) fn all_definitions(&self) -> Vec<&StructMemberDefinition> {
        let mut v = Vec::new();

        fn inner<'a>(m: &'a StructMember, v: &mut Vec<&'a StructMemberDefinition>) {
            match m {
                StructMember::Definition(d) => v.push(d),
                StructMember::IfStatement(statement) => {
                    v.append(&mut statement.all_definitions());
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, v);
                    }
                }
            }
        }

        for m in self.all_members() {
            inner(m, &mut v);
        }

        v
    }

    pub(crate) fn variable_name(&self) -> &str {
        &self.variable_name
    }

    pub(crate) fn equation(&self) -> &Equation {
        &self.equation
    }

    pub(crate) fn contains(&self, m: &StructMember) -> bool {
        if self.members().iter().any(|a| m == a)
            || self.else_ifs().iter().any(|a| a.contains(m))
            || self.else_members().iter().any(|a| m == a)
        {
            return true;
        }

        false
    }

    pub(crate) fn part_of_separate_if_statement(&self) -> bool {
        self.separate_if_statement
    }
}

pub(crate) fn assign_flag_else_payload_names(members: &mut [StructMember]) {
    let mut used_names = HashSet::new();
    collect_member_used_names(members, &mut used_names);
    assign_member_payload_names(members, &mut used_names);
}

fn collect_member_used_names(members: &[StructMember], names: &mut HashSet<String>) {
    for member in members {
        match member {
            StructMember::Definition(definition) => {
                names.insert(definition.name().to_string());
            }
            StructMember::IfStatement(statement) => statement.collect_used_names(names),
            StructMember::OptionalStatement(optional) => {
                names.insert(optional.name().to_string());
                collect_member_used_names(optional.members(), names);
            }
        }
    }
}

fn assign_member_payload_names(members: &mut [StructMember], used_names: &mut HashSet<String>) {
    for member in members {
        match member {
            StructMember::Definition(_) => {}
            StructMember::IfStatement(statement) => {
                statement.assign_flag_else_payload_name(used_names);
            }
            StructMember::OptionalStatement(optional) => {
                assign_member_payload_names(optional.members_mut(), used_names);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Operator {
    Equals,
    NotEquals,
    BitwiseAnd,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "&" => Operator::BitwiseAnd,
            "==" => Operator::Equals,
            "!=" => Operator::NotEquals,
            _ => unreachable!("invalid operator {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Equation {
    Equals { values: Vec<String> },
    NotEquals { value: String },
    BitwiseAnd { values: Vec<String> },
}

impl Equation {
    pub(crate) fn definer_type(&self) -> DefinerType {
        match self {
            Equation::Equals { .. } | Equation::NotEquals { .. } => DefinerType::Enum,
            Equation::BitwiseAnd { .. } => DefinerType::Flag,
        }
    }

    pub(crate) fn contains_enumerator(&self, enumerator: &str) -> bool {
        match self {
            Equation::NotEquals { value } => value == enumerator,
            Equation::Equals { values } | Equation::BitwiseAnd { values } => {
                values.iter().any(|a| a == enumerator)
            }
        }
    }

    pub(crate) fn new(conditions: &[Condition], ty_name: &str, file_info: &FileInfo) -> Self {
        let variable = &conditions[0];
        let variable_name = variable.value.clone();

        let value = conditions
            .iter()
            .map(|a| {
                if a.value != variable_name {
                    non_matching_if_statement_variables(
                        ty_name,
                        &variable_name,
                        &variable.value,
                        file_info,
                    );
                }

                a.equals_value.clone()
            })
            .collect();

        let equation = match variable.operator {
            Operator::Equals => {
                assert!(conditions
                    .iter()
                    .all(|a| matches!(a.operator, Operator::Equals)));

                Equation::Equals { values: value }
            }
            Operator::BitwiseAnd => {
                assert!(conditions
                    .iter()
                    .all(|a| matches!(a.operator, Operator::BitwiseAnd)));

                Equation::BitwiseAnd { values: value }
            }
            Operator::NotEquals => {
                assert_eq!(conditions.len(), 1);
                Equation::NotEquals {
                    value: variable.equals_value.clone(),
                }
            }
        };

        equation
    }
}
