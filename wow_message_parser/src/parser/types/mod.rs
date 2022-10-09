use crate::rust_printer::field_name_to_rust_name;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use version::Version;

pub(crate) mod array;
pub(crate) mod container;
pub(crate) mod definer;
pub(crate) mod if_statement;
pub(crate) mod objects;
pub(crate) mod optional;
pub(crate) mod parsed;
pub(crate) mod sizes;
pub(crate) mod struct_member;
pub(crate) mod tags;
pub(crate) mod test_case;
pub(crate) mod ty;
pub(crate) mod version;

pub(crate) fn compare_name_and_tags(
    name1: &str,
    v1: &[Version],
    name2: &str,
    v2: &[Version],
) -> Ordering {
    name1.cmp(name2).then_with(|| v1.cmp(v2))
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum ObjectType {
    Struct,
    Enum,
    Flag,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Endianness {
    Little,
    Big,
}

impl Endianness {
    pub(crate) fn wireshark_str(&self) -> &str {
        match self {
            Endianness::Little => "ENC_LITTLE_ENDIAN",
            Endianness::Big => "ENC_BIG_ENDIAN",
        }
    }
    pub(crate) fn rust_str(&self) -> &str {
        match self {
            Endianness::Little => "le",
            Endianness::Big => "be",
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub(crate) enum FloatingPointType {
    F32(Endianness),
    F64(Endianness),
}

impl FloatingPointType {
    pub(crate) fn size(&self) -> u8 {
        match self {
            FloatingPointType::F32(_) => 4,
            FloatingPointType::F64(_) => 8,
        }
    }

    pub(crate) fn str(&self) -> &str {
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

    pub(crate) fn rust_str(&self) -> &str {
        match self {
            FloatingPointType::F32(_) => "f32",
            FloatingPointType::F64(_) => "f64",
        }
    }

    pub(crate) fn rust_endian_str(&self) -> &str {
        match self {
            FloatingPointType::F32(e) | FloatingPointType::F64(e) => e.rust_str(),
        }
    }

    pub(crate) fn wireshark_endian_str(&self) -> &str {
        match self {
            FloatingPointType::F32(e) | FloatingPointType::F64(e) => e.wireshark_str(),
        }
    }

    pub(crate) fn doc_endian_str(&self) -> &str {
        match self {
            FloatingPointType::F32(e) | FloatingPointType::F64(e) => match e {
                Endianness::Little => "Little",
                Endianness::Big => "Big",
            },
        }
    }
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone)]
pub(crate) enum IntegerType {
    U8,
    U16(Endianness),
    U32(Endianness),
    U64(Endianness),
    I32(Endianness),
}

impl IntegerType {
    pub(crate) fn size(&self) -> u8 {
        match self {
            IntegerType::U8 => 1,
            IntegerType::U16(_) => 2,
            IntegerType::U32(_) | IntegerType::I32(_) => 4,
            IntegerType::U64(_) => 8,
        }
    }

    pub(crate) fn smallest_value(&self) -> usize {
        0
    }

    pub(crate) fn largest_value(&self) -> usize {
        2_usize.pow(8 * self.size() as u32)
    }

    pub(crate) fn str(&self) -> &str {
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
            IntegerType::I32(e) => match e {
                Endianness::Little => "i32",
                Endianness::Big => "i32_be",
            },
        }
    }

    pub(crate) fn rust_str(&self) -> &str {
        match self {
            IntegerType::U8 => "u8",
            IntegerType::U16(_) => "u16",
            IntegerType::U32(_) => "u32",
            IntegerType::U64(_) => "u64",
            IntegerType::I32(_) => "i32",
        }
    }

    pub(crate) fn rust_endian_str(&self) -> &str {
        match self {
            IntegerType::U8 => "le",
            IntegerType::U16(i)
            | IntegerType::U32(i)
            | IntegerType::U64(i)
            | IntegerType::I32(i) => i.rust_str(),
        }
    }

    pub(crate) fn wireshark_endian_str(&self) -> &str {
        match self {
            IntegerType::U8 => "ENC_LITTLE_ENDIAN",
            IntegerType::U16(i)
            | IntegerType::U32(i)
            | IntegerType::U64(i)
            | IntegerType::I32(i) => i.wireshark_str(),
        }
    }

    pub(crate) fn doc_endian_str(&self) -> &str {
        match self {
            IntegerType::U8 => "-",
            IntegerType::U16(e)
            | IntegerType::U32(e)
            | IntegerType::U64(e)
            | IntegerType::I32(e) => match e {
                Endianness::Little => "Little",
                Endianness::Big => "Big",
            },
        }
    }
}

impl From<&str> for IntegerType {
    fn from(s: &str) -> Self {
        match s {
            "u8" | "Bool" => IntegerType::U8,
            "u16" => IntegerType::U16(Endianness::Little),
            "u16_be" => IntegerType::U16(Endianness::Big),
            "u32" | "Spell" | "Item" => IntegerType::U32(Endianness::Little),
            "u32_be" => IntegerType::U32(Endianness::Big),
            "u64" => IntegerType::U64(Endianness::Little),
            "u64_be" => IntegerType::U64(Endianness::Big),
            "i32" => IntegerType::I32(Endianness::Little),
            "i32_be" => IntegerType::I32(Endianness::Big),
            _ => panic!("invalid basic type attempted to be created as IntegerType"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ContainerValue {
    value: u64,
    original_string: String,
}

impl Display for ContainerValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{original_string} ({value})",
            original_string = self.original_string,
            value = self.value
        )
    }
}

impl ContainerValue {
    pub(crate) fn value(&self) -> u64 {
        self.value
    }

    pub(crate) fn original_string(&self) -> &str {
        &self.original_string
    }

    pub(crate) fn rust_name(&self) -> String {
        field_name_to_rust_name(self.original_string())
    }

    pub(crate) fn new(value: u64, original_string: String) -> Self {
        Self {
            value,
            original_string,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ParsedContainerValue {
    identifier: String,
}

impl ParsedContainerValue {
    pub(crate) fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl From<&str> for ParsedContainerValue {
    fn from(s: &str) -> Self {
        Self {
            identifier: s.to_string(),
        }
    }
}
