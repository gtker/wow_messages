use crate::parser::types::struct_member::StructMember;

#[derive(Debug, Clone, Eq)]
pub(crate) struct OptionalStatement {
    name: String,
    members: Vec<StructMember>,
}

impl PartialEq for OptionalStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl OptionalStatement {
    pub(crate) fn new(name: String, members: Vec<StructMember>) -> Self {
        Self { name, members }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn members(&self) -> &[StructMember] {
        &self.members
    }
}
