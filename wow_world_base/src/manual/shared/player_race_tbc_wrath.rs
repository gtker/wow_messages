use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// Enum containing only the races players are actually allowed to be.
///
/// The regular [Race](crate::tbc::Race) enum also has several races which don't
/// exist and is only partially supported in the client.
/// This enum exists in order to avoid having to check for the extra enumerators every time of use.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

macro_rules! from {
    ($player_race:ty, $race:ty) => {
        impl TryFrom<$race> for $player_race {
            type Error = $race;

            fn try_from(value: $race) -> Result<Self, Self::Error> {
                Ok(match value {
                    <$race>::Human => Self::Human,
                    <$race>::Orc => Self::Orc,
                    <$race>::Dwarf => Self::Dwarf,
                    <$race>::NightElf => Self::NightElf,
                    <$race>::Undead => Self::Undead,
                    <$race>::Tauren => Self::Tauren,
                    <$race>::Gnome => Self::Gnome,
                    <$race>::Troll => Self::Troll,
                    <$race>::BloodElf => Self::BloodElf,
                    <$race>::Draenei => Self::Draenei,
                    race => return Err(race),
                })
            }
        }

        impl From<$player_race> for $race {
            fn from(v: $player_race) -> Self {
                match v {
                    <$player_race>::Human => Self::Human,
                    <$player_race>::Orc => Self::Orc,
                    <$player_race>::Dwarf => Self::Dwarf,
                    <$player_race>::NightElf => Self::NightElf,
                    <$player_race>::Undead => Self::Undead,
                    <$player_race>::Tauren => Self::Tauren,
                    <$player_race>::Gnome => Self::Gnome,
                    <$player_race>::Troll => Self::Troll,
                    <$player_race>::BloodElf => Self::BloodElf,
                    <$player_race>::Draenei => Self::Draenei,
                }
            }
        }
    };
}

#[cfg(feature = "tbc")]
from!(PlayerRace, crate::tbc::Race);
#[cfg(feature = "wrath")]
from!(PlayerRace, crate::wrath::Race);

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
