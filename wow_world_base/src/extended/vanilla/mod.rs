mod actions;
mod base_stats;
pub mod character_features;
mod creature_family;
pub mod exp;
mod item;
mod item_set;
mod items;
mod map;
mod player_race;
pub mod position;
mod skill;
mod skills;
mod spells;
pub mod stats;
pub mod trigger;

pub use actions::*;
pub use base_stats::*;
pub use creature_family::*;
pub use item::*;
pub use item_set::*;
pub use items::*;
pub use map::*;
pub use player_race::*;
pub use skill::*;
pub use skills::*;
pub use spells::*;

macro_rules! vanilla_race_class_match {
    ($function:ident, $ret_type:ty, $race_class:ty) => {
        impl $race_class {
            pub const fn $function(&self) -> $ret_type {
                match self {
                    Self::DwarfHunter => DWARF_HUNTER,
                    Self::DwarfPaladin => DWARF_PALADIN,
                    Self::DwarfPriest => DWARF_PRIEST,
                    Self::DwarfRogue => DWARF_ROGUE,
                    Self::DwarfWarrior => DWARF_WARRIOR,
                    Self::GnomeMage => GNOME_MAGE,
                    Self::GnomeRogue => GNOME_ROGUE,
                    Self::GnomeWarlock => GNOME_WARLOCK,
                    Self::GnomeWarrior => GNOME_WARRIOR,
                    Self::HumanMage => HUMAN_MAGE,
                    Self::HumanPaladin => HUMAN_PALADIN,
                    Self::HumanPriest => HUMAN_PRIEST,
                    Self::HumanRogue => HUMAN_ROGUE,
                    Self::HumanWarlock => HUMAN_WARLOCK,
                    Self::HumanWarrior => HUMAN_WARRIOR,
                    Self::NightElfDruid => NIGHT_ELF_DRUID,
                    Self::NightElfHunter => NIGHT_ELF_HUNTER,
                    Self::NightElfPriest => NIGHT_ELF_PRIEST,
                    Self::NightElfRogue => NIGHT_ELF_ROGUE,
                    Self::NightElfWarrior => NIGHT_ELF_WARRIOR,
                    Self::OrcHunter => ORC_HUNTER,
                    Self::OrcRogue => ORC_ROGUE,
                    Self::OrcShaman => ORC_SHAMAN,
                    Self::OrcWarlock => ORC_WARLOCK,
                    Self::OrcWarrior => ORC_WARRIOR,
                    Self::TaurenDruid => TAUREN_DRUID,
                    Self::TaurenHunter => TAUREN_HUNTER,
                    Self::TaurenShaman => TAUREN_SHAMAN,
                    Self::TaurenWarrior => TAUREN_WARRIOR,
                    Self::TrollHunter => TROLL_HUNTER,
                    Self::TrollMage => TROLL_MAGE,
                    Self::TrollPriest => TROLL_PRIEST,
                    Self::TrollRogue => TROLL_ROGUE,
                    Self::TrollShaman => TROLL_SHAMAN,
                    Self::TrollWarrior => TROLL_WARRIOR,
                    Self::UndeadMage => UNDEAD_MAGE,
                    Self::UndeadPriest => UNDEAD_PRIEST,
                    Self::UndeadRogue => UNDEAD_ROGUE,
                    Self::UndeadWarlock => UNDEAD_WARLOCK,
                    Self::UndeadWarrior => UNDEAD_WARRIOR,
                }
            }
        }
    };
}
pub(crate) use vanilla_race_class_match;

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
