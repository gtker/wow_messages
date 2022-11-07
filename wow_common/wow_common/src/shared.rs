macro_rules! exp_required_to_level_up {
    ($level:expr) => {
        /// Get the current exp required to level up.
        ///
        #[doc = "Values `== 0 || >="]
        #[doc = stringify!($level)]
        #[doc = "` will return [None]."]
        pub const fn exp_required_to_level_up(level: u8) -> Option<i32> {
            if level == 0 || level >= $level {
                return None;
            }

            Some(EXP_REQUIRED_FOR_LEVEL[(level - 1) as usize])
        }
    };
}
pub(crate) use exp_required_to_level_up;

macro_rules! get_base_stats_for {
    ($level:expr) => {
        /// Get the base stats for a race/class/level combination.
        ///
        #[doc = "Values `== 0 || >="]
        #[doc = stringify!($level)]
        #[doc = "` will return [None]."]
        pub const fn get_base_stats_for(combo: RaceClass, level: u8) -> Option<BaseStats> {
            if level > $level || level == 0 {
                return None;
            }

            let level = level - 1;

            Some(get_base_stats(combo)[level as usize])
        }
    };
}

pub(crate) use get_base_stats_for;

macro_rules! race_scale {
    () => {
        pub fn get_race_scale(race: PlayerRace, gender: PlayerGender) -> f32 {
            match (race, gender) {
                (PlayerRace::Tauren, PlayerGender::Male) => 1.35,
                (PlayerRace::Tauren, PlayerGender::Female) => 1.25,
                (_, _) => 1.0,
            }
        }
    };
}

pub(crate) use race_scale;
