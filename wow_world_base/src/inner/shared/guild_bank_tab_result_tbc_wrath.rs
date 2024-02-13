/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm#L8):
/// ```text
/// enum GuildBankTabResult : u8 {
///     NOT_PRESENT = 0;
///     PRESENT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GuildBankTabResult {
    NotPresent,
    Present,
}

impl GuildBankTabResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0x0,
            Self::Present => 0x1,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
            Self::NotPresent,
            Self::Present,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::NotPresent),
            1 => Ok(Self::Present),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl GuildBankTabResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NotPresent => "NOT_PRESENT",
            Self::Present => "PRESENT",
        }
    }

}

const NAME: &str = "GuildBankTabResult";

impl Default for GuildBankTabResult {
    fn default() -> Self {
        Self::NotPresent
    }
}

impl std::fmt::Display for GuildBankTabResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPresent => f.write_str("NotPresent"),
            Self::Present => f.write_str("Present"),
        }
    }
}

impl TryFrom<u8> for GuildBankTabResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for GuildBankTabResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for GuildBankTabResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for GuildBankTabResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for GuildBankTabResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for GuildBankTabResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for GuildBankTabResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for GuildBankTabResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for GuildBankTabResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

