use crate::parser::types::IntegerType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ParsedArrayType {
    Integer(IntegerType),
    Complex(String),
    CString,
    Guid,
    PackedGuid,
}

impl ParsedArrayType {
    pub(crate) fn str(&self) -> String {
        match self {
            ParsedArrayType::Integer(i) => i.str().to_string(),
            ParsedArrayType::Complex(i) => i.clone(),
            ParsedArrayType::CString => "CString".to_string(),
            ParsedArrayType::Guid => "Guid".to_string(),
            ParsedArrayType::PackedGuid => "PackedGuid".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParsedArraySize {
    Fixed(i64),
    Variable(String),
    Endless,
}

impl ParsedArraySize {
    pub(crate) fn str(&self) -> String {
        match self {
            ParsedArraySize::Fixed(i) => i.to_string(),
            ParsedArraySize::Variable(i) => i.clone(),
            ParsedArraySize::Endless => "-".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ParsedArray {
    inner: ParsedArrayType,
    size: ParsedArraySize,
}

impl ParsedArray {
    pub fn new(inner: ParsedArrayType, size: ParsedArraySize) -> Self {
        Self { inner, size }
    }

    pub(crate) fn new_unimplemented() -> Self {
        Self {
            inner: ParsedArrayType::Integer(IntegerType::U8),
            size: ParsedArraySize::Endless,
        }
    }

    pub(crate) fn ty(&self) -> &ParsedArrayType {
        &self.inner
    }

    pub(crate) fn size(&self) -> ParsedArraySize {
        self.size.clone()
    }

    pub(crate) fn str(&self) -> String {
        format!("{}[{}]", self.inner.str(), self.size.str())
    }
}
