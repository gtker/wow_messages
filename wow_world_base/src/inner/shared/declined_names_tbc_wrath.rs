/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm#L15):
/// ```text
/// enum DeclinedNames : u8 {
///     NO = 0;
///     YES = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum DeclinedNames {
    No,
    Yes,
}

impl DeclinedNames {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::No => 0x0,
            Self::Yes => 0x1,
        }
    }

}

impl Default for DeclinedNames {
    fn default() -> Self {
        Self::No
    }
}

impl std::fmt::Display for DeclinedNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::No => f.write_str("No"),
            Self::Yes => f.write_str("Yes"),
        }
    }
}

impl TryFrom<u8> for DeclinedNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::No),
            1 => Ok(Self::Yes),
            v => Err(crate::errors::EnumError::new("DeclinedNames", v as u64),)
        }
    }
}

