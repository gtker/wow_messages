use crate::manual::PlayerGender;
use crate::vanilla::PlayerRace;

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

    pub const fn faction_id(&self) -> i32 {
        match self {
            PlayerRace::Human => 1,
            PlayerRace::Orc => 2,
            PlayerRace::Dwarf => 3,
            PlayerRace::NightElf => 4,
            PlayerRace::Undead => 5,
            PlayerRace::Tauren => 6,
            PlayerRace::Gnome => 115,
            PlayerRace::Troll => 116,
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
