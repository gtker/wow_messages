use crate::error_printer::invalid_self_size_position;
use crate::file_info::FileInfo;
use crate::file_utils::get_import_path;
use crate::parser::types::array::ArrayType;
use crate::parser::types::compare_name_and_tags;
use crate::parser::types::objects::Objects;
use crate::parser::types::sizes::{Sizes, DATETIME_SIZE};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::test_case::TestCase;
use crate::parser::types::ty::Type;
use crate::parser::types::version::{LoginVersion, Version};
use crate::rust_printer::rust_view::RustObject;
use crate::rust_printer::{
    LOGIN_CLIENT_MESSAGE_ENUM_NAME, LOGIN_SERVER_MESSAGE_ENUM_NAME, WORLD_CLIENT_MESSAGE_ENUM_NAME,
    WORLD_SERVER_MESSAGE_ENUM_NAME,
};
use crate::{Object, CONTAINER_SELF_SIZE_FIELD};
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum ContainerType {
    Struct,
    CLogin(u16),
    SLogin(u16),
    Msg(u16),
    CMsg(u16),
    SMsg(u16),
}

impl ContainerType {
    pub(crate) fn str(&self) -> &str {
        match self {
            ContainerType::Struct => "struct",
            ContainerType::CLogin(_) => "clogin",
            ContainerType::SLogin(_) => "slogin",
            ContainerType::Msg(_) => "msg",
            ContainerType::CMsg(_) => "cmsg",
            ContainerType::SMsg(_) => "smsg",
        }
    }

    pub(crate) fn is_top_level(&self) -> bool {
        self != &ContainerType::Struct
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Container {
    name: String,
    object_type: ContainerType,
    sizes: Sizes,
    members: Vec<StructMember>,
    tags: ObjectTags,
    file_info: FileInfo,
    only_has_io_error: bool,
    rust_object_view: RustObject,
}

impl PartialEq<Self> for Container {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.tags().main_versions().collect::<Vec<_>>()
                == other.tags().main_versions().collect::<Vec<_>>()
    }
}

impl Eq for Container {}

impl PartialOrd for Container {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Container {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_first = self.tags().main_versions().collect::<Vec<_>>();
        let other_first = other.tags().main_versions().collect::<Vec<_>>();

        compare_name_and_tags(&self.name, &self_first, &other.name, &other_first)
    }
}

impl Container {
    pub(crate) fn new(
        name: String,
        members: Vec<StructMember>,
        tags: ObjectTags,
        object_type: ContainerType,
        file_info: FileInfo,
        sizes: Sizes,
        only_has_io_error: bool,
        rust_object_view: RustObject,
    ) -> Self {
        Self {
            name,
            object_type,
            sizes,
            members,
            tags,
            file_info,
            only_has_io_error,
            rust_object_view,
        }
    }

    pub(crate) fn empty_body(&self) -> bool {
        self.members.is_empty()
    }

    pub(crate) fn is_pure_movement_info(&self) -> bool {
        let only_one_field = self.members().len() == 1;
        if only_one_field {
            let field = self.members().first().unwrap();
            match field {
                StructMember::Definition(d) => {
                    d.name() == "info"
                        && match d.ty() {
                            Type::Struct { e } => e.name() == "MovementInfo",
                            _ => false,
                        }
                }
                _ => false,
            }
        } else {
            false
        }
    }

    pub(crate) fn opcode(&self) -> u32 {
        match self.container_type() {
            ContainerType::Struct => unreachable!("struct does not have opcode"),
            ContainerType::CLogin(i)
            | ContainerType::SLogin(i)
            | ContainerType::Msg(i)
            | ContainerType::CMsg(i)
            | ContainerType::SMsg(i) => i as u32,
        }
    }

    pub(crate) fn type_definition_in_same_scope(&self, variable_name: &str) -> bool {
        fn inner(
            m: &StructMember,
            ty_scope: &mut u32,
            variable_name: &str,
            variable_scope: &mut u32,
            ty: &str,
            current_scope: &mut u32,
        ) {
            match m {
                StructMember::Definition(d) => {
                    if d.ty().rust_str() == ty {
                        *ty_scope = *current_scope;
                    }
                }
                StructMember::IfStatement(statement) => {
                    if statement.name() == variable_name {
                        *variable_scope = *current_scope;
                    }

                    *current_scope += 1;
                    for m in statement.all_members() {
                        inner(
                            m,
                            ty_scope,
                            variable_name,
                            variable_scope,
                            ty,
                            current_scope,
                        );
                    }
                    *current_scope -= 1;
                }
                StructMember::OptionalStatement(optional) => {
                    *current_scope += 1;
                    for m in optional.members() {
                        inner(
                            m,
                            ty_scope,
                            variable_name,
                            variable_scope,
                            ty,
                            current_scope,
                        );
                    }
                    *current_scope -= 1;
                }
            }
        }

        let ty = self.get_type_of_variable(variable_name);

        let mut ty_scope = 0;
        let mut variable_scope = 0;
        let mut current_scope = 1;

        for m in &self.members {
            inner(
                m,
                &mut ty_scope,
                variable_name,
                &mut variable_scope,
                &ty.rust_str(),
                &mut current_scope,
            );
        }

        ty_scope == variable_scope
    }

