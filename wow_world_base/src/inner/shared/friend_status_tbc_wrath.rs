/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_contact_list.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_contact_list.wowm#L11):
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FriendStatus {
    Offline,
    Online,
    Afk,
    Unknown3,
    Dnd,
}

impl FriendStatus {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Offline => 0x0,
            Self::Online => 0x1,
            Self::Afk => 0x2,
            Self::Unknown3 => 0x3,
            Self::Dnd => 0x4,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl FriendStatus {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Offline => "OFFLINE",
            Self::Online => "ONLINE",
            Self::Afk => "AFK",
            Self::Unknown3 => "UNKNOWN3",
            Self::Dnd => "DND",
        }
    }

}

impl Default for FriendStatus {
    fn default() -> Self {
        Self::Offline
    }
}

impl std::fmt::Display for FriendStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Offline => f.write_str("Offline"),
            Self::Online => f.write_str("Online"),
            Self::Afk => f.write_str("Afk"),
            Self::Unknown3 => f.write_str("Unknown3"),
            Self::Dnd => f.write_str("Dnd"),
        }
    }
}

impl TryFrom<u8> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Offline),
            1 => Ok(Self::Online),
            2 => Ok(Self::Afk),
            3 => Ok(Self::Unknown3),
            4 => Ok(Self::Dnd),
            v => Err(crate::errors::EnumError::new("FriendStatus", v.into()),)
        }
    }
}

