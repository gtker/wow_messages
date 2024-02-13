/// This is both the class field and the sub class field.
/// They are combined into one enum because the value of the sub class depends on the value of the class.
/// The high bits of this are the class field since they are both sent as little endian.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/item_class_and_sub_class.wowm:4`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/item_class_and_sub_class.wowm#L4):
/// ```text
/// enum ItemClassAndSubClass : u64 {
///     CONSUMABLE = 0x0000000000000000;
///     CHEESE_OR_BREAD_OBSOLETE = 0x0000000100000000;
///     LIQUID_OBSOLETE = 0x0000000200000000;
///     BAG = 0x0000000000000001;
///     SOUL_BAG = 0x0000000100000001;
///     HERB_BAG = 0x0000000200000001;
///     ENCHANTING_BAG = 0x0000000300000001;
///     ENGINEERING_BAG = 0x0000000400000001;
///     ONE_HANDED_AXE = 0x0000000000000002;
///     TWO_HANDED_AXE = 0x0000000100000002;
///     BOW = 0x0000000200000002;
///     GUN = 0x0000000300000002;
///     ONE_HANDED_MACE = 0x0000000400000002;
///     TWO_HANDED_MACE = 0x0000000500000002;
///     POLEARM = 0x0000000600000002;
///     ONE_HANDED_SWORD = 0x0000000700000002;
///     TWO_HANDED_SWORD = 0x0000000800000002;
///     OBSOLETE_WEAPON = 0x0000000900000002;
///     STAFF = 0x0000000A00000002;
///     ONE_HANDED_EXOTIC = 0x0000000B00000002;
///     TWO_HANDED_EXOTIC = 0x0000000C00000002;
///     FIST_WEAPON = 0x0000000D00000002;
///     MISCELLANEOUS_WEAPON = 0x0000000E00000002;
///     DAGGER = 0x0000000F00000002;
///     THROWN = 0x0000001000000002;
///     SPEAR = 0x0000001100000002;
///     CROSSBOW = 0x0000001200000002;
///     WAND = 0x0000001300000002;
///     FISHING_POLE = 0x0000001400000002;
///     JEWELRY_OBSOLETE = 0x0000000000000003;
///     MISCELLANEOUS_ARMOR = 0x0000000000000004;
///     CLOTH_ARMOR = 0x0000000100000004;
///     LEATHER_ARMOR = 0x0000000200000004;
///     MAIL_ARMOR = 0x0000000300000004;
///     PLATE_ARMOR = 0x0000000400000004;
///     BUCKLER_OBSOLETE = 0x0000000500000004;
///     SHIELD = 0x0000000600000004;
///     LIBRAM = 0x0000000700000004;
///     IDOL = 0x0000000800000004;
///     TOTEM = 0x0000000900000004;
///     REAGENT = 0x0000000000000005;
///     WAND_OBSOLETE = 0x0000000000000006;
///     BOLT_OBSOLETE = 0x0000000100000006;
///     ARROW = 0x0000000200000006;
///     BULLET = 0x0000000300000006;
///     THROWN_OBSOLETE = 0x0000000400000006;
///     TRADE_GOOD = 0x0000000000000007;
///     PART_TRADE_GOOD = 0x0000000100000007;
///     EXPLOSIVE_TRADE_GOOD = 0x0000000200000007;
///     DEVICE_TRADE_GOOD = 0x0000000300000007;
///     GENERIC_OBSOLETE = 0x0000000000000008;
///     BOOK = 0x0000000000000009;
///     LEATHERWORKING_RECIPE = 0x0000000100000009;
///     TAILORING_RECIPE = 0x0000000200000009;
///     ENGINEERING_RECIPE = 0x0000000300000009;
///     BLACKSMITHING_RECIPE = 0x0000000400000009;
///     COOKING_RECIPE = 0x0000000500000009;
///     ALCHEMY_RECIPE = 0x0000000600000009;
///     FIRST_AID_RECIPE = 0x0000000700000009;
///     ENCHANTING_RECIPE = 0x0000000800000009;
///     FISHING_RECIPE = 0x0000000900000009;
///     MONEY_OBSOLETE = 0x000000000000000A;
///     QUIVER_OBSOLETE = 0x000000000000000B;
///     QUIVER_OBSOLETE1 = 0x000000010000000B;
///     QUIVER = 0x000000020000000B;
///     AMMO_POUCH = 0x000000030000000B;
///     QUEST = 0x000000000000000C;
///     KEY = 0x000000000000000D;
///     LOCKPICK = 0x000000010000000D;
///     PERMANENT = 0x000000000000000E;
///     JUNK = 0x000000000000000F;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemClassAndSubClass {
    Consumable,
    CheeseOrBreadObsolete,
    LiquidObsolete,
    Bag,
    SoulBag,
    HerbBag,
    EnchantingBag,
    EngineeringBag,
    OneHandedAxe,
    TwoHandedAxe,
    Bow,
    Gun,
    OneHandedMace,
    TwoHandedMace,
    Polearm,
    OneHandedSword,
    TwoHandedSword,
    ObsoleteWeapon,
    Staff,
    OneHandedExotic,
    TwoHandedExotic,
    FistWeapon,
    MiscellaneousWeapon,
    Dagger,
    Thrown,
    Spear,
    Crossbow,
    Wand,
    FishingPole,
    JewelryObsolete,
    MiscellaneousArmor,
    ClothArmor,
    LeatherArmor,
    MailArmor,
    PlateArmor,
    BucklerObsolete,
    Shield,
    Libram,
    Idol,
    Totem,
    Reagent,
    WandObsolete,
    BoltObsolete,
    Arrow,
    Bullet,
    ThrownObsolete,
    TradeGood,
    PartTradeGood,
    ExplosiveTradeGood,
    DeviceTradeGood,
    GenericObsolete,
    Book,
    LeatherworkingRecipe,
    TailoringRecipe,
    EngineeringRecipe,
    BlacksmithingRecipe,
    CookingRecipe,
    AlchemyRecipe,
    FirstAidRecipe,
    EnchantingRecipe,
    FishingRecipe,
    MoneyObsolete,
    QuiverObsolete,
    QuiverObsolete1,
    Quiver,
    AmmoPouch,
    Quest,
    Key,
    Lockpick,
    Permanent,
    Junk,
}

