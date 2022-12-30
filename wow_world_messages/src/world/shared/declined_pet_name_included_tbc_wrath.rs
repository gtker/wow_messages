use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm#L24):
/// ```text
/// enum DeclinedPetNameIncluded : u8 {
///     NOT_INCLUDED = 0;
///     INCLUDED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum DeclinedPetNameIncluded {
    NotIncluded,
    Included,
}

impl DeclinedPetNameIncluded {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotIncluded => 0x0,
            Self::Included => 0x1,
        }
    }

}

impl Default for DeclinedPetNameIncluded {
    fn default() -> Self {
        Self::NotIncluded
    }
}

impl std::fmt::Display for DeclinedPetNameIncluded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotIncluded => f.write_str("NotIncluded"),
            Self::Included => f.write_str("Included"),
        }
    }
}

impl TryFrom<u8> for DeclinedPetNameIncluded {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotIncluded),
            1 => Ok(Self::Included),
            v => Err(crate::errors::EnumError::new("DeclinedPetNameIncluded", v as u64),)
        }
    }
}

