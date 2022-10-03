use crate::file_info::FileInfo;
use crate::parser::types::test_case::TestValue;
use crate::Tags;

#[derive(Debug, Clone)]
pub struct ParsedTestCase {
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
pub enum TestCaseValueInitial {
    Single(String),
    Multiple(Vec<ParsedTestCaseMember>),
    ArrayOfMultiple(Vec<Vec<ParsedTestCaseMember>>),
}

#[derive(Debug, Clone)]
pub struct ParsedTestCaseMember {
    pub variable_name: String,
    pub value: TestCaseValueInitial,
    pub verified_value: Option<TestValue>,
    pub tags: Tags,
}

impl ParsedTestCaseMember {
    pub(crate) fn new(name: &str, value: TestCaseValueInitial, tags: Tags) -> Self {
        Self {
            variable_name: name.to_string(),
            value,
            verified_value: None,
            tags,
        }
    }
}