impl ItemClassAndSubClass {
    pub const fn as_int(&self) -> u64 {
        match self {
            Self::Consumable => 0x0,
            Self::CheeseOrBreadObsolete => 0x100000000,
            Self::LiquidObsolete => 0x200000000,
            Self::Bag => 0x1,
            Self::SoulBag => 0x100000001,
            Self::HerbBag => 0x200000001,
            Self::EnchantingBag => 0x300000001,
            Self::EngineeringBag => 0x400000001,
            Self::OneHandedAxe => 0x2,
            Self::TwoHandedAxe => 0x100000002,
            Self::Bow => 0x200000002,
            Self::Gun => 0x300000002,
            Self::OneHandedMace => 0x400000002,
            Self::TwoHandedMace => 0x500000002,
            Self::Polearm => 0x600000002,
            Self::OneHandedSword => 0x700000002,
            Self::TwoHandedSword => 0x800000002,
            Self::ObsoleteWeapon => 0x900000002,
            Self::Staff => 0xa00000002,
            Self::OneHandedExotic => 0xb00000002,
            Self::TwoHandedExotic => 0xc00000002,
            Self::FistWeapon => 0xd00000002,
            Self::MiscellaneousWeapon => 0xe00000002,
            Self::Dagger => 0xf00000002,
            Self::Thrown => 0x1000000002,
            Self::Spear => 0x1100000002,
            Self::Crossbow => 0x1200000002,
            Self::Wand => 0x1300000002,
            Self::FishingPole => 0x1400000002,
            Self::JewelryObsolete => 0x3,
            Self::MiscellaneousArmor => 0x4,
            Self::ClothArmor => 0x100000004,
            Self::LeatherArmor => 0x200000004,
            Self::MailArmor => 0x300000004,
            Self::PlateArmor => 0x400000004,
            Self::BucklerObsolete => 0x500000004,
            Self::Shield => 0x600000004,
            Self::Libram => 0x700000004,
            Self::Idol => 0x800000004,
            Self::Totem => 0x900000004,
            Self::Reagent => 0x5,
            Self::WandObsolete => 0x6,
            Self::BoltObsolete => 0x100000006,
            Self::Arrow => 0x200000006,
            Self::Bullet => 0x300000006,
            Self::ThrownObsolete => 0x400000006,
            Self::TradeGood => 0x7,
            Self::PartTradeGood => 0x100000007,
            Self::ExplosiveTradeGood => 0x200000007,
            Self::DeviceTradeGood => 0x300000007,
            Self::GenericObsolete => 0x8,
            Self::Book => 0x9,
            Self::LeatherworkingRecipe => 0x100000009,
            Self::TailoringRecipe => 0x200000009,
            Self::EngineeringRecipe => 0x300000009,
            Self::BlacksmithingRecipe => 0x400000009,
            Self::CookingRecipe => 0x500000009,
            Self::AlchemyRecipe => 0x600000009,
            Self::FirstAidRecipe => 0x700000009,
            Self::EnchantingRecipe => 0x800000009,
            Self::FishingRecipe => 0x900000009,
            Self::MoneyObsolete => 0xa,
            Self::QuiverObsolete => 0xb,
            Self::QuiverObsolete1 => 0x10000000b,
            Self::Quiver => 0x20000000b,
            Self::AmmoPouch => 0x30000000b,
            Self::Quest => 0xc,
            Self::Key => 0xd,
            Self::Lockpick => 0x10000000d,
            Self::Permanent => 0xe,
            Self::Junk => 0xf,
        }
    }

