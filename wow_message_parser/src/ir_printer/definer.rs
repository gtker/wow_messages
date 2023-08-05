use crate::ir_printer::container::IrIntegerEnumValue;
use crate::ir_printer::{IrFileInfo, IrIntegerType, IrTags};
use crate::parser::types::definer::{Definer, DefinerField};
use crate::parser::types::if_statement::DefinerUsage;
use crate::rust_printer::DefinerType;
use serde::Serialize;
use std::collections::BTreeSet;

pub(crate) fn definers_to_ir<'a>(definers: impl Iterator<Item = &'a Definer>) -> Vec<IrDefiner> {
    definers.map(definer_to_ir).collect()
}

fn definer_to_ir(e: &Definer) -> IrDefiner {
    let fields = e
        .fields()
        .iter()
        .map(|a| -> IrDefinerField { IrDefinerField::from_definer_field(a) })
        .collect();

    let objects_used_in = e
        .objects_used_in()
        .iter()
        .map(|a| ObjectUsedIn {
            object_name: a.0.to_string(),
            definer_usage: IrDefinerUsage::from_definer_usage(a.1),
        })
        .collect();

    IrDefiner {
        name: e.name().to_string(),
        definer_type: IrDefinerType::from_definer_type(e.definer_ty()),
        enumerators: fields,
        integer_type: IrIntegerType::from_integer_type(e.ty()),
        tags: IrTags::from_tags(e.tags()),
        objects_used_in,
        file_info: IrFileInfo {
            file_name: e.file_info().name().to_string(),
            start_position: e.file_info().start_line() as u32,
            end_position: e.file_info().end_line() as u32,
        },
    }
}

#[derive(Debug, Serialize)]
enum IrDefinerType {
    Enum,
    Flag,
}

impl IrDefinerType {
    fn from_definer_type(v: DefinerType) -> Self {
        match v {
            DefinerType::Enum => IrDefinerType::Enum,
            DefinerType::Flag => IrDefinerType::Flag,
        }
    }
}

#[derive(Debug, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum IrDefinerUsage {
    RegularUse,
    InIfStatement,
}

impl IrDefinerUsage {
    fn from_definer_usage(v: DefinerUsage) -> Self {
        match v {
            DefinerUsage::NotInIf => IrDefinerUsage::RegularUse,
            DefinerUsage::InIf => IrDefinerUsage::InIfStatement,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrDefinerField {
    name: String,
    value: IrIntegerEnumValue,
    tags: IrTags,
}

impl IrDefinerField {
    fn from_definer_field(a: &DefinerField) -> Self {
        IrDefinerField {
            name: a.name().to_string(),
            value: IrIntegerEnumValue {
                value: a.value().int().to_string(),
                original_string: a.value().original().to_string(),
            },
            tags: IrTags::from_member_tags(a.tags()),
        }
    }
}

#[derive(Serialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct ObjectUsedIn {
    object_name: String,
    definer_usage: IrDefinerUsage,
}

#[derive(Serialize, Debug)]
pub(crate) struct IrDefiner {
    name: String,
    definer_type: IrDefinerType,
    enumerators: Vec<IrDefinerField>,
    integer_type: IrIntegerType,
    tags: IrTags,
    objects_used_in: BTreeSet<ObjectUsedIn>,
    file_info: IrFileInfo,
}

impl IrDefiner {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
