use crate::parser::types::IntegerType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ParsedArrayType {
    Integer(IntegerType),
    Complex(String),
    CString,
    Guid,
    PackedGuid,
    Spell,
}

impl ParsedArrayType {
    pub(crate) fn str(&self) -> String {
        match self {
            ParsedArrayType::Integer(i) => i.str().to_string(),
            ParsedArrayType::Complex(i) => i.clone(),
            ParsedArrayType::CString => "CString".to_string(),
            ParsedArrayType::Guid => "Guid".to_string(),
            ParsedArrayType::PackedGuid => "PackedGuid".to_string(),
            ParsedArrayType::Spell => "Spell".to_string(),
        }
    }

    pub(crate) fn rust_str(&self) -> String {
        match self {
            ParsedArrayType::Integer(i) => i.rust_str().to_string(),
            ParsedArrayType::Complex(i) => i.clone(),
            ParsedArrayType::CString => "String".to_string(),
            ParsedArrayType::PackedGuid | ParsedArrayType::Guid => "Guid".to_string(),
            ParsedArrayType::Spell => "u32".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParsedArraySize {
    Fixed(i128),
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
    pub compressed: bool,
}

impl ParsedArray {
    pub fn new(inner: ParsedArrayType, size: ParsedArraySize, compressed: bool) -> Self {
        Self {
            inner,
            size,
            compressed,
        }
    }

    pub(crate) fn new_unimplemented() -> Self {
        Self {
            inner: ParsedArrayType::Integer(IntegerType::U8),
            size: ParsedArraySize::Endless,
            compressed: false,
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

    pub(crate) fn rust_str(&self) -> String {
        match &self.size {
            ParsedArraySize::Fixed(i) => format!("[{}; {}]", self.inner.rust_str(), i),
            ParsedArraySize::Variable(_) | ParsedArraySize::Endless => {
                format!("Vec<{}>", self.inner.rust_str())
            }
        }
    }
}
