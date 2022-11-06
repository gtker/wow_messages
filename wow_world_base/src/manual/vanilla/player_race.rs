use crate::vanilla::Race;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// Enum containing only the races players are actually allowed to be.
///
/// The regular [Race](crate::vanilla::Race) enum also has the Goblin race
/// which isn't historically valid and is only partially supported in the client.
/// This enum exists in order to avoid having to check for the enumerator every time of use.
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
}

impl TryFrom<Race> for PlayerRace {
    type Error = Race;

    fn try_from(value: Race) -> Result<Self, Self::Error> {
        Ok(match value {
            Race::Human => Self::Human,
            Race::Orc => Self::Orc,
            Race::Dwarf => Self::Dwarf,
            Race::NightElf => Self::NightElf,
            Race::Undead => Self::Undead,
            Race::Tauren => Self::Tauren,
            Race::Gnome => Self::Gnome,
            Race::Troll => Self::Troll,
            race => return Err(race),
        })
    }
}

impl From<PlayerRace> for Race {
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
        })
    }
}
