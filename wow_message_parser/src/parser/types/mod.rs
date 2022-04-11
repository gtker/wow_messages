use std::fmt::{Display, Formatter};

pub mod objects;
pub mod tags;

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
pub enum Type {
    Integer(IntegerType),
    PackedGuid,
    Guid,
    FloatingPoint(FloatingPointType),
    CString,
    String {
        length: String,
    },
    Array(Array),
    Identifier {
        s: String,
        upcast: Option<IntegerType>,
    },
    UpdateMask,
    AuraMask,
}

impl Type {
    pub fn str(&self) -> String {
        match self {
            Type::Integer(i) => i.str().to_string(),
            Type::CString => "CString".to_string(),
            Type::String { length } => format!("String[{}]", length),
            Type::Array(a) => a.str(),
            Type::Identifier { s, .. } => s.clone(),
            Type::FloatingPoint(i) => i.str().to_string(),
            Type::PackedGuid => "PackedGuid".to_string(),
            Type::Guid => "Guid".to_string(),
            Type::UpdateMask => "UpdateMask".to_string(),
            Type::AuraMask => "AuraMask".to_string(),
        }
    }

    pub fn rust_str(&self) -> String {
        let s = match self {
            Type::Integer(i) => i.rust_str().to_string(),
            Type::FloatingPoint(i) => i.rust_str().to_string(),
            Type::Identifier { s, .. } => s.clone(),
            Type::CString => "String".to_string(),
            Type::Array(a) => a.rust_str(),
            Type::String { .. } => "String".to_string(),
            Type::PackedGuid | Type::Guid => "Guid".to_string(),
            Type::UpdateMask => "UpdateMask".to_string(),
            Type::AuraMask => "AuraMask".to_string(),
        };

        s
    }

    pub fn rust_size_of(&self) -> u8 {
        match self {
            Type::Integer(i) => i.size(),
            _ => panic!("attempting to get size of complex type: '{}'", self.str()),
        }
    }

    pub fn rust_endian_str(&self) -> &str {
        match self {
            Type::Integer(i) => i.rust_endian_str(),
            _ => panic!("endianness attempted for complex type"),
        }
    }

    pub fn with_upcast(s: &str, upcasted: &str) -> Self {
        let t = Self::from_str(s);
        match t {
            Type::Identifier { .. } => {}
            _ => panic!("upcast for type that does not support it"),
        }

        let int = match upcasted {
            "u16" => IntegerType::U16(Endianness::Little),
            "u32" => IntegerType::U32(Endianness::Little),
            "u64" => IntegerType::U64(Endianness::Little),
            "u16_be" => IntegerType::U16(Endianness::Big),
            "u32_be" => IntegerType::U32(Endianness::Big),
            "u64_be" => IntegerType::U64(Endianness::Big),
            _ => panic!("unsupported upcast: {}", upcasted),
        };

        Self::Identifier {
            s: s.to_string(),
            upcast: Some(int),
        }
    }

    pub fn from_str(s: &str) -> Self {
        let s = match s {
            "u8" => Self::Integer(IntegerType::U8),
            "Bool" => Self::Integer(IntegerType::U8),
            "u16" => Self::Integer(IntegerType::U16(Endianness::Little)),
            "u32" => Self::Integer(IntegerType::U32(Endianness::Little)),
            "Spell" => Self::Integer(IntegerType::U32(Endianness::Little)),
            "Item" => Self::Integer(IntegerType::U32(Endianness::Little)),
            "u64" => Self::Integer(IntegerType::U64(Endianness::Little)),
            "Guid" => Self::Guid,
            "PackedGuid" => Self::PackedGuid,
            "AuraMask" => Self::AuraMask,
            "UpdateMask" => Self::UpdateMask,
            "u16_be" => Self::Integer(IntegerType::U16(Endianness::Big)),
            "u32_be" => Self::Integer(IntegerType::U32(Endianness::Big)),
            "u64_be" => Self::Integer(IntegerType::U64(Endianness::Big)),
            "f32" => Self::FloatingPoint(FloatingPointType::F32(Endianness::Little)),
            "f32_be" => Self::FloatingPoint(FloatingPointType::F32(Endianness::Big)),
            "f64" => Self::FloatingPoint(FloatingPointType::F64(Endianness::Little)),
            "f64_be" => Self::FloatingPoint(FloatingPointType::F64(Endianness::Big)),
            "CString" => Self::CString,
            _ => Self::Identifier {
                s: s.to_string(),
                upcast: None,
            },
        };
        match s {
            Type::Identifier { s: i, .. } => {
                if i.contains('[') {
                    let mut i = i.split('[');
                    let array_type = i.next().unwrap();
                    let array_type: Type = Type::from_str(array_type);

                    let amount = i.next().unwrap().strip_suffix(']').unwrap();
                    let parsed = str::parse::<i64>(amount);

                    let size = if let Ok(parsed) = parsed {
                        ArraySize::Fixed(parsed)
                    } else if amount == "-" {
                        ArraySize::Endless
                    } else {
                        ArraySize::Variable(amount.to_string())
                    };

                    match array_type {
                        Type::Integer(i) => Self::Array(Array {
                            inner: ArrayType::Integer(i),
                            size,
                        }),
                        Type::Identifier { s: i, .. } => {
                            if i == "String" {
                                return Self::String {
                                    length: amount.to_string(),
                                };
                            }

                            Self::Array(Array {
                                inner: ArrayType::Complex(i),
                                size,
                            })
                        }
                        Type::CString => Self::Array(Array {
                            inner: ArrayType::CString,
                            size,
                        }),
                        Type::String { .. } => panic!("unsupported"),
                        Type::Array(_) => panic!("unsupported"),
                        Type::FloatingPoint(_) => panic!("unsupported"),
                        Type::UpdateMask => panic!("unsupported"),
                        Type::AuraMask => panic!("unsupported"),
                        Type::PackedGuid => Self::Array(Array {
                            inner: ArrayType::PackedGuid,
                            size,
                        }),
                        Type::Guid => Self::Array(Array {
                            inner: ArrayType::Guid,
                            size,
                        }),
                    }
                } else {
                    Self::Identifier { s: i, upcast: None }
                }
            }
            s => s,
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
