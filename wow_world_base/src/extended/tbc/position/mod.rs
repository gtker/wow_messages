mod positions;

use crate::extended::shared::{position, tbc_starter_positions, vanilla_starter_positions};
use crate::manual::shared::PlayerRace;
use crate::tbc::Map;
pub use positions::*;

position!();

impl PlayerRace {
    pub const fn tbc_starting_position(&self) -> Position {
        match self {
            PlayerRace::Human => HUMAN_START_POSITION,
            PlayerRace::Orc => ORC_START_POSITION,
            PlayerRace::Dwarf => DWARF_START_POSITION,
            PlayerRace::NightElf => NIGHT_ELF_START_POSITION,
            PlayerRace::Undead => UNDEAD_START_POSITION,
            PlayerRace::Tauren => TAUREN_START_POSITION,
            PlayerRace::Gnome => GNOME_START_POSITION,
            PlayerRace::Troll => TROLL_START_POSITION,
            PlayerRace::BloodElf => BLOOD_ELF_START_POSITION,
            PlayerRace::Draenei => DRAENEI_START_POSITION,
        }
    }
}

impl crate::tbc::RaceClass {
    pub const fn starting_position(&self) -> Position {
        self.race().tbc_starting_position()
    }
}

vanilla_starter_positions!();

tbc_starter_positions!();
