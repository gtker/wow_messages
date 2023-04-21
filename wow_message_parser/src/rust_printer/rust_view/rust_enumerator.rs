use crate::parser::types::definer::DefinerValue;
use crate::parser::types::struct_member::StructMember;
use crate::rust_printer::rust_view::rust_member::RustMember;

#[derive(Debug, Clone)]
pub(crate) struct RustEnumerator {
    name: String,
    rust_name: String,
    value: DefinerValue,
    pub members: Vec<RustMember>,
    pub is_main_enumerator: bool,
    pub original_fields: Vec<StructMember>,
    pub contains_elseif: bool,
}

impl RustEnumerator {
    pub(crate) fn all_members(&self) -> Vec<&RustMember> {
        let mut v = Vec::new();

        for m in self.members() {
            v.append(&mut m.all_members());
        }

        v
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn rust_name(&self) -> &str {
        &self.rust_name
    }
    pub(crate) fn value(&self) -> &DefinerValue {
        &self.value
    }
    pub(crate) fn members(&self) -> &[RustMember] {
        &self.members
    }
    pub(crate) fn members_in_struct(&self) -> Vec<&RustMember> {
        self.members.iter().filter(|m| m.in_rust_type).collect()
    }
    pub(crate) fn contains_elseif(&self) -> bool {
        self.contains_elseif
    }

    pub(crate) fn should_not_be_in_flag_types(&self) -> bool {
        self.members.is_empty() || !self.is_main_enumerator
    }

    pub(crate) fn has_members(&self) -> bool {
        !self.members().is_empty()
    }

    pub(crate) fn has_members_in_struct(&self) -> bool {
        !self.members_in_struct().is_empty()
    }
    pub(crate) fn original_fields(&self) -> &[StructMember] {
        &self.original_fields
    }
    pub fn new(
        name: String,
        rust_name: String,
        value: DefinerValue,
        members: Vec<RustMember>,
        is_main_enumerator: bool,
        original_fields: Vec<StructMember>,
        contains_elseif: bool,
    ) -> Self {
        Self {
            name,
            rust_name,
            value,
            members,
            is_main_enumerator,
            original_fields,
            contains_elseif,
        }
    }
}
