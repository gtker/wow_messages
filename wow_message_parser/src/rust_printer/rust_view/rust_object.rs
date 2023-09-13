use crate::parser::types::sizes::Sizes;
use crate::rust_printer::rust_view::rust_definer::RustDefiner;
use crate::rust_printer::rust_view::rust_member::RustMember;
use crate::rust_printer::rust_view::rust_optional::RustOptional;
use crate::rust_printer::rust_view::rust_type::RustType;
use crate::rust_printer::DefinerType;

#[derive(Debug, Clone)]
pub(crate) struct RustObject {
    name: String,
    members: Vec<RustMember>,
    optional: Option<RustOptional>,
    sizes: Sizes,
}

impl RustObject {
    pub(crate) fn all_members(&self) -> Vec<&RustMember> {
        let mut v = Vec::new();

        for m in self.members() {
            v.append(&mut m.all_members());
        }

        if let Some(optional) = self.optional() {
            v.append(&mut optional.all_members());
        }

        v
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn members(&self) -> &[RustMember] {
        &self.members
    }
    pub(crate) fn members_in_struct(&self) -> impl Iterator<Item = &RustMember> {
        self.members.iter().filter(|a| a.in_rust_type)
    }
    pub(crate) fn optional(&self) -> Option<&RustOptional> {
        self.optional.as_ref()
    }
    pub(crate) fn constant_sized(&self) -> bool {
        self.sizes().is_constant().is_some()
    }
    pub(crate) fn sizes(&self) -> Sizes {
        self.sizes
    }

    fn get_rust_definer_from_ty(m: &RustMember) -> Option<RustDefiner> {
        let (
            ty_name,
            original_ty_name,
            enumerators,
            int_ty,
            is_simple,
            definer_type,
            is_elseif,
            has_separate_if_statements,
        ) = match m.ty() {
            RustType::Enum {
                ty_name,
                original_ty_name,
                enumerators,
                int_ty,
                is_simple,
                is_elseif,
                separate_if_statements,
            } => (
                ty_name,
                original_ty_name,
                enumerators,
                int_ty,
                is_simple,
                DefinerType::Enum,
                is_elseif,
                separate_if_statements,
            ),
            RustType::Flag {
                ty_name,
                original_ty_name,
                enumerators,
                int_ty,
                is_simple,
                is_elseif,
            } => (
                ty_name,
                original_ty_name,
                enumerators,
                int_ty,
                is_simple,
                DefinerType::Flag,
                is_elseif,
                &false,
            ),
            _ => return None,
        };

        Some(RustDefiner::new(
            m,
            definer_type,
            enumerators.as_slice(),
            ty_name.as_str(),
            *int_ty,
            *is_simple,
            *is_elseif,
            original_ty_name.as_str(),
            *has_separate_if_statements,
        ))
    }

    pub(crate) fn rust_definers_in_global_scope(&self) -> Vec<RustDefiner> {
        let mut v = Vec::new();

        for m in self.members_in_struct() {
            if let Some(rd) = Self::get_rust_definer_from_ty(m) {
                v.push(rd);
            }
        }

        v
    }

    pub(crate) fn rust_definers_in_enumerator(&self, enumerator_name: &str) -> Vec<RustDefiner> {
        let mut v = Vec::new();

        fn inner<'a>(m: &'a RustMember, enumerator_name: &str, v: &mut Vec<RustDefiner<'a>>) {
            if let Some(rd) = RustObject::get_rust_definer_from_ty(m) {
                for enumerator in rd.enumerators() {
                    if enumerator.name() == enumerator_name {
                        if enumerator.contains_elseif() {
                            match enumerator.members()[0].ty() {
                                RustType::Enum { enumerators, .. } => {
                                    for enumerator in enumerators {
                                        if enumerator.name() == enumerator_name {
                                            for m in enumerator.members_in_struct() {
                                                if let Some(rd) =
                                                    RustObject::get_rust_definer_from_ty(m)
                                                {
                                                    v.push(rd);
                                                } else {
                                                    inner(m, enumerator_name, v);
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            for m in enumerator.members_in_struct() {
                                if let Some(rd) = RustObject::get_rust_definer_from_ty(m) {
                                    v.push(rd);
                                } else {
                                    inner(m, enumerator_name, v);
                                }
                            }
                        }
                    } else {
                        for m in enumerator.members_in_struct() {
                            if RustObject::get_rust_definer_from_ty(m).is_some() {
                                inner(m, enumerator_name, v);
                            }
                        }
                    }
                }
            }
        }

        for m in self.members_in_struct() {
            inner(m, enumerator_name, &mut v);
        }

        v
    }

    pub(crate) fn rust_definers_in_namespace(&self, ty_name: &str) -> Vec<RustDefiner> {
        let rd = self.get_rust_definer(ty_name);

        let mut v = Vec::new();

        for enumerator in rd.enumerators() {
            for m in enumerator.members_in_struct() {
                if let Some(rd) = Self::get_rust_definer_from_ty(m) {
                    v.push(rd);
                }
            }
        }

        v
    }

    pub(crate) fn get_rust_definers(&self) -> Vec<RustDefiner> {
        fn inner<'a>(m: &'a RustMember, v: &mut Vec<RustDefiner<'a>>) {
            if let Some(rd) = RustObject::get_rust_definer_from_ty(m) {
                for enumerator in rd.enumerators() {
                    for m in enumerator.members_in_struct() {
                        inner(m, v);
                    }
                }

                if !rd.is_simple() && !v.iter().any(|a| a.ty_name() == rd.ty_name()) {
                    v.push(rd);
                }
            }
        }

        let mut v = Vec::new();

        for m in self.members_in_struct() {
            inner(m, &mut v);
        }

        v
    }

    pub(crate) fn rust_definer_with_variable_name_and_enumerator<'a>(
        &'a self,
        variable_name: &str,
        enumerator_name: &str,
    ) -> RustDefiner<'a> {
        fn inner<'a>(
            m: &'a RustMember,
            variable_name: &str,
            enumerator_name: &str,
        ) -> Option<RustDefiner<'a>> {
            if let Some(ird) = RustObject::get_rust_definer_from_ty(m) {
                for enumerator in ird.enumerators() {
                    if enumerator.contains_elseif() {
                        match enumerator.members()[0].ty() {
                            RustType::Enum { enumerators, .. } => {
                                for enumerator in enumerators {
                                    for m in enumerator.members() {
                                        if let Some(rd) = inner(m, variable_name, enumerator_name) {
                                            return Some(rd);
                                        }
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }

                    for m in enumerator.members() {
                        if let Some(rd) = inner(m, variable_name, enumerator_name) {
                            return Some(rd);
                        }
                    }
                }

                if ird.variable_name() == variable_name && ird.contains_enumerator(enumerator_name)
                {
                    return Some(ird);
                }
            }
            None
        }

        for m in self.members() {
            if let Some(rd) = inner(m, variable_name, enumerator_name) {
                return rd;
            }
        }

        unreachable!()
    }

    pub(crate) fn get_rust_definer(&self, name: &str) -> RustDefiner {
        let member = self.get_complex_definer_ty(name);

        Self::get_rust_definer_from_ty(member).unwrap()
    }

    pub(crate) fn get_complex_definer_ty(&self, name: &str) -> &RustMember {
        fn inner<'a>(m: &'a RustMember, name: &str) -> Option<&'a RustMember> {
            match m.ty() {
                RustType::Enum {
                    ty_name,
                    enumerators,
                    ..
                }
                | RustType::Flag {
                    ty_name,
                    enumerators,
                    ..
                } => {
                    if ty_name == name {
                        return Some(m);
                    }

                    for e in enumerators {
                        for m in e.members() {
                            if let Some(m) = inner(m, name) {
                                return Some(m);
                            }
                        }
                    }

                    None
                }
                _ => None,
            }
        }

        for m in self.members() {
            if let Some(m) = inner(m, name) {
                return m;
            }
        }

        if let Some(optional) = self.optional() {
            for m in optional.members() {
                if let Some(m) = inner(m, name) {
                    return m;
                }
            }
        }

        unreachable!()
    }
    pub fn new(
        name: String,
        members: Vec<RustMember>,
        optional: Option<RustOptional>,
        sizes: Sizes,
    ) -> Self {
        Self {
            name,
            members,
            optional,
            sizes,
        }
    }
}
