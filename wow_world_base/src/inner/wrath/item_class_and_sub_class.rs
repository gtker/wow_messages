/// This is both the class field and the sub class field.
/// They are combined into one enum because the value of the sub class depends on the value of the class.
/// The high bits of this are the class field since they are both sent as little endian.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/item_class_and_sub_class.wowm:197`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/item_class_and_sub_class.wowm#L197):
/// ```text
/// enum ItemClassAndSubClass : u64 {
///     CONSUMABLE = 0x0000000000000000;
///     FOOD_AND_DRINK = 0x0000000500000000;
///     POTION = 0x0000000100000000;
///     ELIXIR = 0x0000000200000000;
///     FLASK = 0x0000000300000000;
///     BANDAGE = 0x0000000700000000;
///     ITEM_ENHANCEMENT = 0x0000000600000000;
///     SCROLL = 0x0000000400000000;
///     OTHER_CONSUMABLE = 0x0000000800000000;
///     BAG = 0x0000000000000001;
///     SOUL_BAG = 0x0000000100000001;
///     HERB_BAG = 0x0000000200000001;
///     ENCHANTING_BAG = 0x0000000300000001;
///     ENGINEERING_BAG = 0x0000000400000001;
///     GEM_BAG = 0x0000000500000001;
///     MINING_BAG = 0x0000000600000001;
///     LEATHERWORKING_BAG = 0x0000000700000001;
///     INSCRIPTION_BAG = 0x0000000800000001;
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
///     STAFF = 0x0000000a00000002;
///     ONE_HANDED_EXOTIC = 0x0000000b00000002;
///     TWO_HANDED_EXOTIC = 0x0000000c00000002;
///     FIST_WEAPON = 0x0000000d00000002;
///     MISCELLANEOUS_WEAPON = 0x0000000e00000002;
///     DAGGER = 0x0000000f00000002;
///     THROWN = 0x0000001000000002;
///     SPEAR = 0x0000001100000002;
///     CROSSBOW = 0x0000001200000002;
///     WAND = 0x0000001300000002;
///     FISHING_POLE = 0x0000001400000002;
///     GEM_RED = 0x0000000000000003;
///     GEM_BLUE = 0x0000000100000003;
///     GEM_YELLOW = 0x0000000200000003;
///     GEM_PURPLE = 0x0000000300000003;
///     GEM_GREEN = 0x0000000400000003;
///     GEM_ORANGE = 0x0000000500000003;
///     GEM_META = 0x0000000600000003;
///     GEM_SIMPLE = 0x0000000700000003;
///     GEM_PRISMATIC = 0x0000000800000003;
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
///     SIGIL = 0x0000000a00000004;
///     REAGENT = 0x0000000000000005;
///     WAND_OBSOLETE = 0x0000000000000006;
///     BOLT_OBSOLETE = 0x0000000100000006;
///     ARROW = 0x0000000200000006;
///     BULLET = 0x0000000300000006;
///     THROWN_OBSOLETE = 0x0000000400000006;
///     TRADE_GOOD = 0x0000000000000007;
///     ELEMENTAL_TRADE_GOOD = 0x0000000a00000007;
///     CLOTH_TRADE_GOOD = 0x0000000500000007;
///     LEATHER_TRADE_GOOD = 0x0000000600000007;
///     METAL_AND_STONE_TRADE_GOOD = 0x0000000700000007;
///     MEAT_TRADE_GOOD = 0x0000000800000007;
///     HERB_TRADE_GOOD = 0x0000000900000007;
///     ENCHANTING_TRADE_GOOD = 0x0000000c00000007;
///     JEWELCRAFTING_TRADE_GOOD = 0x0000000400000007;
///     PART_TRADE_GOOD = 0x0000000100000007;
///     DEVICE_TRADE_GOOD = 0x0000000300000007;
///     EXPLOSIVE_TRADE_GOOD = 0x0000000200000007;
///     MATERIAL_TRADE_GOOD = 0x0000000d00000007;
///     OTHER_TRADE_GOOD = 0x0000000b00000007;
///     ARMOR_ENCHANTMENT_TRADE_GOOD = 0x0000000e00000007;
///     WEAPON_ENCHANTMENT_TRADE_GOOD = 0x0000000f00000007;
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
///     JEWELCRAFTING_RECIPE = 0x0000000a00000009;
///     INSCRIPTION_RECIPE = 0x0000000b00000009;
///     MONEY_OBSOLETE = 0x000000000000000a;
///     QUIVER_OBSOLETE = 0x000000000000000b;
///     QUIVER_OBSOLETE1 = 0x000000010000000b;
///     QUIVER = 0x000000020000000b;
///     AMMO_POUCH = 0x000000030000000b;
///     QUEST = 0x000000000000000c;
///     KEY = 0x000000000000000d;
///     LOCKPICK = 0x000000010000000d;
///     PERMANENT_OBSOLETE = 0x000000000000000e;
///     MISCELLANEOUS_JUNK = 0x000000000000000f;
///     MISCELLANEOUS_REAGENT = 0x000000010000000f;
///     MISCELLANEOUS_PET = 0x000000020000000f;
///     MISCELLANEOUS_HOLIDAY = 0x000000030000000f;
///     MISCELLANEOUS_OTHER = 0x000000040000000f;
///     MISCELLANEOUS_MOUNT = 0x000000050000000f;
///     WARRIOR_GLYPH = 0x0000000100000010;
///     PALADIN_GLYPH = 0x0000000200000010;
///     HUNTER_GLYPH = 0x0000000300000010;
///     ROGUE_GLYPH = 0x0000000400000010;
///     PRIEST_GLYPH = 0x0000000500000010;
///     DEATH_KNIGHT_GLYPH = 0x0000000600000010;
///     SHAMAN_GLYPH = 0x0000000700000010;
///     MAGE_GLYPH = 0x0000000800000010;
///     WARLOCK_GLYPH = 0x0000000900000010;
///     DRUID_GLYPH = 0x0000000b00000010;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemClassAndSubClass {
    Consumable,
    FoodAndDrink,
    Potion,
    Elixir,
    Flask,
    Bandage,
    ItemEnhancement,
    Scroll,
    OtherConsumable,
    Bag,
    SoulBag,
    HerbBag,
    EnchantingBag,
    EngineeringBag,
    GemBag,
    MiningBag,
    LeatherworkingBag,
    InscriptionBag,
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
    GemRed,
    GemBlue,
    GemYellow,
    GemPurple,
    GemGreen,
    GemOrange,
    GemMeta,
    GemSimple,
    GemPrismatic,
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
    Sigil,
    Reagent,
    WandObsolete,
    BoltObsolete,
    Arrow,
    Bullet,
    ThrownObsolete,
    TradeGood,
    ElementalTradeGood,
    ClothTradeGood,
    LeatherTradeGood,
    MetalAndStoneTradeGood,
    MeatTradeGood,
    HerbTradeGood,
    EnchantingTradeGood,
    JewelcraftingTradeGood,
    PartTradeGood,
    DeviceTradeGood,
    ExplosiveTradeGood,
    MaterialTradeGood,
    OtherTradeGood,
    ArmorEnchantmentTradeGood,
    WeaponEnchantmentTradeGood,
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
    JewelcraftingRecipe,
    InscriptionRecipe,
    MoneyObsolete,
    QuiverObsolete,
    QuiverObsolete1,
    Quiver,
    AmmoPouch,
    Quest,
    Key,
    Lockpick,
    PermanentObsolete,
    MiscellaneousJunk,
    MiscellaneousReagent,
    MiscellaneousPet,
    MiscellaneousHoliday,
    MiscellaneousOther,
    MiscellaneousMount,
    WarriorGlyph,
    PaladinGlyph,
    HunterGlyph,
    RogueGlyph,
    PriestGlyph,
    DeathKnightGlyph,
    ShamanGlyph,
    MageGlyph,
    WarlockGlyph,
    DruidGlyph,
}