    pub(crate) fn is_constant_sized(&self) -> bool {
        self.sizes.is_constant().is_some()
    }

    pub(crate) fn only_has_io_errors(&self) -> bool {
        self.only_has_io_error
    }

    pub(crate) fn get_opcode_import_path(&self, version: Version) -> String {
        // `all` doesn't have an opcodes.rs
        let import_path = match version {
            Version::Login(f) => match f {
                LoginVersion::Specific(_) => get_import_path(version),
                LoginVersion::All => get_import_path(Version::Login(LoginVersion::Specific(3))),
            },
            Version::World(_) => get_import_path(version),
        };

        let enum_name = match self.container_type() {
            ContainerType::CLogin(_) => LOGIN_CLIENT_MESSAGE_ENUM_NAME,
            ContainerType::SLogin(_) => LOGIN_SERVER_MESSAGE_ENUM_NAME,
            ContainerType::SMsg(_) => WORLD_SERVER_MESSAGE_ENUM_NAME,
            ContainerType::CMsg(_) => WORLD_CLIENT_MESSAGE_ENUM_NAME,
            _ => unimplemented!(),
        };

        format!("{}::opcodes::{}", import_path, enum_name,)
    }

    pub(crate) fn any_fields_have_constant_value(&self) -> bool {
        for d in self.all_definitions() {
            if d.value().is_some() {
                return true;
            }
        }

        false
    }

    pub(crate) fn container_type(&self) -> ContainerType {
        self.object_type
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub(crate) fn rust_object(&self) -> &RustObject {
        &self.rust_object_view
    }

    pub(crate) fn size_of_fields_before_size(&self) -> u64 {
        let mut sum = match self.object_type {
            ContainerType::Struct => 0,
            ContainerType::CLogin(_) | ContainerType::SLogin(_) => 0,
            _ => invalid_self_size_position(
                self.name(),
                self.file_info(),
                &format!(
                    "Only login messages can contain '{}'",
                    CONTAINER_SELF_SIZE_FIELD
                ),
            ),
        };
        for field in self.members() {
            match field {
                StructMember::Definition(d) => {
                    match d.ty() {
                        Type::Integer(int_type) => {
                            sum += int_type.size() as u64;
                        }
                        Type::FloatingPoint(float_ty) => {
                            sum += float_ty.size() as u64;
                        }
                        Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
                            let i = if let Some(upcast) = upcast {
                                upcast.size()
                            } else {
                                e.ty().size()
                            } as u64;
                            sum += i;
                        }
                        Type::Bool(i) => sum += i.size() as u64,
                        Type::DateTime => sum += DATETIME_SIZE as u64,
                        Type::Struct { e, .. } => invalid_self_size_position(
                            self.name(),
                            self.file_info(),
                            format!(
                                "'{}' can not come after Struct variable '{}'",
                                CONTAINER_SELF_SIZE_FIELD,
                                e.name(),
                            ),
                        ),
                        Type::PackedGuid => invalid_self_size_position(
                            self.name(),
                            self.file_info(),
                            format!(
                                "'{}' can not come after a PackedGuid variable",
                                CONTAINER_SELF_SIZE_FIELD
                            ),
                        ),
                        Type::Guid => sum += 8_u64,
                        Type::UpdateMask => invalid_self_size_position(
                            self.name(),
                            self.file_info(),
                            format!(
                                "'{}' can not come after an UpdateMask variable",
                                CONTAINER_SELF_SIZE_FIELD
                            ),
                        ),
                        Type::AuraMask => invalid_self_size_position(
                            self.name(),
                            self.file_info(),
                            format!(
                                "'{}' can not come after an AuraMask variable",
                                CONTAINER_SELF_SIZE_FIELD
                            ),
                        ),
                        Type::SizedCString => invalid_self_size_position(
                            self.name(),
                            self.file_info(),
                            format!(
                                "'{}' can not come after a SizedCString variable",
                                CONTAINER_SELF_SIZE_FIELD
                            ),
                        ),
                        Type::CString => invalid_self_size_position(
                            self.name(),
                            self.file_info(),
                            format!(
                                "'{}' can not come after a CString variable",
                                CONTAINER_SELF_SIZE_FIELD
                            ),
                        ),
                        Type::String { .. } => invalid_self_size_position(
                            self.name(),
                            self.file_info(),
                            format!(
                                "'{}' can not come after a String variable",
                                CONTAINER_SELF_SIZE_FIELD
                            ),
                        ),
                        Type::Array(_) => invalid_self_size_position(
                            self.name(),
                            self.file_info(),
                            format!(
                                "'{}' can not come after an array variable",
                                CONTAINER_SELF_SIZE_FIELD
                            ),
                        ),
                    }
                    if let Some(v) = d.value() {
                        if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
                            return sum;
                        }
                    }
                }
                StructMember::IfStatement(_) => invalid_self_size_position(
                    self.name(),
                    self.file_info(),
                    format!(
                        "'{}' can not come after an if statement",
                        CONTAINER_SELF_SIZE_FIELD
                    ),
                ),
                StructMember::OptionalStatement(_) => invalid_self_size_position(
                    self.name(),
                    self.file_info(),
                    format!(
                        "'{}' can not come after an optional statement",
                        CONTAINER_SELF_SIZE_FIELD
                    ),
                ),
            }
        }

