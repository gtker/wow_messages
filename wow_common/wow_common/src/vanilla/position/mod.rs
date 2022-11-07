#[cfg(test)]
mod generator;
mod positions;

pub use positions::*;
use wow_world_base::vanilla::{Map, PlayerRace};

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub map: Map,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: f32,
}

impl Position {
    pub const fn new(map: Map, x: f32, y: f32, z: f32, orientation: f32) -> Self {
        Self {
            map,
            x,
            y,
            z,
            orientation,
        }
    }
}

pub fn get_starting_position(race: PlayerRace) -> Position {
    match race {
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

const HUMAN_START_POSITION: Position =
    Position::new(Map::EasternKingdoms, -8949.95, -132.493, 83.5312, 0.0);

const TAUREN_START_POSITION: Position =
    Position::new(Map::Kalimdor, -2917.58, -257.98, 52.9968, 0.0);

const ORC_START_POSITION: Position = Position::new(Map::Kalimdor, -618.518, -4251.67, 38.718, 0.0);
const TROLL_START_POSITION: Position = ORC_START_POSITION;

const DWARF_START_POSITION: Position =
    Position::new(Map::EasternKingdoms, -6240.32, 331.033, 382.758, 6.17716);
const GNOME_START_POSITION: Position = DWARF_START_POSITION;

const NIGHT_ELF_START_POSITION: Position =
    Position::new(Map::Kalimdor, 10311.3, 832.463, 1326.41, 5.69632);

const UNDEAD_START_POSITION: Position =
    Position::new(Map::EasternKingdoms, 1676.71, 1678.31, 121.67, 2.70526);
