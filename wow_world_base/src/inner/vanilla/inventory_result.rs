/// Any values greater than maximum enum value show as 'bag full'
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status.wowm:182`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status.wowm#L182):
/// ```text
/// enum InventoryResult : u8 {
///     OK = 0;
///     CANT_EQUIP_LEVEL_I = 1;
///     CANT_EQUIP_SKILL = 2;
///     ITEM_DOESNT_GO_TO_SLOT = 3;
///     BAG_FULL = 4;
///     NONEMPTY_BAG_OVER_OTHER_BAG = 5;
///     CANT_TRADE_EQUIP_BAGS = 6;
///     ONLY_AMMO_CAN_GO_HERE = 7;
///     NO_REQUIRED_PROFICIENCY = 8;
///     NO_EQUIPMENT_SLOT_AVAILABLE = 9;
///     YOU_CAN_NEVER_USE_THAT_ITEM = 10;
///     YOU_CAN_NEVER_USE_THAT_ITEM2 = 11;
///     NO_EQUIPMENT_SLOT_AVAILABLE2 = 12;
///     CANT_EQUIP_WITH_TWOHANDED = 13;
///     CANT_DUAL_WIELD = 14;
///     ITEM_DOESNT_GO_INTO_BAG = 15;
///     ITEM_DOESNT_GO_INTO_BAG2 = 16;
///     CANT_CARRY_MORE_OF_THIS = 17;
///     NO_EQUIPMENT_SLOT_AVAILABLE3 = 18;
///     ITEM_CANT_STACK = 19;
///     ITEM_CANT_BE_EQUIPPED = 20;
///     ITEMS_CANT_BE_SWAPPED = 21;
///     SLOT_IS_EMPTY = 22;
///     ITEM_NOT_FOUND = 23;
///     CANT_DROP_SOULBOUND = 24;
///     OUT_OF_RANGE = 25;
///     TRIED_TO_SPLIT_MORE_THAN_COUNT = 26;
///     COULDNT_SPLIT_ITEMS = 27;
///     MISSING_REAGENT = 28;
///     NOT_ENOUGH_MONEY = 29;
///     NOT_A_BAG = 30;
///     CAN_ONLY_DO_WITH_EMPTY_BAGS = 31;
///     DONT_OWN_THAT_ITEM = 32;
///     CAN_EQUIP_ONLY1_QUIVER = 33;
///     MUST_PURCHASE_THAT_BAG_SLOT = 34;
///     TOO_FAR_AWAY_FROM_BANK = 35;
///     ITEM_LOCKED = 36;
///     YOU_ARE_STUNNED = 37;
///     YOU_ARE_DEAD = 38;
///     CANT_DO_RIGHT_NOW = 39;
///     INT_BAG_ERROR = 40;
///     CAN_EQUIP_ONLY1_BOLT = 41;
///     CAN_EQUIP_ONLY1_AMMOPOUCH = 42;
///     STACKABLE_CANT_BE_WRAPPED = 43;
///     EQUIPPED_CANT_BE_WRAPPED = 44;
///     WRAPPED_CANT_BE_WRAPPED = 45;
///     BOUND_CANT_BE_WRAPPED = 46;
///     UNIQUE_CANT_BE_WRAPPED = 47;
///     BAGS_CANT_BE_WRAPPED = 48;
///     ALREADY_LOOTED = 49;
///     INVENTORY_FULL = 50;
///     BANK_FULL = 51;
///     ITEM_IS_CURRENTLY_SOLD_OUT = 52;
///     BAG_FULL3 = 53;
///     ITEM_NOT_FOUND2 = 54;
///     ITEM_CANT_STACK2 = 55;
///     BAG_FULL4 = 56;
///     ITEM_SOLD_OUT = 57;
///     OBJECT_IS_BUSY = 58;
///     NONE = 59;
///     NOT_IN_COMBAT = 60;
///     NOT_WHILE_DISARMED = 61;
///     BAG_FULL6 = 62;
///     CANT_EQUIP_RANK = 63;
///     CANT_EQUIP_REPUTATION = 64;
///     TOO_MANY_SPECIAL_BAGS = 65;
///     LOOT_CANT_LOOT_THAT_NOW = 66;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum InventoryResult {
    Ok,
    CantEquipLevelI,
    CantEquipSkill,
    ItemDoesntGoToSlot,
    BagFull,
    NonemptyBagOverOtherBag,
    CantTradeEquipBags,
    OnlyAmmoCanGoHere,
    NoRequiredProficiency,
    NoEquipmentSlotAvailable,
    YouCanNeverUseThatItem,
    YouCanNeverUseThatItem2,
    NoEquipmentSlotAvailable2,
    CantEquipWithTwohanded,
    CantDualWield,
    ItemDoesntGoIntoBag,
    ItemDoesntGoIntoBag2,
    CantCarryMoreOfThis,
    NoEquipmentSlotAvailable3,
    ItemCantStack,
    ItemCantBeEquipped,
    ItemsCantBeSwapped,
    SlotIsEmpty,
    ItemNotFound,
    CantDropSoulbound,
    OutOfRange,
    TriedToSplitMoreThanCount,
    CouldntSplitItems,
    MissingReagent,
    NotEnoughMoney,
    NotABag,
    CanOnlyDoWithEmptyBags,
    DontOwnThatItem,
    CanEquipOnly1Quiver,
    MustPurchaseThatBagSlot,
    TooFarAwayFromBank,
    ItemLocked,
    YouAreStunned,
    YouAreDead,
    CantDoRightNow,
    IntBagError,
    CanEquipOnly1Bolt,
    CanEquipOnly1Ammopouch,
    StackableCantBeWrapped,
    EquippedCantBeWrapped,
    WrappedCantBeWrapped,
    BoundCantBeWrapped,
    UniqueCantBeWrapped,
    BagsCantBeWrapped,
    AlreadyLooted,
    InventoryFull,
    BankFull,
    ItemIsCurrentlySoldOut,
    BagFull3,
    ItemNotFound2,
    ItemCantStack2,
    BagFull4,
    ItemSoldOut,
    ObjectIsBusy,
    None,
    NotInCombat,
    NotWhileDisarmed,
    BagFull6,
    CantEquipRank,
    CantEquipReputation,
    TooManySpecialBags,
    LootCantLootThatNow,
}

