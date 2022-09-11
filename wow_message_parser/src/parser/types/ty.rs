use crate::container::{
    Container, Sizes, AURA_MASK_MAX_SIZE, AURA_MASK_MIN_SIZE, BOOL_SIZE, DATETIME_SIZE, GUID_SIZE,
    PACKED_GUID_MAX_SIZE, PACKED_GUID_MIN_SIZE, UPDATE_MASK_MAX_SIZE, UPDATE_MASK_MIN_SIZE,
};
use crate::parser::types::objects::Objects;
use crate::parser::types::{
    Array, ArraySize, ArrayType, Endianness, FloatingPointType, IntegerType, ObjectType,
};
use crate::{
    Tags, CSTRING_LARGEST_ALLOWED, CSTRING_SMALLEST_ALLOWED, SIZED_CSTRING_LARGEST_ALLOWED,
    SIZED_CSTRING_SMALLEST_ALLOWED,
};
use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Integer(IntegerType),
    Bool,
    PackedGuid,
    Guid,
    DateTime,
    FloatingPoint(FloatingPointType),
    CString,
    SizedCString,
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
            Type::SizedCString => "SizedCString".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::DateTime => "DateTime".to_string(),
        }
    }

    pub fn rust_str(&self) -> String {
        let s = match self {
            Type::Integer(i) => i.rust_str().to_string(),
            Type::FloatingPoint(i) => i.rust_str().to_string(),
            Type::Identifier { s, .. } => s.clone(),
            Type::CString | Type::SizedCString | Type::String { .. } => "String".to_string(),
            Type::Array(a) => a.rust_str(),
            Type::PackedGuid | Type::Guid => "Guid".to_string(),
            Type::UpdateMask => "UpdateMask".to_string(),
            Type::AuraMask => "AuraMask".to_string(),
            Type::Bool => "bool".to_string(),
            Type::DateTime => "DateTime".to_string(),
        };

        s
    }

    // NOTE: Definers used in if statements do count if statement contents
    pub fn sizes(&self, e: &Container, o: &Objects) -> Sizes {
        let mut sizes = Sizes::new();

        match self {
            Type::Integer(i) => sizes.inc_both(i.size() as usize),
            Type::Bool => sizes.inc_both(BOOL_SIZE.into()),
            Type::Guid => sizes.inc_both(GUID_SIZE as _),
            Type::DateTime => sizes.inc_both(DATETIME_SIZE.into()),
            Type::FloatingPoint(i) => sizes.inc_both(i.size() as usize),
            Type::PackedGuid => sizes.inc(PACKED_GUID_MIN_SIZE as _, PACKED_GUID_MAX_SIZE as _),
            Type::UpdateMask => {
                sizes.inc(UPDATE_MASK_MIN_SIZE as usize, UPDATE_MASK_MAX_SIZE as usize)
            }
            Type::AuraMask => sizes.inc(AURA_MASK_MIN_SIZE as usize, AURA_MASK_MAX_SIZE as usize),
            Type::CString => sizes.inc(CSTRING_SMALLEST_ALLOWED, CSTRING_LARGEST_ALLOWED),
            Type::SizedCString => sizes.inc(
                SIZED_CSTRING_SMALLEST_ALLOWED,
                SIZED_CSTRING_LARGEST_ALLOWED,
            ),
            Type::String { length } => {
                if let Ok(length) = length.parse::<usize>() {
                    sizes.inc(length, length);
                } else {
                    match &e.get_type_of_variable(length) {
                        Type::Integer(i) => sizes.inc(i.smallest_value(), i.largest_value()),
                        _ => unreachable!("string lengths can only be int"),
                    }
                }
            }
            Type::Identifier { s, upcast } => match o.get_object_type_of(s, e.tags()) {
                ObjectType::Enum | ObjectType::Flag => {
                    let s = if let Some(upcast) = upcast {
                        upcast.size()
                    } else {
                        o.get_definer(s, e.tags()).ty().size()
                    } as usize;

                    sizes.inc_both(s);
                }
                ObjectType::Struct => {
                    let c = o.get_container(s, e.tags());
                    sizes += c.create_sizes(o);
                }
                _ => unreachable!(),
            },
            Type::Array(array) => {
                if matches!(array.size(), ArraySize::Endless) {
                    sizes.inc(0, u16::MAX as _);
                    return sizes;
                }

                let (min, max) = match array.size() {
                    ArraySize::Fixed(f) => {
                        let f: usize = f.try_into().unwrap();
                        (f, f)
                    }
                    ArraySize::Variable(f) => match e.get_field_ty(&f) {
                        Type::Integer(i) => (i.smallest_value(), i.largest_value()),
                        _ => unreachable!("only ints can be string lengths"),
                    },
                    ArraySize::Endless => unreachable!(),
                };

                match array.ty() {
                    ArrayType::Integer(i) => {
                        sizes.inc(i.size() as usize * min, i.size() as usize * max)
                    }
                    ArrayType::Guid => {
                        sizes.inc(GUID_SIZE as usize * min, GUID_SIZE as usize * max)
                    }
                    ArrayType::PackedGuid => sizes.inc(
                        PACKED_GUID_MIN_SIZE as usize * min,
                        PACKED_GUID_MAX_SIZE as usize * max,
                    ),
                    ArrayType::CString => sizes.inc(
                        CSTRING_SMALLEST_ALLOWED * min,
                        CSTRING_LARGEST_ALLOWED * max,
                    ),
                    ArrayType::Complex(s) => match o.get_object_type_of(s, e.tags()) {
                        ObjectType::Enum | ObjectType::Flag => {
                            let s = o.get_definer(s, e.tags()).ty().size();
                            sizes.inc(s as usize * min, s as usize * max);
                        }
                        ObjectType::Struct => {
                            let c = o.get_container(s, e.tags()).create_sizes(o);

                            sizes.inc(min * c.minimum(), 0);
                            sizes.inc(0, max.saturating_mul(c.maximum()));
                        }
                        _ => unreachable!(),
                    },
                }
            }
        }

        sizes
    }

    pub fn rust_size_of(&self) -> u8 {
        match self {
            Type::Integer(i) => i.size(),
            _ => panic!("attempting to get size of complex type: '{}'", self.str()),
        }
    }

    pub fn doc_size_of(&self, tags: &Tags, o: &Objects) -> String {
        match self {
            Type::Integer(i) => i.size().to_string(),
            Type::Guid => 8.to_string(),
            Type::FloatingPoint(f) => f.size().to_string(),
            Type::String { length } => length.clone(),
            Type::Identifier { s, upcast } => {
                let sizes = o.get_object(s, tags).sizes();
                if let Some(upcast) = upcast {
                    upcast.size().to_string()
                } else {
                    if let Some(size) = sizes.is_constant() {
                        size.to_string()
                    } else {
                        "-".to_string()
                    }
                }
            }
            Type::Array(_) => "?".to_string(),
            Type::SizedCString
            | Type::CString
            | Type::UpdateMask
            | Type::AuraMask
            | Type::PackedGuid => "-".to_string(),
            Type::Bool => BOOL_SIZE.to_string(),
            Type::DateTime => DATETIME_SIZE.to_string(),
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
            Type::DateTime | Type::Guid => "Little".to_string(),
            Type::FloatingPoint(f) => f.doc_endian_str().to_string(),
            Type::SizedCString
            | Type::Bool
            | Type::String { .. }
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
            "i32" => IntegerType::U64(Endianness::Little),
            "i32_be" => IntegerType::U64(Endianness::Big),
            _ => panic!("unsupported upcast: {}", upcasted),
        };

        Self::Identifier {
            s: s.to_string(),
            upcast: Some(int),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let s = match s {
            "u8" => Self::Integer(IntegerType::U8),
            "Bool" => Self::Bool,
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
            "i32" => Self::Integer(IntegerType::I32(Endianness::Little)),
            "i32_be" => Self::Integer(IntegerType::I32(Endianness::Big)),
            "f32" => Self::FloatingPoint(FloatingPointType::F32(Endianness::Little)),
            "f32_be" => Self::FloatingPoint(FloatingPointType::F32(Endianness::Big)),
            "f64" => Self::FloatingPoint(FloatingPointType::F64(Endianness::Little)),
            "f64_be" => Self::FloatingPoint(FloatingPointType::F64(Endianness::Big)),
            "CString" => Self::CString,
            "SizedCString" => Self::SizedCString,
            "DateTime" => Self::DateTime,
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
                        Type::SizedCString
                        | Type::String { .. }
                        | Type::Array(_)
                        | Type::FloatingPoint(_)
                        | Type::UpdateMask
                        | Type::AuraMask
                        | Type::DateTime
                        | Type::Bool => panic!("unsupported"),
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
