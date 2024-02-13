/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm#L1):
/// ```text
/// enum PetTameFailureReason : u8 {
///     INVALID_CREATURE = 1;
///     TOO_MANY = 2;
///     CREATURE_ALREADY_OWNED = 3;
///     NOT_TAMEABLE = 4;
///     ANOTHER_SUMMON_ACTIVE = 5;
///     UNITS_CANT_TAME = 6;
///     NO_PET_AVAILABLE = 7;
///     INTERNAL_ERROR = 8;
///     TOO_HIGH_LEVEL = 9;
///     DEAD = 10;
///     NOT_DEAD = 11;
///     UNKNOWN_ERROR = 12;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PetTameFailureReason {
    InvalidCreature,
    TooMany,
    CreatureAlreadyOwned,
    NotTameable,
    AnotherSummonActive,
    UnitsCantTame,
    /// not used in taming
    NoPetAvailable,
    InternalError,
    TooHighLevel,
    /// not used in taming
    Dead,
    /// not used in taming
    NotDead,
    UnknownError,
}

impl PetTameFailureReason {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::InvalidCreature => 0x1,
            Self::TooMany => 0x2,
            Self::CreatureAlreadyOwned => 0x3,
            Self::NotTameable => 0x4,
            Self::AnotherSummonActive => 0x5,
            Self::UnitsCantTame => 0x6,
            Self::NoPetAvailable => 0x7,
            Self::InternalError => 0x8,
            Self::TooHighLevel => 0x9,
            Self::Dead => 0xa,
            Self::NotDead => 0xb,
            Self::UnknownError => 0xc,
        }
    }

    pub const fn variants() -> [Self; 12] {
        [
            Self::InvalidCreature,
            Self::TooMany,
            Self::CreatureAlreadyOwned,
            Self::NotTameable,
            Self::AnotherSummonActive,
            Self::UnitsCantTame,
            Self::NoPetAvailable,
            Self::InternalError,
            Self::TooHighLevel,
            Self::Dead,
            Self::NotDead,
            Self::UnknownError,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            1 => Ok(Self::InvalidCreature),
            2 => Ok(Self::TooMany),
            3 => Ok(Self::CreatureAlreadyOwned),
            4 => Ok(Self::NotTameable),
            5 => Ok(Self::AnotherSummonActive),
            6 => Ok(Self::UnitsCantTame),
            7 => Ok(Self::NoPetAvailable),
            8 => Ok(Self::InternalError),
            9 => Ok(Self::TooHighLevel),
            10 => Ok(Self::Dead),
            11 => Ok(Self::NotDead),
            12 => Ok(Self::UnknownError),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl PetTameFailureReason {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::InvalidCreature => "INVALID_CREATURE",
            Self::TooMany => "TOO_MANY",
            Self::CreatureAlreadyOwned => "CREATURE_ALREADY_OWNED",
            Self::NotTameable => "NOT_TAMEABLE",
            Self::AnotherSummonActive => "ANOTHER_SUMMON_ACTIVE",
            Self::UnitsCantTame => "UNITS_CANT_TAME",
            Self::NoPetAvailable => "NO_PET_AVAILABLE",
            Self::InternalError => "INTERNAL_ERROR",
            Self::TooHighLevel => "TOO_HIGH_LEVEL",
            Self::Dead => "DEAD",
            Self::NotDead => "NOT_DEAD",
            Self::UnknownError => "UNKNOWN_ERROR",
        }
    }

}

const NAME: &str = "PetTameFailureReason";

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
            Self::InternalError => f.write_str("InternalError"),
            Self::TooHighLevel => f.write_str("TooHighLevel"),
            Self::Dead => f.write_str("Dead"),
            Self::NotDead => f.write_str("NotDead"),
            Self::UnknownError => f.write_str("UnknownError"),
        }
    }
}

impl TryFrom<u8> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