impl InventoryResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Ok => 0x0,
            Self::CantEquipLevelI => 0x1,
            Self::CantEquipSkill => 0x2,
            Self::ItemDoesntGoToSlot => 0x3,
            Self::BagFull => 0x4,
            Self::NonemptyBagOverOtherBag => 0x5,
            Self::CantTradeEquipBags => 0x6,
            Self::OnlyAmmoCanGoHere => 0x7,
            Self::NoRequiredProficiency => 0x8,
            Self::NoEquipmentSlotAvailable => 0x9,
            Self::YouCanNeverUseThatItem => 0xa,
            Self::YouCanNeverUseThatItem2 => 0xb,
            Self::NoEquipmentSlotAvailable2 => 0xc,
            Self::CantEquipWithTwohanded => 0xd,
            Self::CantDualWield => 0xe,
            Self::ItemDoesntGoIntoBag => 0xf,
            Self::ItemDoesntGoIntoBag2 => 0x10,
            Self::CantCarryMoreOfThis => 0x11,
            Self::NoEquipmentSlotAvailable3 => 0x12,
            Self::ItemCantStack => 0x13,
            Self::ItemCantBeEquipped => 0x14,
            Self::ItemsCantBeSwapped => 0x15,
            Self::SlotIsEmpty => 0x16,
            Self::ItemNotFound => 0x17,
            Self::CantDropSoulbound => 0x18,
            Self::OutOfRange => 0x19,
            Self::TriedToSplitMoreThanCount => 0x1a,
            Self::CouldntSplitItems => 0x1b,
            Self::MissingReagent => 0x1c,
            Self::NotEnoughMoney => 0x1d,
            Self::NotABag => 0x1e,
            Self::CanOnlyDoWithEmptyBags => 0x1f,
            Self::DontOwnThatItem => 0x20,
            Self::CanEquipOnly1Quiver => 0x21,
            Self::MustPurchaseThatBagSlot => 0x22,
            Self::TooFarAwayFromBank => 0x23,
            Self::ItemLocked => 0x24,
            Self::YouAreStunned => 0x25,
            Self::YouAreDead => 0x26,
            Self::CantDoRightNow => 0x27,
            Self::IntBagError => 0x28,
            Self::CanEquipOnly1Bolt => 0x29,
            Self::CanEquipOnly1Ammopouch => 0x2a,
            Self::StackableCantBeWrapped => 0x2b,
            Self::EquippedCantBeWrapped => 0x2c,
            Self::WrappedCantBeWrapped => 0x2d,
            Self::BoundCantBeWrapped => 0x2e,
            Self::UniqueCantBeWrapped => 0x2f,
            Self::BagsCantBeWrapped => 0x30,
            Self::AlreadyLooted => 0x31,
            Self::InventoryFull => 0x32,
            Self::BankFull => 0x33,
            Self::ItemIsCurrentlySoldOut => 0x34,
            Self::BagFull3 => 0x35,
            Self::ItemNotFound2 => 0x36,
            Self::ItemCantStack2 => 0x37,
            Self::BagFull4 => 0x38,
            Self::ItemSoldOut => 0x39,
            Self::ObjectIsBusy => 0x3a,
            Self::None => 0x3b,
            Self::NotInCombat => 0x3c,
            Self::NotWhileDisarmed => 0x3d,
            Self::BagFull6 => 0x3e,
            Self::CantEquipRank => 0x3f,
            Self::CantEquipReputation => 0x40,
            Self::TooManySpecialBags => 0x41,
            Self::LootCantLootThatNow => 0x42,
        }
    }

    pub const fn variants() -> [Self; 67] {
        [
            Self::Ok,
            Self::CantEquipLevelI,
            Self::CantEquipSkill,
            Self::ItemDoesntGoToSlot,
            Self::BagFull,
            Self::NonemptyBagOverOtherBag,
            Self::CantTradeEquipBags,
            Self::OnlyAmmoCanGoHere,
            Self::NoRequiredProficiency,
            Self::NoEquipmentSlotAvailable,
            Self::YouCanNeverUseThatItem,
            Self::YouCanNeverUseThatItem2,
            Self::NoEquipmentSlotAvailable2,
            Self::CantEquipWithTwohanded,
            Self::CantDualWield,
            Self::ItemDoesntGoIntoBag,
            Self::ItemDoesntGoIntoBag2,
            Self::CantCarryMoreOfThis,
            Self::NoEquipmentSlotAvailable3,
            Self::ItemCantStack,
            Self::ItemCantBeEquipped,
            Self::ItemsCantBeSwapped,
            Self::SlotIsEmpty,
            Self::ItemNotFound,
            Self::CantDropSoulbound,
            Self::OutOfRange,
            Self::TriedToSplitMoreThanCount,
            Self::CouldntSplitItems,
            Self::MissingReagent,
            Self::NotEnoughMoney,
            Self::NotABag,
            Self::CanOnlyDoWithEmptyBags,
            Self::DontOwnThatItem,
            Self::CanEquipOnly1Quiver,
            Self::MustPurchaseThatBagSlot,
            Self::TooFarAwayFromBank,
            Self::ItemLocked,
            Self::YouAreStunned,
            Self::YouAreDead,
            Self::CantDoRightNow,
            Self::IntBagError,
            Self::CanEquipOnly1Bolt,
            Self::CanEquipOnly1Ammopouch,
            Self::StackableCantBeWrapped,
            Self::EquippedCantBeWrapped,
            Self::WrappedCantBeWrapped,
            Self::BoundCantBeWrapped,
            Self::UniqueCantBeWrapped,
            Self::BagsCantBeWrapped,
            Self::AlreadyLooted,
            Self::InventoryFull,
            Self::BankFull,
            Self::ItemIsCurrentlySoldOut,
            Self::BagFull3,
            Self::ItemNotFound2,
            Self::ItemCantStack2,
            Self::BagFull4,
            Self::ItemSoldOut,
            Self::ObjectIsBusy,
            Self::None,
            Self::NotInCombat,
            Self::NotWhileDisarmed,
            Self::BagFull6,
            Self::CantEquipRank,
            Self::CantEquipReputation,
            Self::TooManySpecialBags,
            Self::LootCantLootThatNow,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::CantEquipLevelI),
            2 => Ok(Self::CantEquipSkill),
            3 => Ok(Self::ItemDoesntGoToSlot),
            4 => Ok(Self::BagFull),
            5 => Ok(Self::NonemptyBagOverOtherBag),
            6 => Ok(Self::CantTradeEquipBags),
            7 => Ok(Self::OnlyAmmoCanGoHere),
            8 => Ok(Self::NoRequiredProficiency),
            9 => Ok(Self::NoEquipmentSlotAvailable),
            10 => Ok(Self::YouCanNeverUseThatItem),
            11 => Ok(Self::YouCanNeverUseThatItem2),
            12 => Ok(Self::NoEquipmentSlotAvailable2),
            13 => Ok(Self::CantEquipWithTwohanded),
            14 => Ok(Self::CantDualWield),
            15 => Ok(Self::ItemDoesntGoIntoBag),
            16 => Ok(Self::ItemDoesntGoIntoBag2),
            17 => Ok(Self::CantCarryMoreOfThis),
            18 => Ok(Self::NoEquipmentSlotAvailable3),
            19 => Ok(Self::ItemCantStack),
            20 => Ok(Self::ItemCantBeEquipped),
            21 => Ok(Self::ItemsCantBeSwapped),
            22 => Ok(Self::SlotIsEmpty),
            23 => Ok(Self::ItemNotFound),
            24 => Ok(Self::CantDropSoulbound),
            25 => Ok(Self::OutOfRange),
            26 => Ok(Self::TriedToSplitMoreThanCount),
            27 => Ok(Self::CouldntSplitItems),
            28 => Ok(Self::MissingReagent),
            29 => Ok(Self::NotEnoughMoney),
            30 => Ok(Self::NotABag),
            31 => Ok(Self::CanOnlyDoWithEmptyBags),
            32 => Ok(Self::DontOwnThatItem),
            33 => Ok(Self::CanEquipOnly1Quiver),
            34 => Ok(Self::MustPurchaseThatBagSlot),
            35 => Ok(Self::TooFarAwayFromBank),
            36 => Ok(Self::ItemLocked),
            37 => Ok(Self::YouAreStunned),
            38 => Ok(Self::YouAreDead),
            39 => Ok(Self::CantDoRightNow),
            40 => Ok(Self::IntBagError),
            41 => Ok(Self::CanEquipOnly1Bolt),
            42 => Ok(Self::CanEquipOnly1Ammopouch),
            43 => Ok(Self::StackableCantBeWrapped),
            44 => Ok(Self::EquippedCantBeWrapped),
            45 => Ok(Self::WrappedCantBeWrapped),
            46 => Ok(Self::BoundCantBeWrapped),
            47 => Ok(Self::UniqueCantBeWrapped),
            48 => Ok(Self::BagsCantBeWrapped),
            49 => Ok(Self::AlreadyLooted),
            50 => Ok(Self::InventoryFull),
            51 => Ok(Self::BankFull),
            52 => Ok(Self::ItemIsCurrentlySoldOut),
            53 => Ok(Self::BagFull3),
            54 => Ok(Self::ItemNotFound2),
            55 => Ok(Self::ItemCantStack2),
            56 => Ok(Self::BagFull4),
            57 => Ok(Self::ItemSoldOut),
            58 => Ok(Self::ObjectIsBusy),
            59 => Ok(Self::None),
            60 => Ok(Self::NotInCombat),
            61 => Ok(Self::NotWhileDisarmed),
            62 => Ok(Self::BagFull6),
            63 => Ok(Self::CantEquipRank),
            64 => Ok(Self::CantEquipReputation),
            65 => Ok(Self::TooManySpecialBags),
            66 => Ok(Self::LootCantLootThatNow),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl InventoryResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::CantEquipLevelI => "CANT_EQUIP_LEVEL_I",
            Self::CantEquipSkill => "CANT_EQUIP_SKILL",
            Self::ItemDoesntGoToSlot => "ITEM_DOESNT_GO_TO_SLOT",
            Self::BagFull => "BAG_FULL",
            Self::NonemptyBagOverOtherBag => "NONEMPTY_BAG_OVER_OTHER_BAG",
            Self::CantTradeEquipBags => "CANT_TRADE_EQUIP_BAGS",
            Self::OnlyAmmoCanGoHere => "ONLY_AMMO_CAN_GO_HERE",
            Self::NoRequiredProficiency => "NO_REQUIRED_PROFICIENCY",
            Self::NoEquipmentSlotAvailable => "NO_EQUIPMENT_SLOT_AVAILABLE",
            Self::YouCanNeverUseThatItem => "YOU_CAN_NEVER_USE_THAT_ITEM",
            Self::YouCanNeverUseThatItem2 => "YOU_CAN_NEVER_USE_THAT_ITEM2",
            Self::NoEquipmentSlotAvailable2 => "NO_EQUIPMENT_SLOT_AVAILABLE2",
            Self::CantEquipWithTwohanded => "CANT_EQUIP_WITH_TWOHANDED",
            Self::CantDualWield => "CANT_DUAL_WIELD",
            Self::ItemDoesntGoIntoBag => "ITEM_DOESNT_GO_INTO_BAG",
            Self::ItemDoesntGoIntoBag2 => "ITEM_DOESNT_GO_INTO_BAG2",
            Self::CantCarryMoreOfThis => "CANT_CARRY_MORE_OF_THIS",
            Self::NoEquipmentSlotAvailable3 => "NO_EQUIPMENT_SLOT_AVAILABLE3",
            Self::ItemCantStack => "ITEM_CANT_STACK",
            Self::ItemCantBeEquipped => "ITEM_CANT_BE_EQUIPPED",
            Self::ItemsCantBeSwapped => "ITEMS_CANT_BE_SWAPPED",
            Self::SlotIsEmpty => "SLOT_IS_EMPTY",
            Self::ItemNotFound => "ITEM_NOT_FOUND",
            Self::CantDropSoulbound => "CANT_DROP_SOULBOUND",
            Self::OutOfRange => "OUT_OF_RANGE",
            Self::TriedToSplitMoreThanCount => "TRIED_TO_SPLIT_MORE_THAN_COUNT",
            Self::CouldntSplitItems => "COULDNT_SPLIT_ITEMS",
            Self::MissingReagent => "MISSING_REAGENT",
            Self::NotEnoughMoney => "NOT_ENOUGH_MONEY",
            Self::NotABag => "NOT_A_BAG",
            Self::CanOnlyDoWithEmptyBags => "CAN_ONLY_DO_WITH_EMPTY_BAGS",
            Self::DontOwnThatItem => "DONT_OWN_THAT_ITEM",
            Self::CanEquipOnly1Quiver => "CAN_EQUIP_ONLY1_QUIVER",
            Self::MustPurchaseThatBagSlot => "MUST_PURCHASE_THAT_BAG_SLOT",
            Self::TooFarAwayFromBank => "TOO_FAR_AWAY_FROM_BANK",
            Self::ItemLocked => "ITEM_LOCKED",
            Self::YouAreStunned => "YOU_ARE_STUNNED",
            Self::YouAreDead => "YOU_ARE_DEAD",
            Self::CantDoRightNow => "CANT_DO_RIGHT_NOW",
            Self::IntBagError => "INT_BAG_ERROR",
            Self::CanEquipOnly1Bolt => "CAN_EQUIP_ONLY1_BOLT",
            Self::CanEquipOnly1Ammopouch => "CAN_EQUIP_ONLY1_AMMOPOUCH",
            Self::StackableCantBeWrapped => "STACKABLE_CANT_BE_WRAPPED",
            Self::EquippedCantBeWrapped => "EQUIPPED_CANT_BE_WRAPPED",
            Self::WrappedCantBeWrapped => "WRAPPED_CANT_BE_WRAPPED",
            Self::BoundCantBeWrapped => "BOUND_CANT_BE_WRAPPED",
            Self::UniqueCantBeWrapped => "UNIQUE_CANT_BE_WRAPPED",
            Self::BagsCantBeWrapped => "BAGS_CANT_BE_WRAPPED",
            Self::AlreadyLooted => "ALREADY_LOOTED",
            Self::InventoryFull => "INVENTORY_FULL",
            Self::BankFull => "BANK_FULL",
            Self::ItemIsCurrentlySoldOut => "ITEM_IS_CURRENTLY_SOLD_OUT",
            Self::BagFull3 => "BAG_FULL3",
            Self::ItemNotFound2 => "ITEM_NOT_FOUND2",
            Self::ItemCantStack2 => "ITEM_CANT_STACK2",
            Self::BagFull4 => "BAG_FULL4",
            Self::ItemSoldOut => "ITEM_SOLD_OUT",
            Self::ObjectIsBusy => "OBJECT_IS_BUSY",
            Self::None => "NONE",
            Self::NotInCombat => "NOT_IN_COMBAT",
            Self::NotWhileDisarmed => "NOT_WHILE_DISARMED",
            Self::BagFull6 => "BAG_FULL6",
            Self::CantEquipRank => "CANT_EQUIP_RANK",
            Self::CantEquipReputation => "CANT_EQUIP_REPUTATION",
            Self::TooManySpecialBags => "TOO_MANY_SPECIAL_BAGS",
            Self::LootCantLootThatNow => "LOOT_CANT_LOOT_THAT_NOW",
        }
    }

}

