mod positions;

use crate::extended::shared::{position, tbc_starter_positions, vanilla_starter_positions};
use crate::wrath::{Class, Map, PlayerRace};
pub use positions::*;

position!();

impl PlayerRace {
    pub const fn wrath_starting_position(&self, class: Class) -> Position {
        #[allow(clippy::single_match)] // PartialEq is not allowed in const but clippy thinks it is
        match class {
            Class::DeathKnight => return DEATH_KNIGHT_START_POSITION,
            _ => {}
        }

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

impl crate::wrath::RaceClass {
    pub const fn starting_position(&self) -> Position {
        self.race().wrath_starting_position(self.class())
    }
}

vanilla_starter_positions!();

tbc_starter_positions!();

// cmangos has 5 slightly different starting positions depending on race
// This has the assumption that there is only one correct starting position.
const DEATH_KNIGHT_START_POSITION: Position =
    Position::new(Map::EbonHold, 2355.84, -5664.77, 426.028, 3.65997);
