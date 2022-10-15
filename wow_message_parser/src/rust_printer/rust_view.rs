use crate::parser::types::array::{Array, ArrayType};
use crate::parser::types::definer::{Definer, DefinerValue};
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_struct_member::ParsedStructMember;
use crate::parser::types::parsed::parsed_ty::bool_ty_to_string;
use crate::parser::types::sizes::{Sizes, GUID_SIZE, PACKED_GUID_MAX_SIZE, PACKED_GUID_MIN_SIZE};
use crate::parser::types::struct_member::StructMember;
use crate::parser::types::tags::Tags;
use crate::parser::types::ty::Type;
use crate::parser::types::{FloatingPointType, IntegerType};
use crate::rust_printer::{
    field_name_to_rust_name, get_new_flag_type_name, get_new_type_name, get_optional_type_name,
    DefinerType,
};
use crate::{CSTRING_LARGEST_ALLOWED, CSTRING_SMALLEST_ALLOWED};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub(crate) struct RustMember {
    name: String,
    ty: RustType,
    original_ty: String,

    in_rust_type: bool,
    sizes: Sizes,

    tags: Tags,
}

impl RustMember {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn ty(&self) -> &RustType {
        &self.ty
    }
    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }
    pub(crate) fn constant_sized(&self) -> bool {
        self.sizes.is_constant().is_some()
    }

    pub(crate) fn all_members(&self) -> Vec<&RustMember> {
        let mut v = Vec::new();

        v.push(self);
        match self.ty() {
            RustType::Enum { enumerators, .. } | RustType::Flag { enumerators, .. } => {
                for enumerator in enumerators {
                    v.append(&mut enumerator.all_members());
                }
            }
            RustType::Struct { object, .. } => {
                v.append(&mut object.all_members());
            }
            RustType::Array {
                inner_object: Some(o),
                ..
            } => {
                v.append(&mut o.all_members());
            }
            _ => {}
        }

        v
    }

    pub(crate) fn clear_flag_enumerator(&mut self, enumerator: &str) {
        match &mut self.ty {
            RustType::Flag { enumerators, .. } => enumerators
                .iter_mut()
                .find(|a| a.name() == enumerator)
                .unwrap()
                .members
                .clear(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn get_flag_enumerator(&self, enumerator: &str) -> RustEnumerator {
        match self.ty() {
            RustType::Flag { enumerators, .. } => {
                let enumerator = enumerators.iter().find(|a| a.name() == enumerator).unwrap();
                enumerator.clone()
            }
            _ => unreachable!(),
        }
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
            _ => unreachable!(),
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

    fn set_main_enumerators(&mut self, enumerator_names: &[&String]) {
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

    fn set_is_elseif(&mut self) {
        match &mut self.ty {
            RustType::Flag { is_elseif, .. } => *is_elseif = true,
            _ => unreachable!(),
        };
    }

    fn append_members_to_enumerator_not_equal_range(
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

    fn append_members_to_enumerator_not_equal(
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

    fn append_members_to_enumerator_equal_and_set_elseif(
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

    fn append_members_to_enumerator_equal(
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
}

#[derive(Debug, Clone)]
pub(crate) struct RustEnumerator {
    name: String,
    rust_name: String,
    value: DefinerValue,
    members: Vec<RustMember>,
    is_main_enumerator: bool,
    original_fields: Vec<StructMember>,
    contains_elseif: bool,
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
}

#[derive(Debug, Clone)]
pub(crate) enum RustType {
    Integer(IntegerType),
    Bool(IntegerType),
    DateTime,
    Floating(FloatingPointType),
    UpdateMask,
    AuraMask,
    Guid,
    PackedGuid,
    String,
    CString,
    SizedCString,
    Array {
        array: Array,
        inner_sizes: Sizes,
        inner_object: Option<RustObject>,
    },
    Enum {
        ty_name: String,
        original_ty_name: String,
        enumerators: Vec<RustEnumerator>,
        int_ty: IntegerType,
        is_simple: bool,
        is_elseif: bool,
    },
    Flag {
        ty_name: String,
        original_ty_name: String,
        int_ty: IntegerType,
        enumerators: Vec<RustEnumerator>,
        is_simple: bool,
        is_elseif: bool,
    },
    Struct {
        ty_name: String,
        sizes: Sizes,
        object: RustObject,
    },
}

impl RustType {
    pub(crate) fn str(&self) -> String {
        match self {
            RustType::Integer(i) => i.str().to_string(),
            RustType::Floating(f) => f.str().to_string(),
            RustType::String => "String".to_string(),
            RustType::Array { array, .. } => array.str(),
            RustType::Flag { ty_name, .. } | RustType::Enum { ty_name, .. } => ty_name.clone(),
            RustType::Struct { ty_name, .. } => ty_name.clone(),
            RustType::CString => "CString".to_string(),
            RustType::UpdateMask => "UpdateMask".to_string(),
            RustType::AuraMask => "AuraMask".to_string(),
            RustType::PackedGuid | RustType::Guid => "Guid".to_string(),
            RustType::SizedCString => "SizedCString".to_string(),
            RustType::Bool(i) => bool_ty_to_string(i),
            RustType::DateTime => "DateTime".to_string(),
        }
    }

    pub(crate) fn rust_str(&self) -> String {
        self.to_string()
    }
}

impl Display for RustType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustType::Integer(i) => f.write_str(i.rust_str()),
            RustType::Floating(i) => f.write_str(i.rust_str()),
            RustType::SizedCString | RustType::String | RustType::CString => f.write_str("String"),
            RustType::Array { array, .. } => f.write_str(&array.rust_str()),
            RustType::Enum { ty_name, .. } | RustType::Flag { ty_name, .. } => f.write_str(ty_name),
            RustType::Struct { ty_name, .. } => f.write_str(ty_name),
            RustType::UpdateMask => f.write_str("UpdateMask"),
            RustType::AuraMask => f.write_str("AuraMask"),
            RustType::PackedGuid | RustType::Guid => f.write_str("Guid"),
            RustType::Bool(_) => f.write_str("bool"),
            RustType::DateTime => f.write_str("DateTime"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RustOptional {
    name: String,
    ty: String,
    members: Vec<RustMember>,
    tags: Tags,
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
    pub(crate) fn all_members(&self) -> Vec<&RustMember> {
        let mut v = Vec::new();

        for m in self.members() {
            v.append(&mut m.all_members());
        }

        v
    }
    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }
}

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
        let (ty_name, original_ty_name, enumerators, int_ty, is_simple, definer_type, is_elseif) =
            match m.ty().clone() {
                RustType::Enum {
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
                    DefinerType::Enum,
                    is_elseif,
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
                ),
                _ => return None,
            };

        Some(RustDefiner {
            inner: m.clone(),
            enumerators,
            ty_name,
            int_ty,
            is_simple,
            is_elseif,
            original_ty_name,
            definer_type,
        })
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

        fn inner(
            m: &RustMember,
            enumerator_name: &str,
            v: &mut Vec<RustDefiner>,
            container_name: &str,
        ) {
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
                                                    inner(m, enumerator_name, v, container_name);
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
                                    inner(m, enumerator_name, v, container_name);
                                }
                            }
                        }
                    } else {
                        for m in enumerator.members_in_struct() {
                            if RustObject::get_rust_definer_from_ty(m).is_some() {
                                inner(m, enumerator_name, v, container_name);
                            }
                        }
                    }
                }
            }
        }

        for m in self.members_in_struct() {
            inner(m, enumerator_name, &mut v, self.name());
        }

        v
    }

    pub(crate) fn rust_definers_in_namespace(&self, ty_name: &str) -> Vec<RustDefiner> {
        let rd = self.get_rust_definer(ty_name);

        let mut v = Vec::new();

        for enumerator in rd.enumerators {
            for m in enumerator.members_in_struct() {
                if let Some(rd) = Self::get_rust_definer_from_ty(m) {
                    v.push(rd);
                }
            }
        }

        v
    }

    pub(crate) fn get_rust_definers(&self) -> Vec<RustDefiner> {
        fn inner(m: &RustMember, v: &mut Vec<RustDefiner>, container_name: &str) {
            let rd = RustObject::get_rust_definer_from_ty(m);

            if let Some(rd) = rd {
                for enumerator in rd.enumerators() {
                    for m in enumerator.members_in_struct() {
                        inner(m, v, container_name);
                    }
                }

                if !rd.is_simple() && !v.iter().any(|a| a.ty_name() == rd.ty_name()) {
                    v.push(rd);
                }
            }
        }

        let mut v = Vec::new();

        for m in self.members_in_struct() {
            inner(m, &mut v, self.name());
        }

        v
    }

    pub(crate) fn rust_definer_with_variable_name_and_enumerator(
        &self,
        variable_name: &str,
        enumerator_name: &str,
    ) -> RustDefiner {
        fn inner(
            m: &RustMember,
            variable_name: &str,
            enumerator_name: &str,
            container_name: &str,
        ) -> Option<RustDefiner> {
            if let Some(rd) = RustObject::get_rust_definer_from_ty(m) {
                for enumerator in rd.enumerators() {
                    if enumerator.contains_elseif() {
                        match enumerator.members()[0].ty() {
                            RustType::Enum { enumerators, .. } => {
                                for enumerator in enumerators {
                                    for m in enumerator.members() {
                                        if let Some(rd) =
                                            inner(m, variable_name, enumerator_name, container_name)
                                        {
                                            return Some(rd);
                                        }
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }

                    for m in enumerator.members() {
                        if let Some(rd) = inner(m, variable_name, enumerator_name, container_name) {
                            return Some(rd);
                        }
                    }
                }

                if rd.variable_name() == variable_name && rd.contains_enumerator(enumerator_name) {
                    return Some(rd);
                }
            }
            None
        }

        for m in self.members() {
            if let Some(rd) = inner(m, variable_name, enumerator_name, self.name()) {
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

        unreachable!()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RustDefiner {
    inner: RustMember,
    definer_type: DefinerType,
    enumerators: Vec<RustEnumerator>,
    ty_name: String,
    int_ty: IntegerType,
    is_simple: bool,
    is_elseif: bool,
    original_ty_name: String,
}

impl RustDefiner {
    pub(crate) fn all_members(&self) -> Vec<&RustMember> {
        let mut v = Vec::new();

        for enumerator in self.enumerators() {
            v.append(&mut enumerator.all_members());
        }

        v
    }
    pub(crate) fn variable_name(&self) -> &str {
        self.inner().name()
    }
    pub(crate) fn inner(&self) -> &RustMember {
        &self.inner
    }
    pub(crate) fn enumerators(&self) -> &[RustEnumerator] {
        &self.enumerators
    }
    pub(crate) fn complex_flag_enumerators(&self) -> Vec<&RustEnumerator> {
        self.enumerators
            .iter()
            .filter(|a| !a.should_not_be_in_flag_types())
            .collect()
    }
    pub(crate) fn ty_name(&self) -> &str {
        &self.ty_name
    }
    pub(crate) fn int_ty(&self) -> IntegerType {
        self.int_ty
    }
    pub(crate) fn is_simple(&self) -> bool {
        self.is_simple && !self.is_elseif
    }
    pub(crate) fn is_elseif(&self) -> bool {
        self.is_elseif
    }
    pub(crate) fn original_ty_name(&self) -> &str {
        &self.original_ty_name
    }
    pub(crate) fn definer_type(&self) -> DefinerType {
        self.definer_type
    }
    pub(crate) fn get_enumerator(&self, enumerator_name: &str) -> &RustEnumerator {
        for enumerator in self.enumerators() {
            if enumerator.contains_elseif() {
                match enumerator.members()[0].ty() {
                    RustType::Enum { enumerators, .. } => {
                        for enumerator in enumerators {
                            if enumerator.name() == enumerator_name {
                                return enumerator;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if enumerator.name() == enumerator_name {
                return enumerator;
            }
        }

        unreachable!()
    }

    pub(crate) fn contains_enumerator(&self, enumerator_name: &str) -> bool {
        for enumerator in self.enumerators() {
            if enumerator.contains_elseif() {
                match enumerator.members()[0].ty() {
                    RustType::Enum { enumerators, .. } => {
                        for enumerator in enumerators {
                            if enumerator.name() == enumerator_name {
                                return true;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if enumerator.name() == enumerator_name {
                return true;
            }
        }

        false
    }
}

fn create_else_if_flag(
    statement: &IfStatement,
    struct_ty_name: &str,
    current_scope: &mut [RustMember],
    parent_scope: &mut [RustMember],
) {
    assert_eq!(statement.conditional().equations().len(), 1);
    assert!(statement.else_members().is_empty());

    let enumerator = match &statement.conditional().equations()[0] {
        Equation::BitwiseAnd { value } => value.as_str(),
        _ => unreachable!(),
    };

    // Move enumerators into new RustMember
    let main_enum =
        find_subject(current_scope, parent_scope, statement).get_flag_enumerator(enumerator);
    find_subject(current_scope, parent_scope, statement).clear_flag_enumerator(enumerator);
    find_subject(current_scope, parent_scope, statement).set_is_elseif();

    // Push enumerators
    let mut enumerators = vec![main_enum];

    // Append elseifs
    for elseif in statement.else_ifs() {
        let name = match &elseif.conditional().equations()[0] {
            Equation::BitwiseAnd { value } => value.to_string(),
            _ => unreachable!(),
        };
        let enumerator =
            find_subject(current_scope, parent_scope, elseif).pop_flag_enumerator(&name);
        enumerators.push(enumerator);
    }

    let flag_int_ty = match find_subject(current_scope, parent_scope, statement).ty() {
        RustType::Flag { int_ty, .. } => *int_ty,
        _ => unreachable!(),
    };
    let flag_ty_name = &find_subject(current_scope, parent_scope, statement).original_ty;

    // Create new Enum RustMember
    let rm = RustMember {
        name: statement.name().to_string(),
        ty: RustType::Enum {
            ty_name: get_new_flag_type_name(flag_ty_name, &field_name_to_rust_name(enumerator)),
            original_ty_name: flag_ty_name.clone(),
            enumerators,
            int_ty: flag_int_ty,
            is_simple: false,
            is_elseif: true,
        },
        original_ty: struct_ty_name.to_string(),
        in_rust_type: true,
        sizes: Sizes::new(), // TODO Make real?
        tags: Tags::new(),   // TODO Which tags should go in here?
    };

    // Move RustMember into
    find_subject(current_scope, parent_scope, statement)
        .append_members_to_enumerator_equal_and_set_elseif(
            enumerator,
            &[rm],
            &[StructMember::IfStatement(statement.clone())],
        );
}

fn find_subject<'a>(
    current_scope: &'a mut [RustMember],
    parent_scope: &'a mut [RustMember],
    statement: &IfStatement,
) -> &'a mut RustMember {
    let subject = current_scope
        .iter_mut()
        .find(|a| statement.name() == a.name);
    let subject = match subject {
        None => parent_scope
            .iter_mut()
            .find(|a| statement.name() == a.name)
            .unwrap(),
        Some(s) => s,
    };
    subject
}

pub(crate) fn create_if_statement(
    statement: &IfStatement,
    struct_ty_name: &str,
    tags: &Tags,
    containers: &[ParsedContainer],
    definers: &[Definer],
    e: &ParsedContainer,
    current_scope: &mut [RustMember],
    parent_scope: &mut [RustMember],
) {
    let mut reversed = false;
    let mut main_enumerators = Vec::new();

    for i in statement.conditional().equations() {
        match i {
            Equation::BitwiseAnd { value } | Equation::Equals { value } => {
                main_enumerators.push(value)
            }
            Equation::NotEquals { value } => {
                main_enumerators.push(value);
                reversed = true;
            }
        }
    }

    let mut main_enumerator_members = Vec::new();
    let mut main_enumerator_originals = Vec::new();
    for m in statement.members() {
        create_struct_member(
            m,
            struct_ty_name,
            tags,
            e,
            containers,
            definers,
            &mut main_enumerator_members,
            current_scope,
            &mut None,
        );

        main_enumerator_originals.push(m.clone());
    }

    let mut else_enumerator_members = Vec::new();
    let mut else_enumerator_originals = Vec::new();
    for m in statement.else_members() {
        create_struct_member(
            m,
            struct_ty_name,
            tags,
            e,
            containers,
            definers,
            &mut else_enumerator_members,
            current_scope,
            &mut None,
        );

        else_enumerator_originals.push(m.clone());
    }

    find_subject(current_scope, parent_scope, statement).set_main_enumerators(&main_enumerators);
    if reversed {
        // Apply main to all except main_enumerators
        for i in &main_enumerators {
            find_subject(current_scope, parent_scope, statement)
                .append_members_to_enumerator_not_equal(
                    i,
                    &main_enumerator_members,
                    &main_enumerator_originals,
                );
        }

        // Apply other to main_enumerator
        for i in &main_enumerators {
            find_subject(current_scope, parent_scope, statement)
                .append_members_to_enumerator_equal(
                    i,
                    &else_enumerator_members,
                    &else_enumerator_originals,
                );
        }
    } else {
        // Apply main to main_enumerator
        for i in &main_enumerators {
            find_subject(current_scope, parent_scope, statement)
                .append_members_to_enumerator_equal(
                    i,
                    &main_enumerator_members,
                    &main_enumerator_originals,
                );
        }

        // Apply else_if to else_if, ..
        for else_if in statement.else_ifs() {
            let mut else_if_enumerators = Vec::new();
            for i in else_if.conditional().equations() {
                match i {
                    Equation::BitwiseAnd { value } | Equation::Equals { value } => {
                        main_enumerators.push(value);
                        else_if_enumerators.push(value);
                    }
                    Equation::NotEquals { .. } => unreachable!(),
                }
            }

            let mut else_if_enumerator_members = Vec::new();
            let mut else_if_originals = Vec::new();
            for m in else_if.members() {
                create_struct_member(
                    m,
                    struct_ty_name,
                    tags,
                    e,
                    containers,
                    definers,
                    &mut else_if_enumerator_members,
                    current_scope,
                    &mut None,
                );
                else_if_originals.push(m.clone());
            }

            for i in &else_if_enumerators {
                find_subject(current_scope, parent_scope, statement)
                    .append_members_to_enumerator_equal(
                        i,
                        &else_if_enumerator_members,
                        &else_if_originals,
                    );
            }
        }

        // Apply other to other_enumerators
        find_subject(current_scope, parent_scope, statement)
            .append_members_to_enumerator_not_equal_range(
                &main_enumerators,
                &else_enumerator_members,
                &else_enumerator_originals,
            );
    }

    if statement.is_elseif_flag() {
        create_else_if_flag(statement, struct_ty_name, current_scope, parent_scope);
    }
}

pub(crate) fn create_struct_member(
    m: &StructMember,
    struct_ty_name: &str,
    tags: &Tags,
    e: &ParsedContainer,
    containers: &[ParsedContainer],
    definers: &[Definer],
    current_scope: &mut Vec<RustMember>,
    parent_scope: &mut [RustMember],
    optional: &mut Option<RustOptional>,
) {
    match m {
        StructMember::Definition(d) => {
            let mut in_rust_type = true;
            let ty = match d.ty() {
                Type::Integer(i) => {
                    if d.used_as_size_in().is_some() || d.value().is_some() {
                        in_rust_type = false;
                    }
                    RustType::Integer(*i)
                }
                Type::Bool(i) => RustType::Bool(*i),
                Type::DateTime => RustType::DateTime,
                Type::Guid => RustType::Guid,
                Type::PackedGuid => RustType::PackedGuid,
                Type::FloatingPoint(f) => RustType::Floating(*f),
                Type::CString => RustType::CString,
                Type::String { .. } => RustType::String,
                Type::Array(array) => {
                    let mut inner_sizes = Sizes::new();
                    let complex = match array.ty() {
                        ArrayType::Integer(i) => {
                            inner_sizes.inc_both(i.size().into());
                            None
                        }
                        ArrayType::Guid => {
                            inner_sizes.inc_both(GUID_SIZE.into());
                            None
                        }
                        ArrayType::Struct(c) => {
                            inner_sizes += c.sizes();
                            Some(c)
                        }
                        ArrayType::PackedGuid => {
                            inner_sizes
                                .inc(PACKED_GUID_MIN_SIZE.into(), PACKED_GUID_MAX_SIZE.into());
                            None
                        }
                        ArrayType::CString => {
                            inner_sizes.inc(CSTRING_SMALLEST_ALLOWED, CSTRING_LARGEST_ALLOWED);
                            None
                        }
                    };

                    let inner_object = complex.map(|c| c.rust_object().clone());

                    RustType::Array {
                        array: array.clone(),
                        inner_sizes,
                        inner_object,
                    }
                }
                Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
                    let add_types = || -> Vec<RustEnumerator> {
                        let mut enumerators = Vec::new();

                        for field in e.fields() {
                            enumerators.push(RustEnumerator {
                                name: field.name().to_string(),
                                rust_name: field.rust_name().to_string(),
                                value: field.value().clone(),
                                members: vec![],
                                is_main_enumerator: false,
                                original_fields: vec![],
                                contains_elseif: false,
                            });
                        }
                        enumerators
                    };
                    let int_ty = if let Some(upcast) = upcast {
                        *upcast
                    } else {
                        *e.ty()
                    };

                    if e.definer_ty() == DefinerType::Enum {
                        let enumerators = add_types();

                        RustType::Enum {
                            ty_name: e.name().to_string(),
                            original_ty_name: e.name().to_string(),
                            enumerators,
                            int_ty,
                            is_simple: true,
                            is_elseif: false,
                        }
                    } else {
                        let enumerators = add_types();

                        RustType::Flag {
                            ty_name: e.name().to_string(),
                            original_ty_name: e.name().to_string(),
                            int_ty,
                            enumerators,
                            is_simple: true,
                            is_elseif: false,
                        }
                    }
                }
                Type::Struct { e } => {
                    let object = e.rust_object().clone();

                    RustType::Struct {
                        ty_name: e.name().to_string(),
                        sizes: e.sizes(),
                        object,
                    }
                }
                Type::UpdateMask => RustType::UpdateMask,
                Type::AuraMask => RustType::AuraMask,
                Type::SizedCString => RustType::SizedCString,
            };

            let name = d.name().to_string();
            let mut sizes = d.ty().sizes_parsed();

            for m in e.fields() {
                match m {
                    ParsedStructMember::Definition(_) => {}
                    ParsedStructMember::IfStatement(statement) => {
                        if statement.name() != name {
                            continue;
                        }

                        let complex_sizes =
                            ParsedContainer::get_complex_sizes(statement, e, containers, definers);
                        sizes += complex_sizes;
                    }
                    ParsedStructMember::OptionalStatement(_) => {}
                }
            }

            current_scope.push(RustMember {
                name,
                ty,
                original_ty: d.ty().str(),
                in_rust_type,
                sizes,
                tags: d.tags().clone(),
            });
        }
        StructMember::IfStatement(statement) => {
            create_if_statement(
                statement,
                struct_ty_name,
                tags,
                containers,
                definers,
                e,
                current_scope,
                parent_scope,
            );
        }
        StructMember::OptionalStatement(option) => {
            let mut members = Vec::new();

            for i in option.members() {
                create_struct_member(
                    i,
                    struct_ty_name,
                    tags,
                    e,
                    containers,
                    definers,
                    &mut members,
                    current_scope,
                    &mut None,
                );
            }

            *optional = Some(RustOptional {
                name: option.name().to_string(),
                ty: get_optional_type_name(struct_ty_name, option.name()),
                members,
                tags: option.tags().clone(),
            });
        }
    }
}

pub(crate) fn create_rust_object(
    e: &ParsedContainer,
    members: &[StructMember],
    containers: &[ParsedContainer],
    definers: &[Definer],
) -> RustObject {
    let mut v = Vec::new();
    let mut optional = None;

    for m in members {
        create_struct_member(
            m,
            e.name(),
            e.tags(),
            e,
            containers,
            definers,
            &mut v,
            &mut Vec::new(),
            &mut optional,
        );
    }

    for m in &mut v {
        set_simple_objects_name(m, e.name());
    }

    RustObject {
        name: e.name().to_string(),
        members: v,
        optional,
        sizes: e.create_sizes(containers, definers),
    }
}

fn set_simple_objects_name(m: &mut RustMember, struct_ty_name: &str) {
    match &mut m.ty {
        RustType::Enum {
            ty_name,
            is_simple,
            enumerators,
            ..
        }
        | RustType::Flag {
            ty_name,
            is_simple,
            enumerators,
            ..
        } => {
            if !(*is_simple) {
                let definer_ty = ty_name.clone();
                *ty_name = get_new_type_name(struct_ty_name, &definer_ty);

                for e in enumerators {
                    for f in &mut e.members {
                        set_simple_objects_name(f, struct_ty_name);
                    }
                }
            }
        }
        _ => {}
    }
}
