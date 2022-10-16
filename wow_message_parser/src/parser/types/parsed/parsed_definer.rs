use crate::file_info::FileInfo;
use crate::parser::types::definer::{DefinerField, SelfValueDefinerField};
use crate::parser::types::IntegerType;
use crate::{DefinerType, ObjectTags};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ParsedDefiner {
    pub name: String,
    pub definer_ty: DefinerType,
    pub fields: Vec<DefinerField>,
    pub self_value: Option<SelfValueDefinerField>,
    pub basic_type: IntegerType,
    pub tags: ObjectTags,
    pub file_info: FileInfo,
}
impl ParsedDefiner {
    pub(crate) fn new(
        name: &str,
        definer_ty: DefinerType,
        fields: Vec<DefinerField>,
        basic_type: IntegerType,
        self_value: Option<SelfValueDefinerField>,
        tags: ObjectTags,
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

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn tags(&self) -> &ObjectTags {
        &self.tags
    }
}
