use crate::parser::types::if_statement::{Conditional, Equation};
use crate::parser::types::parsed::parsed_struct_member::ParsedStructMember;
use crate::parser::types::ty::Type;
use crate::rust_printer::field_name_to_rust_name;
use crate::DefinerType;

#[derive(Debug, Clone)]
pub struct ParsedIfStatement {
    pub conditional: Conditional,
    pub members: Vec<ParsedStructMember>,
    pub else_ifs: Vec<ParsedIfStatement>,
    pub else_statement_members: Vec<ParsedStructMember>,
    pub original_ty: Option<Type>,
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

    pub(crate) fn is_not_enum(&self) -> bool {
        matches!(self.conditional.equations()[0], Equation::NotEquals { .. })
    }

    pub(crate) fn flag_get_enumerator(&self) -> String {
        assert_eq!(self.get_conditional().equations().len(), 1);

        match &self.get_conditional().equations()[0] {
            Equation::BitwiseAnd { value } => value.to_string(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn flag_get_enumerator_rust_name(&self) -> String {
        field_name_to_rust_name(&self.flag_get_enumerator())
    }

    pub(crate) fn is_elseif_flag(&self) -> bool {
        match self.conditional.equations()[0] {
            Equation::BitwiseAnd { .. } => !self.else_ifs().is_empty(),
            _ => false,
        }
    }

    pub(crate) fn members(&self) -> &[ParsedStructMember] {
        &self.members
    }

    pub(crate) fn else_members(&self) -> &[ParsedStructMember] {
        &self.else_statement_members
    }

    pub(crate) fn original_ty(&self) -> &Type {
        self.original_ty.as_ref().unwrap()
    }

    pub(crate) fn name(&self) -> &str {
        &self.conditional.variable_name()
    }

    pub(crate) fn set_original_ty(&mut self, original_ty: Type) {
        self.original_ty = Some(original_ty)
    }

    pub(crate) fn definer_type(&self) -> DefinerType {
        match self.conditional.equations()[0] {
            Equation::Equals { .. } => DefinerType::Enum,
            Equation::BitwiseAnd { .. } => DefinerType::Flag,
            Equation::NotEquals { .. } => DefinerType::Enum,
        }
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

    pub(crate) fn get_conditional(&self) -> &Conditional {
        &self.conditional
    }
}
