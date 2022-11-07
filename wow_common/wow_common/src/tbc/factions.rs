use wow_world_base::tbc::PlayerRace;

pub const fn get_race_faction(race: PlayerRace) -> i32 {
    match race {
        PlayerRace::Human => 1,
        PlayerRace::Orc => 2,
        PlayerRace::Dwarf => 3,
        PlayerRace::NightElf => 4,
        PlayerRace::Undead => 5,
        PlayerRace::Tauren => 6,
        PlayerRace::Gnome => 115,
        PlayerRace::Troll => 116,
        PlayerRace::BloodElf => 1610,
        PlayerRace::Draenei => 1629,
    }
}
