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

impl StructMember {
    pub(crate) fn is_manual_size_field(&self) -> bool {
        match self {
            StructMember::Definition(d) => d.is_manual_size_field(),
            StructMember::IfStatement(_) | StructMember::OptionalStatement(_) => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct StructMemberDefinition {
    name: String,
    struct_type: Type,
    value: Option<ContainerValue>,
    used_as_size_in: Option<String>,
    is_manual_size_field: bool,
    used_in_if: bool,
    tags: MemberTags,
}

impl StructMemberDefinition {
    pub(crate) fn used_as_size_in(&self) -> &Option<String> {
        &self.used_as_size_in
    }

    pub(crate) fn used_in_if(&self) -> bool {
        self.used_in_if
    }

    pub(crate) fn new(
        name: String,
        struct_type: Type,
        value: Option<ContainerValue>,
        used_as_size_in: Option<String>,
        used_in_if: bool,
        is_manual_size_field: bool,
        tags: MemberTags,
    ) -> Self {
        Self {
            name,
            struct_type,
            value,
            used_as_size_in,
            is_manual_size_field,
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

    pub(crate) fn is_manual_size_field(&self) -> bool {
        self.is_manual_size_field
    }

    pub(crate) fn tags(&self) -> &MemberTags {
        &self.tags
    }
}
