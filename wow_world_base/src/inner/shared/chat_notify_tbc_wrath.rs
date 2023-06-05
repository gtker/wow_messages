/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm:107`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm#L107):
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
///     NOT_IN_AREA_NOTICE = 0x20;
///     NOT_IN_LFG_NOTICE = 0x21;
///     VOICE_ON_NOTICE = 0x22;
///     VOICE_OFF_NOTICE = 0x23;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ChatNotify {
    /// %s joined channel.
    ///
    JoinedNotice,
    /// %s left channel.
    ///
    LeftNotice,
    /// Joined Channel: %s -- You joined
    ///
    YouJoinedNotice,
    /// Left Channel: %s -- You left
    ///
    YouLeftNotice,
    /// Wrong password for %s.
    ///
    WrongPasswordNotice,
    /// Not on channel %s.
    ///
    NotMemberNotice,
    /// Not a moderator of %s.
    ///
    NotModeratorNotice,
    /// %s Password changed by %s.
    ///
    PasswordChangedNotice,
    /// %s Owner changed to %s.
    ///
    OwnerChangedNotice,
    /// %s Player %s was not found.
    ///
    PlayerNotFoundNotice,
    /// %s You are not the channel owner.
    ///
    NotOwnerNotice,
    /// %s Channel owner is %s.
    ///
    ChannelOwnerNotice,
    ModeChangeNotice,
    /// %s Channel announcements enabled by %s.
    ///
    AnnouncementsOnNotice,
    /// %s Channel announcements disabled by %s.
    ///
    AnnouncementsOffNotice,
    /// %s Channel moderation enabled by %s.
    ///
    ModerationOnNotice,
    /// %s Channel moderation disabled by %s.
    ///
    ModerationOffNotice,
    /// %s You do not have permission to speak.
    ///
    MutedNotice,
    /// %s Player %s kicked by %s.
    ///
    PlayerKickedNotice,
    /// %s You are banned from that channel.
    ///
    BannedNotice,
    /// %s Player %s banned by %s.
    ///
    PlayerBannedNotice,
    /// %s Player %s unbanned by %s.
    ///
    PlayerUnbannedNotice,
    /// %s Player %s is not banned.
    ///
    PlayerNotBannedNotice,
    /// %s Player %s is already on the channel.
    ///
    PlayerAlreadyMemberNotice,
    /// %2$s has invited you to join the channel '%1$s'.
    ///
    InviteNotice,
    /// Target is in the wrong alliance for %s.
    ///
    InviteWrongFactionNotice,
    /// Wrong alliance for %s.
    ///
    WrongFactionNotice,
    /// Invalid channel name
    ///
    InvalidNameNotice,
    /// %s is not moderated
    ///
    NotModeratedNotice,
    /// %s You invited %s to join the channel
    ///
    PlayerInvitedNotice,
    /// %s %s has been banned.
    ///
    PlayerInviteBannedNotice,
    /// %s The number of messages that can be sent to this channel is limited, please wait to send another message.
    ///
    ThrottledNotice,
    /// %s You are not in the correct area for this channel. -- The user is trying to send a chat to a zone specific channel, and they're not physically in that zone.
    ///
    NotInAreaNotice,
    /// %s You must be queued in looking for group before joining this channel. -- The user must be in the looking for group system to join LFG chat channels.
    ///
    NotInLfgNotice,
    /// %s Channel voice enabled by %s.
    ///
    VoiceOnNotice,
    /// %s Channel voice disabled by %s.
    ///
    VoiceOffNotice,
}

impl ChatNotify {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::JoinedNotice => 0x0,
            Self::LeftNotice => 0x1,
            Self::YouJoinedNotice => 0x2,
            Self::YouLeftNotice => 0x3,
            Self::WrongPasswordNotice => 0x4,
            Self::NotMemberNotice => 0x5,
            Self::NotModeratorNotice => 0x6,
            Self::PasswordChangedNotice => 0x7,
            Self::OwnerChangedNotice => 0x8,
            Self::PlayerNotFoundNotice => 0x9,
            Self::NotOwnerNotice => 0xa,
            Self::ChannelOwnerNotice => 0xb,
            Self::ModeChangeNotice => 0xc,
            Self::AnnouncementsOnNotice => 0xd,
            Self::AnnouncementsOffNotice => 0xe,
            Self::ModerationOnNotice => 0xf,
            Self::ModerationOffNotice => 0x10,
            Self::MutedNotice => 0x11,
            Self::PlayerKickedNotice => 0x12,
            Self::BannedNotice => 0x13,
            Self::PlayerBannedNotice => 0x14,
            Self::PlayerUnbannedNotice => 0x15,
            Self::PlayerNotBannedNotice => 0x16,
            Self::PlayerAlreadyMemberNotice => 0x17,
            Self::InviteNotice => 0x18,
            Self::InviteWrongFactionNotice => 0x19,
            Self::WrongFactionNotice => 0x1a,
            Self::InvalidNameNotice => 0x1b,
            Self::NotModeratedNotice => 0x1c,
            Self::PlayerInvitedNotice => 0x1d,
            Self::PlayerInviteBannedNotice => 0x1e,
            Self::ThrottledNotice => 0x1f,
            Self::NotInAreaNotice => 0x20,
            Self::NotInLfgNotice => 0x21,
            Self::VoiceOnNotice => 0x22,
            Self::VoiceOffNotice => 0x23,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl ChatNotify {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::JoinedNotice => "JOINED_NOTICE",
            Self::LeftNotice => "LEFT_NOTICE",
            Self::YouJoinedNotice => "YOU_JOINED_NOTICE",
            Self::YouLeftNotice => "YOU_LEFT_NOTICE",
            Self::WrongPasswordNotice => "WRONG_PASSWORD_NOTICE",
            Self::NotMemberNotice => "NOT_MEMBER_NOTICE",
            Self::NotModeratorNotice => "NOT_MODERATOR_NOTICE",
            Self::PasswordChangedNotice => "PASSWORD_CHANGED_NOTICE",
            Self::OwnerChangedNotice => "OWNER_CHANGED_NOTICE",
            Self::PlayerNotFoundNotice => "PLAYER_NOT_FOUND_NOTICE",
            Self::NotOwnerNotice => "NOT_OWNER_NOTICE",
            Self::ChannelOwnerNotice => "CHANNEL_OWNER_NOTICE",
            Self::ModeChangeNotice => "MODE_CHANGE_NOTICE",
            Self::AnnouncementsOnNotice => "ANNOUNCEMENTS_ON_NOTICE",
            Self::AnnouncementsOffNotice => "ANNOUNCEMENTS_OFF_NOTICE",
            Self::ModerationOnNotice => "MODERATION_ON_NOTICE",
            Self::ModerationOffNotice => "MODERATION_OFF_NOTICE",
            Self::MutedNotice => "MUTED_NOTICE",
            Self::PlayerKickedNotice => "PLAYER_KICKED_NOTICE",
            Self::BannedNotice => "BANNED_NOTICE",
            Self::PlayerBannedNotice => "PLAYER_BANNED_NOTICE",
            Self::PlayerUnbannedNotice => "PLAYER_UNBANNED_NOTICE",
            Self::PlayerNotBannedNotice => "PLAYER_NOT_BANNED_NOTICE",
            Self::PlayerAlreadyMemberNotice => "PLAYER_ALREADY_MEMBER_NOTICE",
            Self::InviteNotice => "INVITE_NOTICE",
            Self::InviteWrongFactionNotice => "INVITE_WRONG_FACTION_NOTICE",
            Self::WrongFactionNotice => "WRONG_FACTION_NOTICE",
            Self::InvalidNameNotice => "INVALID_NAME_NOTICE",
            Self::NotModeratedNotice => "NOT_MODERATED_NOTICE",
            Self::PlayerInvitedNotice => "PLAYER_INVITED_NOTICE",
            Self::PlayerInviteBannedNotice => "PLAYER_INVITE_BANNED_NOTICE",
            Self::ThrottledNotice => "THROTTLED_NOTICE",
            Self::NotInAreaNotice => "NOT_IN_AREA_NOTICE",
            Self::NotInLfgNotice => "NOT_IN_LFG_NOTICE",
            Self::VoiceOnNotice => "VOICE_ON_NOTICE",
            Self::VoiceOffNotice => "VOICE_OFF_NOTICE",
        }
    }

}

impl Default for ChatNotify {
    fn default() -> Self {
        Self::JoinedNotice
    }
}

impl std::fmt::Display for ChatNotify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JoinedNotice => f.write_str("JoinedNotice"),
            Self::LeftNotice => f.write_str("LeftNotice"),
            Self::YouJoinedNotice => f.write_str("YouJoinedNotice"),
            Self::YouLeftNotice => f.write_str("YouLeftNotice"),
            Self::WrongPasswordNotice => f.write_str("WrongPasswordNotice"),
            Self::NotMemberNotice => f.write_str("NotMemberNotice"),
            Self::NotModeratorNotice => f.write_str("NotModeratorNotice"),
            Self::PasswordChangedNotice => f.write_str("PasswordChangedNotice"),
            Self::OwnerChangedNotice => f.write_str("OwnerChangedNotice"),
            Self::PlayerNotFoundNotice => f.write_str("PlayerNotFoundNotice"),
            Self::NotOwnerNotice => f.write_str("NotOwnerNotice"),
            Self::ChannelOwnerNotice => f.write_str("ChannelOwnerNotice"),
            Self::ModeChangeNotice => f.write_str("ModeChangeNotice"),
            Self::AnnouncementsOnNotice => f.write_str("AnnouncementsOnNotice"),
            Self::AnnouncementsOffNotice => f.write_str("AnnouncementsOffNotice"),
            Self::ModerationOnNotice => f.write_str("ModerationOnNotice"),
            Self::ModerationOffNotice => f.write_str("ModerationOffNotice"),
            Self::MutedNotice => f.write_str("MutedNotice"),
            Self::PlayerKickedNotice => f.write_str("PlayerKickedNotice"),
            Self::BannedNotice => f.write_str("BannedNotice"),
            Self::PlayerBannedNotice => f.write_str("PlayerBannedNotice"),
            Self::PlayerUnbannedNotice => f.write_str("PlayerUnbannedNotice"),
            Self::PlayerNotBannedNotice => f.write_str("PlayerNotBannedNotice"),
            Self::PlayerAlreadyMemberNotice => f.write_str("PlayerAlreadyMemberNotice"),
            Self::InviteNotice => f.write_str("InviteNotice"),
            Self::InviteWrongFactionNotice => f.write_str("InviteWrongFactionNotice"),
            Self::WrongFactionNotice => f.write_str("WrongFactionNotice"),
            Self::InvalidNameNotice => f.write_str("InvalidNameNotice"),
            Self::NotModeratedNotice => f.write_str("NotModeratedNotice"),
            Self::PlayerInvitedNotice => f.write_str("PlayerInvitedNotice"),
            Self::PlayerInviteBannedNotice => f.write_str("PlayerInviteBannedNotice"),
            Self::ThrottledNotice => f.write_str("ThrottledNotice"),
            Self::NotInAreaNotice => f.write_str("NotInAreaNotice"),
            Self::NotInLfgNotice => f.write_str("NotInLfgNotice"),
            Self::VoiceOnNotice => f.write_str("VoiceOnNotice"),
            Self::VoiceOffNotice => f.write_str("VoiceOffNotice"),
        }
    }
}

impl TryFrom<u8> for ChatNotify {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::JoinedNotice),
            1 => Ok(Self::LeftNotice),
            2 => Ok(Self::YouJoinedNotice),
            3 => Ok(Self::YouLeftNotice),
            4 => Ok(Self::WrongPasswordNotice),
            5 => Ok(Self::NotMemberNotice),
            6 => Ok(Self::NotModeratorNotice),
            7 => Ok(Self::PasswordChangedNotice),
            8 => Ok(Self::OwnerChangedNotice),
            9 => Ok(Self::PlayerNotFoundNotice),
            10 => Ok(Self::NotOwnerNotice),
            11 => Ok(Self::ChannelOwnerNotice),
            12 => Ok(Self::ModeChangeNotice),
            13 => Ok(Self::AnnouncementsOnNotice),
            14 => Ok(Self::AnnouncementsOffNotice),
            15 => Ok(Self::ModerationOnNotice),
            16 => Ok(Self::ModerationOffNotice),
            17 => Ok(Self::MutedNotice),
            18 => Ok(Self::PlayerKickedNotice),
            19 => Ok(Self::BannedNotice),
            20 => Ok(Self::PlayerBannedNotice),
            21 => Ok(Self::PlayerUnbannedNotice),
            22 => Ok(Self::PlayerNotBannedNotice),
            23 => Ok(Self::PlayerAlreadyMemberNotice),
            24 => Ok(Self::InviteNotice),
            25 => Ok(Self::InviteWrongFactionNotice),
            26 => Ok(Self::WrongFactionNotice),
            27 => Ok(Self::InvalidNameNotice),
            28 => Ok(Self::NotModeratedNotice),
            29 => Ok(Self::PlayerInvitedNotice),
            30 => Ok(Self::PlayerInviteBannedNotice),
            31 => Ok(Self::ThrottledNotice),
            32 => Ok(Self::NotInAreaNotice),
            33 => Ok(Self::NotInLfgNotice),
            34 => Ok(Self::VoiceOnNotice),
            35 => Ok(Self::VoiceOffNotice),
            v => Err(crate::errors::EnumError::new("ChatNotify", v as u64),)
        }
    }
}

