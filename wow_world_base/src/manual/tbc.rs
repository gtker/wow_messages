use crate::tbc::Race;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

pub use crate::manual::PlayerGender;

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
            Race::BloodElf => Self::BloodElf,
            Race::Draenei => Self::Draenei,
            race => return Err(race),
        })
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
