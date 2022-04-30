use crate::parser::types::{
    Array, ArraySize, ArrayType, Endianness, FloatingPointType, IntegerType,
};

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

    pub fn doc_size_of(&self) -> String {
        match self {
            Type::Integer(i) => i.size().to_string(),
            Type::Guid => 8.to_string(),
            Type::FloatingPoint(f) => f.size().to_string(),
            Type::String { length } => length.clone(),
            Type::Identifier { .. } | Type::Array(_) => "?".to_string(),
            Type::CString | Type::UpdateMask | Type::AuraMask | Type::PackedGuid => "-".to_string(),
        }
    }

    pub fn rust_endian_str(&self) -> &str {
        match self {
            Type::Integer(i) => i.rust_endian_str(),
            _ => panic!("endianness attempted for complex type"),
        }
    }

    pub fn doc_endian_str(&self) -> String {
        match self {
            Type::Integer(i) => i.doc_endian_str().to_string(),
            Type::Guid => "Little".to_string(),
            Type::FloatingPoint(f) => f.doc_endian_str().to_string(),
            Type::String { .. }
            | Type::Array(_)
            | Type::Identifier { .. }
            | Type::UpdateMask
            | Type::AuraMask
            | Type::CString
            | Type::PackedGuid => "-".to_string(),
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
