use crate::impl_features::{get_impl_features_for_definer, Feature};
use crate::ir_printer::{IrFileInfo, IrIntegerType, IrTags};
use crate::parser::types::definer::{Definer, DefinerField, SelfValueDefinerField};
use crate::parser::types::if_statement::DefinerUsage;
use crate::rust_printer::DefinerType;
use core::convert::From;
use core::option::Option;
use serde::Serialize;
use std::collections::BTreeSet;

pub(crate) fn definers_to_ir<'a>(definers: impl Iterator<Item = &'a Definer>) -> Vec<IrDefiner> {
    definers.map(definer_to_ir).collect()
}

fn definer_to_ir(e: &Definer) -> IrDefiner {
    let fields = e
        .fields()
        .iter()
        .map(|a| -> IrDefinerField { a.into() })
        .collect();

    let objects_used_in = e
        .objects_used_in()
        .iter()
        .map(|a| ObjectUsedIn {
            name: a.0.to_string(),
            usage: a.1.into(),
        })
        .collect();

    IrDefiner {
        name: e.name().to_string(),
        definer_ty: e.definer_ty().into(),
        enumerators: fields,
        self_value: e.self_value().as_ref().map(|a| a.into()),
        integer_type: e.ty().into(),
        tags: IrTags::from_tags(e.tags()),
        objects_used_in,
        file_info: IrFileInfo {
            file_name: e.file_info().name().to_string(),
            start_position: e.file_info().start_line(),
        },
        features: get_impl_features_for_definer(e).to_array(),
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum IrDefinerType {
    Enum,
    Flag,
}

impl From<DefinerType> for IrDefinerType {
    fn from(v: DefinerType) -> Self {
        match v {
            DefinerType::Enum => IrDefinerType::Enum,
            DefinerType::Flag => IrDefinerType::Flag,
        }
    }
}

#[derive(Debug, Serialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IrDefinerUsage {
    UsedButNotInIf,
    InIf,
}

impl From<DefinerUsage> for IrDefinerUsage {
    fn from(v: DefinerUsage) -> Self {
        match v {
            DefinerUsage::Unused => unreachable!("DefinerUsage::Unused encountered"),
            DefinerUsage::NotInIf => IrDefinerUsage::UsedButNotInIf,
            DefinerUsage::InIf => IrDefinerUsage::InIf,
        }
    }
}

#[derive(Debug, Serialize)]
struct IrSelfValueDefinerField {
    name: String,
    tags: IrTags,
}

impl From<&SelfValueDefinerField> for IrSelfValueDefinerField {
    fn from(v: &SelfValueDefinerField) -> Self {
        Self {
            name: v.name().to_string(),
            tags: IrTags::from_member_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrDefinerField {
    name: String,
    value: IrDefinerValue,
    tags: IrTags,
}

impl From<&DefinerField> for IrDefinerField {
    fn from(a: &DefinerField) -> Self {
        IrDefinerField {
            name: a.name().to_string(),
            value: IrDefinerValue {
                int: a.value().int(),
                original: a.value().original().to_string(),
            },
            tags: IrTags::from_member_tags(a.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
struct IrDefinerValue {
    int: i128,
    original: String,
}

#[derive(Serialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct ObjectUsedIn {
    name: String,
    usage: IrDefinerUsage,
}

#[derive(Serialize, Debug)]
pub(crate) struct IrDefiner {
    name: String,
    definer_ty: IrDefinerType,
    enumerators: Vec<IrDefinerField>,
    self_value: Option<IrSelfValueDefinerField>,
    integer_type: IrIntegerType,
    tags: IrTags,
    objects_used_in: BTreeSet<ObjectUsedIn>,
    file_info: IrFileInfo,
    features: Vec<Feature>,
}

impl IrDefiner {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
