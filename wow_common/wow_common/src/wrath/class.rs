use crate::shared::tbc_wrath_display_id;
use wow_world_base::wrath::{Class, PlayerGender, PlayerRace, Power};

tbc_wrath_display_id!();

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
