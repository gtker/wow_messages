use crate::parser::types::struct_member::StructMember;
use crate::rust_printer::rust_view::rust_object::RustObject;

#[derive(Debug, Clone)]
pub(crate) struct ConditionalUnion {
    selector_name: String,
    selector_type_name: String,
    condition_flags: Vec<String>,
    full_when_condition: bool,
    common_members: Vec<StructMember>,
    linear_members: Vec<StructMember>,
    full_members: Vec<StructMember>,
    linear: ConditionalUnionBranch,
    full: ConditionalUnionBranch,
}

#[derive(Debug, Clone)]
pub(crate) struct ConditionalUnionBranch {
    variant_name: &'static str,
    record_name: String,
    object: RustObject,
}

impl ConditionalUnion {
    pub(crate) fn new(
        selector_name: String,
        selector_type_name: String,
        condition_flags: Vec<String>,
        full_when_condition: bool,
        common_members: Vec<StructMember>,
        linear_members: Vec<StructMember>,
        full_members: Vec<StructMember>,
        linear_record_name: String,
        linear_object: RustObject,
        full_record_name: String,
        full_object: RustObject,
    ) -> Self {
        Self {
            selector_name,
            selector_type_name,
            condition_flags,
            full_when_condition,
            common_members,
            linear_members,
            full_members,
            linear: ConditionalUnionBranch {
                variant_name: "Linear",
                record_name: linear_record_name,
                object: linear_object,
            },
            full: ConditionalUnionBranch {
                variant_name: "Full",
                record_name: full_record_name,
                object: full_object,
            },
        }
    }

    pub(crate) fn selector_name(&self) -> &str {
        &self.selector_name
    }

    pub(crate) fn selector_type_name(&self) -> &str {
        &self.selector_type_name
    }

    pub(crate) fn condition_flags(&self) -> &[String] {
        &self.condition_flags
    }

    pub(crate) fn full_condition_expression(&self, selector: &str) -> String {
        self.condition_flags
            .iter()
            .map(|flag| {
                format!(
                    "({selector}.as_int() & {}::{flag}) != 0",
                    self.selector_type_name
                )
            })
            .collect::<Vec<_>>()
            .join(" || ")
    }

    pub(crate) fn full_when_condition(&self) -> bool {
        self.full_when_condition
    }

    pub(crate) fn common_members(&self) -> &[StructMember] {
        &self.common_members
    }

    pub(crate) fn linear_members(&self) -> &[StructMember] {
        &self.linear_members
    }

    pub(crate) fn full_members(&self) -> &[StructMember] {
        &self.full_members
    }

    pub(crate) fn linear(&self) -> &ConditionalUnionBranch {
        &self.linear
    }

    pub(crate) fn full(&self) -> &ConditionalUnionBranch {
        &self.full
    }
}

impl ConditionalUnionBranch {
    pub(crate) fn variant_name(&self) -> &str {
        self.variant_name
    }

    pub(crate) fn record_name(&self) -> &str {
        &self.record_name
    }

    pub(crate) fn object(&self) -> &RustObject {
        &self.object
    }
}
