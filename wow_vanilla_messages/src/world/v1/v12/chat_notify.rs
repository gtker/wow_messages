use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ChatNotify {
    JOINED_NOTICE,
    LEFT_NOTICE,
    YOU_JOINED_NOTICE,
    YOU_LEFT_NOTICE,
    WRONG_PASSWORD_NOTICE,
    NOT_MEMBER_NOTICE,
    NOT_MODERATOR_NOTICE,
    PASSWORD_CHANGED_NOTICE,
    OWNER_CHANGED_NOTICE,
    PLAYER_NOT_FOUND_NOTICE,
    NOT_OWNER_NOTICE,
    CHANNEL_OWNER_NOTICE,
    MODE_CHANGE_NOTICE,
    ANNOUNCEMENTS_ON_NOTICE,
    ANNOUNCEMENTS_OFF_NOTICE,
    MODERATION_ON_NOTICE,
    MODERATION_OFF_NOTICE,
    MUTED_NOTICE,
    PLAYER_KICKED_NOTICE,
    BANNED_NOTICE,
    PLAYER_BANNED_NOTICE,
    PLAYER_UNBANNED_NOTICE,
    PLAYER_NOT_BANNED_NOTICE,
    PLAYER_ALREADY_MEMBER_NOTICE,
    INVITE_NOTICE,
    INVITE_WRONG_FACTION_NOTICE,
    WRONG_FACTION_NOTICE,
    INVALID_NAME_NOTICE,
    NOT_MODERATED_NOTICE,
    PLAYER_INVITED_NOTICE,
    PLAYER_INVITE_BANNED_NOTICE,
    THROTTLED_NOTICE,
}

impl ChatNotify {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::JOINED_NOTICE => 0x0,
            Self::LEFT_NOTICE => 0x1,
            Self::YOU_JOINED_NOTICE => 0x2,
            Self::YOU_LEFT_NOTICE => 0x3,
            Self::WRONG_PASSWORD_NOTICE => 0x4,
            Self::NOT_MEMBER_NOTICE => 0x5,
            Self::NOT_MODERATOR_NOTICE => 0x6,
            Self::PASSWORD_CHANGED_NOTICE => 0x7,
            Self::OWNER_CHANGED_NOTICE => 0x8,
            Self::PLAYER_NOT_FOUND_NOTICE => 0x9,
            Self::NOT_OWNER_NOTICE => 0xa,
            Self::CHANNEL_OWNER_NOTICE => 0xb,
            Self::MODE_CHANGE_NOTICE => 0xc,
            Self::ANNOUNCEMENTS_ON_NOTICE => 0xd,
            Self::ANNOUNCEMENTS_OFF_NOTICE => 0xe,
            Self::MODERATION_ON_NOTICE => 0xf,
            Self::MODERATION_OFF_NOTICE => 0x10,
            Self::MUTED_NOTICE => 0x11,
            Self::PLAYER_KICKED_NOTICE => 0x12,
            Self::BANNED_NOTICE => 0x13,
            Self::PLAYER_BANNED_NOTICE => 0x14,
            Self::PLAYER_UNBANNED_NOTICE => 0x15,
            Self::PLAYER_NOT_BANNED_NOTICE => 0x16,
            Self::PLAYER_ALREADY_MEMBER_NOTICE => 0x17,
            Self::INVITE_NOTICE => 0x18,
            Self::INVITE_WRONG_FACTION_NOTICE => 0x19,
            Self::WRONG_FACTION_NOTICE => 0x1a,
            Self::INVALID_NAME_NOTICE => 0x1b,
            Self::NOT_MODERATED_NOTICE => 0x1c,
            Self::PLAYER_INVITED_NOTICE => 0x1d,
            Self::PLAYER_INVITE_BANNED_NOTICE => 0x1e,
            Self::THROTTLED_NOTICE => 0x1f,
        }
    }

}

impl Default for ChatNotify {
    fn default() -> Self {
        Self::JOINED_NOTICE
    }
}

