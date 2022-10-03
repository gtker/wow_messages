use crate::file_info::FileInfo;
use crate::parser::types::tags::Tags;
use crate::parser::types::test_case::TestValue;

#[derive(Debug, Clone)]
pub struct ParsedTestCase {
    pub subject: String,
    pub members: Vec<ParsedTestCaseMember>,
    pub raw_bytes: Vec<u8>,
    pub tags: Tags,
    pub file_info: FileInfo,
}

impl ParsedTestCase {
    pub fn new(
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

    pub fn subject(&self) -> &str {
        &self.subject
    }
    pub fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub fn members(&self) -> &[ParsedTestCaseMember] {
        &self.members
    }

    pub fn get_member<'a>(t: &'a [ParsedTestCaseMember], member: &str) -> &'a ParsedTestCaseMember {
        t.iter().find(|a| a.name() == member).unwrap_or_else(|| {
            panic!(
                "variable '{}' not found in list of variables with values",
                member
            )
        })
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
    pub fn name(&self) -> &str {
        &self.variable_name
    }

    pub fn value(&self) -> &TestValue {
        self.verified_value.as_ref().unwrap()
    }

    pub fn float_value(&self) -> f64 {
        match self.value() {
            TestValue::FloatingNumber { value, .. } => *value,
            _ => panic!(),
        }
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn new(name: &str, value: TestCaseValueInitial, tags: Tags) -> Self {
        Self {
            variable_name: name.to_string(),
            value,
            verified_value: None,
            tags,
        }
    }
}
