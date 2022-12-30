use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_mode.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_mode.wowm#L3):
/// ```text
/// enum PetEnabled : u8 {
///     ENABLED = 0x8;
///     DISABLED = 0x0;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetEnabled {
    Enabled,
    Disabled,
}

impl PetEnabled {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Enabled => 0x8,
            Self::Disabled => 0x0,
        }
    }

}

impl Default for PetEnabled {
    fn default() -> Self {
        Self::Enabled
    }
}

impl std::fmt::Display for PetEnabled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enabled => f.write_str("Enabled"),
            Self::Disabled => f.write_str("Disabled"),
        }
    }
}

impl TryFrom<u8> for PetEnabled {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            8 => Ok(Self::Enabled),
            0 => Ok(Self::Disabled),
            v => Err(crate::errors::EnumError::new("PetEnabled", v as u64),)
        }
    }
}

