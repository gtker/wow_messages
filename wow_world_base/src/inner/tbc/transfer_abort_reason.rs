/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L21):
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
            v => Err(crate::errors::EnumError::new("TransferAbortReason", v as u64),)
        }
    }
}

