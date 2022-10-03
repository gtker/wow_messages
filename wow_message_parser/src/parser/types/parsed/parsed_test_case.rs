use crate::file_info::FileInfo;
use crate::Tags;

#[derive(Debug, Clone)]
pub(crate) struct ParsedTestCase {
    pub subject: String,
    pub members: Vec<ParsedTestCaseMember>,
    pub raw_bytes: Vec<u8>,
    pub tags: Tags,
    pub file_info: FileInfo,
}

impl ParsedTestCase {
    pub(crate) fn new(
        subject: &str,
        members: Vec<ParsedTestCaseMember>,
        raw_bytes: Vec<u8>,
        tags: Tags,
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
    pub(crate) fn tags(&self) -> &Tags {
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
    pub tags: Tags,
}

impl ParsedTestCaseMember {
    pub(crate) fn new(name: &str, value: ParsedTestValue, tags: Tags) -> Self {
        Self {
            variable_name: name.to_string(),
            value,
            tags,
        }
    }
}
