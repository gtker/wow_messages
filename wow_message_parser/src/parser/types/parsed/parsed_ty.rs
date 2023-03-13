use crate::error_printer::{complex_not_found, recursive_type, unsupported_upcast};
use crate::file_info::FileInfo;
use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::{get_container, get_definer, get_related};
use crate::parser::types::parsed::parsed_array::{ParsedArray, ParsedArraySize, ParsedArrayType};
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::sizes::{
    update_mask_max, Sizes, AURA_MASK_MAX_SIZE, AURA_MASK_MIN_SIZE, DATETIME_SIZE, GOLD_SIZE,
    GUID_SIZE, LEVEL16_SIZE, LEVEL32_SIZE, LEVEL_SIZE, NAMED_GUID_MAX_SIZE, NAMED_GUID_MIN_SIZE,
    PACKED_GUID_MAX_SIZE, PACKED_GUID_MIN_SIZE, UPDATE_MASK_MIN_SIZE,
};
use crate::parser::types::ty::Type;
use crate::parser::types::{Endianness, FloatingPointType, IntegerType};
use crate::{
    CSTRING_LARGEST_ALLOWED, CSTRING_SMALLEST_ALLOWED, ENCHANT_MASK_LARGEST_ALLOWED,
    ENCHANT_MASK_SMALLEST_ALLOWED, INSPECT_TALENT_GEAR_MASK_LARGEST_ALLOWED,
    INSPECT_TALENT_GEAR_MASK_SMALLEST_ALLOWED, MONSTER_MOVE_SPLINE_LARGEST_ALLOWED,
    MONSTER_MOVE_SPLINE_SMALLEST_ALLOWED, SIZED_CSTRING_LARGEST_ALLOWED,
    SIZED_CSTRING_SMALLEST_ALLOWED, STRING_LARGEST_POSSIBLE, STRING_SMALLEST_POSSIBLE,
};
use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum ParsedType {
    Integer(IntegerType),
    Bool(IntegerType),
    PackedGuid,
    Guid,
    NamedGuid,
    DateTime,
    FloatingPoint(FloatingPointType),
    CString,
    SizedCString,
    String,
    Array(ParsedArray),
    Identifier {
        s: String,
        upcast: Option<IntegerType>,
    },
    UpdateMask,
    AuraMask,
    MonsterMoveSpline,
    AchievementDoneArray,
    AchievementInProgressArray,
    EnchantMask,
    InspectTalentGearMask,
    Gold,
    Level,
    Level16,
    Level32,
}

impl ParsedType {
    pub(crate) fn str(&self) -> String {
        match self {
            ParsedType::Integer(i) => i.str().to_string(),
            ParsedType::CString => Type::C_STRING_NAME.to_string(),
            ParsedType::String => Type::STRING_NAME.to_string(),
            ParsedType::Array(a) => a.str(),
            ParsedType::Identifier { s, .. } => s.clone(),
            ParsedType::FloatingPoint(i) => i.str().to_string(),
            ParsedType::PackedGuid => Type::PACKED_GUID_NAME.to_string(),
            ParsedType::Guid => Type::GUID_NAME.to_string(),
            ParsedType::UpdateMask => Type::UPDATE_MASK_NAME.to_string(),
            ParsedType::AuraMask => Type::AURA_MASK_NAME.to_string(),
            ParsedType::SizedCString => Type::SIZED_C_STRING_NAME.to_string(),
            ParsedType::Bool(i) => bool_ty_to_string(i),
            ParsedType::DateTime => Type::DATE_TIME_NAME.to_string(),
            ParsedType::AchievementDoneArray => Type::ACHIEVEMENT_DONE_ARRAY_NAME.to_string(),
            ParsedType::AchievementInProgressArray => {
                Type::ACHIEVEMENT_IN_PROGRESS_ARRAY_NAME.to_string()
            }
            ParsedType::MonsterMoveSpline => Type::MONSTER_MOVE_SPLINES_NAME.to_string(),
            ParsedType::EnchantMask => Type::ENCHANT_MASK_NAME.to_string(),
            ParsedType::InspectTalentGearMask => Type::INSPECT_TALENT_GEAR_MASK_NAME.to_string(),
            ParsedType::Gold => Type::GOLD_NAME.to_string(),
            ParsedType::Level => Type::LEVEL_NAME.to_string(),
            ParsedType::Level16 => Type::LEVEL_NAME16.to_string(),
            ParsedType::Level32 => Type::LEVEL_NAME32.to_string(),
            ParsedType::NamedGuid => Type::NAMED_GUID_NAME.to_string(),
        }
    }

