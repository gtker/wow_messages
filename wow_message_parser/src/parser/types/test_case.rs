use crate::file_info::FileInfo;
use crate::parser::types::tags::Tags;
use crate::parser::types::{ArraySize, VerifiedContainerValue};
use crate::rust_printer::UpdateMaskType;

#[derive(Debug, Clone)]
pub struct TestCase {
    subject: String,
    members: Vec<TestCaseMember>,
    raw_bytes: Vec<u8>,
    tags: Tags,
    file_info: FileInfo,
}

impl TestCase {
    pub fn new(
        subject: String,
        members: Vec<TestCaseMember>,
        raw_bytes: Vec<u8>,
        tags: Tags,
        file_info: FileInfo,
    ) -> Self {
        Self {
            subject,
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

    pub fn members(&self) -> &[TestCaseMember] {
        &self.members
    }

    pub fn get_member<'a>(t: &'a [TestCaseMember], member: &str) -> &'a TestCaseMember {
        t.iter().find(|a| a.name() == member).unwrap_or_else(|| {
            panic!(
                "variable '{}' not found in list of variables with values",
                member
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct TestCaseMember {
    variable_name: String,
    value: TestValue,
    tags: Tags,
}

impl TestCaseMember {
    pub fn name(&self) -> &str {
        &self.variable_name
    }

    pub fn value(&self) -> &TestValue {
        &self.value
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

    pub fn new(name: String, value: TestValue, tags: Tags) -> Self {
        Self {
            variable_name: name,
            value,
            tags,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestUpdateMaskValue {
    ty: UpdateMaskType,
    name: String,
    value: String,
}

impl TestUpdateMaskValue {
    pub fn ty(&self) -> UpdateMaskType {
        self.ty
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn new(ty: UpdateMaskType, name: String, value: String) -> Self {
        Self { ty, name, value }
    }
}

#[derive(Debug, Clone)]
pub enum TestValue {
    Number(VerifiedContainerValue),
    Bool(bool),
    DateTime(VerifiedContainerValue),
    Guid(VerifiedContainerValue),
    FloatingNumber {
        value: f64,
        original_string: String,
    },
    Array {
        values: Vec<usize>,
        size: ArraySize,
    },
    String(String),
    Flag(Vec<String>),
    Enum(VerifiedContainerValue),
    SubObject {
        ty_name: String,
        members: Vec<TestCaseMember>,
    },
    ArrayOfSubObject(String, Vec<Vec<TestCaseMember>>),
    UpdateMask(Vec<TestUpdateMaskValue>),
}

impl TestValue {
    pub fn value(&self) -> &VerifiedContainerValue {
        match self {
            TestValue::Number(i) => i,
            _ => panic!(),
        }
    }

    pub fn original_string(&self) -> &str {
        match self {
            TestValue::Number(i) => i.original_string(),
            TestValue::Enum(i) => i.original_string(),
            TestValue::FloatingNumber {
                original_string, ..
            } => original_string.as_str(),
            TestValue::String(s) => s,
            _ => panic!(),
        }
    }
}
