use crate::parser::types::tags::ObjectTags;

#[derive(Debug, Clone)]
pub(crate) struct ParsedUpdateMaskField {
    object_type: String,
    name: String,
    index_type: String,
    offset: i32,
    tags: ObjectTags,
}

impl ParsedUpdateMaskField {
    pub(crate) fn new(
        object_type: String,
        name: String,
        index_type: String,
        offset: i32,
        tags: ObjectTags,
    ) -> Self {
        Self {
            object_type,
            name,
            index_type,
            offset,
            tags,
        }
    }

    pub(crate) fn object_type(&self) -> &str {
        &self.object_type
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn index_type(&self) -> &str {
        &self.index_type
    }

    pub(crate) fn offset(&self) -> i32 {
        self.offset
    }

    pub(crate) fn tags(&self) -> &ObjectTags {
        &self.tags
    }
}
