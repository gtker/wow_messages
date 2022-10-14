use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm#L15):
/// ```text
/// enum HasDeclinedNames : u8 {
///     NO = 0;
///     YES = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum HasDeclinedNames {
    No,
    Yes,
}

impl HasDeclinedNames {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::No => 0x0,
            Self::Yes => 0x1,
        }
    }

}

impl Default for HasDeclinedNames {
    fn default() -> Self {
        Self::No
    }
}

impl std::fmt::Display for HasDeclinedNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::No => f.write_str("No"),
            Self::Yes => f.write_str("Yes"),
        }
    }
}

impl TryFrom<u8> for HasDeclinedNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::No),
            1 => Ok(Self::Yes),
            v => Err(crate::errors::EnumError::new("HasDeclinedNames", v as u32),)
        }
    }
}

