mod data;

use crate::manual::vanilla::Item;
use crate::vanilla::{Class, InventoryType};
use crate::vanilla::{PlayerRace, RaceClass};
pub use data::*;

pub fn lookup_item(id: u32) -> Option<&'static Item> {
    ITEMS.iter().find(|a| a.entry as u32 == id)
}

impl Item {
    pub const fn possibly_usable_by(&self, race_class: RaceClass) -> bool {
        let race = race_class.race();
        let class = race_class.class();

        let allowed_by_race = self.allowed_race.is_empty()
            || match race_class.race() {
                PlayerRace::Human => self.allowed_race.is_HUMAN(),
                PlayerRace::Orc => self.allowed_race.is_ORC(),
                PlayerRace::Dwarf => self.allowed_race.is_DWARF(),
                PlayerRace::NightElf => self.allowed_race.is_NIGHT_ELF(),
                PlayerRace::Undead => self.allowed_race.is_UNDEAD(),
                PlayerRace::Tauren => self.allowed_race.is_TAUREN(),
                PlayerRace::Gnome => self.allowed_race.is_GNOME(),
                PlayerRace::Troll => self.allowed_race.is_TROLL(),
            };

        let allowed_by_class = self.allowed_class.is_empty()
            || match class {
                Class::Warrior => self.allowed_class.is_WARRIOR(),
                Class::Paladin => self.allowed_class.is_PALADIN(),
                Class::Hunter => self.allowed_class.is_HUNTER(),
                Class::Rogue => self.allowed_class.is_ROGUE(),
                Class::Priest => self.allowed_class.is_PRIEST(),
                Class::Shaman => self.allowed_class.is_SHAMAN(),
                Class::Mage => self.allowed_class.is_MAGE(),
                Class::Warlock => self.allowed_class.is_WARLOCK(),
                Class::Druid => self.allowed_class.is_DRUID(),
            };

        let equip_allowed = match self.inventory_type {
            InventoryType::Head
            | InventoryType::Shoulders
            | InventoryType::Chest
            | InventoryType::Waist
            | InventoryType::Legs
            | InventoryType::Feet
            | InventoryType::Wrists
            | InventoryType::Hands
            | InventoryType::Robe => true, // Check for armor skills
            InventoryType::Weapon
            | InventoryType::TwoHandedWeapon
            | InventoryType::WeaponMainHand
            | InventoryType::WeaponOffHand => true, // check for weapon skills
            InventoryType::Ranged | InventoryType::RangedRight => matches!(
                class,
                Class::Warrior | Class::Rogue | Class::Hunter | Class::Mage
            ), // Check for wands
            InventoryType::Body | InventoryType::NonEquip => true, // more?

            InventoryType::Shield => {
                matches!(class, Class::Warrior | Class::Paladin | Class::Shaman)
            }
            InventoryType::Ammo | InventoryType::Thrown | InventoryType::Quiver => {
                matches!(class, Class::Warrior | Class::Rogue | Class::Hunter)
            }
            InventoryType::Relic => matches!(class, Class::Paladin | Class::Shaman | Class::Druid),
            InventoryType::Holdable
            | InventoryType::Bag
            | InventoryType::Tabard
            | InventoryType::Neck
            | InventoryType::Cloak
            | InventoryType::Finger
            | InventoryType::Trinket => true,
        };

        allowed_by_class && allowed_by_race && equip_allowed
    }
}
