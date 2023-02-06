use crate::shared::player_gender_vanilla_tbc_wrath::PlayerGender;
use crate::vanilla::{Faction, PlayerRace, RaceClass};

impl PlayerRace {
    pub const fn display_id(&self, gender: PlayerGender) -> i32 {
        let race = match self {
            Self::Human => 49,
            Self::Orc => 51,
            Self::Dwarf => 53,
            Self::NightElf => 55,
            Self::Undead => 57,
            Self::Tauren => 59,
            Self::Gnome => 1563,
            Self::Troll => 1478,
        };

        let gender = match gender {
            PlayerGender::Male => 0,
            PlayerGender::Female => 1,
        };

        race + gender
    }

    pub const fn faction_id(&self) -> Faction {
        match self {
            PlayerRace::Human => Faction::PlayerHuman,
            PlayerRace::Orc => Faction::PlayerOrc,
            PlayerRace::Dwarf => Faction::PlayerDwarf,
            PlayerRace::NightElf => Faction::PlayerNightElf,
            PlayerRace::Undead => Faction::PlayerUndead,
            PlayerRace::Tauren => Faction::PlayerTauren,
            PlayerRace::Gnome => Faction::PlayerGnome,
            PlayerRace::Troll => Faction::PlayerTroll,
        }
    }

    pub const fn race_scale(&self, gender: PlayerGender) -> f32 {
        match (self, gender) {
            (Self::Tauren, PlayerGender::Male) => 1.35,
            (Self::Tauren, PlayerGender::Female) => 1.25,
            (_, _) => 1.0,
        }
    }
}

impl RaceClass {
    pub const fn display_id(&self, gender: PlayerGender) -> i32 {
        self.to_race_class().0.display_id(gender)
    }

    pub const fn faction_id(&self) -> Faction {
        self.to_race_class().0.faction_id()
    }

    pub const fn race_scale(&self, gender: PlayerGender) -> f32 {
        self.to_race_class().0.race_scale(gender)
    }
}
