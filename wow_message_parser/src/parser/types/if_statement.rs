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
