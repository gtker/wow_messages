use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::definer::Definer;
use crate::parser::types::parsed::parsed_ty::ParsedType;
use crate::parser::types::sizes::{
    Sizes, GUID_SIZE, PACKED_GUID_MAX_SIZE, PACKED_GUID_MIN_SIZE, UPDATE_MASK_MIN_SIZE,
};
use crate::parser::types::IntegerType;
use crate::{Container, CSTRING_LARGEST_ALLOWED, CSTRING_SMALLEST_ALLOWED};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum Type {
    Integer(IntegerType),
    Bool(IntegerType),
    PackedGuid,
    Guid,
    NamedGuid,
    DateTime,
    FloatingPoint,
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
    UpdateMask {
        max_size: i128,
    },
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
    Seconds,
    Milliseconds,
}

impl Type {
    pub(crate) const F32_NAME: &'static str = "f32";
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
    pub(crate) const SECONDS_NAME: &'static str = "Seconds";
    pub(crate) const MILLISECONDS_NAME: &'static str = "Milliseconds";

    pub(crate) const STRINGS_RUST_NAME: &'static str = "String";
    pub(crate) const GUIDS_RUST_NAME: &'static str = "Guid";
    pub(crate) const BOOLS_RUST_NAME: &'static str = "bool";
    pub(crate) const MONSTER_MOVE_SPLINES_RUST_NAME: &'static str = "Vec<Vector3d>";
    pub(crate) const ACHIEVEMENT_DONE_ARRAY_RUST_NAME: &'static str = "Vec<AchievementDone>";
    pub(crate) const ACHIEVEMENT_IN_PROGRESS_ARRAY_RUST_NAME: &'static str =
        "Vec<AchievementInProgress>";
    pub(crate) const ADDON_ARRAY_RUST_NAME: &'static str = "Vec<Addon>";
    pub(crate) const IP_ADDRESS_RUST_NAME: &'static str = "Ipv4Addr";
    pub(crate) const DURATIONS_RUST_NAME: &'static str = "Duration";

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

            Type::Enum { .. } | Type::Flag { .. } | Type::Struct { .. } => self.str(),

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
            Type::FloatingPoint => ParsedType::FloatingPoint,
            Type::CString => ParsedType::CString,
            Type::SizedCString => ParsedType::SizedCString,
            Type::String => ParsedType::String,
            Type::UpdateMask { .. } => ParsedType::UpdateMask,
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
            Type::Seconds => ParsedType::Seconds,
            Type::Milliseconds => ParsedType::Milliseconds,
        }
    }

    // NOTE: Definers used in if statements count if statement contents
    pub(crate) fn sizes(&self) -> Sizes {
        let mut sizes = Sizes::new();

        match self {
            Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
                let s = if let Some(upcast) = upcast {
                    upcast.size()
                } else {
                    e.ty().size()
                };

                sizes.inc_both(s.into());
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
                    ArraySize::Fixed(f) => (f, f),
                    ArraySize::Variable(f) => match f.ty() {
                        Type::Integer(i) => (i.smallest_array_value(), i.largest_value()),
                        _ => unreachable!("only ints can be string lengths"),
                    },
                    ArraySize::Endless => unreachable!("Endless has already been matched"),
                };

                let (inner_min, inner_max): (i128, i128) = match array.ty() {
                    ArrayType::Integer(i) => (i.size().into(), i.size().into()),
                    ArrayType::Guid => (GUID_SIZE.into(), GUID_SIZE.into()),
                    ArrayType::PackedGuid => {
                        (PACKED_GUID_MIN_SIZE.into(), PACKED_GUID_MAX_SIZE.into())
                    }
                    ArrayType::CString => (
                        CSTRING_SMALLEST_ALLOWED.into(),
                        CSTRING_LARGEST_ALLOWED.into(),
                    ),
                    ArrayType::Struct(c) => {
                        let c = c.sizes();

                        (c.minimum(), c.maximum())
                    }
                };

                sizes.inc(inner_min.saturating_mul(min), inner_max.saturating_mul(max));
            }

            Type::UpdateMask { max_size } => sizes.inc(UPDATE_MASK_MIN_SIZE.into(), *max_size),

            _ => {
                let (min, max) = self.to_parsed_type().min_max_size();
                sizes.inc(min, max);
            }
        }

        sizes
    }

    pub(crate) fn doc_size_of(&self) -> String {
        match self {
            Type::Array(_) => {
                if let Some(size) = self.sizes().is_constant() {
                    size.to_string()
                } else {
                    "?".to_string()
                }
            }
            _ => {
                if let Some(size) = self.sizes().is_constant() {
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

            Type::Seconds
            | Type::Milliseconds
            | Type::FloatingPoint
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
            | Type::UpdateMask { .. }
            | Type::AuraMask
            | Type::CString
            | Type::PackedGuid => "-".to_string(),
        }
    }
}
