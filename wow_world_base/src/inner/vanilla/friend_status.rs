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

    pub const fn variants() -> [Self; 5] {
        [
            Self::Offline,
            Self::Online,
            Self::Afk,
            Self::Unknown3,
            Self::Dnd,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Offline),
            1 => Ok(Self::Online),
            2 => Ok(Self::Afk),
            3 => Ok(Self::Unknown3),
            4 => Ok(Self::Dnd),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
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

const NAME: &str = "FriendStatus";

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
        Self::from_int(value)
    }
}

impl TryFrom<u16> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for FriendStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

