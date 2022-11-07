use wow_world_base::wrath::{Class, PlayerGender, PlayerRace, Power};

pub fn get_display_id_for_player(race: PlayerRace, gender: PlayerGender) -> i32 {
    crate::tbc::class::get_display_id_for_player(race, gender)
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
        Class::DeathKnight => Power::RunicPower,
    }
}
