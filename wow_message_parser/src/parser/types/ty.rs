use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::definer::Definer;
use crate::parser::types::parsed::parsed_ty::bool_ty_to_string;
use crate::parser::types::sizes::{
    update_mask_max, Sizes, AURA_MASK_MAX_SIZE, AURA_MASK_MIN_SIZE, DATETIME_SIZE, GUID_SIZE,
    PACKED_GUID_MAX_SIZE, PACKED_GUID_MIN_SIZE, UPDATE_MASK_MIN_SIZE,
};
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::{FloatingPointType, IntegerType};
use crate::{
    Container, CSTRING_LARGEST_ALLOWED, CSTRING_SMALLEST_ALLOWED, SIZED_CSTRING_LARGEST_ALLOWED,
    SIZED_CSTRING_SMALLEST_ALLOWED, STRING_LARGEST_POSSIBLE, STRING_SMALLEST_POSSIBLE,
};
use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum Type {
    Integer(IntegerType),
    Bool(IntegerType),
    PackedGuid,
    Guid,
    DateTime,
    FloatingPoint(FloatingPointType),
    CString,
    SizedCString,
    String,
    Array(Array),
    Enum {
        e: Definer,
        upcast: Option<IntegerType>,
    },
    Flag {
        e: Definer,
        upcast: Option<IntegerType>,
    },
    Struct {
        e: Container,
    },
    UpdateMask,
    AuraMask,
    AchievementDoneArray,
    AchievementInProgressArray,
}

impl Type {
    pub(crate) fn str(&self) -> String {
        match self {
            Type::Integer(i) => i.str().to_string(),
            Type::CString => "CString".to_string(),
            Type::String => "String".to_string(),
            Type::Array(a) => a.str(),
            Type::Enum { e, .. } | Type::Flag { e, .. } => e.name().to_string(),
            Type::Struct { e, .. } => e.name().to_string(),
            Type::FloatingPoint(i) => i.str().to_string(),
            Type::PackedGuid => "PackedGuid".to_string(),
            Type::Guid => "Guid".to_string(),
            Type::UpdateMask => "UpdateMask".to_string(),
            Type::AuraMask => "AuraMask".to_string(),
            Type::SizedCString => "SizedCString".to_string(),
            Type::Bool(i) => bool_ty_to_string(i),
            Type::DateTime => "DateTime".to_string(),
            Type::AchievementDoneArray => "AchievementDoneArray".to_string(),
            Type::AchievementInProgressArray => "AchievementInProgressArray".to_string(),
        }
    }

    pub(crate) fn rust_str(&self) -> String {
        let s = match self {
            Type::Integer(i) => i.rust_str().to_string(),
            Type::FloatingPoint(i) => i.rust_str().to_string(),
            Type::Enum { e, .. } | Type::Flag { e, .. } => e.name().to_string(),
            Type::Struct { e, .. } => e.name().to_string(),
            Type::CString | Type::SizedCString | Type::String { .. } => "String".to_string(),
            Type::Array(a) => a.rust_str(),
            Type::PackedGuid | Type::Guid => "Guid".to_string(),
            Type::UpdateMask => "UpdateMask".to_string(),
            Type::AuraMask => "AuraMask".to_string(),
            Type::Bool(_) => "bool".to_string(),
            Type::DateTime => "DateTime".to_string(),
            Type::AchievementDoneArray => "AchievementDoneArray".to_string(),
            Type::AchievementInProgressArray => "AchievementInProgressArray".to_string(),
        };

        s
    }

