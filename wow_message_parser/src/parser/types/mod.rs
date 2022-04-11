use std::fmt::{Display, Formatter};

pub mod objects;
pub mod tags;
pub mod ty;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ObjectType {
    Struct,
    CLogin,
    SLogin,
    Enum,
    Flag,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Endianness {
    Little,
    Big,
}

impl Endianness {
    pub fn rust_str(&self) -> &str {
        match self {
            Endianness::Little => "le",
            Endianness::Big => "be",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FloatingPointType {
    F32(Endianness),
    F64(Endianness),
}
impl FloatingPointType {
    pub fn size(&self) -> u8 {
        match self {
            FloatingPointType::F32(_) => 4,
            FloatingPointType::F64(_) => 8,
        }
    }

    pub fn str(&self) -> &str {
        match self {
            FloatingPointType::F32(e) => match e {
                Endianness::Little => "f32",
                Endianness::Big => "f32_be",
            },
            FloatingPointType::F64(e) => match e {
                Endianness::Little => "f64",
                Endianness::Big => "f64_be",
            },
        }
    }

    pub fn rust_str(&self) -> &str {
        match self {
            FloatingPointType::F32(_) => "f32",
            FloatingPointType::F64(_) => "f64",
        }
    }

    pub fn rust_endian_str(&self) -> &str {
        match self {
            FloatingPointType::F32(e) | FloatingPointType::F64(e) => e.rust_str(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum IntegerType {
    U8,
    U16(Endianness),
    U32(Endianness),
    U64(Endianness),
}

impl IntegerType {
    pub fn size(&self) -> u8 {
        match self {
            IntegerType::U8 => 1,
            IntegerType::U16(_) => 2,
            IntegerType::U32(_) => 4,
            IntegerType::U64(_) => 8,
        }
    }

    pub fn str(&self) -> &str {
        match self {
            IntegerType::U8 => "u8",
            IntegerType::U16(e) => match e {
                Endianness::Little => "u16",
                Endianness::Big => "u16_be",
            },
            IntegerType::U32(e) => match e {
                Endianness::Little => "u32",
                Endianness::Big => "u32_be",
            },
            IntegerType::U64(e) => match e {
                Endianness::Little => "u64",
                Endianness::Big => "u64_be",
            },
        }
    }

    pub fn rust_str(&self) -> &str {
        match self {
            IntegerType::U8 => "u8",
            IntegerType::U16(_) => "u16",
            IntegerType::U32(_) => "u32",
            IntegerType::U64(_) => "u64",
        }
    }

    pub fn rust_endian_str(&self) -> &str {
        match self {
            IntegerType::U8 => "le",
            IntegerType::U16(i) | IntegerType::U32(i) | IntegerType::U64(i) => i.rust_str(),
        }
    }
}

impl From<&str> for IntegerType {
    fn from(s: &str) -> Self {
        match s {
            "u8" => IntegerType::U8,
            "Bool" => IntegerType::U8,
            "u16" => IntegerType::U16(Endianness::Little),
            "u16_be" => IntegerType::U16(Endianness::Big),
            "u32" => IntegerType::U32(Endianness::Little),
            "Spell" => IntegerType::U32(Endianness::Little),
            "Item" => IntegerType::U32(Endianness::Little),
            "u32_be" => IntegerType::U32(Endianness::Big),
            "u64" => IntegerType::U64(Endianness::Little),
            "u64_be" => IntegerType::U64(Endianness::Big),
            _ => panic!("invalid basic type attempted to be created as IntegerType"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArrayType {
    Integer(IntegerType),
    Complex(String),
    CString,
    Guid,
    PackedGuid,
}

impl ArrayType {
    pub fn rust_str(&self) -> String {
        match &self {
            ArrayType::Integer(i) => i.rust_str().to_string(),
            ArrayType::Complex(i) => i.clone(),
            ArrayType::CString => "String".to_string(),
            ArrayType::Guid => "Guid".to_string(),
            ArrayType::PackedGuid => "Guid".to_string(),
        }
    }

    pub fn str(&self) -> String {
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
    pub fn str(&self) -> String {
        match self {
            ArraySize::Fixed(i) => i.to_string(),
            ArraySize::Variable(i) => i.clone(),
            ArraySize::Endless => "-".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Array {
    inner: ArrayType,
    size: ArraySize,
}

impl Array {
    pub fn ty(&self) -> &ArrayType {
        &self.inner
    }

    pub fn size(&self) -> ArraySize {
        self.size.clone()
    }

    pub fn str(&self) -> String {
        format!("{}[{}]", self.inner.str(), self.size.str())
    }

    pub fn rust_str(&self) -> String {
        match &self.size {
            ArraySize::Fixed(i) => format!("[{}; {}]", self.inner.rust_str(), i),
            ArraySize::Variable(_) | ArraySize::Endless => {
                format!("Vec<{}>", self.inner.rust_str())
            }
        }
    }

    pub fn rust_size_str(&self) -> String {
        match &self.size {
            ArraySize::Fixed(i) => i.to_string(),
            ArraySize::Variable(s) => s.clone(),
            ArraySize::Endless => panic!(),
        }
    }

    pub fn is_constant_sized_u8_array(&self) -> bool {
        match &self.size() {
            ArraySize::Fixed(_) => matches!(&self.ty(), ArrayType::Integer(IntegerType::U8)),
            ArraySize::Variable(_) => false,
            ArraySize::Endless => false,
        }
    }

    pub fn rust_kind_str(&self) -> String {
        match &self.inner {
            ArrayType::Integer(i) => i.rust_str().to_string(),
            ArrayType::Complex(i) => i.clone(),
            ArrayType::CString => "String".to_string(),
            ArrayType::Guid | ArrayType::PackedGuid => "Guid".to_string(),
        }
    }

    pub fn rust_endian_str(&self) -> String {
        match self.inner {
            ArrayType::Integer(i) => i.rust_endian_str().to_string(),
            _ => panic!("endianness only supported for integer types"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VerifiedContainerValue {
    value: u64,
    original_string: String,
}

impl Display for VerifiedContainerValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{original_string} ({value})",
            original_string = self.original_string,
            value = self.value
        )
    }
}

impl VerifiedContainerValue {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn original_string(&self) -> &str {
        &self.original_string
    }

    pub fn new(value: u64, original_string: String) -> Self {
        Self {
            value,
            original_string,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ContainerValue {
    identifier: String,
}

impl ContainerValue {
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl Display for ContainerValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl From<&str> for ContainerValue {
    fn from(s: &str) -> Self {
        Self {
            identifier: s.to_string(),
        }
    }
}