impl ItemClassAndSubClass {
    pub const fn as_int(&self) -> u64 {
        match self {
            Self::Consumable => 0x0,
            Self::FoodAndDrink => 0x500000000,
            Self::Potion => 0x100000000,
            Self::Elixir => 0x200000000,
            Self::Flask => 0x300000000,
            Self::Bandage => 0x700000000,
            Self::ItemEnhancement => 0x600000000,
            Self::Scroll => 0x400000000,
            Self::OtherConsumable => 0x800000000,
            Self::Bag => 0x1,
            Self::SoulBag => 0x100000001,
            Self::HerbBag => 0x200000001,
            Self::EnchantingBag => 0x300000001,
            Self::EngineeringBag => 0x400000001,
            Self::GemBag => 0x500000001,
            Self::MiningBag => 0x600000001,
            Self::LeatherworkingBag => 0x700000001,
            Self::InscriptionBag => 0x800000001,
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
            Self::GemRed => 0x3,
            Self::GemBlue => 0x100000003,
            Self::GemYellow => 0x200000003,
            Self::GemPurple => 0x300000003,
            Self::GemGreen => 0x400000003,
            Self::GemOrange => 0x500000003,
            Self::GemMeta => 0x600000003,
            Self::GemSimple => 0x700000003,
            Self::GemPrismatic => 0x800000003,
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
            Self::Sigil => 0xa00000004,
            Self::Reagent => 0x5,
            Self::WandObsolete => 0x6,
            Self::BoltObsolete => 0x100000006,
            Self::Arrow => 0x200000006,
            Self::Bullet => 0x300000006,
            Self::ThrownObsolete => 0x400000006,
            Self::TradeGood => 0x7,
            Self::ElementalTradeGood => 0xa00000007,
            Self::ClothTradeGood => 0x500000007,
            Self::LeatherTradeGood => 0x600000007,
            Self::MetalAndStoneTradeGood => 0x700000007,
            Self::MeatTradeGood => 0x800000007,
            Self::HerbTradeGood => 0x900000007,
            Self::EnchantingTradeGood => 0xc00000007,
            Self::JewelcraftingTradeGood => 0x400000007,
            Self::PartTradeGood => 0x100000007,
            Self::DeviceTradeGood => 0x300000007,
            Self::ExplosiveTradeGood => 0x200000007,
            Self::MaterialTradeGood => 0xd00000007,
            Self::OtherTradeGood => 0xb00000007,
            Self::ArmorEnchantmentTradeGood => 0xe00000007,
            Self::WeaponEnchantmentTradeGood => 0xf00000007,
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
            Self::JewelcraftingRecipe => 0xa00000009,
            Self::InscriptionRecipe => 0xb00000009,
            Self::MoneyObsolete => 0xa,
            Self::QuiverObsolete => 0xb,
            Self::QuiverObsolete1 => 0x10000000b,
            Self::Quiver => 0x20000000b,
            Self::AmmoPouch => 0x30000000b,
            Self::Quest => 0xc,
            Self::Key => 0xd,
            Self::Lockpick => 0x10000000d,
            Self::PermanentObsolete => 0xe,
            Self::MiscellaneousJunk => 0xf,
            Self::MiscellaneousReagent => 0x10000000f,
            Self::MiscellaneousPet => 0x20000000f,
            Self::MiscellaneousHoliday => 0x30000000f,
            Self::MiscellaneousOther => 0x40000000f,
            Self::MiscellaneousMount => 0x50000000f,
            Self::WarriorGlyph => 0x100000010,
            Self::PaladinGlyph => 0x200000010,
            Self::HunterGlyph => 0x300000010,
            Self::RogueGlyph => 0x400000010,
            Self::PriestGlyph => 0x500000010,
            Self::DeathKnightGlyph => 0x600000010,
            Self::ShamanGlyph => 0x700000010,
            Self::MageGlyph => 0x800000010,
            Self::WarlockGlyph => 0x900000010,
            Self::DruidGlyph => 0xb00000010,
        }
    }

