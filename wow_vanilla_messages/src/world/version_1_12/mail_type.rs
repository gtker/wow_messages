use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm#L3):
/// ```text
/// enum MailType : u8 {
///     NORMAL = 0;
///     AUCTION = 2;
///     CREATURE = 3;
///     GAMEOBJECT = 4;
///     ITEM = 5;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum MailType {
    NORMAL,
    AUCTION,
    /// # Comment
    /// 
    /// client send CMSG_CREATURE_QUERY on this mailmessagetype
    CREATURE,
    /// # Comment
    /// 
    /// client send CMSG_GAMEOBJECT_QUERY on this mailmessagetype
    GAMEOBJECT,
    /// # Comment
    /// 
    /// client send CMSG_ITEM_QUERY on this mailmessagetype
    ITEM,
}

impl MailType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NORMAL => 0x0,
            Self::AUCTION => 0x2,
            Self::CREATURE => 0x3,
            Self::GAMEOBJECT => 0x4,
            Self::ITEM => 0x5,
        }
    }

}

impl Default for MailType {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl std::fmt::Display for MailType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NORMAL => f.write_str("NORMAL"),
            Self::AUCTION => f.write_str("AUCTION"),
            Self::CREATURE => f.write_str("CREATURE"),
            Self::GAMEOBJECT => f.write_str("GAMEOBJECT"),
            Self::ITEM => f.write_str("ITEM"),
        }
    }
}

impl TryFrom<u8> for MailType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            2 => Ok(Self::AUCTION),
            3 => Ok(Self::CREATURE),
            4 => Ok(Self::GAMEOBJECT),
            5 => Ok(Self::ITEM),
            v => Err(crate::errors::EnumError::new("MailType", v as u32),)
        }
    }
}

