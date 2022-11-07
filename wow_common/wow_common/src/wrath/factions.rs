use wow_world_base::wrath::PlayerRace;

pub const fn get_race_faction(race: PlayerRace) -> i32 {
    let race = match race {
        PlayerRace::Human => wow_world_base::tbc::PlayerRace::Human,
        PlayerRace::Orc => wow_world_base::tbc::PlayerRace::Orc,
        PlayerRace::Dwarf => wow_world_base::tbc::PlayerRace::Dwarf,
        PlayerRace::NightElf => wow_world_base::tbc::PlayerRace::NightElf,
        PlayerRace::Undead => wow_world_base::tbc::PlayerRace::Undead,
        PlayerRace::Tauren => wow_world_base::tbc::PlayerRace::Tauren,
        PlayerRace::Gnome => wow_world_base::tbc::PlayerRace::Gnome,
        PlayerRace::Troll => wow_world_base::tbc::PlayerRace::Troll,
        PlayerRace::BloodElf => wow_world_base::tbc::PlayerRace::BloodElf,
        PlayerRace::Draenei => wow_world_base::tbc::PlayerRace::Draenei,
    };

    crate::tbc::factions::get_race_faction(race)
}
