use crate::error_printer::{complex_not_found, recursive_type};
use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::{get_container, get_definer};
use crate::parser::types::parsed::parsed_array::{ParsedArray, ParsedArraySize, ParsedArrayType};
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::sizes::{
    update_mask_max, Sizes, AURA_MASK_MAX_SIZE, AURA_MASK_MIN_SIZE, DATETIME_SIZE, GUID_SIZE,
    PACKED_GUID_MAX_SIZE, PACKED_GUID_MIN_SIZE, UPDATE_MASK_MIN_SIZE,
};
use crate::parser::types::{Endianness, FloatingPointType, IntegerType};
use crate::{
    CSTRING_LARGEST_ALLOWED, CSTRING_SMALLEST_ALLOWED, SIZED_CSTRING_LARGEST_ALLOWED,
    SIZED_CSTRING_SMALLEST_ALLOWED,
};
use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ParsedType {
    Integer(IntegerType),
    Bool(IntegerType),
    PackedGuid,
    Guid,
    DateTime,
    FloatingPoint(FloatingPointType),
    CString,
    SizedCString,
    String {
        length: String,
    },
    Array(ParsedArray),
    Identifier {
        s: String,
        upcast: Option<IntegerType>,
    },
    UpdateMask,
    AuraMask,
}

impl ParsedType {
    pub(crate) fn str(&self) -> String {
        match self {
            ParsedType::Integer(i) => i.str().to_string(),
            ParsedType::CString => "CString".to_string(),
            ParsedType::String { length } => format!("String[{}]", length),
            ParsedType::Array(a) => a.str(),
            ParsedType::Identifier { s, .. } => s.clone(),
            ParsedType::FloatingPoint(i) => i.str().to_string(),
            ParsedType::PackedGuid => "PackedGuid".to_string(),
            ParsedType::Guid => "Guid".to_string(),
            ParsedType::UpdateMask => "UpdateMask".to_string(),
            ParsedType::AuraMask => "AuraMask".to_string(),
            ParsedType::SizedCString => "SizedCString".to_string(),
            ParsedType::Bool(i) => bool_ty_to_string(i),
            ParsedType::DateTime => "DateTime".to_string(),
        }
    }

    // NOTE: Definers used in if statements count if statement contents
    pub(crate) fn sizes_parsed(
        &self,
        e: &ParsedContainer,
        containers: &[ParsedContainer],
        definers: &[Definer],
    ) -> Sizes {
        let mut sizes = Sizes::new();

        match self {
            ParsedType::Integer(i) => sizes.inc_both(i.size() as usize),
            ParsedType::Bool(i) => sizes.inc_both(i.size() as usize),
            ParsedType::Guid => sizes.inc_both(GUID_SIZE as _),
            ParsedType::DateTime => sizes.inc_both(DATETIME_SIZE.into()),
            ParsedType::FloatingPoint(i) => sizes.inc_both(i.size() as usize),
            ParsedType::PackedGuid => {
                sizes.inc(PACKED_GUID_MIN_SIZE as _, PACKED_GUID_MAX_SIZE as _)
            }
            ParsedType::UpdateMask => {
                sizes.inc(UPDATE_MASK_MIN_SIZE as usize, update_mask_max() as usize)
            }
            ParsedType::AuraMask => {
                sizes.inc(AURA_MASK_MIN_SIZE as usize, AURA_MASK_MAX_SIZE as usize)
            }
            ParsedType::CString => sizes.inc(CSTRING_SMALLEST_ALLOWED, CSTRING_LARGEST_ALLOWED),
            ParsedType::SizedCString => sizes.inc(
                SIZED_CSTRING_SMALLEST_ALLOWED,
                SIZED_CSTRING_LARGEST_ALLOWED,
            ),
            ParsedType::String { length } => {
                if let Ok(length) = length.parse::<usize>() {
                    sizes.inc(length, length);
                } else {
                    match &e.get_type_of_variable(length) {
                        ParsedType::Integer(i) => sizes.inc(i.smallest_value(), i.largest_value()),
                        _ => unreachable!("string lengths can only be int"),
                    }
                }
            }
            ParsedType::Identifier { s, upcast } => {
                if s == e.name() {
                    recursive_type(e.name(), &e.file_info);
                }

                if get_definer(definers, s, e.tags()).is_some() {
                    let s = if let Some(upcast) = upcast {
                        upcast.size()
                    } else {
                        get_definer(definers, s, e.tags()).unwrap().ty().size()
                    } as usize;

                    sizes.inc_both(s);
                } else if let Some(c) = get_container(containers, s, e.tags()) {
                    sizes += c.create_sizes(containers, definers);
                } else {
                    complex_not_found(e.name(), e.tags(), &e.file_info, s);
                }
            }
            ParsedType::Array(array) => {
                if matches!(array.size(), ParsedArraySize::Endless) {
                    sizes.inc(0, u16::MAX as _);
                    return sizes;
                }

                let (min, max) = match array.size() {
                    ParsedArraySize::Fixed(f) => {
                        let f: usize = f.try_into().unwrap();
                        (f, f)
                    }
                    ParsedArraySize::Variable(f) => match e.get_field_ty(&f) {
                        ParsedType::Integer(i) => (i.smallest_value(), i.largest_value()),
                        _ => unreachable!("only ints can be string lengths"),
                    },
                    ParsedArraySize::Endless => unreachable!(),
                };

                match array.ty() {
                    ParsedArrayType::Integer(i) => {
                        sizes.inc(i.size() as usize * min, i.size() as usize * max)
                    }
                    ParsedArrayType::Guid => {
                        sizes.inc(GUID_SIZE as usize * min, GUID_SIZE as usize * max)
                    }
                    ParsedArrayType::PackedGuid => sizes.inc(
                        PACKED_GUID_MIN_SIZE as usize * min,
                        PACKED_GUID_MAX_SIZE as usize * max,
                    ),
                    ParsedArrayType::CString => sizes.inc(
                        CSTRING_SMALLEST_ALLOWED * min,
                        CSTRING_LARGEST_ALLOWED * max,
                    ),
                    ParsedArrayType::Complex(s) => {
                        if let Some(e) = get_definer(definers, s, e.tags()) {
                            let s = e.ty().size();
                            sizes.inc(s as usize * min, s as usize * max);
                        } else if let Some(c) = get_container(containers, s, e.tags()) {
                            let c = c.create_sizes(containers, definers);

                            sizes.inc(min * c.minimum(), 0);
                            sizes.inc(0, max.saturating_mul(c.maximum()));
                        } else {
                            complex_not_found(e.name(), e.tags(), &e.file_info, s);
                        }
                    }
                }
            }
        }

        sizes
    }