    // NOTE: Definers used in if statements count if statement contents
    pub(crate) fn sizes(&self, tags: &ObjectTags) -> Sizes {
        let mut sizes = Sizes::new();

        match self {
            Type::Integer(i) => sizes.inc_both(i.size() as usize),
            Type::Bool(i) => sizes.inc_both(i.size().into()),
            Type::Guid => sizes.inc_both(GUID_SIZE as _),
            Type::DateTime => sizes.inc_both(DATETIME_SIZE.into()),
            Type::FloatingPoint(i) => sizes.inc_both(i.size() as usize),
            Type::PackedGuid => sizes.inc(PACKED_GUID_MIN_SIZE as _, PACKED_GUID_MAX_SIZE as _),
            Type::UpdateMask => {
                let world_version = tags.main_versions().next().unwrap().as_major_world();
                sizes.inc(
                    UPDATE_MASK_MIN_SIZE as usize,
                    update_mask_max(world_version) as usize,
                )
            }
            Type::AuraMask => sizes.inc(AURA_MASK_MIN_SIZE as usize, AURA_MASK_MAX_SIZE as usize),
            Type::CString => sizes.inc(CSTRING_SMALLEST_ALLOWED, CSTRING_LARGEST_ALLOWED),
            Type::SizedCString => sizes.inc(
                SIZED_CSTRING_SMALLEST_ALLOWED,
                SIZED_CSTRING_LARGEST_ALLOWED,
            ),
            Type::String => sizes.inc(STRING_SMALLEST_POSSIBLE, STRING_LARGEST_POSSIBLE),
            Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
                let s = if let Some(upcast) = upcast {
                    upcast.size()
                } else {
                    e.ty().size()
                } as usize;

                sizes.inc_both(s);
            }
            Type::Struct { e } => {
                sizes += e.sizes();
            }
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
                    ArraySize::Variable(f) => match f.ty() {
                        Type::Integer(i) => (i.smallest_value(), i.largest_value()),
                        _ => unreachable!("only ints can be string lengths"),
                    },
                    ArraySize::Endless => unreachable!("Endless has already been matched"),
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
                    ArrayType::Struct(c) => {
                        let c = c.sizes();

                        sizes.inc(min * c.minimum(), 0);
                        sizes.inc(0, max.saturating_mul(c.maximum()));
                    }
                }
            }
            Type::AchievementDoneArray | Type::AchievementInProgressArray => {
                sizes.inc(0, usize::MAX);
            }
        }

        sizes
    }

    pub(crate) fn doc_size_of(&self) -> String {
        match self {
            Type::Integer(i) => i.size().to_string(),
            Type::Guid => 8.to_string(),
            Type::FloatingPoint(f) => f.size().to_string(),
            Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
                if let Some(upcast) = upcast {
                    upcast.size().to_string()
                } else if let Some(size) = e.sizes().is_constant() {
                    size.to_string()
                } else {
                    "-".to_string()
                }
            }
            Type::Struct { e } => {
                let sizes = e.sizes();
                if let Some(size) = sizes.is_constant() {
                    size.to_string()
                } else {
                    "-".to_string()
                }
            }
            Type::Array(_) => "?".to_string(),
            Type::AchievementDoneArray
            | Type::AchievementInProgressArray
            | Type::String
            | Type::SizedCString
            | Type::CString
            | Type::UpdateMask
            | Type::AuraMask
            | Type::PackedGuid => "-".to_string(),
            Type::Bool(i) => i.size().to_string(),
            Type::DateTime => DATETIME_SIZE.to_string(),
        }
    }

    pub(crate) fn doc_endian_str(&self) -> String {
        match self {
            Type::Bool(i) | Type::Integer(i) => i.doc_endian_str().to_string(),
            Type::DateTime | Type::Guid => "Little".to_string(),
            Type::FloatingPoint(f) => f.doc_endian_str().to_string(),
            Type::SizedCString
            | Type::AchievementDoneArray
            | Type::AchievementInProgressArray
            | Type::String { .. }
            | Type::Array(_)
            | Type::Enum { .. }
            | Type::Flag { .. }
            | Type::Struct { .. }
            | Type::UpdateMask
            | Type::AuraMask
            | Type::CString
            | Type::PackedGuid => "-".to_string(),
        }
    }
}
