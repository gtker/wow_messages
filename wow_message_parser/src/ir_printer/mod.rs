mod container;
mod definer;

use crate::file_utils::overwrite_if_not_same_contents;
use crate::ir_printer::container::{containers_to_ir, IrContainer};
use serde::Serialize;
use std::path::Path;

use crate::ir_printer::definer::{definers_to_ir, IrDefiner};
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::{MemberTags, ObjectTags};
use crate::parser::types::version::{AllVersions, LoginVersion, WorldVersion};
use crate::parser::types::{Endianness, IntegerType};

#[derive(Serialize, Debug)]
struct IrFileInfo {
    file_name: String,
    start_position: usize,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub(crate) enum IrEndianness {
    little,
    big,
}

impl From<&Endianness> for IrEndianness {
    fn from(v: &Endianness) -> Self {
        match v {
            Endianness::Little => IrEndianness::little,
            Endianness::Big => IrEndianness::big,
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "type", content = "endianness")]
pub(crate) enum IrIntegerType {
    u8,
    u16(IrEndianness),
    u32(IrEndianness),
    u64(IrEndianness),
    i32(IrEndianness),
}

impl From<&IntegerType> for IrIntegerType {
    fn from(v: &IntegerType) -> Self {
        match v {
            IntegerType::U8 => IrIntegerType::u8,
            IntegerType::U16(e) => IrIntegerType::u16(e.into()),
            IntegerType::U32(e) => IrIntegerType::u32(e.into()),
            IntegerType::U64(e) => IrIntegerType::u64(e.into()),
            IntegerType::I32(e) => IrIntegerType::i32(e.into()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "version")]
pub(crate) enum IrLoginVersion {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "specific")]
    Specific(u8),
}

impl From<LoginVersion> for IrLoginVersion {
    fn from(v: LoginVersion) -> Self {
        match v {
            LoginVersion::Specific(s) => Self::Specific(s),
            LoginVersion::All => Self::All,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "version")]
pub(crate) enum IrWorldVersion {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "major")]
    Major { major: u8 },
    #[serde(rename = "minor")]
    Minor { major: u8, minor: u8 },
    #[serde(rename = "patch")]
    Patch { major: u8, minor: u8, patch: u8 },
    #[serde(rename = "exact")]
    Exact {
        major: u8,
        minor: u8,
        patch: u8,
        exact: u16,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "versions")]
pub(crate) enum IrVersions {
    #[serde(rename = "login")]
    Login(Vec<IrLoginVersion>),
    #[serde(rename = "world")]
    World(Vec<IrWorldVersion>),
}

impl From<WorldVersion> for IrWorldVersion {
    fn from(v: WorldVersion) -> Self {
        match v {
            WorldVersion::Major(m) => Self::Major { major: m },
            WorldVersion::Minor(m, i) => Self::Minor { major: m, minor: i },
            WorldVersion::Patch(m, i, p) => Self::Patch {
                major: m,
                minor: i,
                patch: p,
            },
            WorldVersion::Exact(m, i, p, e) => Self::Exact {
                major: m,
                minor: i,
                patch: p,
                exact: e,
            },
            WorldVersion::All => Self::All,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrTags {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<IrVersions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unimplemented: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compressed: Option<String>,
}

impl IrTags {
    fn from_tags(tags: &ObjectTags) -> Self {
        let description = tags.description().map(|d| d.as_ir_string());

        let comment = tags.comment().map(|d| d.as_ir_string());

        let compressed = if tags.compressed() {
            Some("true".to_string())
        } else {
            None
        };

        let unimplemented = if tags.unimplemented() {
            Some("true".to_string())
        } else {
            None
        };

        let version = Some(match tags.all_versions() {
            AllVersions::Login(l) => IrVersions::Login(l.iter().map(|a| (*a).into()).collect()),
            AllVersions::World(w) => IrVersions::World(w.iter().map(|a| (*a).into()).collect()),
        });

        Self {
            description,
            comment,
            display: None,
            version,
            compressed,
            unimplemented,
        }
    }

    fn from_member_tags(tags: &MemberTags) -> Self {
        let description = tags.description().map(|d| d.as_ir_string());

        let comment = tags.comment().map(|d| d.as_ir_string());

        let compressed = tags.compressed().map(|d| d.to_string());

        Self {
            description,
            comment,
            display: tags.display().map(|a| a.to_owned()),
            version: None,
            compressed,
            unimplemented: None,
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
struct IrObjects {
    version: IrVersion,
    flags: Vec<IrDefiner>,
    enums: Vec<IrDefiner>,
    containers: Vec<IrContainer>,
}

impl IrObjects {
    fn from_regular_objects(o: &Objects) -> Self {
        let mut flags = definers_to_ir(o.flags());
        flags.sort_by(|a, b| a.name().cmp(b.name()));
        let mut enums = definers_to_ir(o.enums());
        enums.sort_by(|a, b| a.name().cmp(b.name()));
        let mut containers = containers_to_ir(&o.all_containers().collect::<Vec<_>>(), o);
        containers.sort_by(|a, b| a.name().cmp(b.name()));

        Self {
            version: IrVersion {
                major: 0,
                minor: 0,
                patch: 0,
            },
            flags,
            enums,
            containers,
        }
    }
}

pub(crate) fn write_intermediate_representation(o: &Objects) {
    const IR_PATH: &str = "intermediate_representation.json";

    let o = IrObjects::from_regular_objects(o);

    let json = serde_json::to_string_pretty(&o).unwrap();

    overwrite_if_not_same_contents(&json, Path::new(IR_PATH));
}
