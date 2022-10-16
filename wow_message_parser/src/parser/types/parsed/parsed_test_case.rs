use crate::file_info::FileInfo;
use crate::parser::types::tags::MemberTags;
use crate::ObjectTags;

#[derive(Debug, Clone)]
pub(crate) struct ParsedTestCase {
    pub subject: String,
    pub members: Vec<ParsedTestCaseMember>,
    pub raw_bytes: Vec<u8>,
    pub tags: ObjectTags,
    pub file_info: FileInfo,
}

impl ParsedTestCase {
    pub(crate) fn new(
        subject: &str,
        members: Vec<ParsedTestCaseMember>,
        raw_bytes: Vec<u8>,
        tags: ObjectTags,
        file_info: FileInfo,
    ) -> Self {
        Self {
            subject: subject.to_string(),
            members,
            raw_bytes,
            tags,
            file_info,
        }
    }

    pub(crate) fn subject(&self) -> &str {
        &self.subject
    }
    pub(crate) fn tags(&self) -> &ObjectTags {
        &self.tags
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ParsedTestValue {
    Single(String),
    Multiple(Vec<ParsedTestCaseMember>),
    ArrayOfMultiple(Vec<Vec<ParsedTestCaseMember>>),
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedTestCaseMember {
    pub variable_name: String,
    pub value: ParsedTestValue,
    pub tags: MemberTags,
}

impl ParsedTestCaseMember {
    pub(crate) fn new(name: &str, value: ParsedTestValue, tags: MemberTags) -> Self {
        Self {
            variable_name: name.to_string(),
            value,
            tags,
        }
    }
}
