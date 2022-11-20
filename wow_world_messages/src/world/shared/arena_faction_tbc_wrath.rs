use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:89`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L89):
/// ```text
/// enum ArenaFaction : u8 {
///     HORDE = 0;
///     ALLIANCE = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ArenaFaction {
    Horde,
    Alliance,
}

impl ArenaFaction {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Horde => 0x0,
            Self::Alliance => 0x1,
        }
    }

}

impl Default for ArenaFaction {
    fn default() -> Self {
        Self::Horde
    }
}

impl std::fmt::Display for ArenaFaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horde => f.write_str("Horde"),
            Self::Alliance => f.write_str("Alliance"),
        }
    }
}

impl TryFrom<u8> for ArenaFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Horde),
            1 => Ok(Self::Alliance),
            v => Err(crate::errors::EnumError::new("ArenaFaction", v as u32),)
        }
    }
}

