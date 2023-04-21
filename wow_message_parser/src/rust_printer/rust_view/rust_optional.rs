use crate::rust_printer::rust_view::rust_member::RustMember;

#[derive(Debug, Clone)]
pub(crate) struct RustOptional {
    name: String,
    ty: String,
    members: Vec<RustMember>,
}

impl RustOptional {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn ty(&self) -> &str {
        &self.ty
    }

    pub(crate) fn members(&self) -> &[RustMember] {
        &self.members
    }

    pub(crate) fn members_in_struct(&self) -> Vec<&RustMember> {
        self.members.iter().filter(|m| m.in_rust_type).collect()
    }

    pub(crate) fn all_members(&self) -> Vec<&RustMember> {
        let mut v = Vec::new();

        for m in self.members() {
            v.append(&mut m.all_members());
        }

        v
    }
    pub fn new(name: String, ty: String, members: Vec<RustMember>) -> Self {
        Self { name, ty, members }
    }
}
