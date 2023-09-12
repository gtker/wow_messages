use crate::file_info::FileInfo;
use crate::file_utils::{
    get_base_internal_shared_path, get_base_shared_path, get_import_path, get_world_shared_path,
};
use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::compare_name_and_tags;
use crate::parser::types::definer::Definer;
use crate::parser::types::objects::Objects;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::test_case::TestCase;
use crate::parser::types::ty::Type;
use crate::parser::types::version::{AllRustVersions, LoginVersion, MajorWorldVersion, Version};
use crate::rust_printer::rust_view::rust_object::RustObject;
use crate::rust_printer::{
    DefinerType, LOGIN_CLIENT_MESSAGE_ENUM_NAME, LOGIN_SERVER_MESSAGE_ENUM_NAME,
    WORLD_CLIENT_MESSAGE_ENUM_NAME, WORLD_SERVER_MESSAGE_ENUM_NAME,
};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

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
    objects_used_in: Option<BTreeSet<String>>,
}

impl PartialEq<Self> for Container {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.tags().all_versions() == other.tags().all_versions()
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
        let self_first = self.tags().all_versions();
        let other_first = other.tags().all_versions();

        compare_name_and_tags(&self.name, self_first, &other.name, other_first)
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
        objects_used_in: Option<BTreeSet<String>>,
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
            objects_used_in,
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

    pub(crate) fn manual_size_field_subtraction(&self) -> Option<u16> {
        match self.object_type {
            ContainerType::Struct => None,
            ContainerType::CLogin(_) | ContainerType::SLogin(_) => self
                .members()
                .iter()
                .find(|a| a.size_of_fields_before_size().is_some())
                .and_then(|a| a.size_of_fields_before_size())
                .map(|a| a as u16),
            ContainerType::Msg(_) | ContainerType::CMsg(_) | ContainerType::SMsg(_) => None,
        }
    }

