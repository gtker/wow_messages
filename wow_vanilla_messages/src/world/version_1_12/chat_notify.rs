use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm#L3):
/// ```text
/// enum ChatNotify : u8 {
///     JOINED_NOTICE = 0x00;
///     LEFT_NOTICE = 0x01;
///     YOU_JOINED_NOTICE = 0x02;
///     YOU_LEFT_NOTICE = 0x03;
///     WRONG_PASSWORD_NOTICE = 0x04;
///     NOT_MEMBER_NOTICE = 0x05;
///     NOT_MODERATOR_NOTICE = 0x06;
///     PASSWORD_CHANGED_NOTICE = 0x07;
///     OWNER_CHANGED_NOTICE = 0x08;
///     PLAYER_NOT_FOUND_NOTICE = 0x09;
///     NOT_OWNER_NOTICE = 0x0A;
///     CHANNEL_OWNER_NOTICE = 0x0B;
///     MODE_CHANGE_NOTICE = 0x0C;
///     ANNOUNCEMENTS_ON_NOTICE = 0x0D;
///     ANNOUNCEMENTS_OFF_NOTICE = 0x0E;
///     MODERATION_ON_NOTICE = 0x0F;
///     MODERATION_OFF_NOTICE = 0x10;
///     MUTED_NOTICE = 0x11;
///     PLAYER_KICKED_NOTICE = 0x12;
///     BANNED_NOTICE = 0x13;
///     PLAYER_BANNED_NOTICE = 0x14;
///     PLAYER_UNBANNED_NOTICE = 0x15;
///     PLAYER_NOT_BANNED_NOTICE = 0x16;
///     PLAYER_ALREADY_MEMBER_NOTICE = 0x17;
///     INVITE_NOTICE = 0x18;
///     INVITE_WRONG_FACTION_NOTICE = 0x19;
///     WRONG_FACTION_NOTICE = 0x1A;
///     INVALID_NAME_NOTICE = 0x1B;
///     NOT_MODERATED_NOTICE = 0x1C;
///     PLAYER_INVITED_NOTICE = 0x1D;
///     PLAYER_INVITE_BANNED_NOTICE = 0x1E;
///     THROTTLED_NOTICE = 0x1F;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ChatNotify {
    /// # Comment
    ///
    /// %s joined channel.
    JOINED_NOTICE,
    /// # Comment
    ///
    /// %s left channel.
    LEFT_NOTICE,
    /// # Comment
    ///
    /// Joined Channel: [%s] -- You joined
    YOU_JOINED_NOTICE,
    /// # Comment
    ///
    /// Left Channel: [%s] -- You left
    YOU_LEFT_NOTICE,
    /// # Comment
    ///
    /// Wrong password for %s.
    WRONG_PASSWORD_NOTICE,
    /// # Comment
    ///
    /// Not on channel %s.
    NOT_MEMBER_NOTICE,
    /// # Comment
    ///
    /// Not a moderator of %s.
    NOT_MODERATOR_NOTICE,
    /// # Comment
    ///
    /// [%s] Password changed by %s.
    PASSWORD_CHANGED_NOTICE,
    /// # Comment
    ///
    /// [%s] Owner changed to %s.
    OWNER_CHANGED_NOTICE,
    /// # Comment
    ///
    /// [%s] Player %s was not found.
    PLAYER_NOT_FOUND_NOTICE,
    /// # Comment
    ///
    /// [%s] You are not the channel owner.
    NOT_OWNER_NOTICE,
    /// # Comment
    ///
    /// [%s] Channel owner is %s.
    CHANNEL_OWNER_NOTICE,
    MODE_CHANGE_NOTICE,
    /// # Comment
    ///
    /// [%s] Channel announcements enabled by %s.
    ANNOUNCEMENTS_ON_NOTICE,
    /// # Comment
    ///
    /// [%s] Channel announcements disabled by %s.
    ANNOUNCEMENTS_OFF_NOTICE,
    /// # Comment
    ///
    /// [%s] Channel moderation enabled by %s.
    MODERATION_ON_NOTICE,
    /// # Comment
    ///
    /// [%s] Channel moderation disabled by %s.
    MODERATION_OFF_NOTICE,
    /// # Comment
    ///
    /// [%s] You do not have permission to speak.
    MUTED_NOTICE,
    /// # Comment
    ///
    /// [%s] Player %s kicked by %s.
    PLAYER_KICKED_NOTICE,
    /// # Comment
    ///
    /// [%s] You are banned from that channel.
    BANNED_NOTICE,
    /// # Comment
    ///
    /// [%s] Player %s banned by %s.
    PLAYER_BANNED_NOTICE,
    /// # Comment
    ///
    /// [%s] Player %s unbanned by %s.
    PLAYER_UNBANNED_NOTICE,
    /// # Comment
    ///
    /// [%s] Player %s is not banned.
    PLAYER_NOT_BANNED_NOTICE,
    /// # Comment
    ///
    /// [%s] Player %s is already on the channel.
    PLAYER_ALREADY_MEMBER_NOTICE,
    /// # Comment
    ///
    /// %2$s has invited you to join the channel '%1$s'.
    INVITE_NOTICE,
    /// # Comment
    ///
    /// Target is in the wrong alliance for %s.
    INVITE_WRONG_FACTION_NOTICE,
    /// # Comment
    ///
    /// Wrong alliance for %s.
    WRONG_FACTION_NOTICE,
    /// # Comment
    ///
    /// Invalid channel name
    INVALID_NAME_NOTICE,
    /// # Comment
    ///
    /// %s is not moderated
    NOT_MODERATED_NOTICE,
    /// # Comment
    ///
    /// [%s] You invited %s to join the channel
    PLAYER_INVITED_NOTICE,
    /// # Comment
    ///
    /// [%s] %s has been banned.
    PLAYER_INVITE_BANNED_NOTICE,
    /// # Comment
    ///
    /// [%s] The number of messages that can be sent to this channel is limited, please wait to send another message.
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
    type Error = crate::errors::EnumError;
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
            v => Err(crate::errors::EnumError::new("ChatNotify", v as u32),)
        }
    }
}

