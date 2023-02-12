/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L1):
/// ```text
/// enum LfgType : u8 {
///     NONE = 0;
///     DUNGEON = 1;
///     RAID = 2;
///     QUEST = 3;
///     ZONE = 4;
///     HEROIC_DUNGEON = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LfgType {
    None,
    Dungeon,
    Raid,
    Quest,
    Zone,
    HeroicDungeon,
}

impl LfgType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Dungeon => 0x1,
            Self::Raid => 0x2,
            Self::Quest => 0x3,
            Self::Zone => 0x4,
            Self::HeroicDungeon => 0x5,
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
            Self::Quest => f.write_str("Quest"),
            Self::Zone => f.write_str("Zone"),
            Self::HeroicDungeon => f.write_str("HeroicDungeon"),
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
            3 => Ok(Self::Quest),
            4 => Ok(Self::Zone),
            5 => Ok(Self::HeroicDungeon),
            v => Err(crate::errors::EnumError::new("LfgType", v as u64),)
        }
    }
}

