use crate::vanilla::{Class, InventoryType, Power};

impl Class {
    pub const fn power_type(&self) -> Power {
        match self {
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

impl crate::vanilla::RaceClass {
    pub const fn power_type(&self) -> Power {
        self.to_race_class().1.power_type()
    }
}

impl crate::tbc::RaceClass {
    pub const fn power_type(&self) -> Power {
        self.class().power_type()
    }
}
