use crate::parser::types::if_statement::IfStatement;
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::ty::Type;
use crate::parser::types::VerifiedContainerValue;
use crate::Tags;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StructMember {
    Definition(StructMemberDefinition),
    IfStatement(IfStatement),
    OptionalStatement(OptionalStatement),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructMemberDefinition {
    name: String,
    struct_type: Type,
    value: Option<VerifiedContainerValue>,
    pub used_as_size_in: Option<String>,
    used_in_if: bool,
    tags: Tags,
}

impl StructMemberDefinition {
    pub(crate) fn struct_type(&self) -> Type {
        self.struct_type.clone()
    }

    pub(crate) fn used_as_size_in(&self) -> &Option<String> {
        &self.used_as_size_in
    }

    pub(crate) fn used_in_if(&self) -> bool {
        self.used_in_if
    }

    pub(crate) fn new(
        name: String,
        struct_type: Type,
        verified_value: Option<VerifiedContainerValue>,
        used_as_size_in: Option<String>,
        used_in_if: bool,

        tags: Tags,
    ) -> Self {
        Self {
            name,
            struct_type,
            value: verified_value,
            used_as_size_in,
            used_in_if,
            tags,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn ty(&self) -> &Type {
        &self.struct_type
    }

    pub(crate) fn value(&self) -> &Option<VerifiedContainerValue> {
        &self.value
    }

    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }
}
