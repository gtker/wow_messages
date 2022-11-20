use crate::vanilla::{Class, Power};

impl Class {
    pub const fn power_type(class: Class) -> Power {
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
}
