use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status.wowm:88`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status.wowm#L88):
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
///     ITEM_UNIQUE_EQUIPABLE = 67;
///     VENDOR_MISSING_TURNINS = 68;
///     NOT_ENOUGH_HONOR_POINTS = 69;
///     NOT_ENOUGH_ARENA_POINTS = 70;
///     ITEM_MAX_COUNT_SOCKETED = 71;
///     MAIL_BOUND_ITEM = 72;
///     NO_SPLIT_WHILE_PROSPECTING = 73;
///     ITEM_MAX_COUNT_EQUIPPED_SOCKETED = 75;
///     ITEM_UNIQUE_EQUIPPABLE_SOCKETED = 76;
///     TOO_MUCH_GOLD = 77;
///     NOT_DURING_ARENA_MATCH = 78;
///     CANNOT_TRADE_THAT = 79;
///     PERSONAL_ARENA_RATING_TOO_LOW = 80;
///     EVENT_AUTOEQUIP_BIND_CONFIRM = 81;
///     ARTEFACTS_ONLY_FOR_OWN_CHARACTERS = 82;
///     ITEM_MAX_LIMIT_CATEGORY_COUNT_EXCEEDED = 84;
///     ITEM_MAX_LIMIT_CATEGORY_SOCKETED_EXCEEDED = 85;
///     SCALING_STAT_ITEM_LEVEL_EXCEEDED = 86;
///     PURCHASE_LEVEL_TOO_LOW = 87;
///     CANT_EQUIP_NEED_TALENT = 88;
///     ITEM_MAX_LIMIT_CATEGORY_EQUIPPED_EXCEEDED = 89;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum InventoryResult {
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
    ItemUniqueEquipable,
    VendorMissingTurnins,
    NotEnoughHonorPoints,
    NotEnoughArenaPoints,
    ItemMaxCountSocketed,
    MailBoundItem,
    NoSplitWhileProspecting,
    ItemMaxCountEquippedSocketed,
    ItemUniqueEquippableSocketed,
    TooMuchGold,
    NotDuringArenaMatch,
    CannotTradeThat,
    PersonalArenaRatingTooLow,
    EventAutoequipBindConfirm,
    ArtefactsOnlyForOwnCharacters,
    ItemMaxLimitCategoryCountExceeded,
    ItemMaxLimitCategorySocketedExceeded,
    ScalingStatItemLevelExceeded,
    PurchaseLevelTooLow,
    CantEquipNeedTalent,
    ItemMaxLimitCategoryEquippedExceeded,
}

impl InventoryResult {
    pub(crate) const fn as_int(&self) -> u8 {
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
            Self::ItemUniqueEquipable => 0x43,
            Self::VendorMissingTurnins => 0x44,
            Self::NotEnoughHonorPoints => 0x45,
            Self::NotEnoughArenaPoints => 0x46,
            Self::ItemMaxCountSocketed => 0x47,
            Self::MailBoundItem => 0x48,
            Self::NoSplitWhileProspecting => 0x49,
            Self::ItemMaxCountEquippedSocketed => 0x4b,
            Self::ItemUniqueEquippableSocketed => 0x4c,
            Self::TooMuchGold => 0x4d,
            Self::NotDuringArenaMatch => 0x4e,
            Self::CannotTradeThat => 0x4f,
            Self::PersonalArenaRatingTooLow => 0x50,
            Self::EventAutoequipBindConfirm => 0x51,
            Self::ArtefactsOnlyForOwnCharacters => 0x52,
            Self::ItemMaxLimitCategoryCountExceeded => 0x54,
            Self::ItemMaxLimitCategorySocketedExceeded => 0x55,
            Self::ScalingStatItemLevelExceeded => 0x56,
            Self::PurchaseLevelTooLow => 0x57,
            Self::CantEquipNeedTalent => 0x58,
            Self::ItemMaxLimitCategoryEquippedExceeded => 0x59,
        }
    }

}

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
            Self::ItemUniqueEquipable => f.write_str("ItemUniqueEquipable"),
            Self::VendorMissingTurnins => f.write_str("VendorMissingTurnins"),
            Self::NotEnoughHonorPoints => f.write_str("NotEnoughHonorPoints"),
            Self::NotEnoughArenaPoints => f.write_str("NotEnoughArenaPoints"),
            Self::ItemMaxCountSocketed => f.write_str("ItemMaxCountSocketed"),
            Self::MailBoundItem => f.write_str("MailBoundItem"),
            Self::NoSplitWhileProspecting => f.write_str("NoSplitWhileProspecting"),
            Self::ItemMaxCountEquippedSocketed => f.write_str("ItemMaxCountEquippedSocketed"),
            Self::ItemUniqueEquippableSocketed => f.write_str("ItemUniqueEquippableSocketed"),
            Self::TooMuchGold => f.write_str("TooMuchGold"),
            Self::NotDuringArenaMatch => f.write_str("NotDuringArenaMatch"),
            Self::CannotTradeThat => f.write_str("CannotTradeThat"),
            Self::PersonalArenaRatingTooLow => f.write_str("PersonalArenaRatingTooLow"),
            Self::EventAutoequipBindConfirm => f.write_str("EventAutoequipBindConfirm"),
            Self::ArtefactsOnlyForOwnCharacters => f.write_str("ArtefactsOnlyForOwnCharacters"),
            Self::ItemMaxLimitCategoryCountExceeded => f.write_str("ItemMaxLimitCategoryCountExceeded"),
            Self::ItemMaxLimitCategorySocketedExceeded => f.write_str("ItemMaxLimitCategorySocketedExceeded"),
            Self::ScalingStatItemLevelExceeded => f.write_str("ScalingStatItemLevelExceeded"),
            Self::PurchaseLevelTooLow => f.write_str("PurchaseLevelTooLow"),
            Self::CantEquipNeedTalent => f.write_str("CantEquipNeedTalent"),
            Self::ItemMaxLimitCategoryEquippedExceeded => f.write_str("ItemMaxLimitCategoryEquippedExceeded"),
        }
    }
}

impl TryFrom<u8> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
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
            67 => Ok(Self::ItemUniqueEquipable),
            68 => Ok(Self::VendorMissingTurnins),
            69 => Ok(Self::NotEnoughHonorPoints),
            70 => Ok(Self::NotEnoughArenaPoints),
            71 => Ok(Self::ItemMaxCountSocketed),
            72 => Ok(Self::MailBoundItem),
            73 => Ok(Self::NoSplitWhileProspecting),
            75 => Ok(Self::ItemMaxCountEquippedSocketed),
            76 => Ok(Self::ItemUniqueEquippableSocketed),
            77 => Ok(Self::TooMuchGold),
            78 => Ok(Self::NotDuringArenaMatch),
            79 => Ok(Self::CannotTradeThat),
            80 => Ok(Self::PersonalArenaRatingTooLow),
            81 => Ok(Self::EventAutoequipBindConfirm),
            82 => Ok(Self::ArtefactsOnlyForOwnCharacters),
            84 => Ok(Self::ItemMaxLimitCategoryCountExceeded),
            85 => Ok(Self::ItemMaxLimitCategorySocketedExceeded),
            86 => Ok(Self::ScalingStatItemLevelExceeded),
            87 => Ok(Self::PurchaseLevelTooLow),
            88 => Ok(Self::CantEquipNeedTalent),
            89 => Ok(Self::ItemMaxLimitCategoryEquippedExceeded),
            v => Err(crate::errors::EnumError::new("InventoryResult", v as u32),)
        }
    }
}

