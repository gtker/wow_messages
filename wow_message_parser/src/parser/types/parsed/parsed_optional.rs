use crate::parser::types::parsed::parsed_struct_member::ParsedStructMember;

#[derive(Debug, Clone, Eq)]
pub(crate) struct ParsedOptionalStatement {
    pub name: String,
    pub members: Vec<ParsedStructMember>,
}

impl PartialEq for ParsedOptionalStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl ParsedOptionalStatement {
    pub(crate) fn new(name: &str, members: Vec<ParsedStructMember>) -> Self {
        Self {
            name: name.to_string(),
            members,
        }
    }

    pub(crate) fn members(&self) -> &[ParsedStructMember] {
        &self.members
    }

    pub(crate) fn members_mut(&mut self) -> &mut [ParsedStructMember] {
        &mut self.members
    }
}
