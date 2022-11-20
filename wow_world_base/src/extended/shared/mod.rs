mod tbc_wrath_player_race;
mod vanilla_tbc_class;

pub use tbc_wrath_player_race::*;
pub use vanilla_tbc_class::*;

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

macro_rules! exploration_exp_for {
    ($level:expr) => {
        /// Get exploration exp for area.
        ///
        #[doc = "`level` `== 0 || >="]
        #[doc = stringify!($level)]
        #[doc = "` will return [None]."]
        pub const fn exploration_exp_for(level: u8, area_level: u8) -> Option<i32> {
            if level == 0 || level > $level {
                return None;
            }

            let level = level - 1;

            let difference = level.abs_diff(area_level);

            Some(if difference > 5 {
                if level < area_level {
                    let level = level + 5;

                    let level = if level > $level { $level } else { level };

                    EXPLORATION_EXP_PER_LEVEL[level as usize]
                } else {
                    let difference = difference as i32;
                    let exploration_percent = (100 - (difference - 5) * 5) / 100;
                    let exp = EXPLORATION_EXP_PER_LEVEL[level as usize];
                    exp * exploration_percent
                }
            } else {
                EXPLORATION_EXP_PER_LEVEL[level as usize]
            })
        }
    };
}
pub(crate) use exploration_exp_for;

macro_rules! get_base_stats_for {
    ($level:expr) => {
        impl RaceClass {
            /// Get the base stats for a race/class/level combination.
            ///
            #[doc = "Values `== 0 || >="]
            #[doc = stringify!($level)]
            #[doc = "` will return [None]."]
            pub const fn base_stats_for(&self, level: u8) -> Option<BaseStats> {
                if level > $level || level == 0 {
                    return None;
                }

                let level = level - 1;

                Some(self.base_stats()[level as usize])
            }
        }
    };
}

pub(crate) use get_base_stats_for;

macro_rules! position {
    () => {
        #[derive(Debug, Copy, Clone)]
        pub struct Position {
            pub map: Map,
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub orientation: f32,
        }

        impl Position {
            pub const fn new(map: Map, x: f32, y: f32, z: f32, orientation: f32) -> Self {
                Self {
                    map,
                    x,
                    y,
                    z,
                    orientation,
                }
            }
        }
    };
}

pub(crate) use position;

macro_rules! vanilla_starter_positions {
    () => {
        const HUMAN_START_POSITION: Position =
            Position::new(Map::EasternKingdoms, -8949.95, -132.493, 83.5312, 0.0);

        const TAUREN_START_POSITION: Position =
            Position::new(Map::Kalimdor, -2917.58, -257.98, 52.9968, 0.0);

        const ORC_START_POSITION: Position =
            Position::new(Map::Kalimdor, -618.518, -4251.67, 38.718, 0.0);
        const TROLL_START_POSITION: Position = ORC_START_POSITION;

        const DWARF_START_POSITION: Position =
            Position::new(Map::EasternKingdoms, -6240.32, 331.033, 382.758, 6.17716);
        const GNOME_START_POSITION: Position = DWARF_START_POSITION;

        const NIGHT_ELF_START_POSITION: Position =
            Position::new(Map::Kalimdor, 10311.3, 832.463, 1326.41, 5.69632);

        const UNDEAD_START_POSITION: Position =
            Position::new(Map::EasternKingdoms, 1676.71, 1678.31, 121.67, 2.70526);
    };
}

pub(crate) use vanilla_starter_positions;

macro_rules! tbc_starter_positions {
    () => {
        const BLOOD_ELF_START_POSITION: Position =
            Position::new(Map::Outland, 10349.6, -6357.29, 33.4026, 5.31605);

        const DRAENEI_START_POSITION: Position =
            Position::new(Map::Outland, -3961.64, -13931.2, 100.615, 2.08364);
    };
}
pub(crate) use tbc_starter_positions;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Action {
    button: u8,
    action: u16,
    ty: u8,
}

impl Action {
    pub const fn new(button: u8, action: u16, ty: u8) -> Self {
        Self { button, action, ty }
    }
}
