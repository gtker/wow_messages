use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::definer::Definer;
use crate::parser::types::parsed::parsed_ty::ParsedType;
use crate::parser::types::sizes::{
    update_mask_max, Sizes, ADDON_ARRAY_MAX, ADDON_ARRAY_MIN, AURA_MASK_MAX_SIZE,
    AURA_MASK_MIN_SIZE, DATETIME_SIZE, GOLD_SIZE, GUID_SIZE, IP_ADDRESS_SIZE, LEVEL16_SIZE,
    LEVEL32_SIZE, LEVEL_SIZE, NAMED_GUID_MAX_SIZE, NAMED_GUID_MIN_SIZE, PACKED_GUID_MAX_SIZE,
    PACKED_GUID_MIN_SIZE, UPDATE_MASK_MIN_SIZE, VARIABLE_ITEM_RANDOM_PROPERTY_MAX_SIZE,
    VARIABLE_ITEM_RANDOM_PROPERTY_MIN_SIZE,
};
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::{FloatingPointType, IntegerType};
use crate::{
    Container, CSTRING_LARGEST_ALLOWED, CSTRING_SMALLEST_ALLOWED, ENCHANT_MASK_LARGEST_ALLOWED,
    ENCHANT_MASK_SMALLEST_ALLOWED, INSPECT_TALENT_GEAR_MASK_LARGEST_ALLOWED,
    INSPECT_TALENT_GEAR_MASK_SMALLEST_ALLOWED, MONSTER_MOVE_SPLINE_LARGEST_ALLOWED,
    MONSTER_MOVE_SPLINE_SMALLEST_ALLOWED, SIZED_CSTRING_LARGEST_ALLOWED,
    SIZED_CSTRING_SMALLEST_ALLOWED, STRING_LARGEST_POSSIBLE, STRING_SMALLEST_POSSIBLE,
};
use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum Type {
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
    MonsterMoveSplines,
    AuraMask,
    AchievementDoneArray,
    AchievementInProgressArray,
    EnchantMask,
    InspectTalentGearMask,
    Gold,
    Level,
    Level16,
    Level32,
    VariableItemRandomProperty,
    AddonArray,
    IpAddress,
}

impl Type {
    pub(crate) const SPELL_NAME: &'static str = "Spell";
    pub(crate) const LEVEL_NAME: &'static str = "Level";
    pub(crate) const LEVEL_NAME16: &'static str = "Level16";
    pub(crate) const LEVEL_NAME32: &'static str = "Level32";
    pub(crate) const GOLD_NAME: &'static str = "Gold";
    pub(crate) const GUID_NAME: &'static str = "Guid";
    pub(crate) const PACKED_GUID_NAME: &'static str = "PackedGuid";
    pub(crate) const AURA_MASK_NAME: &'static str = "AuraMask";
    pub(crate) const UPDATE_MASK_NAME: &'static str = "UpdateMask";
    pub(crate) const DATE_TIME_NAME: &'static str = "DateTime";
    pub(crate) const C_STRING_NAME: &'static str = "CString";
    pub(crate) const SIZED_C_STRING_NAME: &'static str = "SizedCString";
    pub(crate) const STRING_NAME: &'static str = "String";
    pub(crate) const MONSTER_MOVE_SPLINES_NAME: &'static str = "MonsterMoveSplines";
    pub(crate) const ACHIEVEMENT_DONE_ARRAY_NAME: &'static str = "AchievementDoneArray";
    pub(crate) const ACHIEVEMENT_IN_PROGRESS_ARRAY_NAME: &'static str =
        "AchievementInProgressArray";
    pub(crate) const ENCHANT_MASK_NAME: &'static str = "EnchantMask";
    pub(crate) const INSPECT_TALENT_GEAR_MASK_NAME: &'static str = "InspectTalentGearMask";
    pub(crate) const NAMED_GUID_NAME: &'static str = "NamedGuid";
    pub(crate) const VARIABLE_ITEM_RANDOM_PROPERTY_NAME: &'static str =
        "VariableItemRandomProperty";
    pub(crate) const ADDON_ARRAY_NAME: &'static str = "AddonArray";
    pub(crate) const IP_ADDRESS_NAME: &'static str = "IpAddress";

    pub(crate) const STRINGS_RUST_NAME: &'static str = "String";
    pub(crate) const GUIDS_RUST_NAME: &'static str = "Guid";
    pub(crate) const BOOLS_RUST_NAME: &'static str = "bool";
    pub(crate) const MONSTER_MOVE_SPLINES_RUST_NAME: &'static str = "Vec<Vector3d>";
    pub(crate) const ACHIEVEMENT_DONE_ARRAY_RUST_NAME: &'static str = "Vec<AchievementDone>";
    pub(crate) const ACHIEVEMENT_IN_PROGRESS_ARRAY_RUST_NAME: &'static str =
        "Vec<AchievementInProgress>";
    pub(crate) const ADDON_ARRAY_RUST_NAME: &'static str = "Vec<Addon>";
    pub(crate) const IP_ADDRESS_RUST_NAME: &'static str = "Ipv4Addr";