    pub const fn variants() -> [Self; 119] {
        [
            Self::Consumable,
            Self::FoodAndDrink,
            Self::Potion,
            Self::Elixir,
            Self::Flask,
            Self::Bandage,
            Self::ItemEnhancement,
            Self::Scroll,
            Self::OtherConsumable,
            Self::Bag,
            Self::SoulBag,
            Self::HerbBag,
            Self::EnchantingBag,
            Self::EngineeringBag,
            Self::GemBag,
            Self::MiningBag,
            Self::LeatherworkingBag,
            Self::InscriptionBag,
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
            Self::GemRed,
            Self::GemBlue,
            Self::GemYellow,
            Self::GemPurple,
            Self::GemGreen,
            Self::GemOrange,
            Self::GemMeta,
            Self::GemSimple,
            Self::GemPrismatic,
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
            Self::Sigil,
            Self::Reagent,
            Self::WandObsolete,
            Self::BoltObsolete,
            Self::Arrow,
            Self::Bullet,
            Self::ThrownObsolete,
            Self::TradeGood,
            Self::ElementalTradeGood,
            Self::ClothTradeGood,
            Self::LeatherTradeGood,
            Self::MetalAndStoneTradeGood,
            Self::MeatTradeGood,
            Self::HerbTradeGood,
            Self::EnchantingTradeGood,
            Self::JewelcraftingTradeGood,
            Self::PartTradeGood,
            Self::DeviceTradeGood,
            Self::ExplosiveTradeGood,
            Self::MaterialTradeGood,
            Self::OtherTradeGood,
            Self::ArmorEnchantmentTradeGood,
            Self::WeaponEnchantmentTradeGood,
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
            Self::JewelcraftingRecipe,
            Self::InscriptionRecipe,
            Self::MoneyObsolete,
            Self::QuiverObsolete,
            Self::QuiverObsolete1,
            Self::Quiver,
            Self::AmmoPouch,
            Self::Quest,
            Self::Key,
            Self::Lockpick,
            Self::PermanentObsolete,
            Self::MiscellaneousJunk,
            Self::MiscellaneousReagent,
            Self::MiscellaneousPet,
            Self::MiscellaneousHoliday,
            Self::MiscellaneousOther,
            Self::MiscellaneousMount,
            Self::WarriorGlyph,
            Self::PaladinGlyph,
            Self::HunterGlyph,
            Self::RogueGlyph,
            Self::PriestGlyph,
            Self::DeathKnightGlyph,
            Self::ShamanGlyph,
            Self::MageGlyph,
            Self::WarlockGlyph,
            Self::DruidGlyph,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl ItemClassAndSubClass {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Consumable => "CONSUMABLE",
            Self::FoodAndDrink => "FOOD_AND_DRINK",
            Self::Potion => "POTION",
            Self::Elixir => "ELIXIR",
            Self::Flask => "FLASK",
            Self::Bandage => "BANDAGE",
            Self::ItemEnhancement => "ITEM_ENHANCEMENT",
            Self::Scroll => "SCROLL",
            Self::OtherConsumable => "OTHER_CONSUMABLE",
            Self::Bag => "BAG",
            Self::SoulBag => "SOUL_BAG",
            Self::HerbBag => "HERB_BAG",
            Self::EnchantingBag => "ENCHANTING_BAG",
            Self::EngineeringBag => "ENGINEERING_BAG",
            Self::GemBag => "GEM_BAG",
            Self::MiningBag => "MINING_BAG",
            Self::LeatherworkingBag => "LEATHERWORKING_BAG",
            Self::InscriptionBag => "INSCRIPTION_BAG",
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
            Self::GemRed => "GEM_RED",
            Self::GemBlue => "GEM_BLUE",
            Self::GemYellow => "GEM_YELLOW",
            Self::GemPurple => "GEM_PURPLE",
            Self::GemGreen => "GEM_GREEN",
            Self::GemOrange => "GEM_ORANGE",
            Self::GemMeta => "GEM_META",
            Self::GemSimple => "GEM_SIMPLE",
            Self::GemPrismatic => "GEM_PRISMATIC",
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
            Self::Sigil => "SIGIL",
            Self::Reagent => "REAGENT",
            Self::WandObsolete => "WAND_OBSOLETE",
            Self::BoltObsolete => "BOLT_OBSOLETE",
            Self::Arrow => "ARROW",
            Self::Bullet => "BULLET",
            Self::ThrownObsolete => "THROWN_OBSOLETE",
            Self::TradeGood => "TRADE_GOOD",
            Self::ElementalTradeGood => "ELEMENTAL_TRADE_GOOD",
            Self::ClothTradeGood => "CLOTH_TRADE_GOOD",
            Self::LeatherTradeGood => "LEATHER_TRADE_GOOD",
            Self::MetalAndStoneTradeGood => "METAL_AND_STONE_TRADE_GOOD",
            Self::MeatTradeGood => "MEAT_TRADE_GOOD",
            Self::HerbTradeGood => "HERB_TRADE_GOOD",
            Self::EnchantingTradeGood => "ENCHANTING_TRADE_GOOD",
            Self::JewelcraftingTradeGood => "JEWELCRAFTING_TRADE_GOOD",
            Self::PartTradeGood => "PART_TRADE_GOOD",
            Self::DeviceTradeGood => "DEVICE_TRADE_GOOD",
            Self::ExplosiveTradeGood => "EXPLOSIVE_TRADE_GOOD",
            Self::MaterialTradeGood => "MATERIAL_TRADE_GOOD",
            Self::OtherTradeGood => "OTHER_TRADE_GOOD",
            Self::ArmorEnchantmentTradeGood => "ARMOR_ENCHANTMENT_TRADE_GOOD",
            Self::WeaponEnchantmentTradeGood => "WEAPON_ENCHANTMENT_TRADE_GOOD",
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
            Self::JewelcraftingRecipe => "JEWELCRAFTING_RECIPE",
            Self::InscriptionRecipe => "INSCRIPTION_RECIPE",
            Self::MoneyObsolete => "MONEY_OBSOLETE",
            Self::QuiverObsolete => "QUIVER_OBSOLETE",
            Self::QuiverObsolete1 => "QUIVER_OBSOLETE1",
            Self::Quiver => "QUIVER",
            Self::AmmoPouch => "AMMO_POUCH",
            Self::Quest => "QUEST",
            Self::Key => "KEY",
            Self::Lockpick => "LOCKPICK",
            Self::PermanentObsolete => "PERMANENT_OBSOLETE",
            Self::MiscellaneousJunk => "MISCELLANEOUS_JUNK",
            Self::MiscellaneousReagent => "MISCELLANEOUS_REAGENT",
            Self::MiscellaneousPet => "MISCELLANEOUS_PET",
            Self::MiscellaneousHoliday => "MISCELLANEOUS_HOLIDAY",
            Self::MiscellaneousOther => "MISCELLANEOUS_OTHER",
            Self::MiscellaneousMount => "MISCELLANEOUS_MOUNT",
            Self::WarriorGlyph => "WARRIOR_GLYPH",
            Self::PaladinGlyph => "PALADIN_GLYPH",
            Self::HunterGlyph => "HUNTER_GLYPH",
            Self::RogueGlyph => "ROGUE_GLYPH",
            Self::PriestGlyph => "PRIEST_GLYPH",
            Self::DeathKnightGlyph => "DEATH_KNIGHT_GLYPH",
            Self::ShamanGlyph => "SHAMAN_GLYPH",
            Self::MageGlyph => "MAGE_GLYPH",
            Self::WarlockGlyph => "WARLOCK_GLYPH",
            Self::DruidGlyph => "DRUID_GLYPH",
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
            Self::FoodAndDrink => f.write_str("FoodAndDrink"),
            Self::Potion => f.write_str("Potion"),
            Self::Elixir => f.write_str("Elixir"),
            Self::Flask => f.write_str("Flask"),
            Self::Bandage => f.write_str("Bandage"),
            Self::ItemEnhancement => f.write_str("ItemEnhancement"),
            Self::Scroll => f.write_str("Scroll"),
            Self::OtherConsumable => f.write_str("OtherConsumable"),
            Self::Bag => f.write_str("Bag"),
            Self::SoulBag => f.write_str("SoulBag"),
            Self::HerbBag => f.write_str("HerbBag"),
            Self::EnchantingBag => f.write_str("EnchantingBag"),
            Self::EngineeringBag => f.write_str("EngineeringBag"),
            Self::GemBag => f.write_str("GemBag"),
            Self::MiningBag => f.write_str("MiningBag"),
            Self::LeatherworkingBag => f.write_str("LeatherworkingBag"),
            Self::InscriptionBag => f.write_str("InscriptionBag"),
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
            Self::GemRed => f.write_str("GemRed"),
            Self::GemBlue => f.write_str("GemBlue"),
            Self::GemYellow => f.write_str("GemYellow"),
            Self::GemPurple => f.write_str("GemPurple"),
            Self::GemGreen => f.write_str("GemGreen"),
            Self::GemOrange => f.write_str("GemOrange"),
            Self::GemMeta => f.write_str("GemMeta"),
            Self::GemSimple => f.write_str("GemSimple"),
            Self::GemPrismatic => f.write_str("GemPrismatic"),
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
            Self::Sigil => f.write_str("Sigil"),
            Self::Reagent => f.write_str("Reagent"),
            Self::WandObsolete => f.write_str("WandObsolete"),
            Self::BoltObsolete => f.write_str("BoltObsolete"),
            Self::Arrow => f.write_str("Arrow"),
            Self::Bullet => f.write_str("Bullet"),
            Self::ThrownObsolete => f.write_str("ThrownObsolete"),
            Self::TradeGood => f.write_str("TradeGood"),
            Self::ElementalTradeGood => f.write_str("ElementalTradeGood"),
            Self::ClothTradeGood => f.write_str("ClothTradeGood"),
            Self::LeatherTradeGood => f.write_str("LeatherTradeGood"),
            Self::MetalAndStoneTradeGood => f.write_str("MetalAndStoneTradeGood"),
            Self::MeatTradeGood => f.write_str("MeatTradeGood"),
            Self::HerbTradeGood => f.write_str("HerbTradeGood"),
            Self::EnchantingTradeGood => f.write_str("EnchantingTradeGood"),
            Self::JewelcraftingTradeGood => f.write_str("JewelcraftingTradeGood"),
            Self::PartTradeGood => f.write_str("PartTradeGood"),
            Self::DeviceTradeGood => f.write_str("DeviceTradeGood"),
            Self::ExplosiveTradeGood => f.write_str("ExplosiveTradeGood"),
            Self::MaterialTradeGood => f.write_str("MaterialTradeGood"),
            Self::OtherTradeGood => f.write_str("OtherTradeGood"),
            Self::ArmorEnchantmentTradeGood => f.write_str("ArmorEnchantmentTradeGood"),
            Self::WeaponEnchantmentTradeGood => f.write_str("WeaponEnchantmentTradeGood"),
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
            Self::JewelcraftingRecipe => f.write_str("JewelcraftingRecipe"),
            Self::InscriptionRecipe => f.write_str("InscriptionRecipe"),
            Self::MoneyObsolete => f.write_str("MoneyObsolete"),
            Self::QuiverObsolete => f.write_str("QuiverObsolete"),
            Self::QuiverObsolete1 => f.write_str("QuiverObsolete1"),
            Self::Quiver => f.write_str("Quiver"),
            Self::AmmoPouch => f.write_str("AmmoPouch"),
            Self::Quest => f.write_str("Quest"),
            Self::Key => f.write_str("Key"),
            Self::Lockpick => f.write_str("Lockpick"),
            Self::PermanentObsolete => f.write_str("PermanentObsolete"),
            Self::MiscellaneousJunk => f.write_str("MiscellaneousJunk"),
            Self::MiscellaneousReagent => f.write_str("MiscellaneousReagent"),
            Self::MiscellaneousPet => f.write_str("MiscellaneousPet"),
            Self::MiscellaneousHoliday => f.write_str("MiscellaneousHoliday"),
            Self::MiscellaneousOther => f.write_str("MiscellaneousOther"),
            Self::MiscellaneousMount => f.write_str("MiscellaneousMount"),
            Self::WarriorGlyph => f.write_str("WarriorGlyph"),
            Self::PaladinGlyph => f.write_str("PaladinGlyph"),
            Self::HunterGlyph => f.write_str("HunterGlyph"),
            Self::RogueGlyph => f.write_str("RogueGlyph"),
            Self::PriestGlyph => f.write_str("PriestGlyph"),
            Self::DeathKnightGlyph => f.write_str("DeathKnightGlyph"),
            Self::ShamanGlyph => f.write_str("ShamanGlyph"),
            Self::MageGlyph => f.write_str("MageGlyph"),
            Self::WarlockGlyph => f.write_str("WarlockGlyph"),
            Self::DruidGlyph => f.write_str("DruidGlyph"),
        }
    }
}

impl TryFrom<u64> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Consumable),
            21474836480 => Ok(Self::FoodAndDrink),
            4294967296 => Ok(Self::Potion),
            8589934592 => Ok(Self::Elixir),
            12884901888 => Ok(Self::Flask),
            30064771072 => Ok(Self::Bandage),
            25769803776 => Ok(Self::ItemEnhancement),
            17179869184 => Ok(Self::Scroll),
            34359738368 => Ok(Self::OtherConsumable),
            1 => Ok(Self::Bag),
            4294967297 => Ok(Self::SoulBag),
            8589934593 => Ok(Self::HerbBag),
            12884901889 => Ok(Self::EnchantingBag),
            17179869185 => Ok(Self::EngineeringBag),
            21474836481 => Ok(Self::GemBag),
            25769803777 => Ok(Self::MiningBag),
            30064771073 => Ok(Self::LeatherworkingBag),
            34359738369 => Ok(Self::InscriptionBag),
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
            3 => Ok(Self::GemRed),
            4294967299 => Ok(Self::GemBlue),
            8589934595 => Ok(Self::GemYellow),
            12884901891 => Ok(Self::GemPurple),
            17179869187 => Ok(Self::GemGreen),
            21474836483 => Ok(Self::GemOrange),
            25769803779 => Ok(Self::GemMeta),
            30064771075 => Ok(Self::GemSimple),
            34359738371 => Ok(Self::GemPrismatic),
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
            42949672964 => Ok(Self::Sigil),
            5 => Ok(Self::Reagent),
            6 => Ok(Self::WandObsolete),
            4294967302 => Ok(Self::BoltObsolete),
            8589934598 => Ok(Self::Arrow),
            12884901894 => Ok(Self::Bullet),
            17179869190 => Ok(Self::ThrownObsolete),
            7 => Ok(Self::TradeGood),
            42949672967 => Ok(Self::ElementalTradeGood),
            21474836487 => Ok(Self::ClothTradeGood),
            25769803783 => Ok(Self::LeatherTradeGood),
            30064771079 => Ok(Self::MetalAndStoneTradeGood),
            34359738375 => Ok(Self::MeatTradeGood),
            38654705671 => Ok(Self::HerbTradeGood),
            51539607559 => Ok(Self::EnchantingTradeGood),
            17179869191 => Ok(Self::JewelcraftingTradeGood),
            4294967303 => Ok(Self::PartTradeGood),
            12884901895 => Ok(Self::DeviceTradeGood),
            8589934599 => Ok(Self::ExplosiveTradeGood),
            55834574855 => Ok(Self::MaterialTradeGood),
            47244640263 => Ok(Self::OtherTradeGood),
            60129542151 => Ok(Self::ArmorEnchantmentTradeGood),
            64424509447 => Ok(Self::WeaponEnchantmentTradeGood),
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
            42949672969 => Ok(Self::JewelcraftingRecipe),
            47244640265 => Ok(Self::InscriptionRecipe),
            10 => Ok(Self::MoneyObsolete),
            11 => Ok(Self::QuiverObsolete),
            4294967307 => Ok(Self::QuiverObsolete1),
            8589934603 => Ok(Self::Quiver),
            12884901899 => Ok(Self::AmmoPouch),
            12 => Ok(Self::Quest),
            13 => Ok(Self::Key),
            4294967309 => Ok(Self::Lockpick),
            14 => Ok(Self::PermanentObsolete),
            15 => Ok(Self::MiscellaneousJunk),
            4294967311 => Ok(Self::MiscellaneousReagent),
            8589934607 => Ok(Self::MiscellaneousPet),
            12884901903 => Ok(Self::MiscellaneousHoliday),
            17179869199 => Ok(Self::MiscellaneousOther),
            21474836495 => Ok(Self::MiscellaneousMount),
            4294967312 => Ok(Self::WarriorGlyph),
            8589934608 => Ok(Self::PaladinGlyph),
            12884901904 => Ok(Self::HunterGlyph),
            17179869200 => Ok(Self::RogueGlyph),
            21474836496 => Ok(Self::PriestGlyph),
            25769803792 => Ok(Self::DeathKnightGlyph),
            30064771088 => Ok(Self::ShamanGlyph),
            34359738384 => Ok(Self::MageGlyph),
            38654705680 => Ok(Self::WarlockGlyph),
            47244640272 => Ok(Self::DruidGlyph),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u64>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u64>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ItemClassAndSubClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u64>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
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
        TryInto::<u64>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
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

