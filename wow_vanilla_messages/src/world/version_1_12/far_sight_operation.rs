use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm#L3):
/// ```text
/// enum FarSightOperation : u8 {
///     REMOVE = 0;
///     ADD = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum FarSightOperation {
    REMOVE,
    ADD,
}

impl FarSightOperation {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::REMOVE => 0x0,
            Self::ADD => 0x1,
        }
    }

}

impl Default for FarSightOperation {
    fn default() -> Self {
        Self::REMOVE
    }
}

impl std::fmt::Display for FarSightOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::REMOVE => f.write_str("REMOVE"),
            Self::ADD => f.write_str("ADD"),
        }
    }
}

impl TryFrom<u8> for FarSightOperation {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::REMOVE),
            1 => Ok(Self::ADD),
            v => Err(crate::errors::EnumError::new("FarSightOperation", v as u32),)
        }
    }
}

