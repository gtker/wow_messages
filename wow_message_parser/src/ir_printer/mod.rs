mod container;
mod definer;
mod update_mask;

use crate::file_info::FileInfo;
use crate::file_utils::overwrite_if_not_same_contents;
use crate::ir_printer::container::{containers_to_ir, IrContainer};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

use crate::ir_printer::definer::{definers_to_ir, IrDefiner};
use crate::ir_printer::update_mask::IrUpdateMaskMember;
use crate::parser::types::array::ArrayType;
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::if_statement::IfStatement;
use crate::parser::types::objects::Objects;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::{MemberTags, ObjectTags};
use crate::parser::types::ty::Type;
use crate::parser::types::version::{AllVersions, LoginVersion, MajorWorldVersion, WorldVersion};
use crate::parser::types::IntegerType;
use crate::path_utils::intermediate_representation;
use crate::rust_printer::{tbc_fields, vanilla_fields, wrath_fields};

#[derive(Serialize, Clone, Debug)]
struct IrFileInfo {
    file_name: String,
    start_position: u32,
    end_position: u32,
}

impl IrFileInfo {
    pub(crate) fn from_file_info(file_info: &FileInfo) -> Self {
        IrFileInfo {
            file_name: file_info.name().to_string(),
            start_position: file_info.start_line() as u32,
            end_position: file_info.end_line() as u32,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
pub(crate) enum IrIntegerType {
    U8,
    I8,
    U16,
    U32,
    U64,
    I16,
    I32,
    I64,
    U48,
}

impl IrIntegerType {
    fn from_integer_type(v: &IntegerType) -> Self {
        match v {
            IntegerType::U8 => Self::U8,
            IntegerType::U16 => Self::U16,
            IntegerType::U32 => Self::U32,
            IntegerType::U64 => Self::U64,
            IntegerType::I32 => Self::I32,
            IntegerType::I8 => Self::I8,
            IntegerType::I16 => Self::I16,
            IntegerType::I64 => Self::I64,
            IntegerType::U48 => Self::U48,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "login_version_tag", content = "versions")]
pub(crate) enum IrLoginVersion {
    All,
    Specific(Vec<u8>),
}

impl IrLoginVersion {
    fn from_login_versions(versions: &BTreeSet<LoginVersion>) -> Self {
        if versions.contains(&LoginVersion::All) {
            return Self::All;
        }

        Self::Specific(
            versions
                .iter()
                .map(|a| match a {
                    LoginVersion::Specific(v) => *v,
                    LoginVersion::All => unreachable!(),
                })
                .collect(),
        )
    }
}
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "world_version_tag", content = "versions")]
#[serde(rename_all = "snake_case")]
pub(crate) enum IrWorldVersionOuter {
    All,
    Specific(Vec<IrWorldVersion>),
}

impl IrWorldVersionOuter {
    fn from_version(v: &BTreeSet<WorldVersion>) -> Self {
        if v.contains(&WorldVersion::All) {
            return Self::All;
        }

        Self::Specific(
            v.iter()
                .copied()
                .map(IrWorldVersion::from_world_version)
                .collect(),
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrWorldVersion {
    major: u8,
    minor: Option<u8>,
    patch: Option<u8>,
    build: Option<u16>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "version_type_tag", content = "version_type")]
pub(crate) enum IrVersions {
    Login(IrLoginVersion),
    World(IrWorldVersionOuter),
}

impl IrWorldVersion {
    fn from_world_version(v: WorldVersion) -> Self {
        let (m, i, p, b) = match v {
            WorldVersion::Major(m) => (m, None, None, None),
            WorldVersion::Minor(m, i) => (m, Some(i), None, None),
            WorldVersion::Patch(m, i, p) => (m, Some(i), Some(p), None),
            WorldVersion::Exact(m, i, p, e) => (m, Some(i), Some(p), Some(e)),
            WorldVersion::All => unreachable!(),
        };

        Self {
            major: m,
            minor: i,
            patch: p,
            build: b,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrTags {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<IrVersions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unimplemented: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compressed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_network_type: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    used_in_update_mask: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_length: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_range: Option<IrValidRange>,
}

#[derive(Clone, Debug, Serialize)]
struct IrValidRange {
    from: String,
    to: String,
}

impl IrTags {
    fn from_tags(tags: &ObjectTags) -> Self {
        let comment = tags.comment().map(|d| d.as_ir_string());

        let compressed = if tags.compressed() { Some(true) } else { None };

        let unimplemented = if tags.unimplemented() {
            Some(true)
        } else {
            None
        };

        let non_network_type = if tags.non_network_type() {
            Some(true)
        } else {
            None
        };

        let used_in_update_mask = if tags.used_in_update_mask() {
            Some(true)
        } else {
            None
        };

        let version = Some(match tags.all_versions() {
            AllVersions::Login(l) => {
                let versions = IrLoginVersion::from_login_versions(l);
                IrVersions::Login(versions)
            }
            AllVersions::World(w) => {
                let versions = IrWorldVersionOuter::from_version(w);
                IrVersions::World(versions)
            }
        });

        Self {
            comment,
            display: None,
            version,
            compressed,
            unimplemented,
            non_network_type,
            used_in_update_mask,
            maximum_length: None,
            valid_range: None,
        }
    }

    fn from_member_tags(tags: &MemberTags) -> Self {
        let comment = tags.comment().map(|d| d.as_ir_string());

        Self {
            comment,
            display: tags.display().map(|a| a.to_owned()),
            version: None,
            compressed: None,
            unimplemented: None,
            non_network_type: None,
            used_in_update_mask: None,
            maximum_length: tags.maximum_length().map(|a| a.to_string()),
            valid_range: tags.valid_range().map(|(from, to)| IrValidRange {
                from: from.to_string(),
                to: to.to_string(),
            }),
        }
    }
}

#[derive(Debug, Serialize)]
struct IrVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[derive(Debug, Serialize)]
struct TypeObjects {
    flags: Vec<IrDefiner>,
    enums: Vec<IrDefiner>,
    structs: Vec<IrContainer>,
    messages: Vec<IrContainer>,
}

impl TypeObjects {
    fn only_type(o: &Objects, predicate: impl Fn(&ObjectTags) -> bool) -> Self {
        let mut flags = definers_to_ir(o.flags().iter().filter(|a| predicate(a.tags())));
        flags.sort_by(|a, b| a.name().cmp(b.name()));

        let mut enums = definers_to_ir(o.enums().iter().filter(|a| predicate(a.tags())));
        enums.sort_by(|a, b| a.name().cmp(b.name()));

        let structs = o
            .structs()
            .iter()
            .filter(|a| predicate(a.tags()))
            .cloned()
            .collect::<Vec<_>>();
        let structs = Self::structs_in_order(&structs, o);
        let mut messages = o
            .messages()
            .iter()
            .filter(|a| predicate(a.tags()) && a.container_type() != ContainerType::Struct)
            .cloned()
            .collect::<Vec<_>>();
        messages.sort_by(|a, b| {
            a.opcode()
                .cmp(&b.opcode())
                .then(a.tags().all_versions().cmp(b.tags().all_versions()))
        });
        let messages = containers_to_ir(&messages, o);

        TypeObjects {
            flags,
            enums,
            structs,
            messages,
        }
    }

    fn structs_in_order(structs: &[Container], o: &Objects) -> Vec<IrContainer> {
        fn inner_d(out: &mut Vec<Container>, d: &StructMemberDefinition) {
            match d.ty() {
                Type::Array(array) => {
                    if let ArrayType::Struct(e) = array.ty() {
                        inner_container(out, e);
                    }
                }
                Type::Struct { e } => {
                    inner_container(out, e);
                }
                _ => {}
            }
        }

        fn inner_if(out: &mut Vec<Container>, statement: &IfStatement) {
            for m in statement.members() {
                inner(out, m);
            }

            for elseif in statement.else_ifs() {
                inner_if(out, elseif);
            }

            for m in statement.else_members() {
                inner(out, m);
            }
        }

        fn inner(out: &mut Vec<Container>, m: &StructMember) {
            match m {
                StructMember::Definition(d) => {
                    inner_d(out, d);
                }
                StructMember::IfStatement(statement) => {
                    inner_if(out, statement);
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(out, m);
                    }
                }
            }
        }

        fn inner_container(out: &mut Vec<Container>, e: &Container) {
            if e.tags().used_in_update_mask() {
                return;
            }

            if out.contains(e) {
                return;
            }

            for m in e.members() {
                inner(out, m);
            }

            if !out.contains(e) {
                out.push(e.clone());
            }
        }

        let mut out = Vec::with_capacity(structs.len());

        for e in structs {
            inner_container(&mut out, e);
        }

        containers_to_ir(&out, o)
    }
}

#[derive(Debug, Serialize)]
struct IntegerTypeInformation {
    size: u8,
    bits: u8,
}

#[derive(Debug, Serialize)]
struct IrObjects {
    version: IrVersion,
    login: TypeObjects,
    distinct_login_versions_other_than_all: BTreeSet<u8>,
    login_version_opcodes: BTreeMap<String, u8>,
    integer_type_information: BTreeMap<IrIntegerType, IntegerTypeInformation>,
    vanilla_update_mask: Vec<IrUpdateMaskMember>,
    tbc_update_mask: Vec<IrUpdateMaskMember>,
    wrath_update_mask: Vec<IrUpdateMaskMember>,
    world: TypeObjects,
}

impl IrObjects {
    fn from_regular_objects(o: &Objects) -> Self {
        let login = TypeObjects::only_type(o, |a| a.has_login_version());
        let world = TypeObjects::only_type(o, |a| a.has_world_version());

        let distinct_login_versions_other_than_all = o
            .get_login_versions_with_objects()
            .iter()
            .filter_map(|a| match a {
                LoginVersion::Specific(v) => Some(*v),
                LoginVersion::All => None,
            })
            .collect();

        let login_version_opcodes = {
            let mut v = BTreeMap::new();

            for o in o.messages().iter().filter(|a| a.tags().has_login_version()) {
                let name_without_client_or_server =
                    o.name().replace("_Client", "").replace("_Server", "");
                if let Some(c) = v.get(&name_without_client_or_server) {
                    assert_eq!(*c, o.opcode() as u8)
                } else {
                    v.insert(name_without_client_or_server, o.opcode() as u8);
                }
            }

            v
        };

        Self {
            version: IrVersion {
                major: 0,
                minor: 1,
                patch: 0,
            },
            login,
            distinct_login_versions_other_than_all,
            login_version_opcodes,
            integer_type_information: create_integer_type_information(),
            vanilla_update_mask: IrUpdateMaskMember::new_array(
                vanilla_fields::FIELDS,
                o,
                MajorWorldVersion::Vanilla,
            ),
            tbc_update_mask: IrUpdateMaskMember::new_array(
                tbc_fields::FIELDS,
                o,
                MajorWorldVersion::BurningCrusade,
            ),
            wrath_update_mask: IrUpdateMaskMember::new_array(
                wrath_fields::FIELDS,
                o,
                MajorWorldVersion::Wrath,
            ),
            world,
        }
    }
}

fn create_integer_type_information() -> BTreeMap<IrIntegerType, IntegerTypeInformation> {
    let mut v = BTreeMap::new();

    for t in IntegerType::values() {
        v.insert(
            IrIntegerType::from_integer_type(&t),
            IntegerTypeInformation {
                size: t.size(),
                bits: t.size() * 8,
            },
        );
    }

    v
}

pub(crate) fn write_intermediate_representation(o: &Objects) {
    let o = IrObjects::from_regular_objects(o);

    let json = serde_json::to_string_pretty(&o).unwrap();

    overwrite_if_not_same_contents(&json, &intermediate_representation());
}
