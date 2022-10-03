use crate::file_info::FileInfo;
use crate::file_utils::get_import_path;
use crate::parser::types::objects::{Object, Objects};
use crate::parser::types::sizes::{Sizes, BOOL_SIZE, DATETIME_SIZE};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::{LoginVersion, Tags};
use crate::parser::types::test_case::TestCase;
use crate::parser::types::ty::Type;
use crate::parser::types::{compare_name_and_tags, ArrayType, ObjectType};
use crate::rust_printer::rust_view::RustObject;
use crate::rust_printer::{
    Version, LOGIN_CLIENT_MESSAGE_ENUM_NAME, LOGIN_SERVER_MESSAGE_ENUM_NAME,
    WORLD_CLIENT_MESSAGE_ENUM_NAME, WORLD_SERVER_MESSAGE_ENUM_NAME,
};
use crate::CONTAINER_SELF_SIZE_FIELD;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ContainerType {
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
}

#[derive(Debug, Clone)]
pub struct Container {
    name: String,
    object_type: ContainerType,
    sizes: Sizes,
    members: Vec<StructMember>,
    tags: Tags,
    tests: Vec<TestCase>,
    file_info: FileInfo,
    only_has_io_error: bool,
    rust_object_view: RustObject,
}

impl PartialEq<Self> for Container {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.tags().first_and_main_versions() == other.tags().first_and_main_versions()
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
        let self_first = self.tags().first_version();
        let other_first = other.tags().first_version();

        compare_name_and_tags(&self.name, &[self_first], &other.name, &[other_first])
    }
}

