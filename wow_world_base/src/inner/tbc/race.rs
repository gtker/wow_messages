/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/race.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/race.wowm#L30):
/// ```text
/// enum Race : u8 {
///     HUMAN = 1;
///     ORC = 2;
///     DWARF = 3;
///     NIGHT_ELF = 4;
///     UNDEAD = 5;
///     TAUREN = 6;
///     GNOME = 7;
///     TROLL = 8;
///     GOBLIN = 9;
///     BLOOD_ELF = 10;
///     DRAENEI = 11;
///     FEL_ORC = 12;
///     NAGA = 13;
///     BROKEN = 14;
///     SKELETON = 15;
///     VRYKUL = 16;
///     TUSKARR = 17;
///     FOREST_TROLL = 18;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Race {
    Human,
    Orc,
    Dwarf,
    NightElf,
    Undead,
    Tauren,
    Gnome,
    Troll,
    Goblin,
    BloodElf,
    Draenei,
    FelOrc,
    Naga,
    Broken,
    Skeleton,
    Vrykul,
    Tuskarr,
    ForestTroll,
}

impl Race {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Human => 0x1,
            Self::Orc => 0x2,
            Self::Dwarf => 0x3,
            Self::NightElf => 0x4,
            Self::Undead => 0x5,
            Self::Tauren => 0x6,
            Self::Gnome => 0x7,
            Self::Troll => 0x8,
            Self::Goblin => 0x9,
            Self::BloodElf => 0xa,
            Self::Draenei => 0xb,
            Self::FelOrc => 0xc,
            Self::Naga => 0xd,
            Self::Broken => 0xe,
            Self::Skeleton => 0xf,
            Self::Vrykul => 0x10,
            Self::Tuskarr => 0x11,
            Self::ForestTroll => 0x12,
        }
    }

}

impl Default for Race {
    fn default() -> Self {
        Self::Human
    }
}

impl std::fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Human => f.write_str("Human"),
            Self::Orc => f.write_str("Orc"),
            Self::Dwarf => f.write_str("Dwarf"),
            Self::NightElf => f.write_str("NightElf"),
            Self::Undead => f.write_str("Undead"),
            Self::Tauren => f.write_str("Tauren"),
            Self::Gnome => f.write_str("Gnome"),
            Self::Troll => f.write_str("Troll"),
            Self::Goblin => f.write_str("Goblin"),
            Self::BloodElf => f.write_str("BloodElf"),
            Self::Draenei => f.write_str("Draenei"),
            Self::FelOrc => f.write_str("FelOrc"),
            Self::Naga => f.write_str("Naga"),
            Self::Broken => f.write_str("Broken"),
            Self::Skeleton => f.write_str("Skeleton"),
            Self::Vrykul => f.write_str("Vrykul"),
            Self::Tuskarr => f.write_str("Tuskarr"),
            Self::ForestTroll => f.write_str("ForestTroll"),
        }
    }
}

impl TryFrom<u8> for Race {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Human),
            2 => Ok(Self::Orc),
            3 => Ok(Self::Dwarf),
            4 => Ok(Self::NightElf),
            5 => Ok(Self::Undead),
            6 => Ok(Self::Tauren),
            7 => Ok(Self::Gnome),
            8 => Ok(Self::Troll),
            9 => Ok(Self::Goblin),
            10 => Ok(Self::BloodElf),
            11 => Ok(Self::Draenei),
            12 => Ok(Self::FelOrc),
            13 => Ok(Self::Naga),
            14 => Ok(Self::Broken),
            15 => Ok(Self::Skeleton),
            16 => Ok(Self::Vrykul),
            17 => Ok(Self::Tuskarr),
            18 => Ok(Self::ForestTroll),
            v => Err(crate::errors::EnumError::new("Race", v as u64),)
        }
    }
}

