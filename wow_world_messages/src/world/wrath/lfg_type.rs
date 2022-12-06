use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_update_lfg_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_update_lfg_list.wowm#L1):
/// ```text
/// enum LfgType : u8 {
///     NONE = 0;
///     DUNGEON = 1;
///     RAID = 2;
///     ZONE = 4;
///     HEROIC = 5;
///     RANDOM = 6;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LfgType {
    None,
    Dungeon,
    Raid,
    Zone,
    Heroic,
    Random,
}

impl LfgType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Dungeon => 0x1,
            Self::Raid => 0x2,
            Self::Zone => 0x4,
            Self::Heroic => 0x5,
            Self::Random => 0x6,
        }
    }

}

impl Default for LfgType {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for LfgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Dungeon => f.write_str("Dungeon"),
            Self::Raid => f.write_str("Raid"),
            Self::Zone => f.write_str("Zone"),
            Self::Heroic => f.write_str("Heroic"),
            Self::Random => f.write_str("Random"),
        }
    }
}

impl TryFrom<u8> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Dungeon),
            2 => Ok(Self::Raid),
            4 => Ok(Self::Zone),
            5 => Ok(Self::Heroic),
            6 => Ok(Self::Random),
            v => Err(crate::errors::EnumError::new("LfgType", v as u32),)
        }
    }
}

