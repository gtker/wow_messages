use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum Class {
    Warrior,
    Paladin,
    Hunter,
    Rogue,
    Priest,
    DeathKnight,
    Shaman,
    Mage,
    Warlock,
    Druid,
}

impl Class {
    pub(crate) fn const_name(&self) -> &'static str {
        match self {
            Class::Warrior => "WARRIOR",
            Class::Paladin => "PALADIN",
            Class::Hunter => "HUNTER",
            Class::Rogue => "ROGUE",
            Class::Priest => "PRIEST",
            Class::DeathKnight => "DEATH_KNIGHT",
            Class::Shaman => "SHAMAN",
            Class::Mage => "MAGE",
            Class::Warlock => "WARLOCK",
            Class::Druid => "DRUID",
        }
    }

    pub(crate) fn as_int(&self) -> u8 {
        match self {
            Class::Warrior => 1,
            Class::Paladin => 2,
            Class::Hunter => 3,
            Class::Rogue => 4,
            Class::Priest => 5,
            Class::DeathKnight => 6,
            Class::Shaman => 7,
            Class::Mage => 8,
            Class::Warlock => 9,
            Class::Druid => 11,
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Class::Warrior => "Warrior",
            Class::Paladin => "Paladin",
            Class::Hunter => "Hunter",
            Class::Rogue => "Rogue",
            Class::Priest => "Priest",
            Class::DeathKnight => "DeathKnight",
            Class::Shaman => "Shaman",
            Class::Mage => "Mage",
            Class::Warlock => "Warlock",
            Class::Druid => "Druid",
        })
    }
}

impl TryFrom<u8> for Class {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Warrior,
            2 => Self::Paladin,
            3 => Self::Hunter,
            4 => Self::Rogue,
            5 => Self::Priest,
            6 => Self::DeathKnight,
            7 => Self::Shaman,
            8 => Self::Mage,
            9 => Self::Warlock,
            11 => Self::Druid,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum Race {
    Human,
    Orc,
    Dwarf,
    NightElf,
    Undead,
    Tauren,
    Gnome,
    Troll,
    BloodElf,
    Draenei,
}

impl Race {
    pub(crate) fn const_name(&self) -> &'static str {
        match self {
            Race::Human => "HUMAN",
            Race::Orc => "ORC",
            Race::Dwarf => "DWARF",
            Race::NightElf => "NIGHT_ELF",
            Race::Undead => "UNDEAD",
            Race::Tauren => "TAUREN",
            Race::Gnome => "GNOME",
            Race::Troll => "TROLL",
            Race::BloodElf => "BLOOD_ELF",
            Race::Draenei => "DRAENEI",
        }
    }

    pub(crate) fn as_int(&self) -> u8 {
        match self {
            Race::Human => 1,
            Race::Orc => 2,
            Race::Dwarf => 3,
            Race::NightElf => 4,
            Race::Undead => 5,
            Race::Tauren => 6,
            Race::Gnome => 7,
            Race::Troll => 8,
            Race::BloodElf => 10,
            Race::Draenei => 11,
        }
    }
}

impl Display for Race {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Race::Human => "Human",
            Race::Orc => "Orc",
            Race::Dwarf => "Dwarf",
            Race::NightElf => "NightElf",
            Race::Undead => "Undead",
            Race::Tauren => "Tauren",
            Race::Gnome => "Gnome",
            Race::Troll => "Troll",
            Race::BloodElf => "BloodElf",
            Race::Draenei => "Draenei",
        })
    }
}

impl TryFrom<u8> for Race {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Human,
            2 => Self::Orc,
            3 => Self::Dwarf,
            4 => Self::NightElf,
            5 => Self::Undead,
            6 => Self::Tauren,
            7 => Self::Gnome,
            8 => Self::Troll,
            10 => Self::BloodElf,
            11 => Self::Draenei,
            _ => return Err(()),
        })
    }
}
