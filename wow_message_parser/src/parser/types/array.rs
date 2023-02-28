use crate::parser::types::struct_member::StructMemberDefinition;
use crate::parser::types::IntegerType;
use crate::Container;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ArrayType {
    Integer(IntegerType),
    Struct(Box<Container>),
    CString,
    Guid,
    PackedGuid,
}

impl ArrayType {
    pub(crate) fn rust_str(&self) -> String {
        match &self {
            ArrayType::Integer(i) => i.rust_str().to_string(),
            ArrayType::Struct(c) => c.name().to_string(),
            ArrayType::CString => "String".to_string(),
            ArrayType::Guid => "Guid".to_string(),
            ArrayType::PackedGuid => "Guid".to_string(),
        }
    }

    pub(crate) fn str(&self) -> String {
        match self {
            ArrayType::Integer(i) => i.str().to_string(),
            ArrayType::Struct(i) => i.name().to_string(),
            ArrayType::CString => "CString".to_string(),
            ArrayType::Guid => "Guid".to_string(),
            ArrayType::PackedGuid => "PackedGuid".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ArraySize {
    Fixed(i64),
    Variable(Box<StructMemberDefinition>),
    Endless,
}

impl ArraySize {
    pub(crate) fn str(&self) -> String {
        match self {
            ArraySize::Fixed(i) => i.to_string(),
            ArraySize::Variable(m) => m.name().to_string(),
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

    pub(crate) fn inner_type_is_copy(&self) -> bool {
        match self.ty() {
            ArrayType::PackedGuid | ArrayType::Integer(_) | ArrayType::Guid => true,
            ArrayType::Struct(c) => c.is_constant_sized(),

            ArrayType::CString => false,
        }
    }
}
