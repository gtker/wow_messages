/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_messagechat.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_messagechat.wowm#L1):
/// ```text
/// enum PlayerChatTag : u8 {
///     NONE = 0;
///     AFK = 1;
///     DND = 2;
///     GM = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PlayerChatTag {
    None,
    Afk,
    Dnd,
    Gm,
}

impl PlayerChatTag {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Afk => 0x1,
            Self::Dnd => 0x2,
            Self::Gm => 0x3,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::None,
            Self::Afk,
            Self::Dnd,
            Self::Gm,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Afk),
            2 => Ok(Self::Dnd),
            3 => Ok(Self::Gm),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl PlayerChatTag {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Afk => "AFK",
            Self::Dnd => "DND",
            Self::Gm => "GM",
        }
    }

}

const NAME: &str = "PlayerChatTag";

impl Default for PlayerChatTag {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for PlayerChatTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Afk => f.write_str("Afk"),
            Self::Dnd => f.write_str("Dnd"),
            Self::Gm => f.write_str("Gm"),
        }
    }
}

impl TryFrom<u8> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

