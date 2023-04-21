use crate::parser::types::array::ArrayType;
use crate::parser::types::struct_member::StructMember;
use crate::parser::types::tags::MemberTags;
use crate::rust_printer::rust_view::rust_enumerator::RustEnumerator;
use crate::rust_printer::rust_view::rust_type::RustType;

#[derive(Debug, Clone)]
pub(crate) struct RustMember {
    name: String,
    pub ty: RustType,
    pub original_ty: String,

    pub in_rust_type: bool,

    tags: MemberTags,
}

impl RustMember {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn ty(&self) -> &RustType {
        &self.ty
    }
    pub(crate) fn tags(&self) -> &MemberTags {
        &self.tags
    }
    pub(crate) fn all_members(&self) -> Vec<&RustMember> {
        let mut v = self.all_members_without_self();
        v.push(self);
        v
    }

    pub(crate) fn all_members_without_self(&self) -> Vec<&RustMember> {
        let mut v = Vec::new();

        match self.ty() {
            RustType::Enum { enumerators, .. } | RustType::Flag { enumerators, .. } => {
                for enumerator in enumerators {
                    v.append(&mut enumerator.all_members());
                }
            }
            RustType::Struct { object, .. } => {
                v.append(&mut object.all_members());
            }
            RustType::Array { array, .. } => match array.ty() {
                ArrayType::Struct(c) => {
                    v.append(&mut c.rust_object().all_members());
                }
                ArrayType::Integer(_)
                | ArrayType::CString
                | ArrayType::Guid
                | ArrayType::PackedGuid => {}
            },
            _ => {}
        }

        v
    }

    pub(crate) fn pop_flag_enumerator(&mut self, enumerator: &str) -> RustEnumerator {
        match &mut self.ty {
            RustType::Flag { enumerators, .. } => {
                let (index, _) = enumerators
                    .iter()
                    .enumerate()
                    .find(|a| a.1.name() == enumerator)
                    .expect(enumerator);
                let enumerator = enumerators[index].clone();
                enumerators.remove(index);
                enumerator
            }
            _ => unreachable!("pop_flag_enumerator is not flag"),
        }
    }

    pub(crate) fn size_comment(&self) -> String {
        format!(" // {}: {}", self.name(), self.ty().str())
    }

    pub(crate) fn struct_initialization_string(&self) -> String {
        let is_if = match self.ty() {
            RustType::Enum { is_simple, .. } => *is_simple,
            _ => true,
        };
        if is_if {
            format!("{},", self.name())
        } else {
            format!("{name}: {name}_if,", name = self.name())
        }
    }

    pub(crate) fn set_main_enumerators(&mut self, enumerator_names: &[&String]) {
        let enums = match &mut self.ty {
            RustType::Enum { enumerators, .. } | RustType::Flag { enumerators, .. } => enumerators,
            _ => unreachable!(),
        };

        for &name in enumerator_names {
            let e = enums
                .iter_mut()
                .find(|a| a.name() == name)
                .unwrap_or_else(|| {
                    unreachable!(
                        "Should be caught when turning ParsedIfStatements into IfStatements"
                    )
                });
            e.is_main_enumerator = true;
        }
    }

    pub(crate) fn append_members_to_enumerator_not_equal_range(
        &mut self,
        enumerator_name: &[&String],
        members: &[RustMember],
        original_fields: &[StructMember],
    ) {
        let enums = match &mut self.ty {
            RustType::Enum {
                is_simple,
                enumerators,
                ..
            }
            | RustType::Flag {
                is_simple,
                enumerators,
                ..
            } => {
                *is_simple = false;
                enumerators
            }
            _ => unreachable!(),
        };

        let enums = enums.iter_mut().filter(|a| {
            let mut equal = false;
            for &name in enumerator_name {
                if a.name() == name {
                    equal = true;
                }
            }

            !equal
        });
        for e in enums {
            for f in original_fields {
                if !e.original_fields().contains(f) {
                    e.original_fields.push(f.clone());
                }
            }
            e.members.append(&mut members.to_vec());
            e.members.sort_by(|a, b| a.name.cmp(&b.name));
            e.members.dedup_by(|a, b| a.name.eq(&b.name));
        }
    }

    pub(crate) fn append_members_to_enumerator_not_equal(
        &mut self,
        enumerator_name: &str,
        members: &[RustMember],
        original_fields: &[StructMember],
    ) {
        let enums = match &mut self.ty {
            RustType::Enum {
                is_simple,
                enumerators,
                ..
            }
            | RustType::Flag {
                is_simple,
                enumerators,
                ..
            } => {
                *is_simple = false;
                enumerators
            }
            _ => unreachable!(),
        };

        let enums = enums.iter_mut().filter(|a| a.name() != enumerator_name);
        for e in enums {
            for f in original_fields {
                if !e.original_fields().contains(f) {
                    e.original_fields.push(f.clone());
                }
            }
            e.members.append(&mut members.to_vec());
            e.members.sort_by(|a, b| a.name.cmp(&b.name));
            e.members.dedup_by(|a, b| a.name.eq(&b.name));
        }
    }

    pub(crate) fn append_members_to_enumerator_equal_and_set_elseif(
        &mut self,
        enumerator_name: &str,
        members: &[RustMember],
        original_fields: &[StructMember],
    ) {
        self.append_members_to_enumerator_equal(enumerator_name, members, original_fields);

        let enums = match &mut self.ty {
            RustType::Enum {
                is_simple,
                enumerators,
                ..
            }
            | RustType::Flag {
                is_simple,
                enumerators,
                ..
            } => {
                *is_simple = false;
                enumerators
            }
            _ => unreachable!(),
        };

        let enums = enums
            .iter_mut()
            .find(|a| a.name() == enumerator_name)
            .unwrap();
        enums.contains_elseif = true;
    }

    pub(crate) fn append_members_to_enumerator_equal(
        &mut self,
        enumerator_name: &str,
        members: &[RustMember],
        original_fields: &[StructMember],
    ) {
        let enums = match &mut self.ty {
            RustType::Enum {
                is_simple,
                enumerators,
                ..
            }
            | RustType::Flag {
                is_simple,
                enumerators,
                ..
            } => {
                *is_simple = false;
                enumerators
            }
            _ => unreachable!(),
        };

        let enums = enums.iter_mut().filter(|a| a.name() == enumerator_name);
        for e in enums {
            for f in original_fields {
                if !e.original_fields().contains(f) {
                    e.original_fields.push(f.clone());
                }
            }
            e.members.append(&mut members.to_vec());
            e.members.sort_by(|a, b| a.name.cmp(&b.name));
            e.members.dedup_by(|a, b| a.name.eq(&b.name));
        }
    }
    pub fn new(
        name: String,
        ty: RustType,
        original_ty: String,
        in_rust_type: bool,
        tags: MemberTags,
    ) -> Self {
        Self {
            name,
            ty,
            original_ty,
            in_rust_type,
            tags,
        }
    }
}
