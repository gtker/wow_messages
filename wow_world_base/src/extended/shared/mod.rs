#[cfg(any(feature = "tbc", feature = "wrath"))]
mod tbc_wrath_player_race;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub use tbc_wrath_player_race::*;

#[cfg(any(feature = "vanilla", feature = "tbc"))]
mod vanilla_tbc_class;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub use vanilla_tbc_class::*;

#[cfg(any(feature = "vanilla", feature = "tbc"))]
mod vanilla_tbc_item_quality;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
mod vanilla_tbc_wrath_auction_house;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use vanilla_tbc_wrath_auction_house::*;

#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub use vanilla_tbc_item_quality::*;

macro_rules! exp_required_to_level_up {
    ($level:expr, $exp_required:expr) => {
        /// Get the current exp required to level up.
        ///
        #[doc = "Values `== 0 || >="]
        #[doc = stringify!($level)]
        #[doc = "` will return [None]."]
        pub const fn exp_required_to_level_up(level: u8) -> Option<i32> {
            if level == 0 || level >= $level {
                return None;
            }

            Some($exp_required[(level - 1) as usize])
        }
    };
}
pub(crate) use exp_required_to_level_up;

macro_rules! exploration_exp_for {
    ($level:expr, $exploration_exp:expr) => {
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

                    $exploration_exp[level as usize]
                } else {
                    let difference = difference as i32;
                    let exploration_percent = (100 - (difference - 5) * 5) / 100;
                    let exp = $exploration_exp[level as usize];
                    exp * exploration_percent
                }
            } else {
                $exploration_exp[level as usize]
            })
        }
    };
}
pub(crate) use exploration_exp_for;