        sum
    }

    pub(crate) fn tags(&self) -> &ObjectTags {
        &self.tags
    }

    pub(crate) fn sizes(&self) -> Sizes {
        self.sizes
    }

    pub(crate) fn all_definitions_transitively(&self) -> Vec<StructMemberDefinition> {
        fn inner(m: &StructMember, v: &mut Vec<StructMemberDefinition>) {
            match m {
                StructMember::Definition(d) => {
                    v.push(d.clone());
                    match d.ty() {
                        Type::Struct { e } => {
                            for m in e.members() {
                                inner(m, v);
                            }
                        }
                        Type::Array(array) => {
                            if let ArrayType::Struct(c) = array.ty() {
                                for m in c.members() {
                                    inner(m, v);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members() {
                        inner(m, v);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, v);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.members() {
            inner(m, &mut v);
        }

        v
    }

    pub(crate) fn all_definitions(&self) -> Vec<&StructMemberDefinition> {
        fn inner<'a>(m: &'a StructMember, v: &mut Vec<&'a StructMemberDefinition>) {
            match m {
                StructMember::Definition(d) => v.push(d),
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members() {
                        inner(m, v);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, v);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.members() {
            inner(m, &mut v);
        }

        v
    }

    pub(crate) fn contains_update_mask(&self) -> bool {
        for d in self.all_definitions() {
            if d.ty() == &Type::UpdateMask {
                return true;
            }
        }

        false
    }

    pub(crate) fn contains_update_mask_transitively(&self) -> bool {
        for d in self.all_definitions_transitively() {
            if d.ty() == &Type::UpdateMask {
                return true;
            }
        }

        false
    }

    pub(crate) fn contains_aura_mask(&self) -> bool {
        for d in self.all_definitions() {
            if d.ty() == &Type::AuraMask {
                return true;
            }
        }

        false
    }

    pub(crate) fn contains_aura_mask_transitively(&self) -> bool {
        for d in self.all_definitions_transitively() {
            if d.ty() == &Type::AuraMask {
                return true;
            }
        }

        false
    }

    pub(crate) fn contains_datetime(&self) -> bool {
        for d in self.all_definitions() {
            if d.ty() == &Type::DateTime {
                return true;
            }
        }

        false
    }

    pub(crate) fn contains_guid_or_packed_guid(&self) -> bool {
        for d in self.all_definitions() {
            match d.ty() {
                Type::PackedGuid | Type::Guid => return true,
                Type::Array(array) => match array.ty() {
                    ArrayType::Guid | ArrayType::PackedGuid => return true,
                    _ => {}
                },
                _ => {}
            }
        }

        false
    }

    pub(crate) fn contains_guid_or_packed_guid_transitively(&self) -> bool {
        for d in self.all_definitions_transitively() {
            match d.ty() {
                Type::PackedGuid | Type::Guid => return true,
                Type::Array(array) => match array.ty() {
                    ArrayType::Guid | ArrayType::PackedGuid => return true,
                    _ => {}
                },
                _ => {}
            }
        }

        false
    }

    pub(crate) fn get_objects_needing_import(&self) -> BTreeSet<Object> {
        let mut v = BTreeSet::new();

        for d in self.all_definitions() {
            match d.struct_type() {
                Type::Array(a) => {
                    if let ArrayType::Struct(c) = a.ty() {
                        v.insert(Object::Container(*c.clone()));
                    }
                }
                Type::Enum { e, .. } => {
                    v.insert(Object::Enum(e.clone()));
                }
                Type::Flag { e, .. } => {
                    v.insert(Object::Flag(e.clone()));
                }
                Type::Struct { e } => {
                    v.insert(Object::Container(e.clone()));
                }
                _ => {}
            }
        }

        v
    }

    pub(crate) fn get_type_of_variable(&self, variable_name: &str) -> Type {
        for d in self.all_definitions() {
            if d.name() == variable_name {
                return d.ty().clone();
            }
        }

        panic!("unable to find variable name {}", variable_name)
    }

    pub(crate) fn tests(&self, o: &Objects) -> Vec<TestCase> {
        o.tests()
            .iter()
            .filter(|a| a.subject() == self.name() && self.tags().fulfills_all(a.tags()))
            .cloned()
            .collect()
    }

    pub(crate) fn members(&self) -> &[StructMember] {
        self.members.as_slice()
    }
}
