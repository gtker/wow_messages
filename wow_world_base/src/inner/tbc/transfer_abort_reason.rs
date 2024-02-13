/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L20):
/// ```text
/// enum TransferAbortReason : u8 {
///     NONE = 0x00;
///     IS_FULL = 0x01;
///     NOT_FOUND = 0x02;
///     TOO_MANY_INSTANCES = 0x03;
///     ZONE_IS_IN_COMBAT = 0x05;
///     INSUFFICIENT_EXPANSION_LEVEL = 0x06;
///     DIFFICULTY_NOT_AVAILABLE = 0x07;
///     MISSING_DIFFICULTY = 8;
///     ZONE_IN_COMBAT = 9;
///     INSTANCE_IS_FULL = 10;
///     NOT_ALLOWED = 11;
///     HAS_BIND = 12;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TransferAbortReason {
    None,
    IsFull,
    NotFound,
    TooManyInstances,
    ZoneIsInCombat,
    InsufficientExpansionLevel,
    DifficultyNotAvailable,
    MissingDifficulty,
    ZoneInCombat,
    InstanceIsFull,
    NotAllowed,
    HasBind,
}

impl TransferAbortReason {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::IsFull => 0x1,
            Self::NotFound => 0x2,
            Self::TooManyInstances => 0x3,
            Self::ZoneIsInCombat => 0x5,
            Self::InsufficientExpansionLevel => 0x6,
            Self::DifficultyNotAvailable => 0x7,
            Self::MissingDifficulty => 0x8,
            Self::ZoneInCombat => 0x9,
            Self::InstanceIsFull => 0xa,
            Self::NotAllowed => 0xb,
            Self::HasBind => 0xc,
        }
    }

    pub const fn variants() -> [Self; 12] {
        [
            Self::None,
            Self::IsFull,
            Self::NotFound,
            Self::TooManyInstances,
            Self::ZoneIsInCombat,
            Self::InsufficientExpansionLevel,
            Self::DifficultyNotAvailable,
            Self::MissingDifficulty,
            Self::ZoneInCombat,
            Self::InstanceIsFull,
            Self::NotAllowed,
            Self::HasBind,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::IsFull),
            2 => Ok(Self::NotFound),
            3 => Ok(Self::TooManyInstances),
            5 => Ok(Self::ZoneIsInCombat),
            6 => Ok(Self::InsufficientExpansionLevel),
            7 => Ok(Self::DifficultyNotAvailable),
            8 => Ok(Self::MissingDifficulty),
            9 => Ok(Self::ZoneInCombat),
            10 => Ok(Self::InstanceIsFull),
            11 => Ok(Self::NotAllowed),
            12 => Ok(Self::HasBind),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl TransferAbortReason {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::IsFull => "IS_FULL",
            Self::NotFound => "NOT_FOUND",
            Self::TooManyInstances => "TOO_MANY_INSTANCES",
            Self::ZoneIsInCombat => "ZONE_IS_IN_COMBAT",
            Self::InsufficientExpansionLevel => "INSUFFICIENT_EXPANSION_LEVEL",
            Self::DifficultyNotAvailable => "DIFFICULTY_NOT_AVAILABLE",
            Self::MissingDifficulty => "MISSING_DIFFICULTY",
            Self::ZoneInCombat => "ZONE_IN_COMBAT",
            Self::InstanceIsFull => "INSTANCE_IS_FULL",
            Self::NotAllowed => "NOT_ALLOWED",
            Self::HasBind => "HAS_BIND",
        }
    }

}

const NAME: &str = "TransferAbortReason";

impl Default for TransferAbortReason {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for TransferAbortReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::IsFull => f.write_str("IsFull"),
            Self::NotFound => f.write_str("NotFound"),
            Self::TooManyInstances => f.write_str("TooManyInstances"),
            Self::ZoneIsInCombat => f.write_str("ZoneIsInCombat"),
            Self::InsufficientExpansionLevel => f.write_str("InsufficientExpansionLevel"),
            Self::DifficultyNotAvailable => f.write_str("DifficultyNotAvailable"),
            Self::MissingDifficulty => f.write_str("MissingDifficulty"),
            Self::ZoneInCombat => f.write_str("ZoneInCombat"),
            Self::InstanceIsFull => f.write_str("InstanceIsFull"),
            Self::NotAllowed => f.write_str("NotAllowed"),
            Self::HasBind => f.write_str("HasBind"),
        }
    }
}

impl TryFrom<u8> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