macro_rules! get_base_stats_for {
    ($level:expr, $race_class:ty, $base_stats:ty) => {
        impl $race_class {
            /// Get the base stats for a race/class/level combination.
            ///
            #[doc = "Values `== 0 || >="]
            #[doc = stringify!($level)]
            #[doc = "` will return [None]."]
            pub const fn base_stats_for(&self, level: u8) -> Option<$base_stats> {
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
    ($map:ty) => {
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        pub struct Position {
            pub map: $map,
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub orientation: f32,
        }

        impl Position {
            pub const fn new(map: $map, x: f32, y: f32, z: f32, orientation: f32) -> Self {
                Self {
                    map,
                    x,
                    y,
                    z,
                    orientation,
                }
            }
        }

        impl From<Position> for $crate::shared::vector3d_vanilla_tbc_wrath::Vector3d {
            fn from(v: Position) -> $crate::shared::vector3d_vanilla_tbc_wrath::Vector3d {
                $crate::shared::vector3d_vanilla_tbc_wrath::Vector3d {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                }
            }
        }

        impl From<Position> for $crate::shared::vector2d_vanilla_tbc_wrath::Vector2d {
            fn from(v: Position) -> $crate::shared::vector2d_vanilla_tbc_wrath::Vector2d {
                $crate::shared::vector2d_vanilla_tbc_wrath::Vector2d { x: v.x, y: v.y }
            }
        }
    };
}

pub(crate) use position;

macro_rules! vanilla_starter_positions {
    ($position:ty, $map:ty) => {
        const HUMAN_START_POSITION: $position =
            <$position>::new(<$map>::EasternKingdoms, -8949.95, -132.493, 83.5312, 0.0);

        const TAUREN_START_POSITION: $position =
            <$position>::new(<$map>::Kalimdor, -2917.58, -257.98, 52.9968, 0.0);

        const ORC_START_POSITION: $position =
            <$position>::new(<$map>::Kalimdor, -618.518, -4251.67, 38.718, 0.0);
        const TROLL_START_POSITION: $position = ORC_START_POSITION;

        const DWARF_START_POSITION: $position =
            <$position>::new(<$map>::EasternKingdoms, -6240.32, 331.033, 382.758, 6.17716);
        const GNOME_START_POSITION: $position = DWARF_START_POSITION;

        const NIGHT_ELF_START_POSITION: $position =
            <$position>::new(<$map>::Kalimdor, 10311.3, 832.463, 1326.41, 5.69632);

        const UNDEAD_START_POSITION: $position =
            <$position>::new(<$map>::EasternKingdoms, 1676.71, 1678.31, 121.67, 2.70526);
    };
}

pub(crate) use vanilla_starter_positions;

#[cfg(any(feature = "tbc", feature = "wrath"))]
macro_rules! tbc_starter_positions {
    ($position:ty, $map:ty) => {
        const BLOOD_ELF_START_POSITION: $position =
            <$position>::new(<$map>::Outland, 10349.6, -6357.29, 33.4026, 5.31605);

        const DRAENEI_START_POSITION: $position =
            <$position>::new(<$map>::Outland, -3961.64, -13931.2, 100.615, 2.08364);
    };
}
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub(crate) use tbc_starter_positions;

macro_rules! area_trigger {
    ($position:ty) => {
        #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
        pub enum AreaTrigger {
            Circle {
                position: $position,
                radius: f32,
            },
            Square {
                position: $position,
                /// Size along the x axis.
                length: f32,
                /// Size along the y axis.
                width: f32,
                /// Size along the z axis.
                height: f32,
                /// Rotation about the Z axis
                yaw: f32,
            },
        }

        impl AreaTrigger {
            pub fn contains(&self, player: $position) -> bool {
                match *self {
                    AreaTrigger::Circle { position, radius } => {
                        position.map == player.map
                            && $crate::geometry::is_within_distance(
                                position.into(),
                                player.into(),
                                radius,
                            )
                    }
                    AreaTrigger::Square {
                        position,
                        length,
                        width,
                        height,
                        yaw,
                    } => {
                        position.map == player.map
                            && $crate::geometry::is_within_square(
                                player.into(),
                                position.into(),
                                length,
                                width,
                                height,
                                yaw,
                            )
                    }
                }
            }
        }
    };
}
pub(crate) use area_trigger;

macro_rules! verify_trigger {
    ($array_ty:ty, $triggers:expr, $position:ty) => {
        #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
        pub enum TriggerResult {
            NotFound,
            NotInsideTrigger($array_ty),
            Success($array_ty),
        }

        pub fn verify_trigger(player: $position, trigger: u32) -> TriggerResult {
            let t = match $triggers.iter().find(|(id, _)| *id == trigger) {
                None => return TriggerResult::NotFound,
                Some(t) => &t.1,
            };

            if t.0.contains(player) {
                TriggerResult::Success(t)
            } else {
                TriggerResult::NotInsideTrigger(t)
            }
        }
    };
}
pub(crate) use verify_trigger;

#[cfg(any(feature = "tbc", feature = "wrath"))]
macro_rules! tbc_wrath_trigger {
    ($position:ty) => {
        #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
        pub enum Trigger {
            Inn,
            Quest {
                quest_id: u32,
            },
            Teleport {
                location: $position,
                required_level: u8,
                required_item: u32,
                required_quest: u32,
                failed_text: Option<&'static str>,
                heroic_keys: Option<&'static [u32]>,
                heroic_required_quest: u32,
            },
        }
    };
}
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub(crate) use tbc_wrath_trigger;

macro_rules! vanilla_trigger {
    ($position:ty) => {
        #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
        pub enum Trigger {
            Inn,
            Quest {
                quest_id: u32,
            },
            Teleport {
                location: $position,
                required_level: u8,
                required_item: u32,
                required_quest: u32,
                failed_text: Option<&'static str>,
            },
        }
    };
}
pub(crate) use vanilla_trigger;

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

// Item Quality Colors
pub(crate) const GREY: [u8; 4] = [0xff, 0x9d, 0x9d, 0x9d];
pub(crate) const WHITE: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
pub(crate) const GREEN: [u8; 4] = [0xff, 0x1e, 0xff, 0x00];
pub(crate) const BLUE: [u8; 4] = [0xff, 0x00, 0x70, 0xdd];
pub(crate) const PURPLE: [u8; 4] = [0xff, 0xa3, 0x35, 0xee];
pub(crate) const ORANGE: [u8; 4] = [0xff, 0xff, 0x80, 0x00];
pub(crate) const LIGHT_YELLOW: [u8; 4] = [0xff, 0xe6, 0xcc, 0x80];
