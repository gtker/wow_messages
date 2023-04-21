mod positions;

use crate::extended::shared::{position, tbc_starter_positions, vanilla_starter_positions};
use crate::shared::player_race_tbc_wrath::PlayerRace;
use crate::wrath::{Class, Map};
pub use positions::*;

position!(crate::wrath::Map);

#[cfg(feature = "wrath")]
impl PlayerRace {
    pub const fn wrath_starting_position(&self, class: Class) -> Position {
        #[allow(clippy::single_match)] // PartialEq is not allowed in const but clippy thinks it is
        match class {
            Class::DeathKnight => return DEATH_KNIGHT_START_POSITION,
            _ => {}
        }

        match self {
            Self::Human => HUMAN_START_POSITION,
            Self::Orc => ORC_START_POSITION,
            Self::Dwarf => DWARF_START_POSITION,
            Self::NightElf => NIGHT_ELF_START_POSITION,
            Self::Undead => UNDEAD_START_POSITION,
            Self::Tauren => TAUREN_START_POSITION,
            Self::Gnome => GNOME_START_POSITION,
            Self::Troll => TROLL_START_POSITION,
            Self::BloodElf => BLOOD_ELF_START_POSITION,
            Self::Draenei => DRAENEI_START_POSITION,
        }
    }
}

#[cfg(feature = "wrath")]
impl crate::wrath::RaceClass {
    pub const fn starting_position(&self) -> Position {
        self.race().wrath_starting_position(self.class())
    }
}

vanilla_starter_positions!(Position, Map);

tbc_starter_positions!(Position, Map);

// cmangos has 5 slightly different starting positions depending on race
// This has the assumption that there is only one correct starting position.
const DEATH_KNIGHT_START_POSITION: Position =
    Position::new(Map::EbonHold, 2355.84, -5664.77, 426.028, 3.65997);
