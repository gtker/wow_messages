use crate::error_printer::invalid_integer_type;
use crate::file_info::FileInfo;
use crate::parser::types::parsed::parsed_ty::ParsedType;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::version::AllVersions;
use crate::rust_printer::field_name_to_rust_name;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

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
    v1: &AllVersions,
    name2: &str,
    v2: &AllVersions,
) -> Ordering {
    name1.cmp(name2).then_with(|| v1.cmp(v2))
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone)]
pub(crate) enum IntegerType {
    U8,
    U16,
    U32,
    U48,
    U64,
    I8,
    I16,
    I32,
    I64,
}

impl IntegerType {
    pub(crate) fn values() -> [Self; 9] {
        [
            Self::U8,
            Self::U16,
            Self::U32,
            Self::U48,
            Self::U64,
            Self::I8,
            Self::I16,
            Self::I32,
            Self::I64,
        ]
    }

    pub(crate) fn size(&self) -> u8 {
        match self {
            IntegerType::I8 | IntegerType::U8 => 1,
            IntegerType::I16 | IntegerType::U16 => 2,
            IntegerType::U32 | IntegerType::I32 => 4,
            IntegerType::I64 | IntegerType::U64 => 8,

            IntegerType::U48 => 6,
        }
    }

    pub(crate) fn sizes(&self) -> Sizes {
        Sizes::exact(self.size().into(), self.size().into())
    }

    pub(crate) fn smallest_value(&self) -> i128 {
        match self {
            IntegerType::U8
            | IntegerType::U16
            | IntegerType::U32
            | IntegerType::U48
            | IntegerType::U64 => 0,
            IntegerType::I8 => i8::MIN.into(),
            IntegerType::I16 => i16::MIN.into(),
            IntegerType::I32 => i32::MIN.into(),
            IntegerType::I64 => i64::MIN.into(),
        }
    }

    pub(crate) fn smallest_array_value(&self) -> i128 {
        0
    }

    pub(crate) fn largest_value(&self) -> i128 {
        2_i128.pow(8 * self.size() as u32)
    }

    pub(crate) fn is_signed(&self) -> bool {
        matches!(
            self,
            IntegerType::I8 | IntegerType::I16 | IntegerType::I32 | IntegerType::I64
        )
    }

    pub(crate) fn str(&self) -> &str {
        match self {
            IntegerType::U8 => "u8",
            IntegerType::U16 => "u16",
            IntegerType::U32 => "u32",
            IntegerType::U64 => "u64",
            IntegerType::I32 => "i32",
            IntegerType::I8 => "i8",
            IntegerType::I16 => "i16",
            IntegerType::I64 => "i64",

            IntegerType::U48 => "u48",
        }
    }

    pub(crate) fn rust_str(&self) -> &str {
        match self {
            IntegerType::U8 => "u8",
            IntegerType::U16 => "u16",
            IntegerType::U32 => "u32",
            IntegerType::U64 => "u64",
            IntegerType::I32 => "i32",
            IntegerType::I8 => "i8",
            IntegerType::I16 => "i16",
            IntegerType::I64 => "i64",

            IntegerType::U48 => "u64",
        }
    }

    pub(crate) fn c_str(&self) -> &'static str {
        match self {
            IntegerType::U8 => "unsigned char",
            IntegerType::U16 => "unsigned short",
            IntegerType::U32 => "unsigned int",
            IntegerType::U64 => "unsigned long long",
            IntegerType::I8 => "signed char",
            IntegerType::I16 => "signed short",
            IntegerType::I32 => "signed int",
            IntegerType::I64 => "signed long long",
            IntegerType::U48 => "unsigned long long",
        }
    }

    pub(crate) fn doc_endian_str(&self) -> &str {
        match self {
            IntegerType::I8 | IntegerType::U8 => "-",
            IntegerType::I16
            | IntegerType::I64
            | IntegerType::U16
            | IntegerType::U32
            | IntegerType::U64
            | IntegerType::I32
            | IntegerType::U48 => "Little",
        }
    }

    pub(crate) fn from_str(s: &str, ty_name: &str, file_info: &FileInfo) -> Self {
        let t = ParsedType::from_str(s, false);
        match t {
            ParsedType::Integer(e) => e,
            _ => {
                invalid_integer_type(ty_name, s, file_info);
            }
        }
    }

    pub(crate) fn try_from_types() -> &'static [(i128, bool, &'static str)] {
        &[
            (1, false, "u8"),
            (2, false, "u16"),
            (4, false, "u32"),
            (8, false, "u64"),
            (1, true, "i8"),
            (2, true, "i16"),
            (4, true, "i32"),
            (8, true, "i64"),
            (2, false, "usize"),
        ]
    }

    pub(crate) fn conversion_is_infallible(
        &self,
        from_size: i128,
        from_signed: bool,
        from_ty: &str,
    ) -> bool {
        let size: i128 = self.size().into();

        if from_ty == "usize" {
            return false;
        }

        let signedness_differs = self.is_signed() != from_signed;
        if from_size > size && !signedness_differs {
            false
        } else if from_size < size && !signedness_differs {
            true
        } else {
            signedness_differs && from_size == size
        }
    }

    pub(crate) fn try_from_trait_name(
        &self,
        from_size: i128,
        from_signed: bool,
        from_ty: &str,
    ) -> String {
        if self.conversion_is_infallible(from_size, from_signed, from_ty) {
            format!("From<{from_ty}>")
        } else {
            format!("TryFrom<{from_ty}>")
        }
    }

    pub(crate) fn try_from_function_name(
        &self,
        from_size: i128,
        from_signed: bool,
        from_ty: &str,
    ) -> String {
        if self.conversion_is_infallible(from_size, from_signed, from_ty) {
            format!("fn from(value: {from_ty}) -> Self")
        } else {
            format!("fn try_from(value: {from_ty}) -> Result<Self, Self::Error>")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub(crate) struct ContainerValue {
    value: i128,
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
    pub(crate) fn value(&self) -> i128 {
        self.value
    }

    pub(crate) fn original_string(&self) -> &str {
        &self.original_string
    }

    pub(crate) fn rust_name(&self) -> String {
        field_name_to_rust_name(self.original_string())
    }

    pub(crate) fn new(value: i128, original_string: String) -> Self {
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
