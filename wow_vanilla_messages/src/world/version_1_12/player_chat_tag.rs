use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm#L3):
/// ```text
/// enum PlayerChatTag : u8 {
///     NONE = 0;
///     AFK = 1;
///     DND = 2;
///     GM = 3;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PlayerChatTag {
    NONE,
    AFK,
    DND,
    GM,
}

impl PlayerChatTag {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::AFK => 0x1,
            Self::DND => 0x2,
            Self::GM => 0x3,
        }
    }

}

impl Default for PlayerChatTag {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for PlayerChatTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::AFK => f.write_str("AFK"),
            Self::DND => f.write_str("DND"),
            Self::GM => f.write_str("GM"),
        }
    }
}

impl TryFrom<u8> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::AFK),
            2 => Ok(Self::DND),
            3 => Ok(Self::GM),
            v => Err(crate::errors::EnumError::new("PlayerChatTag", v as u32),)
        }
    }
}

