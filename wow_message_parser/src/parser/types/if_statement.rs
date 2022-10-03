use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::rust_printer::field_name_to_rust_name;
use crate::DefinerType;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DefinerUsage {
    Unused,
    NotInIf,
    InIf,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub conditional: Conditional,
    members: Vec<StructMember>,
    else_ifs: Vec<IfStatement>,
    else_statement_members: Vec<StructMember>,
    original_ty: Option<Type>,
}

impl Eq for IfStatement {}

impl PartialEq for IfStatement {
    fn eq(&self, other: &Self) -> bool {
        self.members.first().unwrap() == other.members.first().unwrap()
    }
}

impl IfStatement {
    pub fn new(
        conditional: Conditional,
        members: Vec<StructMember>,
        else_ifs: Vec<IfStatement>,
        else_statement_members: Vec<StructMember>,
    ) -> Self {
        Self {
            conditional,
            members,
            else_ifs,
            else_statement_members,
            original_ty: None,
        }
    }

    pub fn is_not_enum(&self) -> bool {
        matches!(self.conditional.equations[0], Equation::NotEquals { .. })
    }

    pub fn flag_get_enumerator(&self) -> String {
        assert_eq!(self.get_conditional().equations.len(), 1);

        match &self.get_conditional().equations[0] {
            Equation::BitwiseAnd { value } => value.to_string(),
            _ => unreachable!(),
        }
    }

    pub fn flag_get_enumerator_rust_name(&self) -> String {
        field_name_to_rust_name(&self.flag_get_enumerator())
    }

    pub fn is_elseif_flag(&self) -> bool {
        match self.conditional.equations[0] {
            Equation::BitwiseAnd { .. } => !self.else_ifs().is_empty(),
            _ => false,
        }
    }

    pub fn members(&self) -> &[StructMember] {
        &self.members
    }

    pub fn members_mut(&mut self) -> &mut [StructMember] {
        &mut self.members
    }

    pub fn member_enumerators(&self) -> Vec<&str> {
        let mut v = Vec::new();

        for eq in &self.conditional.equations {
            match eq {
                Equation::BitwiseAnd { value }
                | Equation::Equals { value }
                | Equation::NotEquals { value } => {
                    v.push(value.as_str());
                }
            }
        }

        v
    }

    pub fn else_members(&self) -> &[StructMember] {
        &self.else_statement_members
    }

    pub fn else_members_mut(&mut self) -> &mut [StructMember] {
        &mut self.else_statement_members
    }

    pub fn original_ty(&self) -> &Type {
        self.original_ty.as_ref().unwrap()
    }

    pub fn name(&self) -> &str {
        &self.conditional.variable_name
    }

    pub fn set_original_ty(&mut self, original_ty: Type) {
        self.original_ty = Some(original_ty)
    }

    pub fn definer_type(&self) -> DefinerType {
        match self.conditional.equations[0] {
            Equation::Equals { .. } => DefinerType::Enum,
            Equation::BitwiseAnd { .. } => DefinerType::Flag,
            Equation::NotEquals { .. } => DefinerType::Enum,
        }
    }

    pub fn else_ifs_mut(&mut self) -> &mut [IfStatement] {
        &mut self.else_ifs
    }

    pub fn else_ifs(&self) -> &[IfStatement] {
        &self.else_ifs
    }

    pub fn all_members_mut(&mut self) -> impl Iterator<Item = &mut StructMember> + '_ {
        self.members
            .iter_mut()
            .chain(self.else_ifs.iter_mut().flat_map(|a| a.members.iter_mut()))
            .chain(self.else_statement_members.iter_mut())
    }

    pub fn all_members(&self) -> impl Iterator<Item = &StructMember> {
        let else_ifs = self.else_ifs.iter().flat_map(|a| a.members());
        self.members()
            .iter()
            .chain(else_ifs)
            .chain(&self.else_statement_members)
    }

    pub fn get_conditional(&self) -> &Conditional {
        &self.conditional
    }

    pub fn get_variable_names_for_members(&self) -> Vec<&StructMemberDefinition> {
        let mut v = Vec::new();

        for m in &self.members {
            match m {
                StructMember::Definition(d) => {
                    v.push(d);
                }
                StructMember::IfStatement(_) => {}
                StructMember::OptionalStatement(_) => {}
            }
        }

        v
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
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
            _ => panic!("invalid operator {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Conditional {
    variable_name: String,
    equations: Vec<Equation>,
}

impl Conditional {
    pub fn variable_name(&self) -> &str {
        &self.variable_name
    }

    pub fn equations(&self) -> &[Equation] {
        &self.equations
    }

    pub fn new(conditions: &[Condition]) -> Self {
        let variable_name = conditions[0].value.to_string();

        let mut equations = Vec::new();
        for c in conditions {
            if c.value != variable_name {
                panic!(
                    "matching variable in if statement '||' is not the same, '{}' and '{}'",
                    variable_name, c.value
                );
            }

            let v = match c.operator {
                Operator::Equals => Equation::Equals {
                    value: c.equals_value.to_owned(),
                },
                Operator::BitwiseAnd => Equation::BitwiseAnd {
                    value: c.equals_value.to_owned(),
                },
                Operator::NotEquals => Equation::NotEquals {
                    value: c.equals_value.to_owned(),
                },
            };
            equations.push(v);
        }

        Self {
            variable_name,
            equations,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Equation {
    Equals { value: String },
    NotEquals { value: String },
    BitwiseAnd { value: String },
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub value: String,
    pub operator: Operator,
    pub equals_value: String,
}

impl Condition {
    pub fn new(value: &str, equals_value: &str, operator: Operator) -> Self {
        Self {
            value: value.to_string(),
            operator,
            equals_value: equals_value.to_string(),
        }
    }
}
