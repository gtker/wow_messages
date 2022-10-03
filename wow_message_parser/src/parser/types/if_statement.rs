use crate::parser::types::struct_member::StructMember;
use crate::parser::types::ty::Type;
use crate::rust_printer::field_name_to_rust_name;
use crate::DefinerType;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) enum DefinerUsage {
    Unused,
    NotInIf,
    InIf,
}

#[derive(Debug, Clone)]
pub(crate) struct IfStatement {
    conditional: Conditional,
    members: Vec<StructMember>,
    else_ifs: Vec<IfStatement>,
    else_statement_members: Vec<StructMember>,
    original_ty: Type,
}

impl Eq for IfStatement {}

impl PartialEq for IfStatement {
    fn eq(&self, other: &Self) -> bool {
        self.members.first().unwrap() == other.members.first().unwrap()
    }
}

impl IfStatement {
    pub(crate) fn new(
        conditional: Conditional,
        members: Vec<StructMember>,
        else_ifs: Vec<IfStatement>,
        else_statement_members: Vec<StructMember>,
        original_ty: Type,
    ) -> Self {
        Self {
            conditional,
            members,
            else_ifs,
            else_statement_members,
            original_ty,
        }
    }

    pub(crate) fn is_not_enum(&self) -> bool {
        matches!(self.conditional.equations[0], Equation::NotEquals { .. })
    }

    pub(crate) fn flag_get_enumerator(&self) -> String {
        assert_eq!(self.conditional().equations.len(), 1);

        match &self.conditional().equations[0] {
            Equation::BitwiseAnd { value } => value.to_string(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn flag_get_enumerator_rust_name(&self) -> String {
        field_name_to_rust_name(&self.flag_get_enumerator())
    }

    pub(crate) fn is_elseif_flag(&self) -> bool {
        match self.conditional.equations[0] {
            Equation::BitwiseAnd { .. } => !self.else_ifs().is_empty(),
            _ => false,
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

    pub(crate) fn name(&self) -> &str {
        &self.conditional.variable_name
    }

    pub(crate) fn definer_type(&self) -> DefinerType {
        match self.conditional.equations[0] {
            Equation::Equals { .. } => DefinerType::Enum,
            Equation::BitwiseAnd { .. } => DefinerType::Flag,
            Equation::NotEquals { .. } => DefinerType::Enum,
        }
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

    pub(crate) fn conditional(&self) -> &Conditional {
        &self.conditional
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
            _ => panic!("invalid operator {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Conditional {
    variable_name: String,
    equations: Vec<Equation>,
}

impl Conditional {
    pub(crate) fn variable_name(&self) -> &str {
        &self.variable_name
    }

    pub(crate) fn equations(&self) -> &[Equation] {
        &self.equations
    }

    pub(crate) fn new(conditions: &[Condition]) -> Self {
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
pub(crate) enum Equation {
    Equals { value: String },
    NotEquals { value: String },
    BitwiseAnd { value: String },
}

#[derive(Debug, Clone)]
pub(crate) struct Condition {
    pub value: String,
    pub operator: Operator,
    pub equals_value: String,
}

impl Condition {
    pub(crate) fn new(value: &str, equals_value: &str, operator: Operator) -> Self {
        Self {
            value: value.to_string(),
            operator,
            equals_value: equals_value.to_string(),
        }
    }
}
