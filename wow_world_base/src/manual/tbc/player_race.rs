use crate::{tbc, wrath};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// Enum containing only the races players are actually allowed to be.
///
/// The regular [Race](crate::tbc::Race) enum also has several races which don't
/// exist and is only partially supported in the client.
/// This enum exists in order to avoid having to check for the extra enumerators every time of use.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PlayerRace {
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

impl TryFrom<tbc::Race> for PlayerRace {
    type Error = tbc::Race;

    fn try_from(value: tbc::Race) -> Result<Self, Self::Error> {
        Ok(match value {
            tbc::Race::Human => Self::Human,
            tbc::Race::Orc => Self::Orc,
            tbc::Race::Dwarf => Self::Dwarf,
            tbc::Race::NightElf => Self::NightElf,
            tbc::Race::Undead => Self::Undead,
            tbc::Race::Tauren => Self::Tauren,
            tbc::Race::Gnome => Self::Gnome,
            tbc::Race::Troll => Self::Troll,
            tbc::Race::BloodElf => Self::BloodElf,
            tbc::Race::Draenei => Self::Draenei,
            race => return Err(race),
        })
    }
}

impl From<PlayerRace> for tbc::Race {
    fn from(v: PlayerRace) -> Self {
        match v {
            PlayerRace::Human => Self::Human,
            PlayerRace::Orc => Self::Orc,
            PlayerRace::Dwarf => Self::Dwarf,
            PlayerRace::NightElf => Self::NightElf,
            PlayerRace::Undead => Self::Undead,
            PlayerRace::Tauren => Self::Tauren,
            PlayerRace::Gnome => Self::Gnome,
            PlayerRace::Troll => Self::Troll,
            PlayerRace::BloodElf => Self::BloodElf,
            PlayerRace::Draenei => Self::Draenei,
        }
    }
}

impl Default for PlayerRace {
    fn default() -> Self {
        Self::Human
    }
}

impl Display for PlayerRace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PlayerRace::Human => "Human",
            PlayerRace::Orc => "Orc",
            PlayerRace::Dwarf => "Dwarf",
            PlayerRace::NightElf => "Night Elf",
            PlayerRace::Undead => "Undead",
            PlayerRace::Tauren => "Tauren",
            PlayerRace::Gnome => "Gnome",
            PlayerRace::Troll => "Troll",
            PlayerRace::BloodElf => "Blood Elf",
            PlayerRace::Draenei => "Draenei",
        })
    }
}

impl TryFrom<wrath::Race> for PlayerRace {
    type Error = wrath::Race;

    fn try_from(value: wrath::Race) -> Result<Self, Self::Error> {
        Ok(match value {
            wrath::Race::Human => Self::Human,
            wrath::Race::Orc => Self::Orc,
            wrath::Race::Dwarf => Self::Dwarf,
            wrath::Race::NightElf => Self::NightElf,
            wrath::Race::Undead => Self::Undead,
            wrath::Race::Tauren => Self::Tauren,
            wrath::Race::Gnome => Self::Gnome,
            wrath::Race::Troll => Self::Troll,
            wrath::Race::BloodElf => Self::BloodElf,
            wrath::Race::Draenei => Self::Draenei,
            race => return Err(race),
        })
    }
}

impl From<PlayerRace> for wrath::Race {
    fn from(v: PlayerRace) -> Self {
        match v {
            PlayerRace::Human => Self::Human,
            PlayerRace::Orc => Self::Orc,
            PlayerRace::Dwarf => Self::Dwarf,
            PlayerRace::NightElf => Self::NightElf,
            PlayerRace::Undead => Self::Undead,
            PlayerRace::Tauren => Self::Tauren,
            PlayerRace::Gnome => Self::Gnome,
            PlayerRace::Troll => Self::Troll,
            PlayerRace::BloodElf => Self::BloodElf,
            PlayerRace::Draenei => Self::Draenei,
        }
    }
}
