mod positions;

use crate::shared::{position, tbc_starter_positions, vanilla_starter_positions};
pub use positions::*;
use wow_world_base::wrath::{Class, Map, PlayerRace};

position!();

pub fn get_starting_position(race: PlayerRace, class: Class) -> Position {
    match class {
        Class::DeathKnight => return DEATH_KNIGHT_START_POSITION,
        _ => {}
    }

    match race {
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

vanilla_starter_positions!();

tbc_starter_positions!();

// cmangos has 5 slightly different starting positions depending on race
// This has the assumption that there is only one correct starting position.
const DEATH_KNIGHT_START_POSITION: Position =
    Position::new(Map::EbonHold, 2355.84, -5664.77, 426.028, 3.65997);