    pub const fn variants() -> [Self; 71] {
        [
            Self::Consumable,
            Self::CheeseOrBreadObsolete,
            Self::LiquidObsolete,
            Self::Bag,
            Self::SoulBag,
            Self::HerbBag,
            Self::EnchantingBag,
            Self::EngineeringBag,
            Self::OneHandedAxe,
            Self::TwoHandedAxe,
            Self::Bow,
            Self::Gun,
            Self::OneHandedMace,
            Self::TwoHandedMace,
            Self::Polearm,
            Self::OneHandedSword,
            Self::TwoHandedSword,
            Self::ObsoleteWeapon,
            Self::Staff,
            Self::OneHandedExotic,
            Self::TwoHandedExotic,
            Self::FistWeapon,
            Self::MiscellaneousWeapon,
            Self::Dagger,
            Self::Thrown,
            Self::Spear,
            Self::Crossbow,
            Self::Wand,
            Self::FishingPole,
            Self::JewelryObsolete,
            Self::MiscellaneousArmor,
            Self::ClothArmor,
            Self::LeatherArmor,
            Self::MailArmor,
            Self::PlateArmor,
            Self::BucklerObsolete,
            Self::Shield,
            Self::Libram,
            Self::Idol,
            Self::Totem,
            Self::Reagent,
            Self::WandObsolete,
            Self::BoltObsolete,
            Self::Arrow,
            Self::Bullet,
            Self::ThrownObsolete,
            Self::TradeGood,
            Self::PartTradeGood,
            Self::ExplosiveTradeGood,
            Self::DeviceTradeGood,
            Self::GenericObsolete,
            Self::Book,
            Self::LeatherworkingRecipe,
            Self::TailoringRecipe,
            Self::EngineeringRecipe,
            Self::BlacksmithingRecipe,
            Self::CookingRecipe,
            Self::AlchemyRecipe,
            Self::FirstAidRecipe,
            Self::EnchantingRecipe,
            Self::FishingRecipe,
            Self::MoneyObsolete,
            Self::QuiverObsolete,
            Self::QuiverObsolete1,
            Self::Quiver,
            Self::AmmoPouch,
            Self::Quest,
            Self::Key,
            Self::Lockpick,
            Self::Permanent,
            Self::Junk,
        ]
    }

