use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_push_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_push_result.wowm#L3):
/// ```text
/// enum NewItemSource : u32 {
///     LOOTED = 0;
///     FROM_NPC = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum NewItemSource {
    LOOTED,
    FROM_NPC,
}

impl NewItemSource {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::LOOTED => 0x0,
            Self::FROM_NPC => 0x1,
        }
    }

}

impl Default for NewItemSource {
    fn default() -> Self {
        Self::LOOTED
    }
}

impl std::fmt::Display for NewItemSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LOOTED => f.write_str("LOOTED"),
            Self::FROM_NPC => f.write_str("FROM_NPC"),
        }
    }
}

impl TryFrom<u32> for NewItemSource {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LOOTED),
            1 => Ok(Self::FROM_NPC),
            v => Err(crate::errors::EnumError::new("NewItemSource", v as u32),)
        }
    }
}

