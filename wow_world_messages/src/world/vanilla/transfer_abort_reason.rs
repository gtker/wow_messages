use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L1):
/// ```text
/// enum TransferAbortReason : u8 {
///     NONE = 0x00;
///     IS_FULL = 0x01;
///     NOT_FOUND = 0x02;
///     TOO_MANY_INSTANCES = 0x03;
///     ZONE_IS_IN_COMBAT = 0x05;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TransferAbortReason {
    None,
    IsFull,
    NotFound,
    TooManyInstances,
    ZoneIsInCombat,
}

impl TransferAbortReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::IsFull => 0x1,
            Self::NotFound => 0x2,
            Self::TooManyInstances => 0x3,
            Self::ZoneIsInCombat => 0x5,
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
        }
    }
}

impl TryFrom<u8> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::IsFull),
            2 => Ok(Self::NotFound),
            3 => Ok(Self::TooManyInstances),
            5 => Ok(Self::ZoneIsInCombat),
            v => Err(crate::errors::EnumError::new("TransferAbortReason", v as u32),)
        }
    }
}

