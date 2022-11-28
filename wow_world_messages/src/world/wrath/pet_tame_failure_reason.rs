use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm#L30):
/// ```text
/// enum PetTameFailureReason : u8 {
///     INVALID_CREATURE = 1;
///     TOO_MANY = 2;
///     CREATURE_ALREADY_OWNED = 3;
///     NOT_TAMEABLE = 4;
///     ANOTHER_SUMMON_ACTIVE = 5;
///     UNITS_CANT_TAME = 6;
///     NO_PET_AVAILABLE = 7;
///     INTERNA_LERROR = 8;
///     TOO_HIGH_LEVEL = 9;
///     DEAD = 10;
///     NOT_DEAD = 11;
///     CANT_CONTROL_EXOTIC = 12;
///     UNKNOWN_ERROR = 13;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetTameFailureReason {
    InvalidCreature,
    TooMany,
    CreatureAlreadyOwned,
    NotTameable,
    AnotherSummonActive,
    UnitsCantTame,
    /// not used in taming
    ///
    NoPetAvailable,
    InternaLerror,
    TooHighLevel,
    /// not used in taming
    ///
    Dead,
    /// not used in taming
    ///
    NotDead,
    CantControlExotic,
    UnknownError,
}

impl PetTameFailureReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::InvalidCreature => 0x1,
            Self::TooMany => 0x2,
            Self::CreatureAlreadyOwned => 0x3,
            Self::NotTameable => 0x4,
            Self::AnotherSummonActive => 0x5,
            Self::UnitsCantTame => 0x6,
            Self::NoPetAvailable => 0x7,
            Self::InternaLerror => 0x8,
            Self::TooHighLevel => 0x9,
            Self::Dead => 0xa,
            Self::NotDead => 0xb,
            Self::CantControlExotic => 0xc,
            Self::UnknownError => 0xd,
        }
    }

}

impl Default for PetTameFailureReason {
    fn default() -> Self {
        Self::InvalidCreature
    }
}

impl std::fmt::Display for PetTameFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCreature => f.write_str("InvalidCreature"),
            Self::TooMany => f.write_str("TooMany"),
            Self::CreatureAlreadyOwned => f.write_str("CreatureAlreadyOwned"),
            Self::NotTameable => f.write_str("NotTameable"),
            Self::AnotherSummonActive => f.write_str("AnotherSummonActive"),
            Self::UnitsCantTame => f.write_str("UnitsCantTame"),
            Self::NoPetAvailable => f.write_str("NoPetAvailable"),
            Self::InternaLerror => f.write_str("InternaLerror"),
            Self::TooHighLevel => f.write_str("TooHighLevel"),
            Self::Dead => f.write_str("Dead"),
            Self::NotDead => f.write_str("NotDead"),
            Self::CantControlExotic => f.write_str("CantControlExotic"),
            Self::UnknownError => f.write_str("UnknownError"),
        }
    }
}

impl TryFrom<u8> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::InvalidCreature),
            2 => Ok(Self::TooMany),
            3 => Ok(Self::CreatureAlreadyOwned),
            4 => Ok(Self::NotTameable),
            5 => Ok(Self::AnotherSummonActive),
            6 => Ok(Self::UnitsCantTame),
            7 => Ok(Self::NoPetAvailable),
            8 => Ok(Self::InternaLerror),
            9 => Ok(Self::TooHighLevel),
            10 => Ok(Self::Dead),
            11 => Ok(Self::NotDead),
            12 => Ok(Self::CantControlExotic),
            13 => Ok(Self::UnknownError),
            v => Err(crate::errors::EnumError::new("PetTameFailureReason", v as u32),)
        }
    }
}

