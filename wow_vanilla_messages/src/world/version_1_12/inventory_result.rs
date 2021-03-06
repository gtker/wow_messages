use std::convert::{TryFrom, TryInto};

/// Any values greater than maximum enum value show as 'bag full'
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status.wowm#L3):
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
pub enum InventoryResult {
    OK,
    CANT_EQUIP_LEVEL_I,
    CANT_EQUIP_SKILL,
    ITEM_DOESNT_GO_TO_SLOT,
    BAG_FULL,
    NONEMPTY_BAG_OVER_OTHER_BAG,
    CANT_TRADE_EQUIP_BAGS,
    ONLY_AMMO_CAN_GO_HERE,
    NO_REQUIRED_PROFICIENCY,
    NO_EQUIPMENT_SLOT_AVAILABLE,
    YOU_CAN_NEVER_USE_THAT_ITEM,
    YOU_CAN_NEVER_USE_THAT_ITEM2,
    NO_EQUIPMENT_SLOT_AVAILABLE2,
    CANT_EQUIP_WITH_TWOHANDED,
    CANT_DUAL_WIELD,
    ITEM_DOESNT_GO_INTO_BAG,
    ITEM_DOESNT_GO_INTO_BAG2,
    CANT_CARRY_MORE_OF_THIS,
    NO_EQUIPMENT_SLOT_AVAILABLE3,
    ITEM_CANT_STACK,
    ITEM_CANT_BE_EQUIPPED,
    ITEMS_CANT_BE_SWAPPED,
    SLOT_IS_EMPTY,
    ITEM_NOT_FOUND,
    CANT_DROP_SOULBOUND,
    OUT_OF_RANGE,
    TRIED_TO_SPLIT_MORE_THAN_COUNT,
    COULDNT_SPLIT_ITEMS,
    MISSING_REAGENT,
    NOT_ENOUGH_MONEY,
    NOT_A_BAG,
    CAN_ONLY_DO_WITH_EMPTY_BAGS,
    DONT_OWN_THAT_ITEM,
    CAN_EQUIP_ONLY1_QUIVER,
    MUST_PURCHASE_THAT_BAG_SLOT,
    TOO_FAR_AWAY_FROM_BANK,
    ITEM_LOCKED,
    YOU_ARE_STUNNED,
    YOU_ARE_DEAD,
    CANT_DO_RIGHT_NOW,
    INT_BAG_ERROR,
    CAN_EQUIP_ONLY1_BOLT,
    CAN_EQUIP_ONLY1_AMMOPOUCH,
    STACKABLE_CANT_BE_WRAPPED,
    EQUIPPED_CANT_BE_WRAPPED,
    WRAPPED_CANT_BE_WRAPPED,
    BOUND_CANT_BE_WRAPPED,
    UNIQUE_CANT_BE_WRAPPED,
    BAGS_CANT_BE_WRAPPED,
    ALREADY_LOOTED,
    INVENTORY_FULL,
    BANK_FULL,
    ITEM_IS_CURRENTLY_SOLD_OUT,
    BAG_FULL3,
    ITEM_NOT_FOUND2,
    ITEM_CANT_STACK2,
    BAG_FULL4,
    ITEM_SOLD_OUT,
    OBJECT_IS_BUSY,
    NONE,
    NOT_IN_COMBAT,
    NOT_WHILE_DISARMED,
    BAG_FULL6,
    CANT_EQUIP_RANK,
    CANT_EQUIP_REPUTATION,
    TOO_MANY_SPECIAL_BAGS,
    LOOT_CANT_LOOT_THAT_NOW,
}