const NAME: &str = "InventoryResult";

impl Default for InventoryResult {
    fn default() -> Self {
        Self::Ok
    }
}

impl std::fmt::Display for InventoryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::CantEquipLevelI => f.write_str("CantEquipLevelI"),
            Self::CantEquipSkill => f.write_str("CantEquipSkill"),
            Self::ItemDoesntGoToSlot => f.write_str("ItemDoesntGoToSlot"),
            Self::BagFull => f.write_str("BagFull"),
            Self::NonemptyBagOverOtherBag => f.write_str("NonemptyBagOverOtherBag"),
            Self::CantTradeEquipBags => f.write_str("CantTradeEquipBags"),
            Self::OnlyAmmoCanGoHere => f.write_str("OnlyAmmoCanGoHere"),
            Self::NoRequiredProficiency => f.write_str("NoRequiredProficiency"),
            Self::NoEquipmentSlotAvailable => f.write_str("NoEquipmentSlotAvailable"),
            Self::YouCanNeverUseThatItem => f.write_str("YouCanNeverUseThatItem"),
            Self::YouCanNeverUseThatItem2 => f.write_str("YouCanNeverUseThatItem2"),
            Self::NoEquipmentSlotAvailable2 => f.write_str("NoEquipmentSlotAvailable2"),
            Self::CantEquipWithTwohanded => f.write_str("CantEquipWithTwohanded"),
            Self::CantDualWield => f.write_str("CantDualWield"),
            Self::ItemDoesntGoIntoBag => f.write_str("ItemDoesntGoIntoBag"),
            Self::ItemDoesntGoIntoBag2 => f.write_str("ItemDoesntGoIntoBag2"),
            Self::CantCarryMoreOfThis => f.write_str("CantCarryMoreOfThis"),
            Self::NoEquipmentSlotAvailable3 => f.write_str("NoEquipmentSlotAvailable3"),
            Self::ItemCantStack => f.write_str("ItemCantStack"),
            Self::ItemCantBeEquipped => f.write_str("ItemCantBeEquipped"),
            Self::ItemsCantBeSwapped => f.write_str("ItemsCantBeSwapped"),
            Self::SlotIsEmpty => f.write_str("SlotIsEmpty"),
            Self::ItemNotFound => f.write_str("ItemNotFound"),
            Self::CantDropSoulbound => f.write_str("CantDropSoulbound"),
            Self::OutOfRange => f.write_str("OutOfRange"),
            Self::TriedToSplitMoreThanCount => f.write_str("TriedToSplitMoreThanCount"),
            Self::CouldntSplitItems => f.write_str("CouldntSplitItems"),
            Self::MissingReagent => f.write_str("MissingReagent"),
            Self::NotEnoughMoney => f.write_str("NotEnoughMoney"),
            Self::NotABag => f.write_str("NotABag"),
            Self::CanOnlyDoWithEmptyBags => f.write_str("CanOnlyDoWithEmptyBags"),
            Self::DontOwnThatItem => f.write_str("DontOwnThatItem"),
            Self::CanEquipOnly1Quiver => f.write_str("CanEquipOnly1Quiver"),
            Self::MustPurchaseThatBagSlot => f.write_str("MustPurchaseThatBagSlot"),
            Self::TooFarAwayFromBank => f.write_str("TooFarAwayFromBank"),
            Self::ItemLocked => f.write_str("ItemLocked"),
            Self::YouAreStunned => f.write_str("YouAreStunned"),
            Self::YouAreDead => f.write_str("YouAreDead"),
            Self::CantDoRightNow => f.write_str("CantDoRightNow"),
            Self::IntBagError => f.write_str("IntBagError"),
            Self::CanEquipOnly1Bolt => f.write_str("CanEquipOnly1Bolt"),
            Self::CanEquipOnly1Ammopouch => f.write_str("CanEquipOnly1Ammopouch"),
            Self::StackableCantBeWrapped => f.write_str("StackableCantBeWrapped"),
            Self::EquippedCantBeWrapped => f.write_str("EquippedCantBeWrapped"),
            Self::WrappedCantBeWrapped => f.write_str("WrappedCantBeWrapped"),
            Self::BoundCantBeWrapped => f.write_str("BoundCantBeWrapped"),
            Self::UniqueCantBeWrapped => f.write_str("UniqueCantBeWrapped"),
            Self::BagsCantBeWrapped => f.write_str("BagsCantBeWrapped"),
            Self::AlreadyLooted => f.write_str("AlreadyLooted"),
            Self::InventoryFull => f.write_str("InventoryFull"),
            Self::BankFull => f.write_str("BankFull"),
            Self::ItemIsCurrentlySoldOut => f.write_str("ItemIsCurrentlySoldOut"),
            Self::BagFull3 => f.write_str("BagFull3"),
            Self::ItemNotFound2 => f.write_str("ItemNotFound2"),
            Self::ItemCantStack2 => f.write_str("ItemCantStack2"),
            Self::BagFull4 => f.write_str("BagFull4"),
            Self::ItemSoldOut => f.write_str("ItemSoldOut"),
            Self::ObjectIsBusy => f.write_str("ObjectIsBusy"),
            Self::None => f.write_str("None"),
            Self::NotInCombat => f.write_str("NotInCombat"),
            Self::NotWhileDisarmed => f.write_str("NotWhileDisarmed"),
            Self::BagFull6 => f.write_str("BagFull6"),
            Self::CantEquipRank => f.write_str("CantEquipRank"),
            Self::CantEquipReputation => f.write_str("CantEquipReputation"),
            Self::TooManySpecialBags => f.write_str("TooManySpecialBags"),
            Self::LootCantLootThatNow => f.write_str("LootCantLootThatNow"),
        }
    }
}

impl TryFrom<u8> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

