use crate::shared::vanilla_tbc_power;
use wow_world_base::tbc::{Class, PlayerGender, PlayerRace, Power};

pub fn get_display_id_for_player(race: PlayerRace, gender: PlayerGender) -> i32 {
    let race = match race {
        PlayerRace::Human => 49,
        PlayerRace::Orc => 51,
        PlayerRace::Dwarf => 53,
        PlayerRace::NightElf => 55,
        PlayerRace::Undead => 57,
        PlayerRace::Tauren => 59,
        PlayerRace::Gnome => 1563,
        PlayerRace::Troll => 1478,
        // Other races have males as the low id but blood elves are reversed
        PlayerRace::BloodElf => match gender {
            PlayerGender::Male => 15476,
            PlayerGender::Female => 15475,
        },
        PlayerRace::Draenei => 16125,
    };

    let gender = match gender {
        PlayerGender::Male => 0,
        PlayerGender::Female => 1,
    };

    race + gender
}

vanilla_tbc_power!();
