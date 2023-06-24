/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_petition_query_response.wowm:53`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_petition_query_response.wowm#L53):
/// ```text
/// enum CharterType : u8 {
///     GUILD = 0;
///     ARENA = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum CharterType {
    Guild,
    Arena,
}

impl CharterType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Guild => 0x0,
            Self::Arena => 0x1,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
            Self::Guild,
            Self::Arena,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl CharterType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Guild => "GUILD",
            Self::Arena => "ARENA",
        }
    }

}

const NAME: &str = "CharterType";

impl Default for CharterType {
    fn default() -> Self {
        Self::Guild
    }
}

impl std::fmt::Display for CharterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Guild => f.write_str("Guild"),
            Self::Arena => f.write_str("Arena"),
        }
    }
}

impl TryFrom<u8> for CharterType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Guild),
            1 => Ok(Self::Arena),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for CharterType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for CharterType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for CharterType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for CharterType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for CharterType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for CharterType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for CharterType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for CharterType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