    pub(crate) fn with_upcast(s: &str, upcasted: &str) -> Self {
        let t = Self::from_str(s);
        match t {
            ParsedType::Identifier { .. } => {}
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
    pub(crate) fn from_str(s: &str) -> Self {
        let s = match s {
            "u8" => Self::Integer(IntegerType::U8),
            "Bool" => Self::Bool(IntegerType::U8),
            "Bool16" => Self::Bool(IntegerType::U16(Endianness::Little)),
            "Bool32" => Self::Bool(IntegerType::U32(Endianness::Little)),
            "Bool64" => Self::Bool(IntegerType::U64(Endianness::Little)),
            "u16" => Self::Integer(IntegerType::U16(Endianness::Little)),
            "u32" => Self::Integer(IntegerType::U32(Endianness::Little)),
            "Spell" | "Milliseconds" | "Seconds" | "Copper" | "Item" => {
                Self::Integer(IntegerType::U32(Endianness::Little))
            }
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
            ParsedType::Identifier { s: i, .. } => {
                if i.contains('[') {
                    let mut i = i.split('[');
                    let array_type = i.next().unwrap();
                    let array_type: ParsedType = ParsedType::from_str(array_type);

                    let amount = i.next().unwrap().strip_suffix(']').unwrap();
                    let parsed = str::parse::<i64>(amount);

                    let size = if let Ok(parsed) = parsed {
                        ParsedArraySize::Fixed(parsed)
                    } else if amount == "-" {
                        ParsedArraySize::Endless
                    } else {
                        ParsedArraySize::Variable(amount.to_string())
                    };

                    match array_type {
                        ParsedType::Integer(i) => {
                            Self::Array(ParsedArray::new(ParsedArrayType::Integer(i), size))
                        }
                        ParsedType::Identifier { s: i, .. } => {
                            if i == "String" {
                                return Self::String {
                                    length: amount.to_string(),
                                };
                            }

                            Self::Array(ParsedArray::new(ParsedArrayType::Complex(i), size))
                        }
                        ParsedType::CString => {
                            Self::Array(ParsedArray::new(ParsedArrayType::CString, size))
                        }
                        ParsedType::SizedCString
                        | ParsedType::String { .. }
                        | ParsedType::Array(_)
                        | ParsedType::FloatingPoint(_)
                        | ParsedType::UpdateMask
                        | ParsedType::AuraMask
                        | ParsedType::DateTime
                        | ParsedType::Bool(_) => panic!("unsupported"),
                        ParsedType::PackedGuid => {
                            Self::Array(ParsedArray::new(ParsedArrayType::PackedGuid, size))
                        }
                        ParsedType::Guid => {
                            Self::Array(ParsedArray::new(ParsedArrayType::Guid, size))
                        }
                    }
                } else {
                    Self::Identifier { s: i, upcast: None }
                }
            }
            s => s,
        }
    }
}

pub(crate) fn bool_ty_to_string(i: &IntegerType) -> String {
    match i {
        IntegerType::U8 => "Bool".to_string(),
        IntegerType::U16(_) => "Bool16".to_string(),
        IntegerType::I32(_) | IntegerType::U32(_) => "Bool32".to_string(),
        IntegerType::U64(_) => "Bool64".to_string(),
    }
}
