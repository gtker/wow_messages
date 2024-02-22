#[rustfmt::skip]
mod actions;
mod base_stats;
pub mod character_features;
mod class;
#[rustfmt::skip]
mod creature_family;
#[rustfmt::skip]
pub mod exp;
mod item;
mod item_quality;
mod item_set;
mod items;
mod map;
pub mod position;
mod skill;
#[rustfmt::skip]
mod skills;
#[rustfmt::skip]
mod spells;
pub mod stats;
pub mod trigger;

pub use items::*;

macro_rules! wrath_race_class_match {
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
                    Self::DraeneiWarrior => DRAENEI_WARRIOR,
                    Self::DraeneiPaladin => DRAENEI_PALADIN,
                    Self::DraeneiHunter => DRAENEI_HUNTER,
                    Self::DraeneiPriest => DRAENEI_PRIEST,
                    Self::DraeneiShaman => DRAENEI_SHAMAN,
                    Self::DraeneiMage => DRAENEI_MAGE,
                    Self::BloodElfPaladin => BLOOD_ELF_PALADIN,
                    Self::BloodElfHunter => BLOOD_ELF_HUNTER,
                    Self::BloodElfRogue => BLOOD_ELF_ROGUE,
                    Self::BloodElfPriest => BLOOD_ELF_PRIEST,
                    Self::BloodElfMage => BLOOD_ELF_MAGE,
                    Self::BloodElfWarlock => BLOOD_ELF_WARLOCK,
                    Self::HumanDeathKnight => HUMAN_DEATH_KNIGHT,
                    Self::OrcDeathKnight => ORC_DEATH_KNIGHT,
                    Self::DwarfDeathKnight => DWARF_DEATH_KNIGHT,
                    Self::NightElfDeathKnight => NIGHT_ELF_DEATH_KNIGHT,
                    Self::UndeadDeathKnight => UNDEAD_DEATH_KNIGHT,
                    Self::TaurenDeathKnight => TAUREN_DEATH_KNIGHT,
                    Self::GnomeDeathKnight => GNOME_DEATH_KNIGHT,
                    Self::TrollDeathKnight => TROLL_DEATH_KNIGHT,
                    Self::BloodElfDeathKnight => BLOOD_ELF_DEATH_KNIGHT,
                    Self::DraeneiDeathKnight => DRAENEI_DEATH_KNIGHT,
                }
            }
        }
    };
}
pub(crate) use wrath_race_class_match;