impl std::fmt::Display for ChatNotify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JOINED_NOTICE => f.write_str("JOINED_NOTICE"),
            Self::LEFT_NOTICE => f.write_str("LEFT_NOTICE"),
            Self::YOU_JOINED_NOTICE => f.write_str("YOU_JOINED_NOTICE"),
            Self::YOU_LEFT_NOTICE => f.write_str("YOU_LEFT_NOTICE"),
            Self::WRONG_PASSWORD_NOTICE => f.write_str("WRONG_PASSWORD_NOTICE"),
            Self::NOT_MEMBER_NOTICE => f.write_str("NOT_MEMBER_NOTICE"),
            Self::NOT_MODERATOR_NOTICE => f.write_str("NOT_MODERATOR_NOTICE"),
            Self::PASSWORD_CHANGED_NOTICE => f.write_str("PASSWORD_CHANGED_NOTICE"),
            Self::OWNER_CHANGED_NOTICE => f.write_str("OWNER_CHANGED_NOTICE"),
            Self::PLAYER_NOT_FOUND_NOTICE => f.write_str("PLAYER_NOT_FOUND_NOTICE"),
            Self::NOT_OWNER_NOTICE => f.write_str("NOT_OWNER_NOTICE"),
            Self::CHANNEL_OWNER_NOTICE => f.write_str("CHANNEL_OWNER_NOTICE"),
            Self::MODE_CHANGE_NOTICE => f.write_str("MODE_CHANGE_NOTICE"),
            Self::ANNOUNCEMENTS_ON_NOTICE => f.write_str("ANNOUNCEMENTS_ON_NOTICE"),
            Self::ANNOUNCEMENTS_OFF_NOTICE => f.write_str("ANNOUNCEMENTS_OFF_NOTICE"),
            Self::MODERATION_ON_NOTICE => f.write_str("MODERATION_ON_NOTICE"),
            Self::MODERATION_OFF_NOTICE => f.write_str("MODERATION_OFF_NOTICE"),
            Self::MUTED_NOTICE => f.write_str("MUTED_NOTICE"),
            Self::PLAYER_KICKED_NOTICE => f.write_str("PLAYER_KICKED_NOTICE"),
            Self::BANNED_NOTICE => f.write_str("BANNED_NOTICE"),
            Self::PLAYER_BANNED_NOTICE => f.write_str("PLAYER_BANNED_NOTICE"),
            Self::PLAYER_UNBANNED_NOTICE => f.write_str("PLAYER_UNBANNED_NOTICE"),
            Self::PLAYER_NOT_BANNED_NOTICE => f.write_str("PLAYER_NOT_BANNED_NOTICE"),
            Self::PLAYER_ALREADY_MEMBER_NOTICE => f.write_str("PLAYER_ALREADY_MEMBER_NOTICE"),
            Self::INVITE_NOTICE => f.write_str("INVITE_NOTICE"),
            Self::INVITE_WRONG_FACTION_NOTICE => f.write_str("INVITE_WRONG_FACTION_NOTICE"),
            Self::WRONG_FACTION_NOTICE => f.write_str("WRONG_FACTION_NOTICE"),
            Self::INVALID_NAME_NOTICE => f.write_str("INVALID_NAME_NOTICE"),
            Self::NOT_MODERATED_NOTICE => f.write_str("NOT_MODERATED_NOTICE"),
            Self::PLAYER_INVITED_NOTICE => f.write_str("PLAYER_INVITED_NOTICE"),
            Self::PLAYER_INVITE_BANNED_NOTICE => f.write_str("PLAYER_INVITE_BANNED_NOTICE"),
            Self::THROTTLED_NOTICE => f.write_str("THROTTLED_NOTICE"),
        }
    }
}

impl TryFrom<u8> for ChatNotify {
    type Error = ChatNotifyError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::JOINED_NOTICE),
            1 => Ok(Self::LEFT_NOTICE),
            2 => Ok(Self::YOU_JOINED_NOTICE),
            3 => Ok(Self::YOU_LEFT_NOTICE),
            4 => Ok(Self::WRONG_PASSWORD_NOTICE),
            5 => Ok(Self::NOT_MEMBER_NOTICE),
            6 => Ok(Self::NOT_MODERATOR_NOTICE),
            7 => Ok(Self::PASSWORD_CHANGED_NOTICE),
            8 => Ok(Self::OWNER_CHANGED_NOTICE),
            9 => Ok(Self::PLAYER_NOT_FOUND_NOTICE),
            10 => Ok(Self::NOT_OWNER_NOTICE),
            11 => Ok(Self::CHANNEL_OWNER_NOTICE),
            12 => Ok(Self::MODE_CHANGE_NOTICE),
            13 => Ok(Self::ANNOUNCEMENTS_ON_NOTICE),
            14 => Ok(Self::ANNOUNCEMENTS_OFF_NOTICE),
            15 => Ok(Self::MODERATION_ON_NOTICE),
            16 => Ok(Self::MODERATION_OFF_NOTICE),
            17 => Ok(Self::MUTED_NOTICE),
            18 => Ok(Self::PLAYER_KICKED_NOTICE),
            19 => Ok(Self::BANNED_NOTICE),
            20 => Ok(Self::PLAYER_BANNED_NOTICE),
            21 => Ok(Self::PLAYER_UNBANNED_NOTICE),
            22 => Ok(Self::PLAYER_NOT_BANNED_NOTICE),
            23 => Ok(Self::PLAYER_ALREADY_MEMBER_NOTICE),
            24 => Ok(Self::INVITE_NOTICE),
            25 => Ok(Self::INVITE_WRONG_FACTION_NOTICE),
            26 => Ok(Self::WRONG_FACTION_NOTICE),
            27 => Ok(Self::INVALID_NAME_NOTICE),
            28 => Ok(Self::NOT_MODERATED_NOTICE),
            29 => Ok(Self::PLAYER_INVITED_NOTICE),
            30 => Ok(Self::PLAYER_INVITE_BANNED_NOTICE),
            31 => Ok(Self::THROTTLED_NOTICE),
            _ => Err(ChatNotifyError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct ChatNotifyError {
    value: u8,
}

impl ChatNotifyError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for ChatNotifyError {}
impl std::fmt::Display for ChatNotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ChatNotify': '{}'", self.value))
    }
}