impl InventoryResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::OK => 0x0,
            Self::CANT_EQUIP_LEVEL_I => 0x1,
            Self::CANT_EQUIP_SKILL => 0x2,
            Self::ITEM_DOESNT_GO_TO_SLOT => 0x3,
            Self::BAG_FULL => 0x4,
            Self::NONEMPTY_BAG_OVER_OTHER_BAG => 0x5,
            Self::CANT_TRADE_EQUIP_BAGS => 0x6,
            Self::ONLY_AMMO_CAN_GO_HERE => 0x7,
            Self::NO_REQUIRED_PROFICIENCY => 0x8,
            Self::NO_EQUIPMENT_SLOT_AVAILABLE => 0x9,
            Self::YOU_CAN_NEVER_USE_THAT_ITEM => 0xa,
            Self::YOU_CAN_NEVER_USE_THAT_ITEM2 => 0xb,
            Self::NO_EQUIPMENT_SLOT_AVAILABLE2 => 0xc,
            Self::CANT_EQUIP_WITH_TWOHANDED => 0xd,
            Self::CANT_DUAL_WIELD => 0xe,
            Self::ITEM_DOESNT_GO_INTO_BAG => 0xf,
            Self::ITEM_DOESNT_GO_INTO_BAG2 => 0x10,
            Self::CANT_CARRY_MORE_OF_THIS => 0x11,
            Self::NO_EQUIPMENT_SLOT_AVAILABLE3 => 0x12,
            Self::ITEM_CANT_STACK => 0x13,
            Self::ITEM_CANT_BE_EQUIPPED => 0x14,
            Self::ITEMS_CANT_BE_SWAPPED => 0x15,
            Self::SLOT_IS_EMPTY => 0x16,
            Self::ITEM_NOT_FOUND => 0x17,
            Self::CANT_DROP_SOULBOUND => 0x18,
            Self::OUT_OF_RANGE => 0x19,
            Self::TRIED_TO_SPLIT_MORE_THAN_COUNT => 0x1a,
            Self::COULDNT_SPLIT_ITEMS => 0x1b,
            Self::MISSING_REAGENT => 0x1c,
            Self::NOT_ENOUGH_MONEY => 0x1d,
            Self::NOT_A_BAG => 0x1e,
            Self::CAN_ONLY_DO_WITH_EMPTY_BAGS => 0x1f,
            Self::DONT_OWN_THAT_ITEM => 0x20,
            Self::CAN_EQUIP_ONLY1_QUIVER => 0x21,
            Self::MUST_PURCHASE_THAT_BAG_SLOT => 0x22,
            Self::TOO_FAR_AWAY_FROM_BANK => 0x23,
            Self::ITEM_LOCKED => 0x24,
            Self::YOU_ARE_STUNNED => 0x25,
            Self::YOU_ARE_DEAD => 0x26,
            Self::CANT_DO_RIGHT_NOW => 0x27,
            Self::INT_BAG_ERROR => 0x28,
            Self::CAN_EQUIP_ONLY1_BOLT => 0x29,
            Self::CAN_EQUIP_ONLY1_AMMOPOUCH => 0x2a,
            Self::STACKABLE_CANT_BE_WRAPPED => 0x2b,
            Self::EQUIPPED_CANT_BE_WRAPPED => 0x2c,
            Self::WRAPPED_CANT_BE_WRAPPED => 0x2d,
            Self::BOUND_CANT_BE_WRAPPED => 0x2e,
            Self::UNIQUE_CANT_BE_WRAPPED => 0x2f,
            Self::BAGS_CANT_BE_WRAPPED => 0x30,
            Self::ALREADY_LOOTED => 0x31,
            Self::INVENTORY_FULL => 0x32,
            Self::BANK_FULL => 0x33,
            Self::ITEM_IS_CURRENTLY_SOLD_OUT => 0x34,
            Self::BAG_FULL3 => 0x35,
            Self::ITEM_NOT_FOUND2 => 0x36,
            Self::ITEM_CANT_STACK2 => 0x37,
            Self::BAG_FULL4 => 0x38,
            Self::ITEM_SOLD_OUT => 0x39,
            Self::OBJECT_IS_BUSY => 0x3a,
            Self::NONE => 0x3b,
            Self::NOT_IN_COMBAT => 0x3c,
            Self::NOT_WHILE_DISARMED => 0x3d,
            Self::BAG_FULL6 => 0x3e,
            Self::CANT_EQUIP_RANK => 0x3f,
            Self::CANT_EQUIP_REPUTATION => 0x40,
            Self::TOO_MANY_SPECIAL_BAGS => 0x41,
            Self::LOOT_CANT_LOOT_THAT_NOW => 0x42,
        }
    }

}

impl Default for InventoryResult {
    fn default() -> Self {
        Self::OK
    }
}

