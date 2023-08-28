use crate::parser::types::if_statement::IfStatement;
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::tags::MemberTags;
use crate::parser::types::ty::Type;
use crate::parser::types::{ContainerValue, IntegerType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum StructMember {
    Definition(StructMemberDefinition),
    IfStatement(IfStatement),
    OptionalStatement(OptionalStatement),
}

impl StructMember {
    pub(crate) fn size_of_fields_before_size(&self) -> Option<i128> {
        match self {
            StructMember::Definition(d) => d.size_of_fields_before_size(),
            StructMember::IfStatement(_) | StructMember::OptionalStatement(_) => None,
        }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub(crate) struct StructMemberDefinition {
    name: String,
    struct_type: Type,
    value: Option<ContainerValue>,
    used_as_size_in: Option<String>,
    is_manual_size_field: Option<i128>,
    used_in_if: bool,
    tags: MemberTags,
}

impl StructMemberDefinition {
    pub(crate) fn compressed_definition(&self) -> Option<StructMemberDefinition> {
        match self.ty() {
            Type::Array(array) => {
                if array.compressed() {
                    Some(StructMemberDefinition {
                        name: format!("{}_decompressed_size", self.name()),
                        struct_type: Type::Integer(IntegerType::U32),
                        value: None,
                        used_as_size_in: None,
                        is_manual_size_field: None,
                        used_in_if: false,
                        tags: Default::default(),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
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
        value: Option<ContainerValue>,
        used_as_size_in: Option<String>,
        used_in_if: bool,
        is_manual_size_field: Option<i128>,
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

    pub(crate) fn size_of_fields_before_size(&self) -> Option<i128> {
        self.is_manual_size_field
    }

    pub(crate) fn is_manual_size_field(&self) -> bool {
        self.is_manual_size_field.is_some()
    }

    pub(crate) fn tags(&self) -> &MemberTags {
        &self.tags
    }
}
