use crate::parser::types::struct_member::StructMember;
use crate::Tags;

#[derive(Debug, Clone, Eq)]
pub struct OptionalStatement {
    name: String,
    members: Vec<StructMember>,
    tags: Tags,
}

impl PartialEq for OptionalStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl OptionalStatement {
    pub fn new(name: &str, members: Vec<StructMember>) -> Self {
        Self {
            name: name.to_string(),
            members,
            tags: Tags::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn members(&self) -> &[StructMember] {
        &self.members
    }

    pub fn members_mut(&mut self) -> &mut [StructMember] {
        &mut self.members
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }
}
