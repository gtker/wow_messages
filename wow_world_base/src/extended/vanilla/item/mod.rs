use crate::manual::vanilla::Item;
use crate::vanilla::{Class, InventoryType, ItemClassAndSubClass};
use crate::vanilla::{PlayerRace, RaceClass};

impl Item {
    /// Returns true if the item is usable by the [`RaceClass`].
    ///
    /// Non-equipable items along with bags and quivers will always return false.
    pub const fn possibly_equipable_by(&self, race_class: RaceClass) -> bool {
        let class = race_class.class();

        let allowed_by_race = self.allowed_race().is_empty()
            || match race_class.race() {
                PlayerRace::Human => self.allowed_race().is_HUMAN(),
                PlayerRace::Orc => self.allowed_race().is_ORC(),
                PlayerRace::Dwarf => self.allowed_race().is_DWARF(),
                PlayerRace::NightElf => self.allowed_race().is_NIGHT_ELF(),
                PlayerRace::Undead => self.allowed_race().is_UNDEAD(),
                PlayerRace::Tauren => self.allowed_race().is_TAUREN(),
                PlayerRace::Gnome => self.allowed_race().is_GNOME(),
                PlayerRace::Troll => self.allowed_race().is_TROLL(),
            };

        let allowed_by_class = self.allowed_class().is_empty()
            || match class {
                Class::Warrior => self.allowed_class().is_WARRIOR(),
                Class::Paladin => self.allowed_class().is_PALADIN(),
                Class::Hunter => self.allowed_class().is_HUNTER(),
                Class::Rogue => self.allowed_class().is_ROGUE(),
                Class::Priest => self.allowed_class().is_PRIEST(),
                Class::Shaman => self.allowed_class().is_SHAMAN(),
                Class::Mage => self.allowed_class().is_MAGE(),
                Class::Warlock => self.allowed_class().is_WARLOCK(),
                Class::Druid => self.allowed_class().is_DRUID(),
            };

        let equip_allowed = match self.class_and_sub_class() {
            ItemClassAndSubClass::Consumable
            | ItemClassAndSubClass::CheeseOrBreadObsolete
            | ItemClassAndSubClass::LiquidObsolete
            | ItemClassAndSubClass::Bag
            | ItemClassAndSubClass::SoulBag
            | ItemClassAndSubClass::HerbBag
            | ItemClassAndSubClass::EnchantingBag
            | ItemClassAndSubClass::ObsoleteWeapon
            | ItemClassAndSubClass::Reagent
            | ItemClassAndSubClass::WandObsolete
            | ItemClassAndSubClass::BoltObsolete
            | ItemClassAndSubClass::Arrow
            | ItemClassAndSubClass::Bullet
            | ItemClassAndSubClass::ThrownObsolete
            | ItemClassAndSubClass::TradeGood
            | ItemClassAndSubClass::PartTradeGood
            | ItemClassAndSubClass::ExplosiveTradeGood
            | ItemClassAndSubClass::DeviceTradeGood
            | ItemClassAndSubClass::GenericObsolete
            | ItemClassAndSubClass::Book
            | ItemClassAndSubClass::LeatherworkingRecipe
            | ItemClassAndSubClass::TailoringRecipe
            | ItemClassAndSubClass::EngineeringRecipe
            | ItemClassAndSubClass::BlacksmithingRecipe
            | ItemClassAndSubClass::CookingRecipe
            | ItemClassAndSubClass::AlchemyRecipe
            | ItemClassAndSubClass::FirstAidRecipe
            | ItemClassAndSubClass::EnchantingRecipe
            | ItemClassAndSubClass::FishingRecipe
            | ItemClassAndSubClass::MoneyObsolete
            | ItemClassAndSubClass::QuiverObsolete
            | ItemClassAndSubClass::QuiverObsolete1
            | ItemClassAndSubClass::Quiver
            | ItemClassAndSubClass::AmmoPouch
            | ItemClassAndSubClass::Quest
            | ItemClassAndSubClass::Key
            | ItemClassAndSubClass::Lockpick
            | ItemClassAndSubClass::Permanent
            | ItemClassAndSubClass::Junk
            | ItemClassAndSubClass::JewelryObsolete
            | ItemClassAndSubClass::BucklerObsolete
            | ItemClassAndSubClass::EngineeringBag => false,

            ItemClassAndSubClass::OneHandedAxe | ItemClassAndSubClass::TwoHandedAxe => {
                matches!(class, Class::Hunter | Class::Warrior | Class::Paladin)
            }
            ItemClassAndSubClass::OneHandedMace => {
                matches!(
                    class,
                    Class::Warrior | Class::Druid | Class::Paladin | Class::Priest | Class::Rogue
                )
            }
            ItemClassAndSubClass::TwoHandedMace => {
                matches!(class, Class::Warrior | Class::Druid | Class::Paladin)
            }
            ItemClassAndSubClass::Polearm => {
                matches!(class, Class::Hunter | Class::Warrior | Class::Paladin)
            }
            ItemClassAndSubClass::OneHandedSword => {
                matches!(
                    class,
                    Class::Hunter
                        | Class::Warrior
                        | Class::Mage
                        | Class::Paladin
                        | Class::Rogue
                        | Class::Warlock
                )
            }
            ItemClassAndSubClass::TwoHandedSword => {
                matches!(
                    class,
                    Class::Hunter | Class::Warrior | Class::Mage | Class::Paladin
                )
            }
            ItemClassAndSubClass::Staff => {
                matches!(
                    class,
                    Class::Hunter
                        | Class::Warrior
                        | Class::Druid
                        | Class::Mage
                        | Class::Priest
                        | Class::Warlock
                )
            }
            ItemClassAndSubClass::FistWeapon => {
                matches!(
                    class,
                    Class::Hunter | Class::Warrior | Class::Druid | Class::Rogue
                )
            }
            ItemClassAndSubClass::Dagger => {
                matches!(
                    class,
                    Class::Hunter
                        | Class::Warrior
                        | Class::Druid
                        | Class::Mage
                        | Class::Priest
                        | Class::Rogue
                        | Class::Warlock
                )
            }
            ItemClassAndSubClass::Wand => {
                matches!(class, Class::Mage | Class::Warlock | Class::Priest)
            }

            ItemClassAndSubClass::Crossbow
            | ItemClassAndSubClass::Thrown
            | ItemClassAndSubClass::Bow
            | ItemClassAndSubClass::Gun => {
                matches!(class, Class::Warrior | Class::Rogue | Class::Hunter)
            }

            ItemClassAndSubClass::ClothArmor => true,
            ItemClassAndSubClass::LeatherArmor => {
                !matches!(class, Class::Mage | Class::Warlock | Class::Priest)
            }
            ItemClassAndSubClass::MailArmor => matches!(
                class,
                Class::Paladin | Class::Warrior | Class::Hunter | Class::Shaman
            ),
            ItemClassAndSubClass::PlateArmor => matches!(class, Class::Paladin | Class::Warrior),

            ItemClassAndSubClass::Shield => {
                matches!(class, Class::Warrior | Class::Paladin | Class::Shaman)
            }
            ItemClassAndSubClass::Libram => matches!(class, Class::Paladin),
            ItemClassAndSubClass::Idol => matches!(class, Class::Druid),
            ItemClassAndSubClass::Totem => matches!(class, Class::Shaman),

            ItemClassAndSubClass::FishingPole
            | ItemClassAndSubClass::Spear
            | ItemClassAndSubClass::MiscellaneousWeapon
            | ItemClassAndSubClass::MiscellaneousArmor => true,
            // cmangos has no items with these types
            ItemClassAndSubClass::OneHandedExotic | ItemClassAndSubClass::TwoHandedExotic => false,
        };

        let inventory_type_is_equip = !matches!(self.inventory_type(), InventoryType::NonEquip);

        allowed_by_class && allowed_by_race && equip_allowed && inventory_type_is_equip
    }
}