    pub const fn from_int(value: u64) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Consumable),
            4294967296 => Ok(Self::CheeseOrBreadObsolete),
            8589934592 => Ok(Self::LiquidObsolete),
            1 => Ok(Self::Bag),
            4294967297 => Ok(Self::SoulBag),
            8589934593 => Ok(Self::HerbBag),
            12884901889 => Ok(Self::EnchantingBag),
            17179869185 => Ok(Self::EngineeringBag),
            2 => Ok(Self::OneHandedAxe),
            4294967298 => Ok(Self::TwoHandedAxe),
            8589934594 => Ok(Self::Bow),
            12884901890 => Ok(Self::Gun),
            17179869186 => Ok(Self::OneHandedMace),
            21474836482 => Ok(Self::TwoHandedMace),
            25769803778 => Ok(Self::Polearm),
            30064771074 => Ok(Self::OneHandedSword),
            34359738370 => Ok(Self::TwoHandedSword),
            38654705666 => Ok(Self::ObsoleteWeapon),
            42949672962 => Ok(Self::Staff),
            47244640258 => Ok(Self::OneHandedExotic),
            51539607554 => Ok(Self::TwoHandedExotic),
            55834574850 => Ok(Self::FistWeapon),
            60129542146 => Ok(Self::MiscellaneousWeapon),
            64424509442 => Ok(Self::Dagger),
            68719476738 => Ok(Self::Thrown),
            73014444034 => Ok(Self::Spear),
            77309411330 => Ok(Self::Crossbow),
            81604378626 => Ok(Self::Wand),
            85899345922 => Ok(Self::FishingPole),
            3 => Ok(Self::JewelryObsolete),
            4 => Ok(Self::MiscellaneousArmor),
            4294967300 => Ok(Self::ClothArmor),
            8589934596 => Ok(Self::LeatherArmor),
            12884901892 => Ok(Self::MailArmor),
            17179869188 => Ok(Self::PlateArmor),
            21474836484 => Ok(Self::BucklerObsolete),
            25769803780 => Ok(Self::Shield),
            30064771076 => Ok(Self::Libram),
            34359738372 => Ok(Self::Idol),
            38654705668 => Ok(Self::Totem),
            5 => Ok(Self::Reagent),
            6 => Ok(Self::WandObsolete),
            4294967302 => Ok(Self::BoltObsolete),
            8589934598 => Ok(Self::Arrow),
            12884901894 => Ok(Self::Bullet),
            17179869190 => Ok(Self::ThrownObsolete),
            7 => Ok(Self::TradeGood),
            4294967303 => Ok(Self::PartTradeGood),
            8589934599 => Ok(Self::ExplosiveTradeGood),
            12884901895 => Ok(Self::DeviceTradeGood),
            8 => Ok(Self::GenericObsolete),
            9 => Ok(Self::Book),
            4294967305 => Ok(Self::LeatherworkingRecipe),
            8589934601 => Ok(Self::TailoringRecipe),
            12884901897 => Ok(Self::EngineeringRecipe),
            17179869193 => Ok(Self::BlacksmithingRecipe),
            21474836489 => Ok(Self::CookingRecipe),
            25769803785 => Ok(Self::AlchemyRecipe),
            30064771081 => Ok(Self::FirstAidRecipe),
            34359738377 => Ok(Self::EnchantingRecipe),
            38654705673 => Ok(Self::FishingRecipe),
            10 => Ok(Self::MoneyObsolete),
            11 => Ok(Self::QuiverObsolete),
            4294967307 => Ok(Self::QuiverObsolete1),
            8589934603 => Ok(Self::Quiver),
            12884901899 => Ok(Self::AmmoPouch),
            12 => Ok(Self::Quest),
            13 => Ok(Self::Key),
            4294967309 => Ok(Self::Lockpick),
            14 => Ok(Self::Permanent),
            15 => Ok(Self::Junk),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ItemClassAndSubClass {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Consumable => "CONSUMABLE",
            Self::CheeseOrBreadObsolete => "CHEESE_OR_BREAD_OBSOLETE",
            Self::LiquidObsolete => "LIQUID_OBSOLETE",
            Self::Bag => "BAG",
            Self::SoulBag => "SOUL_BAG",
            Self::HerbBag => "HERB_BAG",
            Self::EnchantingBag => "ENCHANTING_BAG",
            Self::EngineeringBag => "ENGINEERING_BAG",
            Self::OneHandedAxe => "ONE_HANDED_AXE",
            Self::TwoHandedAxe => "TWO_HANDED_AXE",
            Self::Bow => "BOW",
            Self::Gun => "GUN",
            Self::OneHandedMace => "ONE_HANDED_MACE",
            Self::TwoHandedMace => "TWO_HANDED_MACE",
            Self::Polearm => "POLEARM",
            Self::OneHandedSword => "ONE_HANDED_SWORD",
            Self::TwoHandedSword => "TWO_HANDED_SWORD",
            Self::ObsoleteWeapon => "OBSOLETE_WEAPON",
            Self::Staff => "STAFF",
            Self::OneHandedExotic => "ONE_HANDED_EXOTIC",
            Self::TwoHandedExotic => "TWO_HANDED_EXOTIC",
            Self::FistWeapon => "FIST_WEAPON",
            Self::MiscellaneousWeapon => "MISCELLANEOUS_WEAPON",
            Self::Dagger => "DAGGER",
            Self::Thrown => "THROWN",
            Self::Spear => "SPEAR",
            Self::Crossbow => "CROSSBOW",
            Self::Wand => "WAND",
            Self::FishingPole => "FISHING_POLE",
            Self::JewelryObsolete => "JEWELRY_OBSOLETE",
            Self::MiscellaneousArmor => "MISCELLANEOUS_ARMOR",
            Self::ClothArmor => "CLOTH_ARMOR",
            Self::LeatherArmor => "LEATHER_ARMOR",
            Self::MailArmor => "MAIL_ARMOR",
            Self::PlateArmor => "PLATE_ARMOR",
            Self::BucklerObsolete => "BUCKLER_OBSOLETE",
            Self::Shield => "SHIELD",
            Self::Libram => "LIBRAM",
            Self::Idol => "IDOL",
            Self::Totem => "TOTEM",
            Self::Reagent => "REAGENT",
            Self::WandObsolete => "WAND_OBSOLETE",
            Self::BoltObsolete => "BOLT_OBSOLETE",
            Self::Arrow => "ARROW",
            Self::Bullet => "BULLET",
            Self::ThrownObsolete => "THROWN_OBSOLETE",
            Self::TradeGood => "TRADE_GOOD",
            Self::PartTradeGood => "PART_TRADE_GOOD",
            Self::ExplosiveTradeGood => "EXPLOSIVE_TRADE_GOOD",
            Self::DeviceTradeGood => "DEVICE_TRADE_GOOD",
            Self::GenericObsolete => "GENERIC_OBSOLETE",
            Self::Book => "BOOK",
            Self::LeatherworkingRecipe => "LEATHERWORKING_RECIPE",
            Self::TailoringRecipe => "TAILORING_RECIPE",
            Self::EngineeringRecipe => "ENGINEERING_RECIPE",
            Self::BlacksmithingRecipe => "BLACKSMITHING_RECIPE",
            Self::CookingRecipe => "COOKING_RECIPE",
            Self::AlchemyRecipe => "ALCHEMY_RECIPE",
            Self::FirstAidRecipe => "FIRST_AID_RECIPE",
            Self::EnchantingRecipe => "ENCHANTING_RECIPE",
            Self::FishingRecipe => "FISHING_RECIPE",
            Self::MoneyObsolete => "MONEY_OBSOLETE",
            Self::QuiverObsolete => "QUIVER_OBSOLETE",
            Self::QuiverObsolete1 => "QUIVER_OBSOLETE1",
            Self::Quiver => "QUIVER",
            Self::AmmoPouch => "AMMO_POUCH",
            Self::Quest => "QUEST",
            Self::Key => "KEY",
            Self::Lockpick => "LOCKPICK",
            Self::Permanent => "PERMANENT",
            Self::Junk => "JUNK",
        }
    }

}

