use crate::manual::PlayerGender;

impl crate::tbc::PlayerRace {
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
            // Other races have males as the low id but blood elves are reversed
            Self::BloodElf => match gender {
                PlayerGender::Male => return 15476,
                PlayerGender::Female => return 15475,
            },
            Self::Draenei => 16125,
        };

        let gender = match gender {
            PlayerGender::Male => 0,
            PlayerGender::Female => 1,
        };

        race + gender
    }

    pub const fn faction_id(&self) -> i32 {
        match self {
            Self::Human => 1,
            Self::Orc => 2,
            Self::Dwarf => 3,
            Self::NightElf => 4,
            Self::Undead => 5,
            Self::Tauren => 6,
            Self::Gnome => 115,
            Self::Troll => 116,
            Self::BloodElf => 1610,
            Self::Draenei => 1629,
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