impl std::fmt::Display for InventoryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => f.write_str("OK"),
            Self::CANT_EQUIP_LEVEL_I => f.write_str("CANT_EQUIP_LEVEL_I"),
            Self::CANT_EQUIP_SKILL => f.write_str("CANT_EQUIP_SKILL"),
            Self::ITEM_DOESNT_GO_TO_SLOT => f.write_str("ITEM_DOESNT_GO_TO_SLOT"),
            Self::BAG_FULL => f.write_str("BAG_FULL"),
            Self::NONEMPTY_BAG_OVER_OTHER_BAG => f.write_str("NONEMPTY_BAG_OVER_OTHER_BAG"),
            Self::CANT_TRADE_EQUIP_BAGS => f.write_str("CANT_TRADE_EQUIP_BAGS"),
            Self::ONLY_AMMO_CAN_GO_HERE => f.write_str("ONLY_AMMO_CAN_GO_HERE"),
            Self::NO_REQUIRED_PROFICIENCY => f.write_str("NO_REQUIRED_PROFICIENCY"),
            Self::NO_EQUIPMENT_SLOT_AVAILABLE => f.write_str("NO_EQUIPMENT_SLOT_AVAILABLE"),
            Self::YOU_CAN_NEVER_USE_THAT_ITEM => f.write_str("YOU_CAN_NEVER_USE_THAT_ITEM"),
            Self::YOU_CAN_NEVER_USE_THAT_ITEM2 => f.write_str("YOU_CAN_NEVER_USE_THAT_ITEM2"),
            Self::NO_EQUIPMENT_SLOT_AVAILABLE2 => f.write_str("NO_EQUIPMENT_SLOT_AVAILABLE2"),
            Self::CANT_EQUIP_WITH_TWOHANDED => f.write_str("CANT_EQUIP_WITH_TWOHANDED"),
            Self::CANT_DUAL_WIELD => f.write_str("CANT_DUAL_WIELD"),
            Self::ITEM_DOESNT_GO_INTO_BAG => f.write_str("ITEM_DOESNT_GO_INTO_BAG"),
            Self::ITEM_DOESNT_GO_INTO_BAG2 => f.write_str("ITEM_DOESNT_GO_INTO_BAG2"),
            Self::CANT_CARRY_MORE_OF_THIS => f.write_str("CANT_CARRY_MORE_OF_THIS"),
            Self::NO_EQUIPMENT_SLOT_AVAILABLE3 => f.write_str("NO_EQUIPMENT_SLOT_AVAILABLE3"),
            Self::ITEM_CANT_STACK => f.write_str("ITEM_CANT_STACK"),
            Self::ITEM_CANT_BE_EQUIPPED => f.write_str("ITEM_CANT_BE_EQUIPPED"),
            Self::ITEMS_CANT_BE_SWAPPED => f.write_str("ITEMS_CANT_BE_SWAPPED"),
            Self::SLOT_IS_EMPTY => f.write_str("SLOT_IS_EMPTY"),
            Self::ITEM_NOT_FOUND => f.write_str("ITEM_NOT_FOUND"),
            Self::CANT_DROP_SOULBOUND => f.write_str("CANT_DROP_SOULBOUND"),
            Self::OUT_OF_RANGE => f.write_str("OUT_OF_RANGE"),
            Self::TRIED_TO_SPLIT_MORE_THAN_COUNT => f.write_str("TRIED_TO_SPLIT_MORE_THAN_COUNT"),
            Self::COULDNT_SPLIT_ITEMS => f.write_str("COULDNT_SPLIT_ITEMS"),
            Self::MISSING_REAGENT => f.write_str("MISSING_REAGENT"),
            Self::NOT_ENOUGH_MONEY => f.write_str("NOT_ENOUGH_MONEY"),
            Self::NOT_A_BAG => f.write_str("NOT_A_BAG"),
            Self::CAN_ONLY_DO_WITH_EMPTY_BAGS => f.write_str("CAN_ONLY_DO_WITH_EMPTY_BAGS"),
            Self::DONT_OWN_THAT_ITEM => f.write_str("DONT_OWN_THAT_ITEM"),
            Self::CAN_EQUIP_ONLY1_QUIVER => f.write_str("CAN_EQUIP_ONLY1_QUIVER"),
            Self::MUST_PURCHASE_THAT_BAG_SLOT => f.write_str("MUST_PURCHASE_THAT_BAG_SLOT"),
            Self::TOO_FAR_AWAY_FROM_BANK => f.write_str("TOO_FAR_AWAY_FROM_BANK"),
            Self::ITEM_LOCKED => f.write_str("ITEM_LOCKED"),
            Self::YOU_ARE_STUNNED => f.write_str("YOU_ARE_STUNNED"),
            Self::YOU_ARE_DEAD => f.write_str("YOU_ARE_DEAD"),
            Self::CANT_DO_RIGHT_NOW => f.write_str("CANT_DO_RIGHT_NOW"),
            Self::INT_BAG_ERROR => f.write_str("INT_BAG_ERROR"),
            Self::CAN_EQUIP_ONLY1_BOLT => f.write_str("CAN_EQUIP_ONLY1_BOLT"),
            Self::CAN_EQUIP_ONLY1_AMMOPOUCH => f.write_str("CAN_EQUIP_ONLY1_AMMOPOUCH"),
            Self::STACKABLE_CANT_BE_WRAPPED => f.write_str("STACKABLE_CANT_BE_WRAPPED"),
            Self::EQUIPPED_CANT_BE_WRAPPED => f.write_str("EQUIPPED_CANT_BE_WRAPPED"),
            Self::WRAPPED_CANT_BE_WRAPPED => f.write_str("WRAPPED_CANT_BE_WRAPPED"),
            Self::BOUND_CANT_BE_WRAPPED => f.write_str("BOUND_CANT_BE_WRAPPED"),
            Self::UNIQUE_CANT_BE_WRAPPED => f.write_str("UNIQUE_CANT_BE_WRAPPED"),
            Self::BAGS_CANT_BE_WRAPPED => f.write_str("BAGS_CANT_BE_WRAPPED"),
            Self::ALREADY_LOOTED => f.write_str("ALREADY_LOOTED"),
            Self::INVENTORY_FULL => f.write_str("INVENTORY_FULL"),
            Self::BANK_FULL => f.write_str("BANK_FULL"),
            Self::ITEM_IS_CURRENTLY_SOLD_OUT => f.write_str("ITEM_IS_CURRENTLY_SOLD_OUT"),
            Self::BAG_FULL3 => f.write_str("BAG_FULL3"),
            Self::ITEM_NOT_FOUND2 => f.write_str("ITEM_NOT_FOUND2"),
            Self::ITEM_CANT_STACK2 => f.write_str("ITEM_CANT_STACK2"),
            Self::BAG_FULL4 => f.write_str("BAG_FULL4"),
            Self::ITEM_SOLD_OUT => f.write_str("ITEM_SOLD_OUT"),
            Self::OBJECT_IS_BUSY => f.write_str("OBJECT_IS_BUSY"),
            Self::NONE => f.write_str("NONE"),
            Self::NOT_IN_COMBAT => f.write_str("NOT_IN_COMBAT"),
            Self::NOT_WHILE_DISARMED => f.write_str("NOT_WHILE_DISARMED"),
            Self::BAG_FULL6 => f.write_str("BAG_FULL6"),
            Self::CANT_EQUIP_RANK => f.write_str("CANT_EQUIP_RANK"),
            Self::CANT_EQUIP_REPUTATION => f.write_str("CANT_EQUIP_REPUTATION"),
            Self::TOO_MANY_SPECIAL_BAGS => f.write_str("TOO_MANY_SPECIAL_BAGS"),
            Self::LOOT_CANT_LOOT_THAT_NOW => f.write_str("LOOT_CANT_LOOT_THAT_NOW"),
        }
    }
}

