use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:76`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L76):
/// ```text
/// enum RandomBg : u8 {
///     NOT_RANDOM = 0;
///     RANDOM = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum RandomBg {
    NotRandom,
    Random,
}

impl RandomBg {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotRandom => 0x0,
            Self::Random => 0x1,
        }
    }

}

impl Default for RandomBg {
    fn default() -> Self {
        Self::NotRandom
    }
}

impl std::fmt::Display for RandomBg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotRandom => f.write_str("NotRandom"),
            Self::Random => f.write_str("Random"),
        }
    }
}

impl TryFrom<u8> for RandomBg {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotRandom),
            1 => Ok(Self::Random),
            v => Err(crate::errors::EnumError::new("RandomBg", v as u32),)
        }
    }
}

