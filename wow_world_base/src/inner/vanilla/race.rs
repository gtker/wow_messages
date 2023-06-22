/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/race.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/race.wowm#L15):
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
        }
    }

}

#[cfg(feature = "print-testcase")]
impl Race {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Human => "HUMAN",
            Self::Orc => "ORC",
            Self::Dwarf => "DWARF",
            Self::NightElf => "NIGHT_ELF",
            Self::Undead => "UNDEAD",
            Self::Tauren => "TAUREN",
            Self::Gnome => "GNOME",
            Self::Troll => "TROLL",
            Self::Goblin => "GOBLIN",
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
            v => Err(crate::errors::EnumError::new("Race", v.into()),)
        }
    }
}

