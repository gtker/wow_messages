use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_chat_restricted.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_chat_restricted.wowm#L13):
/// ```text
/// enum ChatRestrictionType : u8 {
///     CHAT_RESTRICTED = 0;
///     CHAT_THROTTLED = 1;
///     USER_SQUELCHED = 2;
///     YELL_RESTRICTED = 3;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ChatRestrictionType {
    ChatRestricted,
    ChatThrottled,
    UserSquelched,
    YellRestricted,
}

impl ChatRestrictionType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::ChatRestricted => 0x0,
            Self::ChatThrottled => 0x1,
            Self::UserSquelched => 0x2,
            Self::YellRestricted => 0x3,
        }
    }

}

impl Default for ChatRestrictionType {
    fn default() -> Self {
        Self::ChatRestricted
    }
}

impl std::fmt::Display for ChatRestrictionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChatRestricted => f.write_str("ChatRestricted"),
            Self::ChatThrottled => f.write_str("ChatThrottled"),
            Self::UserSquelched => f.write_str("UserSquelched"),
            Self::YellRestricted => f.write_str("YellRestricted"),
        }
    }
}

impl TryFrom<u8> for ChatRestrictionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ChatRestricted),
            1 => Ok(Self::ChatThrottled),
            2 => Ok(Self::UserSquelched),
            3 => Ok(Self::YellRestricted),
            v => Err(crate::errors::EnumError::new("ChatRestrictionType", v as u32),)
        }
    }
}

