use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::{get_container, get_definer};
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::sizes::{
    Sizes, GUID_SIZE, PACKED_GUID_MAX_SIZE, PACKED_GUID_MIN_SIZE, SPELL_SIZE,
};
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::IntegerType;
use crate::{CSTRING_LARGEST_ALLOWED, CSTRING_SMALLEST_ALLOWED};

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

    pub(crate) fn inner_sizes(
        &self,
        containers: &[ParsedContainer],
        definers: &[Definer],
        tags: &ObjectTags,
    ) -> Sizes {
        match self.ty() {
            ParsedArrayType::Integer(i) => i.sizes(),
            ParsedArrayType::Complex(s) => {
                if let Some(definer) = get_definer(definers, s, tags) {
                    definer.ty().sizes()
                } else if let Some(container) = get_container(containers, s, tags) {
                    container.create_sizes(containers, definers)
                } else {
                    panic!()
                }
            }
            ParsedArrayType::CString => Sizes::exact(
                CSTRING_SMALLEST_ALLOWED.into(),
                CSTRING_LARGEST_ALLOWED.into(),
            ),
            ParsedArrayType::Guid => Sizes::exact(GUID_SIZE.into(), GUID_SIZE.into()),
            ParsedArrayType::PackedGuid => {
                Sizes::exact(PACKED_GUID_MIN_SIZE.into(), PACKED_GUID_MAX_SIZE.into())
            }
            ParsedArrayType::Spell => Sizes::exact(SPELL_SIZE.into(), SPELL_SIZE.into()),
        }
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