    pub(crate) fn enum_separate_if_statement_variables(&self) -> Vec<String> {
        let mut v = Vec::new();

        for m in self.all_members() {
            match m {
                StructMember::Definition(_) => {}
                StructMember::IfStatement(statement) => {
                    if statement.part_of_separate_if_statement()
                        && statement.definer_type() == DefinerType::Enum
                    {
                        for d in statement.all_definitions() {
                            let variable_name = statement.name();
                            let definition_name = d.name();
                            v.push(format!("{variable_name}_if_{definition_name}"));
                        }
                    }
                }
                StructMember::OptionalStatement(_) => {}
            }
        }

        v
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

        format!("{import_path}::opcodes::{enum_name}",)
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

    pub(crate) fn objects_used_in(&self) -> Option<&BTreeSet<String>> {
        self.objects_used_in.as_ref()
    }

    pub(crate) fn rust_object(&self) -> &RustObject {
        &self.rust_object_view
    }

    pub(crate) fn tags(&self) -> &ObjectTags {
        &self.tags
    }

    pub(crate) fn sizes(&self) -> Sizes {
        self.sizes
    }

    pub(crate) fn all_definitions_transitively(&self) -> Vec<&StructMemberDefinition> {
        fn inner<'a>(m: &'a StructMember, v: &mut Vec<&'a StructMemberDefinition>) {
            match m {
                StructMember::Definition(d) => {
                    v.push(d);
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

    pub(crate) fn all_members(&self) -> Vec<&StructMember> {
        fn inner<'a>(m: &'a StructMember, v: &mut Vec<&'a StructMember>) {
            v.push(m);
            match m {
                StructMember::Definition(_) => {}
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

    pub(crate) fn all_definers(&self) -> Vec<&Definer> {
        let mut v = Vec::new();
        for d in self.all_definitions_transitively() {
            match d.ty() {
                Type::Enum { e, .. } => v.push(e),
                Type::Flag { e, .. } => v.push(e),
                _ => {}
            }
        }

        v.sort_by(|a, b| a.name().cmp(b.name()));

        v
    }

    pub(crate) fn contains_update_mask_transitively(&self) -> bool {
        for d in self.all_definitions_transitively() {
            if matches!(d.ty(), &Type::UpdateMask { .. }) {
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

    pub(crate) fn contains_compressed_variable(&self) -> bool {
        self.all_definitions().iter().any(|a| match a.ty() {
            Type::Array(array) => array.compressed(),
            _ => false,
        })
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

    pub(crate) fn cfg_guard(&self) -> Option<String> {
        match self.tags().all_rust_versions() {
            AllRustVersions::World(l) => {
                if l.len() == 1 {
                    Some(format!(
                        "#[cfg(feature = \"{}\")]",
                        l.first().unwrap().module_name()
                    ))
                } else {
                    use std::fmt::Write;

                    let mut s = "#[cfg(any(".to_string();

                    for (i, v) in l.iter().enumerate() {
                        if i != 0 {
                            write!(s, ", ").unwrap();
                        }

                        write!(s, "feature = \"{}\"", v.module_name()).unwrap();
                    }

                    write!(s, "))]").unwrap();

                    Some(s)
                }
            }
            AllRustVersions::Login(_) => None,
        }
    }

    pub(crate) fn base_read_write_full_import_path(&self) -> String {
        let versions = self
            .tags()
            .world_versions()
            .map(|a| a.try_as_major_world().unwrap())
            .collect::<Vec<_>>();

        if versions.len() == 1 {
            format!(
                "wow_world_base::{}::{}",
                versions[0].module_name(),
                self.name()
            )
        } else {
            format!(
                "{}::{}",
                get_base_shared_path(self.name(), self.tags()),
                self.name()
            )
        }
    }

    pub(crate) fn get_import_path_from_world(&self) -> String {
        let t = self.tags();

        if t.has_world_version() && t.shared() {
            if t.is_in_base() {
                get_base_shared_path(self.name(), t)
            } else {
                get_world_shared_path(self.name(), t)
            }
        } else {
            let (v, _) = t.first_and_main_versions();
            let version = if t.has_login_version() {
                t.import_version()
            } else {
                v
            };

            get_import_path(version)
        }
    }

    pub(crate) fn get_imports(
        &self,
        version: Version,
        should_print_transitive_imports: bool,
    ) -> BTreeMap<String, BTreeSet<String>> {
        fn object_prefix(s: &ObjectTags, e: &ObjectTags, version: Version, name: &str) -> String {
            if s.has_world_version() && s.shared() {
                if e.is_in_base() && s.is_in_base() {
                    get_base_internal_shared_path(name, e)
                } else if e.is_in_base() {
                    get_base_shared_path(name, e)
                } else {
                    get_world_shared_path(name, e)
                }
            } else {
                let version = if !version.is_world() {
                    e.import_version()
                } else {
                    version
                };

                get_import_path(version)
            }
        }

        let mut v: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        let members = if should_print_transitive_imports {
            self.all_definitions_transitively()
        } else {
            self.all_definitions()
        };
        for a in members {
            let name = a.ty().rust_str();

            let (prefix, ty) = match a.ty() {
                Type::MonsterMoveSplines
                | Type::CString
                | Type::SizedCString
                | Type::String
                | Type::Integer(_)
                | Type::Bool(_)
                | Type::FloatingPoint => continue,

                Type::DateTime | Type::PackedGuid | Type::Guid => ("crate".to_string(), name),

                Type::VariableItemRandomProperty
                | Type::NamedGuid
                | Type::EnchantMask
                | Type::InspectTalentGearMask
                | Type::AuraMask
                | Type::UpdateMask { .. } => (
                    format!("crate::{}", version.as_major_world().module_name()),
                    name,
                ),

                Type::AchievementDoneArray => (
                    format!("crate::{}", version.as_major_world().module_name()),
                    "AchievementDone".to_string(),
                ),
                Type::AchievementInProgressArray => (
                    format!("crate::{}", version.as_major_world().module_name()),
                    "AchievementInProgress".to_string(),
                ),
                Type::AddonArray => {
                    let tags = ObjectTags::new_with_world_versions(&[
                        MajorWorldVersion::BurningCrusade,
                        MajorWorldVersion::Wrath,
                    ]);

                    let pre = object_prefix(self.tags(), &tags, version, "Addon");
                    (pre, "Addon".to_string())
                }

                Type::Seconds | Type::Milliseconds => ("std::time".to_string(), name),
                Type::IpAddress => ("std::net".to_string(), name),

                Type::Population => ("crate::all::population".to_string(), name),

                Type::Gold => (format!("crate::shared::gold_vanilla_tbc_wrath"), name),
                Type::Level | Type::Level16 | Type::Level32 => {
                    (format!("crate::shared::level_vanilla_tbc_wrath"), name)
                }

                Type::Array(array) => {
                    let name = array.rust_str_inner();
                    match array.ty() {
                        ArrayType::CString | ArrayType::Integer(_) => continue,

                        ArrayType::Guid | ArrayType::PackedGuid => ("crate".to_string(), name),

                        ArrayType::Struct(e) => {
                            let pre = object_prefix(self.tags(), e.tags(), version, e.name());
                            (pre, name)
                        }
                    }
                }

                Type::Enum { e, .. } | Type::Flag { e, .. } => {
                    let pre = object_prefix(self.tags(), e.tags(), version, e.name());
                    (pre, name)
                }
                Type::Struct { e } => {
                    let pre = object_prefix(self.tags(), e.tags(), version, e.name());
                    (pre, name)
                }
            };

            if let Some(tys) = v.get_mut(&prefix) {
                tys.insert(ty);
            } else {
                let mut tys = BTreeSet::new();
                tys.insert(ty);

                v.insert(prefix, tys);
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

        panic!("unable to find variable name {variable_name}")
    }

    pub(crate) fn tests<'a>(&self, o: &'a Objects) -> Vec<&'a TestCase> {
        o.tests()
            .iter()
            .filter(|a| a.subject() == self.name() && self.tags().fulfills_all(a.tags()))
            .collect()
    }

    pub(crate) fn should_print_test_case_string(&self, o: &Objects) -> bool {
        self.tests(o).is_empty()
    }

    pub(crate) fn members(&self) -> &[StructMember] {
        self.members.as_slice()
    }

    pub(crate) fn create_update_mask_members(&self) -> Vec<Vec<UpdateMaskMember>> {
        let mut members = Vec::new();

        let mut temp_definitions = Vec::new();

        let mut offset = 0;

        for m in self.members() {
            match m {
                StructMember::Definition(d) => {
                    if d.value().is_some() {
                        members.push(vec![]);

                        offset = 0;
                        continue;
                    }

                    match d.ty() {
                        Type::Guid => {
                            assert_eq!(offset, 0);

                            temp_definitions.push(UpdateMaskMember {
                                member: d.clone(),
                                offset: 0,
                                size: 8,
                            });

                            members.push(temp_definitions.clone());

                            temp_definitions.clear();

                            // Guids take 8 bytes, so next int is empty
                            members.push(vec![]);
                        }

                        Type::FloatingPoint
                        | Type::DateTime
                        | Type::Gold
                        | Type::Level32
                        | Type::Milliseconds
                        | Type::Seconds => {
                            assert_eq!(offset, 0);

                            temp_definitions.push(UpdateMaskMember {
                                member: d.clone(),
                                offset: 0,
                                size: 4,
                            });

                            members.push(temp_definitions.clone());

                            temp_definitions.clear();
                        }

                        Type::Bool(i) | Type::Integer(i) => {
                            temp_definitions.push(UpdateMaskMember {
                                member: d.clone(),
                                offset,
                                size: i.size() as i32,
                            });

                            offset += i.size() as i32;
                        }

                        Type::Level => {
                            temp_definitions.push(UpdateMaskMember {
                                member: d.clone(),
                                offset,
                                size: 1,
                            });

                            offset += 1;
                        }
                        Type::Level16 => {
                            temp_definitions.push(UpdateMaskMember {
                                member: d.clone(),
                                offset,
                                size: 2,
                            });

                            offset += 2;
                        }

                        Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
                            let size = if let Some(i) = upcast {
                                i.size()
                            } else {
                                e.ty().size()
                            } as i32;

                            temp_definitions.push(UpdateMaskMember {
                                member: d.clone(),
                                offset,
                                size,
                            });

                            offset += size;
                        }

                        Type::Array(array) => match array.size() {
                            ArraySize::Fixed(size) => {
                                let inner_size = {
                                    let s = array.ty().sizes();
                                    assert!(s.is_constant().is_some());

                                    s.maximum()
                                };

                                let size = (size * inner_size) as i32;

                                temp_definitions.push(UpdateMaskMember {
                                    member: d.clone(),
                                    offset,
                                    size,
                                });

                                if size < 4 {
                                    offset += size;
                                } else if size == 4 {
                                    assert_eq!(offset, 0);

                                    members.push(temp_definitions.clone());

                                    temp_definitions.clear();
                                } else {
                                    assert_eq!(size % 4, 0);

                                    members.push(temp_definitions.clone());
                                    temp_definitions.clear();

                                    for _ in 0..((size / 4) - 1) {
                                        members.push(vec![]);
                                    }

                                    offset = 0;
                                }
                            }
                            ArraySize::Variable(_) | ArraySize::Endless => {
                                unreachable!("variable and endless array for update mask")
                            }
                        },

                        Type::IpAddress
                        | Type::PackedGuid
                        | Type::NamedGuid
                        | Type::CString
                        | Type::SizedCString
                        | Type::String
                        | Type::AuraMask
                        | Type::AchievementDoneArray
                        | Type::AchievementInProgressArray
                        | Type::EnchantMask
                        | Type::InspectTalentGearMask
                        | Type::Population
                        | Type::VariableItemRandomProperty
                        | Type::AddonArray
                        | Type::Struct { .. }
                        | Type::UpdateMask { .. }
                        | Type::MonsterMoveSplines => {
                            unreachable!("invalid update mask struct member")
                        }
                    }
                }
                StructMember::IfStatement(_) | StructMember::OptionalStatement(_) => {
                    unreachable!("optional or if in update mask struct")
                }
            }

            if offset >= 4 {
                members.push(temp_definitions.clone());

                temp_definitions.clear();

                assert_eq!(offset, 4);

                offset = 0;
            }
        }

        members
    }
}
#[derive(Clone, Debug)]
pub(crate) struct UpdateMaskMember {
    pub member: StructMemberDefinition,
    pub offset: i32,
    pub size: i32,
}