const NAME: &str = "ItemClassAndSubClass";

impl Default for ItemClassAndSubClass {
    fn default() -> Self {
        Self::Consumable
    }
}

impl std::fmt::Display for ItemClassAndSubClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Consumable => f.write_str("Consumable"),
            Self::CheeseOrBreadObsolete => f.write_str("CheeseOrBreadObsolete"),
            Self::LiquidObsolete => f.write_str("LiquidObsolete"),
            Self::Bag => f.write_str("Bag"),
            Self::SoulBag => f.write_str("SoulBag"),
            Self::HerbBag => f.write_str("HerbBag"),
            Self::EnchantingBag => f.write_str("EnchantingBag"),
            Self::EngineeringBag => f.write_str("EngineeringBag"),
            Self::OneHandedAxe => f.write_str("OneHandedAxe"),
            Self::TwoHandedAxe => f.write_str("TwoHandedAxe"),
            Self::Bow => f.write_str("Bow"),
            Self::Gun => f.write_str("Gun"),
            Self::OneHandedMace => f.write_str("OneHandedMace"),
            Self::TwoHandedMace => f.write_str("TwoHandedMace"),
            Self::Polearm => f.write_str("Polearm"),
            Self::OneHandedSword => f.write_str("OneHandedSword"),
            Self::TwoHandedSword => f.write_str("TwoHandedSword"),
            Self::ObsoleteWeapon => f.write_str("ObsoleteWeapon"),
            Self::Staff => f.write_str("Staff"),
            Self::OneHandedExotic => f.write_str("OneHandedExotic"),
            Self::TwoHandedExotic => f.write_str("TwoHandedExotic"),
            Self::FistWeapon => f.write_str("FistWeapon"),
            Self::MiscellaneousWeapon => f.write_str("MiscellaneousWeapon"),
            Self::Dagger => f.write_str("Dagger"),
            Self::Thrown => f.write_str("Thrown"),
            Self::Spear => f.write_str("Spear"),
            Self::Crossbow => f.write_str("Crossbow"),
            Self::Wand => f.write_str("Wand"),
            Self::FishingPole => f.write_str("FishingPole"),
            Self::JewelryObsolete => f.write_str("JewelryObsolete"),
            Self::MiscellaneousArmor => f.write_str("MiscellaneousArmor"),
            Self::ClothArmor => f.write_str("ClothArmor"),
            Self::LeatherArmor => f.write_str("LeatherArmor"),
            Self::MailArmor => f.write_str("MailArmor"),
            Self::PlateArmor => f.write_str("PlateArmor"),
            Self::BucklerObsolete => f.write_str("BucklerObsolete"),
            Self::Shield => f.write_str("Shield"),
            Self::Libram => f.write_str("Libram"),
            Self::Idol => f.write_str("Idol"),
            Self::Totem => f.write_str("Totem"),
            Self::Reagent => f.write_str("Reagent"),
            Self::WandObsolete => f.write_str("WandObsolete"),
            Self::BoltObsolete => f.write_str("BoltObsolete"),
            Self::Arrow => f.write_str("Arrow"),
            Self::Bullet => f.write_str("Bullet"),
            Self::ThrownObsolete => f.write_str("ThrownObsolete"),
            Self::TradeGood => f.write_str("TradeGood"),
            Self::PartTradeGood => f.write_str("PartTradeGood"),
            Self::ExplosiveTradeGood => f.write_str("ExplosiveTradeGood"),
            Self::DeviceTradeGood => f.write_str("DeviceTradeGood"),
            Self::GenericObsolete => f.write_str("GenericObsolete"),
            Self::Book => f.write_str("Book"),
            Self::LeatherworkingRecipe => f.write_str("LeatherworkingRecipe"),
            Self::TailoringRecipe => f.write_str("TailoringRecipe"),
            Self::EngineeringRecipe => f.write_str("EngineeringRecipe"),
            Self::BlacksmithingRecipe => f.write_str("BlacksmithingRecipe"),
            Self::CookingRecipe => f.write_str("CookingRecipe"),
            Self::AlchemyRecipe => f.write_str("AlchemyRecipe"),
            Self::FirstAidRecipe => f.write_str("FirstAidRecipe"),
            Self::EnchantingRecipe => f.write_str("EnchantingRecipe"),
            Self::FishingRecipe => f.write_str("FishingRecipe"),
            Self::MoneyObsolete => f.write_str("MoneyObsolete"),
            Self::QuiverObsolete => f.write_str("QuiverObsolete"),
            Self::QuiverObsolete1 => f.write_str("QuiverObsolete1"),
            Self::Quiver => f.write_str("Quiver"),
            Self::AmmoPouch => f.write_str("AmmoPouch"),
            Self::Quest => f.write_str("Quest"),
            Self::Key => f.write_str("Key"),
            Self::Lockpick => f.write_str("Lockpick"),
            Self::Permanent => f.write_str("Permanent"),
            Self::Junk => f.write_str("Junk"),
        }
    }
}

impl TryFrom<u64> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u32> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<i8> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u64>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u64>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u64>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<usize> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u64>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

