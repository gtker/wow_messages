use crate::file_info::FileInfo;
use crate::parser::types::definer::{DefinerField, SelfValueDefinerField};
use crate::parser::types::tags::Tags;
use crate::parser::types::IntegerType;
use crate::rust_printer::DefinerType;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParsedDefiner {
    pub name: String,
    pub definer_ty: DefinerType,
    pub fields: Vec<DefinerField>,
    pub self_value: Option<SelfValueDefinerField>,
    pub basic_type: IntegerType,
    pub tags: Tags,
    pub file_info: FileInfo,
}
impl ParsedDefiner {
    pub fn new(
        name: &str,
        definer_ty: DefinerType,
        fields: Vec<DefinerField>,
        basic_type: IntegerType,
        self_value: Option<SelfValueDefinerField>,
        tags: Tags,
        file_info: FileInfo,
    ) -> Self {
        Self {
            name: name.to_string(),
            definer_ty,
            fields,
            self_value,
            basic_type,
            tags,
            file_info,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub fn self_check(&self) {
        let mut h = HashMap::new();

        for field in &self.fields {
            if let Some(other) = h.get(&field.value().int()) {
                panic!(
                    "Definer '{}' already has field with value '{}' ('{}'), '{}' must not overlap. {:?}",
                    self.name(),
                    field.value().int(),
                    other,
                    field.name(),
                    self.file_info()
                )
            }

            h.insert(field.value().int(), field.name().to_string());
        }
    }
}
