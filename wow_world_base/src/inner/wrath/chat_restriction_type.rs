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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ChatRestrictionType {
    ChatRestricted,
    ChatThrottled,
    UserSquelched,
    YellRestricted,
}

impl ChatRestrictionType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::ChatRestricted => 0x0,
            Self::ChatThrottled => 0x1,
            Self::UserSquelched => 0x2,
            Self::YellRestricted => 0x3,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::ChatRestricted,
            Self::ChatThrottled,
            Self::UserSquelched,
            Self::YellRestricted,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::ChatRestricted),
            1 => Ok(Self::ChatThrottled),
            2 => Ok(Self::UserSquelched),
            3 => Ok(Self::YellRestricted),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ChatRestrictionType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::ChatRestricted => "CHAT_RESTRICTED",
            Self::ChatThrottled => "CHAT_THROTTLED",
            Self::UserSquelched => "USER_SQUELCHED",
            Self::YellRestricted => "YELL_RESTRICTED",
        }
    }

}

const NAME: &str = "ChatRestrictionType";

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
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for ChatRestrictionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ChatRestrictionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ChatRestrictionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ChatRestrictionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for ChatRestrictionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ChatRestrictionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ChatRestrictionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ChatRestrictionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