    pub(crate) fn str(&self) -> String {
        match self {
            Type::Array(s) => s.str(),
            Type::Enum { e, .. } | Type::Flag { e, .. } => e.name().to_string(),
            Type::Struct { e } => e.name().to_string(),

            _ => self.to_parsed_type().str(),
        }
    }

    pub(crate) fn rust_str(&self) -> String {
        match self {
            Type::Array(s) => s.rust_str(),
            Type::Enum { e, .. } | Type::Flag { e, .. } => e.name().to_string(),
            Type::Struct { e } => e.name().to_string(),

            _ => self.to_parsed_type().rust_str(),
        }
    }

    fn to_parsed_type(&self) -> ParsedType {
        match self {
            Type::Integer(i) => ParsedType::Integer(*i),
            Type::Bool(i) => ParsedType::Bool(*i),
            Type::PackedGuid => ParsedType::PackedGuid,
            Type::Guid => ParsedType::Guid,
            Type::NamedGuid => ParsedType::NamedGuid,
            Type::DateTime => ParsedType::DateTime,
            Type::FloatingPoint(i) => ParsedType::FloatingPoint(*i),
            Type::CString => ParsedType::CString,
            Type::SizedCString => ParsedType::SizedCString,
            Type::String => ParsedType::String,
            Type::UpdateMask => ParsedType::UpdateMask,
            Type::MonsterMoveSplines => ParsedType::MonsterMoveSpline,
            Type::AuraMask => ParsedType::AuraMask,
            Type::AchievementDoneArray => ParsedType::AchievementDoneArray,
            Type::AchievementInProgressArray => ParsedType::AchievementInProgressArray,
            Type::EnchantMask => ParsedType::EnchantMask,
            Type::InspectTalentGearMask => ParsedType::InspectTalentGearMask,
            Type::Gold => ParsedType::Gold,
            Type::Level => ParsedType::Level,
            Type::Level16 => ParsedType::Level16,
            Type::Level32 => ParsedType::Level32,
            Type::VariableItemRandomProperty => ParsedType::VariableItemRandomProperty,
            Type::AddonArray => ParsedType::AddonArray,
            Type::IpAddress => ParsedType::IpAddress,

            Type::Array(_) | Type::Enum { .. } | Type::Flag { .. } | Type::Struct { .. } => {
                panic!("invalid conversion")
            }
        }
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
            Type::MonsterMoveSplines => {
                sizes.inc(
                    MONSTER_MOVE_SPLINE_SMALLEST_ALLOWED,
                    MONSTER_MOVE_SPLINE_LARGEST_ALLOWED,
                );
            }
            Type::EnchantMask => {
                sizes.inc(ENCHANT_MASK_SMALLEST_ALLOWED, ENCHANT_MASK_LARGEST_ALLOWED)
            }
            Type::InspectTalentGearMask => sizes.inc(
                INSPECT_TALENT_GEAR_MASK_SMALLEST_ALLOWED,
                INSPECT_TALENT_GEAR_MASK_LARGEST_ALLOWED,
            ),
            Type::Gold => sizes.inc_both(GOLD_SIZE.into()),
            Type::Level => sizes.inc_both(LEVEL_SIZE.into()),
            Type::Level16 => sizes.inc_both(LEVEL16_SIZE),
            Type::Level32 => sizes.inc_both(LEVEL32_SIZE),
            Type::NamedGuid => sizes.inc(NAMED_GUID_MIN_SIZE, NAMED_GUID_MAX_SIZE),
            Type::VariableItemRandomProperty => sizes.inc(
                VARIABLE_ITEM_RANDOM_PROPERTY_MIN_SIZE,
                VARIABLE_ITEM_RANDOM_PROPERTY_MAX_SIZE,
            ),
            Type::AddonArray => sizes.inc(ADDON_ARRAY_MIN, ADDON_ARRAY_MAX),
            Type::IpAddress => sizes.inc_both(IP_ADDRESS_SIZE),
        }

        sizes
    }

    pub(crate) fn doc_size_of(&self, tags: &ObjectTags) -> String {
        match self {
            Type::Array(_) => {
                if let Some(size) = self.sizes(tags).is_constant() {
                    size.to_string()
                } else {
                    "?".to_string()
                }
            }
            _ => {
                if let Some(size) = self.sizes(tags).is_constant() {
                    size.to_string()
                } else {
                    "-".to_string()
                }
            }
        }
    }

    pub(crate) fn doc_endian_str(&self) -> String {
        match self {
            Type::Bool(i) | Type::Integer(i) => i.doc_endian_str().to_string(),

            Type::FloatingPoint(_)
            | Type::Level16
            | Type::Level32
            | Type::Gold
            | Type::DateTime
            | Type::Guid => "Little".to_string(),

            Type::IpAddress => "Big".to_string(),

            Type::AddonArray
            | Type::VariableItemRandomProperty
            | Type::NamedGuid
            | Type::Level
            | Type::EnchantMask
            | Type::InspectTalentGearMask
            | Type::MonsterMoveSplines
            | Type::SizedCString
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
