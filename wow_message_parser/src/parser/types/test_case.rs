use crate::file_info::FileInfo;
use crate::parser::types::array::ArraySize;
use crate::parser::types::tags::{MemberTags, ObjectTags};
use crate::parser::types::ContainerValue;
use crate::rust_printer::UpdateMaskType;
use crate::Container;

#[derive(Debug, Clone)]
pub(crate) struct TestCase {
    subject: String,
    members: Vec<TestCaseMember>,
    raw_bytes: Vec<u8>,
    tags: ObjectTags,
    file_info: FileInfo,
}

impl TestCase {
    pub(crate) fn new(
        subject: String,
        members: Vec<TestCaseMember>,
        raw_bytes: Vec<u8>,
        tags: ObjectTags,
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

    pub(crate) fn subject(&self) -> &str {
        &self.subject
    }
    pub(crate) fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }

    pub(crate) fn tags(&self) -> &ObjectTags {
        &self.tags
    }

    pub(crate) fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub(crate) fn members(&self) -> &[TestCaseMember] {
        &self.members
    }

    pub(crate) fn get_member<'a>(t: &'a [TestCaseMember], member: &str) -> &'a TestCaseMember {
        t.iter().find(|a| a.name() == member).unwrap_or_else(|| {
            panic!("variable '{member}' not found in list of variables with values")
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TestCaseMember {
    variable_name: String,
    value: TestValue,
    tags: MemberTags,
}

impl TestCaseMember {
    pub(crate) fn name(&self) -> &str {
        &self.variable_name
    }

    pub(crate) fn value(&self) -> &TestValue {
        &self.value
    }

    pub(crate) fn tags(&self) -> &MemberTags {
        &self.tags
    }

    pub(crate) fn new(name: String, value: TestValue, tags: MemberTags) -> Self {
        Self {
            variable_name: name,
            value,
            tags,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TestUpdateMaskValue {
    ty: UpdateMaskType,
    name: String,
    value: String,
}

impl TestUpdateMaskValue {
    pub(crate) fn ty(&self) -> UpdateMaskType {
        self.ty
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn new(ty: UpdateMaskType, name: String, value: String) -> Self {
        Self { ty, name, value }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum TestValue {
    Number(ContainerValue),
    Bool(bool),
    DateTime(ContainerValue),
    Guid(ContainerValue),
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
    Enum(ContainerValue),
    SubObject {
        c: Container,
        members: Vec<TestCaseMember>,
    },
    ArrayOfSubObject(Container, Vec<Vec<TestCaseMember>>),
    UpdateMask(Vec<TestUpdateMaskValue>),
    IpAddress(ContainerValue),
}
