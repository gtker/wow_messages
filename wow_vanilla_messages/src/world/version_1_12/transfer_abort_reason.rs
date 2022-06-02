use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L3):
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
    NONE,
    IS_FULL,
    NOT_FOUND,
    TOO_MANY_INSTANCES,
    ZONE_IS_IN_COMBAT,
}

impl TransferAbortReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::IS_FULL => 0x1,
            Self::NOT_FOUND => 0x2,
            Self::TOO_MANY_INSTANCES => 0x3,
            Self::ZONE_IS_IN_COMBAT => 0x5,
        }
    }

}

impl Default for TransferAbortReason {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for TransferAbortReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::IS_FULL => f.write_str("IS_FULL"),
            Self::NOT_FOUND => f.write_str("NOT_FOUND"),
            Self::TOO_MANY_INSTANCES => f.write_str("TOO_MANY_INSTANCES"),
            Self::ZONE_IS_IN_COMBAT => f.write_str("ZONE_IS_IN_COMBAT"),
        }
    }
}

impl TryFrom<u8> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::IS_FULL),
            2 => Ok(Self::NOT_FOUND),
            3 => Ok(Self::TOO_MANY_INSTANCES),
            5 => Ok(Self::ZONE_IS_IN_COMBAT),
            v => Err(crate::errors::EnumError::new("TransferAbortReason", v as u32),)
        }
    }
}

