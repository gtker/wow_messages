/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm#L15):
/// ```text
/// enum ItemSlot : u8 {
///     HEAD = 0;
///     NECK = 1;
///     SHOULDERS = 2;
///     SHIRT = 3;
///     CHEST = 4;
///     WAIST = 5;
///     LEGS = 6;
///     BOOTS = 7;
///     WRIST = 8;
///     HANDS = 9;
///     RING1 = 10;
///     RING2 = 11;
///     TRINKET1 = 12;
///     TRINKET2 = 13;
///     BACK = 14;
///     MAIN_HAND = 15;
///     OFF_HAND = 16;
///     RANGED_OR_RELIC = 17;
///     TABARD = 18;
///     BAG1 = 19;
///     BAG2 = 20;
///     BAG3 = 21;
///     BAG4 = 22;
///     INVENTORY_0 = 23;
///     INVENTORY_1 = 24;
///     INVENTORY_2 = 25;
///     INVENTORY_3 = 26;
///     INVENTORY_4 = 27;
///     INVENTORY_5 = 28;
///     INVENTORY_6 = 29;
///     INVENTORY_7 = 30;
///     INVENTORY_8 = 31;
///     INVENTORY_9 = 32;
///     INVENTORY_10 = 33;
///     INVENTORY_11 = 34;
///     INVENTORY_12 = 35;
///     INVENTORY_13 = 36;
///     INVENTORY_14 = 37;
///     INVENTORY_15 = 38;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemSlot {
    Head,
    Neck,
    Shoulders,
    Shirt,
    Chest,
    Waist,
    Legs,
    Boots,
    Wrist,
    Hands,
    Ring1,
    Ring2,
    Trinket1,
    Trinket2,
    Back,
    MainHand,
    OffHand,
    RangedOrRelic,
    Tabard,
    Bag1,
    Bag2,
    Bag3,
    Bag4,
    Inventory0,
    Inventory1,
    Inventory2,
    Inventory3,
    Inventory4,
    Inventory5,
    Inventory6,
    Inventory7,
    Inventory8,
    Inventory9,
    Inventory10,
    Inventory11,
    Inventory12,
    Inventory13,
    Inventory14,
    Inventory15,
}

impl ItemSlot {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Head => 0x0,
            Self::Neck => 0x1,
            Self::Shoulders => 0x2,
            Self::Shirt => 0x3,
            Self::Chest => 0x4,
            Self::Waist => 0x5,
            Self::Legs => 0x6,
            Self::Boots => 0x7,
            Self::Wrist => 0x8,
            Self::Hands => 0x9,
            Self::Ring1 => 0xa,
            Self::Ring2 => 0xb,
            Self::Trinket1 => 0xc,
            Self::Trinket2 => 0xd,
            Self::Back => 0xe,
            Self::MainHand => 0xf,
            Self::OffHand => 0x10,
            Self::RangedOrRelic => 0x11,
            Self::Tabard => 0x12,
            Self::Bag1 => 0x13,
            Self::Bag2 => 0x14,
            Self::Bag3 => 0x15,
            Self::Bag4 => 0x16,
            Self::Inventory0 => 0x17,
            Self::Inventory1 => 0x18,
            Self::Inventory2 => 0x19,
            Self::Inventory3 => 0x1a,
            Self::Inventory4 => 0x1b,
            Self::Inventory5 => 0x1c,
            Self::Inventory6 => 0x1d,
            Self::Inventory7 => 0x1e,
            Self::Inventory8 => 0x1f,
            Self::Inventory9 => 0x20,
            Self::Inventory10 => 0x21,
            Self::Inventory11 => 0x22,
            Self::Inventory12 => 0x23,
            Self::Inventory13 => 0x24,
            Self::Inventory14 => 0x25,
            Self::Inventory15 => 0x26,
        }
    }

}

impl Default for ItemSlot {
    fn default() -> Self {
        Self::Head
    }
}

