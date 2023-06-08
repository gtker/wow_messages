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
        match value {
            0 => Ok(Self::NotPresent),
            1 => Ok(Self::Present),
            v => Err(crate::errors::EnumError::new("GuildBankTabResult", v as u64),)
        }
    }
}

