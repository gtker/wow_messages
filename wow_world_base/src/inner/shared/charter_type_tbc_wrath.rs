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
            v => Err(crate::errors::EnumError::new("CharterType", v.into()),)
        }
    }
}

