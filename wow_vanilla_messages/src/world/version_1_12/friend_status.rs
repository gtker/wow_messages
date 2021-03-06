use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_list.wowm#L3):
/// ```text
/// enum FriendStatus : u8 {
///     OFFLINE = 0;
///     ONLINE = 1;
///     AFK = 2;
///     UNKNOWN3 = 3;
///     DND = 4;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum FriendStatus {
    OFFLINE,
    ONLINE,
    AFK,
    UNKNOWN3,
    DND,
}

impl FriendStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::OFFLINE => 0x0,
            Self::ONLINE => 0x1,
            Self::AFK => 0x2,
            Self::UNKNOWN3 => 0x3,
            Self::DND => 0x4,
        }
    }

}

impl Default for FriendStatus {
    fn default() -> Self {
        Self::OFFLINE
    }
}

impl std::fmt::Display for FriendStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OFFLINE => f.write_str("OFFLINE"),
            Self::ONLINE => f.write_str("ONLINE"),
            Self::AFK => f.write_str("AFK"),
            Self::UNKNOWN3 => f.write_str("UNKNOWN3"),
            Self::DND => f.write_str("DND"),
        }
    }
}

impl TryFrom<u8> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OFFLINE),
            1 => Ok(Self::ONLINE),
            2 => Ok(Self::AFK),
            3 => Ok(Self::UNKNOWN3),
            4 => Ok(Self::DND),
            v => Err(crate::errors::EnumError::new("FriendStatus", v as u32),)
        }
    }
}

