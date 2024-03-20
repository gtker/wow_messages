use crate::parser::types::if_statement::{Conditional, Equation, Operator};
use crate::parser::types::parsed::parsed_struct_member::ParsedStructMember;
use crate::parser::types::parsed::parsed_ty::ParsedType;

#[derive(Debug, Clone)]
pub(crate) struct ParsedIfStatement {
    pub conditional: Conditional,
    pub members: Vec<ParsedStructMember>,
    pub else_ifs: Vec<ParsedIfStatement>,
    pub else_statement_members: Vec<ParsedStructMember>,
    pub original_ty: Option<ParsedType>,
}

impl Eq for ParsedIfStatement {}

impl PartialEq for ParsedIfStatement {
    fn eq(&self, other: &Self) -> bool {
        self.members.first().unwrap() == other.members.first().unwrap()
    }
}

impl ParsedIfStatement {
    pub(crate) fn new(
        conditional: Conditional,
        members: Vec<ParsedStructMember>,
        else_ifs: Vec<ParsedIfStatement>,
        else_statement_members: Vec<ParsedStructMember>,
    ) -> Self {
        Self {
            conditional,
            members,
            else_ifs,
            else_statement_members,
            original_ty: None,
        }
    }

    pub(crate) fn members(&self) -> &[ParsedStructMember] {
        &self.members
    }

    pub(crate) fn else_members(&self) -> &[ParsedStructMember] {
        &self.else_statement_members
    }

    pub(crate) fn variable_name(&self) -> &str {
        &self.conditional.variable_name
    }

    pub(crate) fn set_original_ty(&mut self, original_ty: ParsedType) {
        self.original_ty = Some(original_ty)
    }

    pub(crate) fn else_ifs_mut(&mut self) -> &mut [ParsedIfStatement] {
        &mut self.else_ifs
    }

    pub(crate) fn else_ifs(&self) -> &[ParsedIfStatement] {
        &self.else_ifs
    }

    pub(crate) fn all_members_mut(&mut self) -> impl Iterator<Item = &mut ParsedStructMember> + '_ {
        self.members
            .iter_mut()
            .chain(self.else_ifs.iter_mut().flat_map(|a| a.members.iter_mut()))
            .chain(self.else_statement_members.iter_mut())
    }

    pub(crate) fn all_members(&self) -> impl Iterator<Item = &ParsedStructMember> {
        let else_ifs = self.else_ifs.iter().flat_map(|a| a.members());
        self.members()
            .iter()
            .chain(else_ifs)
            .chain(&self.else_statement_members)
    }

    pub(crate) fn equation(&self) -> &Equation {
        &self.conditional.equation
    }
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
