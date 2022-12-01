mod positions;

use crate::extended::shared::{position, vanilla_starter_positions};
use crate::vanilla::{Map, PlayerRace};
pub use positions::*;

position!();

impl PlayerRace {
    pub const fn starting_position(&self) -> Position {
        match self {
            PlayerRace::Human => HUMAN_START_POSITION,
            PlayerRace::Orc => ORC_START_POSITION,
            PlayerRace::Dwarf => DWARF_START_POSITION,
            PlayerRace::NightElf => NIGHT_ELF_START_POSITION,
            PlayerRace::Undead => UNDEAD_START_POSITION,
            PlayerRace::Tauren => TAUREN_START_POSITION,
            PlayerRace::Gnome => GNOME_START_POSITION,
            PlayerRace::Troll => TROLL_START_POSITION,
        }
    }
}

impl crate::vanilla::RaceClass {
    pub const fn starting_position(&self) -> Position {
        self.race().starting_position()
    }
}

vanilla_starter_positions!();
