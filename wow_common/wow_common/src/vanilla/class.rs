use wow_world_base::vanilla::{Class, PlayerGender, PlayerRace, Power};

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
    };

    let gender = match gender {
        PlayerGender::Male => 0,
        PlayerGender::Female => 1,
    };

    race + gender
}

pub fn get_power_for_class(class: Class) -> Power {
    match class {
        Class::Warrior => Power::Rage,
        Class::Rogue => Power::Energy,
        Class::Paladin
        | Class::Hunter
        | Class::Priest
        | Class::Shaman
        | Class::Mage
        | Class::Warlock
        | Class::Druid => Power::Mana,
    }
}
