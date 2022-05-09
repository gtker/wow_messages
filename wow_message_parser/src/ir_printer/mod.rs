mod container;
mod definer;

use crate::file_utils::write_string_to_file;
use crate::ir_printer::container::{containers_to_ir, IrContainer};
use serde::Serialize;
use std::path::Path;

use crate::ir_printer::definer::{definers_to_ir, IrDefiner};
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::Tags;
use crate::parser::types::{Endianness, IntegerType};

#[derive(Serialize, Debug)]
struct IrFileInfo {
    file_name: String,
    start_position: usize,
}

#[derive(Debug, Serialize)]
pub enum IrEndianness {
    Little,
    Big,
}

impl From<&Endianness> for IrEndianness {
    fn from(v: &Endianness) -> Self {
        match v {
            Endianness::Little => IrEndianness::Little,
            Endianness::Big => IrEndianness::Big,
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum IrIntegerType {
    u8,
    u16(IrEndianness),
    u32(IrEndianness),
    u64(IrEndianness),
}

impl From<&IntegerType> for IrIntegerType {
    fn from(v: &IntegerType) -> Self {
        match v {
            IntegerType::U8 => IrIntegerType::u8,
            IntegerType::U16(e) => IrIntegerType::u16(e.into()),
            IntegerType::U32(e) => IrIntegerType::u32(e.into()),
            IntegerType::U64(e) => IrIntegerType::u64(e.into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrTags {
    description: Option<String>,
    comment: Option<String>,
    display: Option<String>,
}

impl IrTags {
    fn from_tags(tags: &Tags) -> Self {
        Self {
            description: tags.description().map(|a| a.to_owned()),
            comment: tags.comment().map(|a| a.to_owned()),
            display: tags.display().map(|a| a.to_owned()),
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
        let mut containers = containers_to_ir(&o.all_containers().collect::<Vec<_>>());
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

pub fn write_intermediate_representation(o: &Objects) {
    const IR_PATH: &str = "intermediate_representation.json";

    let o = IrObjects::from_regular_objects(o);

    let json = serde_json::to_string_pretty(&o).unwrap();

    write_string_to_file(&json, Path::new(IR_PATH));
}