impl Container {
    pub(crate) fn new(
        name: String,
        members: Vec<StructMember>,
        tags: Tags,
        object_type: ContainerType,
        file_info: FileInfo,
        tests: Vec<TestCase>,
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
            tests,
            file_info,
            only_has_io_error,
            rust_object_view,
        }
    }

    pub(crate) fn empty_body(&self) -> bool {
        self.members.is_empty()
    }

    pub(crate) fn is_pure_movement_info(&self) -> bool {
        let only_one_field = self.fields().len() == 1;
        if only_one_field {
            let field = self.fields().first().unwrap();
            match field {
                StructMember::Definition(d) => {
                    d.name() == "info"
                        && match d.ty() {
                            Type::Identifier { s, .. } => s == "MovementInfo",
                            _ => false,
                        }
                }
                _ => false,
            }
        } else {
            false
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

        match self.container_type() {
            ContainerType::CLogin(_) => {
                format!(
                    "{}::opcodes::{}",
                    import_path, LOGIN_CLIENT_MESSAGE_ENUM_NAME
                )
            }
            ContainerType::SLogin(_) => {
                format!(
                    "{}::opcodes::{}",
                    import_path, LOGIN_SERVER_MESSAGE_ENUM_NAME
                )
            }
            ContainerType::SMsg(_) => {
                format!(
                    "{}::opcodes::{}",
                    import_path, WORLD_SERVER_MESSAGE_ENUM_NAME
                )
            }
            ContainerType::CMsg(_) => {
                format!(
                    "{}::opcodes::{}",
                    import_path, WORLD_CLIENT_MESSAGE_ENUM_NAME
                )
            }
            _ => unimplemented!(),
        }
    }

    pub(crate) fn any_fields_have_constant_value(&self) -> bool {
        for d in self.all_definitions() {
            if d.verified_value().is_some() {
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

    pub(crate) fn size_of_fields_before_size(&self, o: &Objects) -> u64 {
        let mut sum = match self.object_type {
            ContainerType::Struct => 0,
            ContainerType::CLogin(_) | ContainerType::SLogin(_) => 0,
            _ => panic!(),
        };
        for field in self.fields() {
            match field {
                StructMember::Definition(d) => {
                    match d.ty() {
                        Type::Integer(int_type) => {
                            sum += int_type.size() as u64;
                        }
                        Type::FloatingPoint(float_ty) => {
                            sum += float_ty.size() as u64;
                        }
                        Type::CString => panic!("string before size"),
                        Type::String { .. } => panic!("string before size"),
                        Type::Array(_) => panic!("array before size"),
                        Type::Identifier { s: identifier, .. } => {
                            sum += o.get_size_of_complex(identifier);
                        }
                        Type::PackedGuid => panic!("packed guid before size"),
                        Type::Guid => sum += 8_u64,
                        Type::UpdateMask => panic!(),
                        Type::AuraMask => panic!(),
                        Type::SizedCString => panic!(),
                        Type::Bool => sum += BOOL_SIZE as u64,
                        Type::DateTime => sum += DATETIME_SIZE as u64,
                    }
                    if let Some(v) = d.verified_value() {
                        if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
                            return sum;
                        }
                    }
                }
                StructMember::IfStatement(_) => panic!("if statement before size"),
                StructMember::OptionalStatement(_) => panic!("optional statement before size"),
            }
        }

        sum
    }

    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }

    pub(crate) fn sizes(&self) -> Sizes {
        self.sizes
    }

    pub(crate) fn all_definitions_transitively(&self, o: &Objects) -> Vec<StructMemberDefinition> {
        fn inner(m: &StructMember, v: &mut Vec<StructMemberDefinition>, o: &Objects, tags: &Tags) {
            match m {
                StructMember::Definition(d) => {
                    v.push(d.clone());
                    match d.ty() {
                        Type::Identifier { s, .. } => {
                            if let Object::Container(e) = o.get_object(s, tags) {
                                for m in e.fields() {
                                    inner(m, v, o, tags);
                                }
                            }
                        }
                        Type::Array(array) => {
                            if let ArrayType::Complex(s) = array.ty() {
                                if let Object::Container(e) = o.get_object(s, tags) {
                                    for m in e.fields() {
                                        inner(m, v, o, tags);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members() {
                        inner(m, v, o, tags);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, v, o, tags);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.fields() {
            inner(m, &mut v, o, self.tags());
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

        for m in self.fields() {
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

    pub(crate) fn contains_update_mask_transitively(&self, o: &Objects) -> bool {
        for d in self.all_definitions_transitively(o) {
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

    pub(crate) fn contains_aura_mask_transitively(&self, o: &Objects) -> bool {
        for d in self.all_definitions_transitively(o) {
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

    pub(crate) fn contains_guid_or_packed_guid_transitively(&self, o: &Objects) -> bool {
        for d in self.all_definitions_transitively(o) {
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

    pub(crate) fn get_types_needing_import_recursively<'a>(
        &'a self,
        o: &'a Objects,
    ) -> Vec<String> {
        let mut v = self.get_complex_types();

        let mut v2 = Vec::new();
        for t in &v {
            match o.get_object_type_of(t, self.tags()) {
                ObjectType::Struct | ObjectType::CLogin | ObjectType::SLogin => {
                    let mut types = o
                        .get_container(t, self.tags())
                        .get_types_needing_import_recursively(o);
                    v2.append(&mut types);
                }
                ObjectType::Enum | ObjectType::Flag => {}
            }
        }
        v.append(&mut v2);

        v.sort_unstable();
        v.dedup();

        v
    }

    pub(crate) fn get_types_needing_import(&self) -> Vec<String> {
        self.get_complex_types()
    }

    fn get_complex_types(&self) -> Vec<String> {
        let mut v = Vec::new();

        for d in self.all_definitions() {
            match d.struct_type() {
                Type::Array(a) => {
                    if let ArrayType::Complex(i) = a.ty() {
                        v.push(i.clone());
                    }
                }
                Type::Identifier { s, .. } => {
                    v.push(s.clone());
                }
                _ => {}
            }
        }

        v.sort_unstable();
        v.dedup();
        v
    }

    pub(crate) fn get_type_of_variable(&self, variable_name: &str) -> Type {
        for d in self.all_definitions() {
            if d.name() == variable_name {
                return d.ty().clone();
            }
        }

        panic!("unable to find type {}", variable_name)
    }

    pub(crate) fn tests(&self) -> &[TestCase] {
        &self.tests
    }

    pub(crate) fn fields(&self) -> &[StructMember] {
        self.members.as_slice()
    }
}
