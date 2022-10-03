use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::{get_container, get_definer};
use crate::parser::types::objects::Objects;
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_ty::ParsedType;
use crate::parser::types::sizes::{
    update_mask_max, Sizes, AURA_MASK_MAX_SIZE, AURA_MASK_MIN_SIZE, BOOL_SIZE, DATETIME_SIZE,
    GUID_SIZE, PACKED_GUID_MAX_SIZE, PACKED_GUID_MIN_SIZE, UPDATE_MASK_MIN_SIZE,
};
use crate::parser::types::{Array, ArraySize, ArrayType, FloatingPointType, IntegerType};
use crate::{
    Tags, CSTRING_LARGEST_ALLOWED, CSTRING_SMALLEST_ALLOWED, SIZED_CSTRING_LARGEST_ALLOWED,
    SIZED_CSTRING_SMALLEST_ALLOWED,
};
use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum Type {
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
    pub(crate) fn str(&self) -> String {
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

    pub(crate) fn rust_str(&self) -> String {
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

    // NOTE: Definers used in if statements count if statement contents
    pub(crate) fn sizes_parsed(
        &self,
        e: &ParsedContainer,
        containers: &[ParsedContainer],
        definers: &[Definer],
    ) -> Sizes {
        let mut sizes = Sizes::new();

        match self {
            Type::Integer(i) => sizes.inc_both(i.size() as usize),
            Type::Bool => sizes.inc_both(BOOL_SIZE.into()),
            Type::Guid => sizes.inc_both(GUID_SIZE as _),
            Type::DateTime => sizes.inc_both(DATETIME_SIZE.into()),
            Type::FloatingPoint(i) => sizes.inc_both(i.size() as usize),
            Type::PackedGuid => sizes.inc(PACKED_GUID_MIN_SIZE as _, PACKED_GUID_MAX_SIZE as _),
            Type::UpdateMask => sizes.inc(
                UPDATE_MASK_MIN_SIZE as usize,
                update_mask_max(e.tags().first_version().as_major_world()) as usize,
            ),
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
                        ParsedType::Integer(i) => sizes.inc(i.smallest_value(), i.largest_value()),
                        _ => unreachable!("string lengths can only be int"),
                    }
                }
            }
            Type::Identifier { s, upcast } => {
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
                    unreachable!()
                }
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
                    ArraySize::Variable(f) => match e.get_field_ty(&f) {
                        ParsedType::Integer(i) => (i.smallest_value(), i.largest_value()),
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
                    ArrayType::Complex(s) => {
                        if let Some(e) = get_definer(definers, s, e.tags()) {
                            let s = e.ty().size();
                            sizes.inc(s as usize * min, s as usize * max);
                        } else if let Some(c) = get_container(containers, s, e.tags()) {
                            let c = c.create_sizes(containers, definers);

                            sizes.inc(min * c.minimum(), 0);
                            sizes.inc(0, max.saturating_mul(c.maximum()));
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        }

        sizes
    }

    pub(crate) fn doc_size_of(&self, tags: &Tags, o: &Objects) -> String {
        match self {
            Type::Integer(i) => i.size().to_string(),
            Type::Guid => 8.to_string(),
            Type::FloatingPoint(f) => f.size().to_string(),
            Type::String { length } => length.clone(),
            Type::Identifier { s, upcast } => {
                let sizes = o.get_object(s, tags).sizes();
                if let Some(upcast) = upcast {
                    upcast.size().to_string()
                } else if let Some(size) = sizes.is_constant() {
                    size.to_string()
                } else {
                    "-".to_string()
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

    pub(crate) fn doc_endian_str(&self) -> String {
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
}
