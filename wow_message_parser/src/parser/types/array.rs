use crate::parser::types::{IntegerType, ObjectType};
use crate::{Objects, Tags};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ArrayType {
    Integer(IntegerType),
    Complex(String),
    CString,
    Guid,
    PackedGuid,
}

impl ArrayType {
    pub(crate) fn rust_str(&self) -> String {
        match &self {
            ArrayType::Integer(i) => i.rust_str().to_string(),
            ArrayType::Complex(i) => i.clone(),
            ArrayType::CString => "String".to_string(),
            ArrayType::Guid => "Guid".to_string(),
            ArrayType::PackedGuid => "Guid".to_string(),
        }
    }

    pub(crate) fn str(&self) -> String {
        match self {
            ArrayType::Integer(i) => i.str().to_string(),
            ArrayType::Complex(i) => i.clone(),
            ArrayType::CString => "CString".to_string(),
            ArrayType::Guid => "Guid".to_string(),
            ArrayType::PackedGuid => "PackedGuid".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArraySize {
    Fixed(i64),
    Variable(String),
    Endless,
}

impl ArraySize {
    pub(crate) fn str(&self) -> String {
        match self {
            ArraySize::Fixed(i) => i.to_string(),
            ArraySize::Variable(i) => i.clone(),
            ArraySize::Endless => "-".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Array {
    inner: ArrayType,
    size: ArraySize,
}

impl Array {
    pub fn new(inner: ArrayType, size: ArraySize) -> Self {
        Self { inner, size }
    }

    pub(crate) fn new_unimplemented() -> Self {
        Self {
            inner: ArrayType::Integer(IntegerType::U8),
            size: ArraySize::Endless,
        }
    }

    pub(crate) fn ty(&self) -> &ArrayType {
        &self.inner
    }

    pub(crate) fn size(&self) -> ArraySize {
        self.size.clone()
    }

    pub(crate) fn str(&self) -> String {
        format!("{}[{}]", self.inner.str(), self.size.str())
    }

    pub(crate) fn rust_str(&self) -> String {
        match &self.size {
            ArraySize::Fixed(i) => format!("[{}; {}]", self.inner.rust_str(), i),
            ArraySize::Variable(_) | ArraySize::Endless => {
                format!("Vec<{}>", self.inner.rust_str())
            }
        }
    }

    pub(crate) fn is_byte_array(&self) -> bool {
        matches!(self.ty(), ArrayType::Integer(IntegerType::U8))
    }

    pub(crate) fn is_constant_sized_u8_array(&self) -> bool {
        match &self.size() {
            ArraySize::Fixed(_) => matches!(&self.ty(), ArrayType::Integer(IntegerType::U8)),
            ArraySize::Variable(_) => false,
            ArraySize::Endless => false,
        }
    }

    pub(crate) fn inner_type_is_constant_sized(&self, tags: &Tags, o: &Objects) -> bool {
        match self.ty() {
            ArrayType::Integer(_) | ArrayType::Guid => true,
            ArrayType::CString | ArrayType::PackedGuid => false,
            ArrayType::Complex(ident) => match o.get_object_type_of(ident, tags) {
                ObjectType::Struct => o.get_container(ident, tags).is_constant_sized(),
                ObjectType::Enum | ObjectType::Flag => true,
            },
        }
    }
}