impl std::fmt::Display for ItemSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Head => f.write_str("Head"),
            Self::Neck => f.write_str("Neck"),
            Self::Shoulders => f.write_str("Shoulders"),
            Self::Shirt => f.write_str("Shirt"),
            Self::Chest => f.write_str("Chest"),
            Self::Waist => f.write_str("Waist"),
            Self::Legs => f.write_str("Legs"),
            Self::Boots => f.write_str("Boots"),
            Self::Wrist => f.write_str("Wrist"),
            Self::Hands => f.write_str("Hands"),
            Self::Ring1 => f.write_str("Ring1"),
            Self::Ring2 => f.write_str("Ring2"),
            Self::Trinket1 => f.write_str("Trinket1"),
            Self::Trinket2 => f.write_str("Trinket2"),
            Self::Back => f.write_str("Back"),
            Self::MainHand => f.write_str("MainHand"),
            Self::OffHand => f.write_str("OffHand"),
            Self::RangedOrRelic => f.write_str("RangedOrRelic"),
            Self::Tabard => f.write_str("Tabard"),
            Self::Bag1 => f.write_str("Bag1"),
            Self::Bag2 => f.write_str("Bag2"),
            Self::Bag3 => f.write_str("Bag3"),
            Self::Bag4 => f.write_str("Bag4"),
            Self::Inventory0 => f.write_str("Inventory0"),
            Self::Inventory1 => f.write_str("Inventory1"),
            Self::Inventory2 => f.write_str("Inventory2"),
            Self::Inventory3 => f.write_str("Inventory3"),
            Self::Inventory4 => f.write_str("Inventory4"),
            Self::Inventory5 => f.write_str("Inventory5"),
            Self::Inventory6 => f.write_str("Inventory6"),
            Self::Inventory7 => f.write_str("Inventory7"),
            Self::Inventory8 => f.write_str("Inventory8"),
            Self::Inventory9 => f.write_str("Inventory9"),
            Self::Inventory10 => f.write_str("Inventory10"),
            Self::Inventory11 => f.write_str("Inventory11"),
            Self::Inventory12 => f.write_str("Inventory12"),
            Self::Inventory13 => f.write_str("Inventory13"),
            Self::Inventory14 => f.write_str("Inventory14"),
            Self::Inventory15 => f.write_str("Inventory15"),
        }
    }
}

impl TryFrom<u8> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Head),
            1 => Ok(Self::Neck),
            2 => Ok(Self::Shoulders),
            3 => Ok(Self::Shirt),
            4 => Ok(Self::Chest),
            5 => Ok(Self::Waist),
            6 => Ok(Self::Legs),
            7 => Ok(Self::Boots),
            8 => Ok(Self::Wrist),
            9 => Ok(Self::Hands),
            10 => Ok(Self::Ring1),
            11 => Ok(Self::Ring2),
            12 => Ok(Self::Trinket1),
            13 => Ok(Self::Trinket2),
            14 => Ok(Self::Back),
            15 => Ok(Self::MainHand),
            16 => Ok(Self::OffHand),
            17 => Ok(Self::RangedOrRelic),
            18 => Ok(Self::Tabard),
            19 => Ok(Self::Bag1),
            20 => Ok(Self::Bag2),
            21 => Ok(Self::Bag3),
            22 => Ok(Self::Bag4),
            23 => Ok(Self::Inventory0),
            24 => Ok(Self::Inventory1),
            25 => Ok(Self::Inventory2),
            26 => Ok(Self::Inventory3),
            27 => Ok(Self::Inventory4),
            28 => Ok(Self::Inventory5),
            29 => Ok(Self::Inventory6),
            30 => Ok(Self::Inventory7),
            31 => Ok(Self::Inventory8),
            32 => Ok(Self::Inventory9),
            33 => Ok(Self::Inventory10),
            34 => Ok(Self::Inventory11),
            35 => Ok(Self::Inventory12),
            36 => Ok(Self::Inventory13),
            37 => Ok(Self::Inventory14),
            38 => Ok(Self::Inventory15),
            v => Err(crate::errors::EnumError::new("ItemSlot", v as u64),)
        }
    }
}

