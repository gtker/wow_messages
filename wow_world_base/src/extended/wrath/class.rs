use crate::wrath::{Class, PlayerGender, PlayerRace, Power};

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
            Class::DeathKnight => Power::RunicPower,
        }
    }
}

impl crate::wrath::RaceClass {
    pub const fn power_type(&self) -> Power {
        self.to_race_class().1.power_type()
    }
}