impl TryFrom<u8> for InventoryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OK),
            1 => Ok(Self::CANT_EQUIP_LEVEL_I),
            2 => Ok(Self::CANT_EQUIP_SKILL),
            3 => Ok(Self::ITEM_DOESNT_GO_TO_SLOT),
            4 => Ok(Self::BAG_FULL),
            5 => Ok(Self::NONEMPTY_BAG_OVER_OTHER_BAG),
            6 => Ok(Self::CANT_TRADE_EQUIP_BAGS),
            7 => Ok(Self::ONLY_AMMO_CAN_GO_HERE),
            8 => Ok(Self::NO_REQUIRED_PROFICIENCY),
            9 => Ok(Self::NO_EQUIPMENT_SLOT_AVAILABLE),
            10 => Ok(Self::YOU_CAN_NEVER_USE_THAT_ITEM),
            11 => Ok(Self::YOU_CAN_NEVER_USE_THAT_ITEM2),
            12 => Ok(Self::NO_EQUIPMENT_SLOT_AVAILABLE2),
            13 => Ok(Self::CANT_EQUIP_WITH_TWOHANDED),
            14 => Ok(Self::CANT_DUAL_WIELD),
            15 => Ok(Self::ITEM_DOESNT_GO_INTO_BAG),
            16 => Ok(Self::ITEM_DOESNT_GO_INTO_BAG2),
            17 => Ok(Self::CANT_CARRY_MORE_OF_THIS),
            18 => Ok(Self::NO_EQUIPMENT_SLOT_AVAILABLE3),
            19 => Ok(Self::ITEM_CANT_STACK),
            20 => Ok(Self::ITEM_CANT_BE_EQUIPPED),
            21 => Ok(Self::ITEMS_CANT_BE_SWAPPED),
            22 => Ok(Self::SLOT_IS_EMPTY),
            23 => Ok(Self::ITEM_NOT_FOUND),
            24 => Ok(Self::CANT_DROP_SOULBOUND),
            25 => Ok(Self::OUT_OF_RANGE),
            26 => Ok(Self::TRIED_TO_SPLIT_MORE_THAN_COUNT),
            27 => Ok(Self::COULDNT_SPLIT_ITEMS),
            28 => Ok(Self::MISSING_REAGENT),
            29 => Ok(Self::NOT_ENOUGH_MONEY),
            30 => Ok(Self::NOT_A_BAG),
            31 => Ok(Self::CAN_ONLY_DO_WITH_EMPTY_BAGS),
            32 => Ok(Self::DONT_OWN_THAT_ITEM),
            33 => Ok(Self::CAN_EQUIP_ONLY1_QUIVER),
            34 => Ok(Self::MUST_PURCHASE_THAT_BAG_SLOT),
            35 => Ok(Self::TOO_FAR_AWAY_FROM_BANK),
            36 => Ok(Self::ITEM_LOCKED),
            37 => Ok(Self::YOU_ARE_STUNNED),
            38 => Ok(Self::YOU_ARE_DEAD),
            39 => Ok(Self::CANT_DO_RIGHT_NOW),
            40 => Ok(Self::INT_BAG_ERROR),
            41 => Ok(Self::CAN_EQUIP_ONLY1_BOLT),
            42 => Ok(Self::CAN_EQUIP_ONLY1_AMMOPOUCH),
            43 => Ok(Self::STACKABLE_CANT_BE_WRAPPED),
            44 => Ok(Self::EQUIPPED_CANT_BE_WRAPPED),
            45 => Ok(Self::WRAPPED_CANT_BE_WRAPPED),
            46 => Ok(Self::BOUND_CANT_BE_WRAPPED),
            47 => Ok(Self::UNIQUE_CANT_BE_WRAPPED),
            48 => Ok(Self::BAGS_CANT_BE_WRAPPED),
            49 => Ok(Self::ALREADY_LOOTED),
            50 => Ok(Self::INVENTORY_FULL),
            51 => Ok(Self::BANK_FULL),
            52 => Ok(Self::ITEM_IS_CURRENTLY_SOLD_OUT),
            53 => Ok(Self::BAG_FULL3),
            54 => Ok(Self::ITEM_NOT_FOUND2),
            55 => Ok(Self::ITEM_CANT_STACK2),
            56 => Ok(Self::BAG_FULL4),
            57 => Ok(Self::ITEM_SOLD_OUT),
            58 => Ok(Self::OBJECT_IS_BUSY),
            59 => Ok(Self::NONE),
            60 => Ok(Self::NOT_IN_COMBAT),
            61 => Ok(Self::NOT_WHILE_DISARMED),
            62 => Ok(Self::BAG_FULL6),
            63 => Ok(Self::CANT_EQUIP_RANK),
            64 => Ok(Self::CANT_EQUIP_REPUTATION),
            65 => Ok(Self::TOO_MANY_SPECIAL_BAGS),
            66 => Ok(Self::LOOT_CANT_LOOT_THAT_NOW),
            v => Err(crate::errors::EnumError::new("InventoryResult", v as u32),)
        }
    }
}

