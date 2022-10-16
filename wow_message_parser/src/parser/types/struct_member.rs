use crate::parser::types::if_statement::IfStatement;
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::tags::MemberTags;
use crate::parser::types::ty::Type;
use crate::parser::types::ContainerValue;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum StructMember {
    Definition(StructMemberDefinition),
    IfStatement(IfStatement),
    OptionalStatement(OptionalStatement),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct StructMemberDefinition {
    name: String,
    struct_type: Type,
    value: Option<ContainerValue>,
    used_as_size_in: Option<String>,
    used_in_if: bool,
    tags: MemberTags,
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
        verified_value: Option<ContainerValue>,
        used_as_size_in: Option<String>,
        used_in_if: bool,

        tags: MemberTags,
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

    pub(crate) fn value(&self) -> &Option<ContainerValue> {
        &self.value
    }

    pub(crate) fn tags(&self) -> &MemberTags {
        &self.tags
    }
}
