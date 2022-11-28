use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm#L23):
/// ```text
/// enum DispelMethod : u8 {
///     DISPELLED = 0;
///     CLEANSED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum DispelMethod {
    Dispelled,
    Cleansed,
}

impl DispelMethod {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Dispelled => 0x0,
            Self::Cleansed => 0x1,
        }
    }

}

impl Default for DispelMethod {
    fn default() -> Self {
        Self::Dispelled
    }
}

impl std::fmt::Display for DispelMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dispelled => f.write_str("Dispelled"),
            Self::Cleansed => f.write_str("Cleansed"),
        }
    }
}

impl TryFrom<u8> for DispelMethod {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dispelled),
            1 => Ok(Self::Cleansed),
            v => Err(crate::errors::EnumError::new("DispelMethod", v as u32),)
        }
    }
}

