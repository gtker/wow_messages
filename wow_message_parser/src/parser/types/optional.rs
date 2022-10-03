use crate::parser::types::struct_member::StructMember;
use crate::Tags;

#[derive(Debug, Clone, Eq)]
pub(crate) struct OptionalStatement {
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
    pub(crate) fn new(name: String, members: Vec<StructMember>, tags: Tags) -> Self {
        Self {
            name,
            members,
            tags,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn members(&self) -> &[StructMember] {
        &self.members
    }

    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }
}
