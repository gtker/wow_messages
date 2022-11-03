use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:38`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L38):
/// ```text
/// enum DungeonDifficulty : u8 {
///     NORMAL = 0;
///     HEROIC = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum DungeonDifficulty {
    Normal,
    Heroic,
}

impl DungeonDifficulty {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0x0,
            Self::Heroic => 0x1,
        }
    }

}

impl Default for DungeonDifficulty {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for DungeonDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Heroic => f.write_str("Heroic"),
        }
    }
}

impl TryFrom<u8> for DungeonDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Heroic),
            v => Err(crate::errors::EnumError::new("DungeonDifficulty", v as u32),)
        }
    }
}