    pub(crate) fn rust_str(&self) -> String {
        match self {
            ParsedType::Integer(i) => i.rust_str().to_string(),
            ParsedType::SizedCString | ParsedType::CString | ParsedType::String => {
                Type::STRINGS_RUST_NAME.to_string()
            }
            ParsedType::Array(a) => a.rust_str(),
            ParsedType::Identifier { s, .. } => s.clone(),
            ParsedType::FloatingPoint(i) => i.rust_str().to_string(),
            ParsedType::Guid | ParsedType::PackedGuid => Type::GUIDS_RUST_NAME.to_string(),
            ParsedType::UpdateMask => Type::UPDATE_MASK_NAME.to_string(),
            ParsedType::AuraMask => Type::AURA_MASK_NAME.to_string(),
            ParsedType::Bool(_) => Type::BOOLS_RUST_NAME.to_string(),
            ParsedType::DateTime => Type::DATE_TIME_NAME.to_string(),
            ParsedType::AchievementDoneArray => Type::ACHIEVEMENT_DONE_ARRAY_NAME.to_string(),
            ParsedType::AchievementInProgressArray => {
                Type::ACHIEVEMENT_IN_PROGRESS_ARRAY_NAME.to_string()
            }
            ParsedType::MonsterMoveSpline => Type::MONSTER_MOVE_SPLINES_NAME.to_string(),
            ParsedType::EnchantMask => Type::ENCHANT_MASK_NAME.to_string(),
            ParsedType::InspectTalentGearMask => Type::INSPECT_TALENT_GEAR_MASK_NAME.to_string(),
            ParsedType::Gold => Type::GOLD_NAME.to_string(),
            ParsedType::Level16 | ParsedType::Level32 | ParsedType::Level => {
                Type::LEVEL_NAME.to_string()
            }
            ParsedType::NamedGuid => Type::NAMED_GUID_NAME.to_string(),
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
                let world_version = e.tags().main_versions().next().unwrap().as_major_world();
                sizes.inc(
                    UPDATE_MASK_MIN_SIZE as usize,
                    update_mask_max(world_version) as usize,
                )
            }
            ParsedType::AuraMask => {
                sizes.inc(AURA_MASK_MIN_SIZE as usize, AURA_MASK_MAX_SIZE as usize)
            }
            ParsedType::CString => sizes.inc(CSTRING_SMALLEST_ALLOWED, CSTRING_LARGEST_ALLOWED),
            ParsedType::SizedCString => sizes.inc(
                SIZED_CSTRING_SMALLEST_ALLOWED,
                SIZED_CSTRING_LARGEST_ALLOWED,
            ),
            ParsedType::String => {
                sizes.inc(STRING_SMALLEST_POSSIBLE, STRING_LARGEST_POSSIBLE);
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
                    let related = get_related(containers, definers, s);
                    complex_not_found(e.name(), e.tags(), &e.file_info, s, &related);
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
                        _ => panic!("only ints can be string lengths"),
                    },
                    ParsedArraySize::Endless => panic!(),
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
                            let related = get_related(containers, definers, s);
                            complex_not_found(e.name(), e.tags(), &e.file_info, s, &related);
                        }
                    }
                }
            }
            ParsedType::AchievementDoneArray | ParsedType::AchievementInProgressArray => {
                sizes.inc(0, usize::MAX);
            }
            ParsedType::MonsterMoveSpline => {
                sizes.inc(
                    MONSTER_MOVE_SPLINE_SMALLEST_ALLOWED,
                    MONSTER_MOVE_SPLINE_LARGEST_ALLOWED,
                );
            }
            ParsedType::EnchantMask => {
                sizes.inc(ENCHANT_MASK_SMALLEST_ALLOWED, ENCHANT_MASK_LARGEST_ALLOWED);
            }
            ParsedType::InspectTalentGearMask => sizes.inc(
                INSPECT_TALENT_GEAR_MASK_SMALLEST_ALLOWED,
                INSPECT_TALENT_GEAR_MASK_LARGEST_ALLOWED,
            ),
            ParsedType::Gold => sizes.inc_both(GOLD_SIZE.into()),
            ParsedType::Level => sizes.inc_both(LEVEL_SIZE.into()),
            ParsedType::Level16 => sizes.inc_both(LEVEL16_SIZE),
            ParsedType::Level32 => sizes.inc_both(LEVEL32_SIZE),
            ParsedType::NamedGuid => {
                sizes.inc(NAMED_GUID_MIN_SIZE.into(), NAMED_GUID_MAX_SIZE.into())
            }
        }

        sizes
    }

    pub(crate) fn with_upcast(
        s: &str,
        upcasted: &str,
        container_name: &str,
        variable_name: &str,
        file_info: &FileInfo,
    ) -> Self {
        let t = Self::from_str(s);
        match t {
            ParsedType::Identifier { .. } => {}
            _ => {
                unsupported_upcast(container_name, variable_name, s, upcasted, file_info);
            }
        }

        let int = IntegerType::from_str(upcasted, s, file_info);

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
            "u16_be" => Self::Integer(IntegerType::U16(Endianness::Big)),
            "u16" => Self::Integer(IntegerType::U16(Endianness::Little)),
            "u32_be" => Self::Integer(IntegerType::U32(Endianness::Big)),
            "u32" => Self::Integer(IntegerType::U32(Endianness::Little)),
            "u64" => Self::Integer(IntegerType::U64(Endianness::Little)),
            "u64_be" => Self::Integer(IntegerType::U64(Endianness::Big)),
            "i8" => Self::Integer(IntegerType::I8),
            "i16" => Self::Integer(IntegerType::I16(Endianness::Little)),
            "i16_be" => Self::Integer(IntegerType::I16(Endianness::Big)),
            "i32" => Self::Integer(IntegerType::I32(Endianness::Little)),
            "i32_be" => Self::Integer(IntegerType::I32(Endianness::Big)),
            "i64" => Self::Integer(IntegerType::I64(Endianness::Little)),
            "i64_be" => Self::Integer(IntegerType::I64(Endianness::Big)),
            "f32" => Self::FloatingPoint(FloatingPointType::F32(Endianness::Little)),
            "f32_be" => Self::FloatingPoint(FloatingPointType::F32(Endianness::Big)),
            "f64" => Self::FloatingPoint(FloatingPointType::F64(Endianness::Little)),
            "f64_be" => Self::FloatingPoint(FloatingPointType::F64(Endianness::Big)),

            "u48" => Self::Integer(IntegerType::U48),

            "Item16" | "Spell16" => Self::Integer(IntegerType::U16(Endianness::Little)),
            Type::LEVEL_NAME => Self::Level,
            Type::LEVEL_NAME16 => Self::Level16,
            Type::LEVEL_NAME32 => Self::Level32,
            Type::SPELL_NAME | "Milliseconds" | "Seconds" | "Item" => {
                Self::Integer(IntegerType::U32(Endianness::Little))
            }
            Type::GOLD_NAME => Self::Gold,
            Type::GUID_NAME => Self::Guid,
            Type::PACKED_GUID_NAME => Self::PackedGuid,
            Type::NAMED_GUID_NAME => Self::NamedGuid,
            Type::AURA_MASK_NAME => Self::AuraMask,
            Type::UPDATE_MASK_NAME => Self::UpdateMask,
            Type::C_STRING_NAME => Self::CString,
            Type::SIZED_C_STRING_NAME => Self::SizedCString,
            Type::DATE_TIME_NAME => Self::DateTime,
            Type::STRING_NAME => Self::String,
            Type::MONSTER_MOVE_SPLINES_NAME => Self::MonsterMoveSpline,
            Type::ACHIEVEMENT_DONE_ARRAY_NAME => Self::AchievementDoneArray,
            Type::ACHIEVEMENT_IN_PROGRESS_ARRAY_NAME => Self::AchievementInProgressArray,
            Type::ENCHANT_MASK_NAME => Self::EnchantMask,
            Type::INSPECT_TALENT_GEAR_MASK_NAME => Self::InspectTalentGearMask,
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
                            Self::Array(ParsedArray::new(ParsedArrayType::Complex(i), size))
                        }
                        ParsedType::CString => {
                            Self::Array(ParsedArray::new(ParsedArrayType::CString, size))
                        }
                        ParsedType::NamedGuid
                        | ParsedType::Level16
                        | ParsedType::Level32
                        | ParsedType::Level
                        | ParsedType::Gold
                        | ParsedType::EnchantMask
                        | ParsedType::InspectTalentGearMask
                        | ParsedType::AchievementDoneArray
                        | ParsedType::AchievementInProgressArray
                        | ParsedType::MonsterMoveSpline
                        | ParsedType::SizedCString
                        | ParsedType::String { .. }
                        | ParsedType::Array(_)
                        | ParsedType::FloatingPoint(_)
                        | ParsedType::UpdateMask
                        | ParsedType::AuraMask
                        | ParsedType::DateTime
                        | ParsedType::Bool(_) => unimplemented!("unsupported"),
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
        IntegerType::I8 | IntegerType::U8 => "Bool".to_string(),
        IntegerType::I16(_) | IntegerType::U16(_) => "Bool16".to_string(),
        IntegerType::I32(_) | IntegerType::U32(_) => "Bool32".to_string(),
        IntegerType::I64(_) | IntegerType::U64(_) => "Bool64".to_string(),
        IntegerType::U48 => unreachable!(),
    }
}
