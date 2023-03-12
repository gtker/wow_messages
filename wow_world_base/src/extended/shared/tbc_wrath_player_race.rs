use crate::shared::player_gender_vanilla_tbc_wrath::PlayerGender;

#[cfg(any(feature = "tbc", feature = "wrath"))]
impl crate::shared::player_race_tbc_wrath::PlayerRace {
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

    #[cfg(feature = "tbc")]
    pub const fn faction_id_tbc(&self) -> crate::tbc::Faction {
        match self {
            Self::Human => crate::tbc::Faction::PlayerHuman,
            Self::Orc => crate::tbc::Faction::PlayerOrc,
            Self::Dwarf => crate::tbc::Faction::PlayerDwarf,
            Self::NightElf => crate::tbc::Faction::PlayerNightElf,
            Self::Undead => crate::tbc::Faction::PlayerUndead,
            Self::Tauren => crate::tbc::Faction::PlayerTauren,
            Self::Gnome => crate::tbc::Faction::PlayerGnome,
            Self::Troll => crate::tbc::Faction::PlayerTroll,
            Self::BloodElf => crate::tbc::Faction::PlayerBloodElf,
            Self::Draenei => crate::tbc::Faction::PlayerDraenei,
        }
    }

    #[cfg(feature = "wrath")]
    pub const fn faction_id_wrath(&self) -> crate::wrath::Faction {
        match self {
            Self::Human => crate::wrath::Faction::PlayerHuman,
            Self::Orc => crate::wrath::Faction::PlayerOrc,
            Self::Dwarf => crate::wrath::Faction::PlayerDwarf,
            Self::NightElf => crate::wrath::Faction::PlayerNightElf,
            Self::Undead => crate::wrath::Faction::PlayerUndead,
            Self::Tauren => crate::wrath::Faction::PlayerTauren,
            Self::Gnome => crate::wrath::Faction::PlayerGnome,
            Self::Troll => crate::wrath::Faction::PlayerTroll,
            Self::BloodElf => crate::wrath::Faction::PlayerBloodElf,
            Self::Draenei => crate::wrath::Faction::PlayerDraenei,
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

#[cfg(feature = "tbc")]
impl crate::tbc::RaceClass {
    pub const fn display_id(&self, gender: PlayerGender) -> i32 {
        self.race().display_id(gender)
    }

    pub const fn faction_id_tbc(&self) -> crate::tbc::Faction {
        self.race().faction_id_tbc()
    }

    pub const fn race_scale(&self, gender: PlayerGender) -> f32 {
        self.race().race_scale(gender)
    }
}

#[cfg(feature = "wrath")]
impl crate::wrath::RaceClass {
    pub const fn display_id(&self, gender: PlayerGender) -> i32 {
        self.race().display_id(gender)
    }

    pub const fn faction_id_wrath(&self) -> crate::wrath::Faction {
        self.race().faction_id_wrath()
    }

    pub const fn race_scale(&self, gender: PlayerGender) -> f32 {
        self.race().race_scale(gender)
    }
}
