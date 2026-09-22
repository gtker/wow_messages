/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm#L22):
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
///     BANK_1 = 39;
///     BANK_2 = 40;
///     BANK_3 = 41;
///     BANK_4 = 42;
///     BANK_5 = 43;
///     BANK_6 = 44;
///     BANK_7 = 45;
///     BANK_8 = 46;
///     BANK_9 = 47;
///     BANK_10 = 48;
///     BANK_11 = 49;
///     BANK_12 = 50;
///     BANK_13 = 51;
///     BANK_14 = 52;
///     BANK_15 = 53;
///     BANK_16 = 54;
///     BANK_17 = 55;
///     BANK_18 = 56;
///     BANK_19 = 57;
///     BANK_20 = 58;
///     BANK_21 = 59;
///     BANK_22 = 60;
///     BANK_23 = 61;
///     BANK_24 = 62;
///     BANK_BAG_SLOT_1 = 63;
///     BANK_BAG_SLOT_2 = 64;
///     BANK_BAG_SLOT_3 = 65;
///     BANK_BAG_SLOT_4 = 66;
///     BANK_BAG_SLOT_5 = 67;
///     BANK_BAG_SLOT_6 = 68;
///     VENDOR_BUYBACK_1 = 69;
///     VENDOR_BUYBACK_2 = 70;
///     VENDOR_BUYBACK_3 = 71;
///     VENDOR_BUYBACK_4 = 72;
///     VENDOR_BUYBACK_5 = 73;
///     VENDOR_BUYBACK_6 = 74;
///     VENDOR_BUYBACK_7 = 75;
///     VENDOR_BUYBACK_8 = 76;
///     VENDOR_BUYBACK_9 = 77;
///     VENDOR_BUYBACK_10 = 78;
///     VENDOR_BUYBACK_11 = 79;
///     VENDOR_BUYBACK_12 = 80;
///     KEYRING_1 = 81;
///     KEYRING_2 = 82;
///     KEYRING_3 = 83;
///     KEYRING_4 = 84;
///     KEYRING_5 = 85;
///     KEYRING_6 = 86;
///     KEYRING_7 = 87;
///     KEYRING_8 = 88;
///     KEYRING_9 = 89;
///     KEYRING_10 = 90;
///     KEYRING_11 = 91;
///     KEYRING_12 = 92;
///     KEYRING_13 = 93;
///     KEYRING_14 = 94;
///     KEYRING_15 = 95;
///     KEYRING_16 = 96;
///     KEYRING_17 = 97;
///     KEYRING_18 = 98;
///     KEYRING_19 = 99;
///     KEYRING_20 = 100;
///     KEYRING_21 = 101;
///     KEYRING_22 = 102;
///     KEYRING_23 = 103;
///     KEYRING_24 = 104;
///     KEYRING_25 = 105;
///     KEYRING_26 = 106;
///     KEYRING_27 = 107;
///     KEYRING_28 = 108;
///     KEYRING_29 = 109;
///     KEYRING_30 = 110;
///     KEYRING_31 = 111;
///     KEYRING_32 = 112;
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
    Bank1,
    Bank2,
    Bank3,
    Bank4,
    Bank5,
    Bank6,
    Bank7,
    Bank8,
    Bank9,
    Bank10,
    Bank11,
    Bank12,
    Bank13,
    Bank14,
    Bank15,
    Bank16,
    Bank17,
    Bank18,
    Bank19,
    Bank20,
    Bank21,
    Bank22,
    Bank23,
    Bank24,
    BankBagSlot1,
    BankBagSlot2,
    BankBagSlot3,
    BankBagSlot4,
    BankBagSlot5,
    BankBagSlot6,
    VendorBuyback1,
    VendorBuyback2,
    VendorBuyback3,
    VendorBuyback4,
    VendorBuyback5,
    VendorBuyback6,
    VendorBuyback7,
    VendorBuyback8,
    VendorBuyback9,
    VendorBuyback10,
    VendorBuyback11,
    VendorBuyback12,
    Keyring1,
    Keyring2,
    Keyring3,
    Keyring4,
    Keyring5,
    Keyring6,
    Keyring7,
    Keyring8,
    Keyring9,
    Keyring10,
    Keyring11,
    Keyring12,
    Keyring13,
    Keyring14,
    Keyring15,
    Keyring16,
    Keyring17,
    Keyring18,
    Keyring19,
    Keyring20,
    Keyring21,
    Keyring22,
    Keyring23,
    Keyring24,
    Keyring25,
    Keyring26,
    Keyring27,
    Keyring28,
    Keyring29,
    Keyring30,
    Keyring31,
    Keyring32,
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
            Self::Bank1 => 0x27,
            Self::Bank2 => 0x28,
            Self::Bank3 => 0x29,
            Self::Bank4 => 0x2a,
            Self::Bank5 => 0x2b,
            Self::Bank6 => 0x2c,
            Self::Bank7 => 0x2d,
            Self::Bank8 => 0x2e,
            Self::Bank9 => 0x2f,
            Self::Bank10 => 0x30,
            Self::Bank11 => 0x31,
            Self::Bank12 => 0x32,
            Self::Bank13 => 0x33,
            Self::Bank14 => 0x34,
            Self::Bank15 => 0x35,
            Self::Bank16 => 0x36,
            Self::Bank17 => 0x37,
            Self::Bank18 => 0x38,
            Self::Bank19 => 0x39,
            Self::Bank20 => 0x3a,
            Self::Bank21 => 0x3b,
            Self::Bank22 => 0x3c,
            Self::Bank23 => 0x3d,
            Self::Bank24 => 0x3e,
            Self::BankBagSlot1 => 0x3f,
            Self::BankBagSlot2 => 0x40,
            Self::BankBagSlot3 => 0x41,
            Self::BankBagSlot4 => 0x42,
            Self::BankBagSlot5 => 0x43,
            Self::BankBagSlot6 => 0x44,
            Self::VendorBuyback1 => 0x45,
            Self::VendorBuyback2 => 0x46,
            Self::VendorBuyback3 => 0x47,
            Self::VendorBuyback4 => 0x48,
            Self::VendorBuyback5 => 0x49,
            Self::VendorBuyback6 => 0x4a,
            Self::VendorBuyback7 => 0x4b,
            Self::VendorBuyback8 => 0x4c,
            Self::VendorBuyback9 => 0x4d,
            Self::VendorBuyback10 => 0x4e,
            Self::VendorBuyback11 => 0x4f,
            Self::VendorBuyback12 => 0x50,
            Self::Keyring1 => 0x51,
            Self::Keyring2 => 0x52,
            Self::Keyring3 => 0x53,
            Self::Keyring4 => 0x54,
            Self::Keyring5 => 0x55,
            Self::Keyring6 => 0x56,
            Self::Keyring7 => 0x57,
            Self::Keyring8 => 0x58,
            Self::Keyring9 => 0x59,
            Self::Keyring10 => 0x5a,
            Self::Keyring11 => 0x5b,
            Self::Keyring12 => 0x5c,
            Self::Keyring13 => 0x5d,
            Self::Keyring14 => 0x5e,
            Self::Keyring15 => 0x5f,
            Self::Keyring16 => 0x60,
            Self::Keyring17 => 0x61,
            Self::Keyring18 => 0x62,
            Self::Keyring19 => 0x63,
            Self::Keyring20 => 0x64,
            Self::Keyring21 => 0x65,
            Self::Keyring22 => 0x66,
            Self::Keyring23 => 0x67,
            Self::Keyring24 => 0x68,
            Self::Keyring25 => 0x69,
            Self::Keyring26 => 0x6a,
            Self::Keyring27 => 0x6b,
            Self::Keyring28 => 0x6c,
            Self::Keyring29 => 0x6d,
            Self::Keyring30 => 0x6e,
            Self::Keyring31 => 0x6f,
            Self::Keyring32 => 0x70,
        }
    }

    pub const fn variants() -> [Self; 113] {
        [
            Self::Head,
            Self::Neck,
            Self::Shoulders,
            Self::Shirt,
            Self::Chest,
            Self::Waist,
            Self::Legs,
            Self::Boots,
            Self::Wrist,
            Self::Hands,
            Self::Ring1,
            Self::Ring2,
            Self::Trinket1,
            Self::Trinket2,
            Self::Back,
            Self::MainHand,
            Self::OffHand,
            Self::RangedOrRelic,
            Self::Tabard,
            Self::Bag1,
            Self::Bag2,
            Self::Bag3,
            Self::Bag4,
            Self::Inventory0,
            Self::Inventory1,
            Self::Inventory2,
            Self::Inventory3,
            Self::Inventory4,
            Self::Inventory5,
            Self::Inventory6,
            Self::Inventory7,
            Self::Inventory8,
            Self::Inventory9,
            Self::Inventory10,
            Self::Inventory11,
            Self::Inventory12,
            Self::Inventory13,
            Self::Inventory14,
            Self::Inventory15,
            Self::Bank1,
            Self::Bank2,
            Self::Bank3,
            Self::Bank4,
            Self::Bank5,
            Self::Bank6,
            Self::Bank7,
            Self::Bank8,
            Self::Bank9,
            Self::Bank10,
            Self::Bank11,
            Self::Bank12,
            Self::Bank13,
            Self::Bank14,
            Self::Bank15,
            Self::Bank16,
            Self::Bank17,
            Self::Bank18,
            Self::Bank19,
            Self::Bank20,
            Self::Bank21,
            Self::Bank22,
            Self::Bank23,
            Self::Bank24,
            Self::BankBagSlot1,
            Self::BankBagSlot2,
            Self::BankBagSlot3,
            Self::BankBagSlot4,
            Self::BankBagSlot5,
            Self::BankBagSlot6,
            Self::VendorBuyback1,
            Self::VendorBuyback2,
            Self::VendorBuyback3,
            Self::VendorBuyback4,
            Self::VendorBuyback5,
            Self::VendorBuyback6,
            Self::VendorBuyback7,
            Self::VendorBuyback8,
            Self::VendorBuyback9,
            Self::VendorBuyback10,
            Self::VendorBuyback11,
            Self::VendorBuyback12,
            Self::Keyring1,
            Self::Keyring2,
            Self::Keyring3,
            Self::Keyring4,
            Self::Keyring5,
            Self::Keyring6,
            Self::Keyring7,
            Self::Keyring8,
            Self::Keyring9,
            Self::Keyring10,
            Self::Keyring11,
            Self::Keyring12,
            Self::Keyring13,
            Self::Keyring14,
            Self::Keyring15,
            Self::Keyring16,
            Self::Keyring17,
            Self::Keyring18,
            Self::Keyring19,
            Self::Keyring20,
            Self::Keyring21,
            Self::Keyring22,
            Self::Keyring23,
            Self::Keyring24,
            Self::Keyring25,
            Self::Keyring26,
            Self::Keyring27,
            Self::Keyring28,
            Self::Keyring29,
            Self::Keyring30,
            Self::Keyring31,
            Self::Keyring32,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
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
            39 => Ok(Self::Bank1),
            40 => Ok(Self::Bank2),
            41 => Ok(Self::Bank3),
            42 => Ok(Self::Bank4),
            43 => Ok(Self::Bank5),
            44 => Ok(Self::Bank6),
            45 => Ok(Self::Bank7),
            46 => Ok(Self::Bank8),
            47 => Ok(Self::Bank9),
            48 => Ok(Self::Bank10),
            49 => Ok(Self::Bank11),
            50 => Ok(Self::Bank12),
            51 => Ok(Self::Bank13),
            52 => Ok(Self::Bank14),
            53 => Ok(Self::Bank15),
            54 => Ok(Self::Bank16),
            55 => Ok(Self::Bank17),
            56 => Ok(Self::Bank18),
            57 => Ok(Self::Bank19),
            58 => Ok(Self::Bank20),
            59 => Ok(Self::Bank21),
            60 => Ok(Self::Bank22),
            61 => Ok(Self::Bank23),
            62 => Ok(Self::Bank24),
            63 => Ok(Self::BankBagSlot1),
            64 => Ok(Self::BankBagSlot2),
            65 => Ok(Self::BankBagSlot3),
            66 => Ok(Self::BankBagSlot4),
            67 => Ok(Self::BankBagSlot5),
            68 => Ok(Self::BankBagSlot6),
            69 => Ok(Self::VendorBuyback1),
            70 => Ok(Self::VendorBuyback2),
            71 => Ok(Self::VendorBuyback3),
            72 => Ok(Self::VendorBuyback4),
            73 => Ok(Self::VendorBuyback5),
            74 => Ok(Self::VendorBuyback6),
            75 => Ok(Self::VendorBuyback7),
            76 => Ok(Self::VendorBuyback8),
            77 => Ok(Self::VendorBuyback9),
            78 => Ok(Self::VendorBuyback10),
            79 => Ok(Self::VendorBuyback11),
            80 => Ok(Self::VendorBuyback12),
            81 => Ok(Self::Keyring1),
            82 => Ok(Self::Keyring2),
            83 => Ok(Self::Keyring3),
            84 => Ok(Self::Keyring4),
            85 => Ok(Self::Keyring5),
            86 => Ok(Self::Keyring6),
            87 => Ok(Self::Keyring7),
            88 => Ok(Self::Keyring8),
            89 => Ok(Self::Keyring9),
            90 => Ok(Self::Keyring10),
            91 => Ok(Self::Keyring11),
            92 => Ok(Self::Keyring12),
            93 => Ok(Self::Keyring13),
            94 => Ok(Self::Keyring14),
            95 => Ok(Self::Keyring15),
            96 => Ok(Self::Keyring16),
            97 => Ok(Self::Keyring17),
            98 => Ok(Self::Keyring18),
            99 => Ok(Self::Keyring19),
            100 => Ok(Self::Keyring20),
            101 => Ok(Self::Keyring21),
            102 => Ok(Self::Keyring22),
            103 => Ok(Self::Keyring23),
            104 => Ok(Self::Keyring24),
            105 => Ok(Self::Keyring25),
            106 => Ok(Self::Keyring26),
            107 => Ok(Self::Keyring27),
            108 => Ok(Self::Keyring28),
            109 => Ok(Self::Keyring29),
            110 => Ok(Self::Keyring30),
            111 => Ok(Self::Keyring31),
            112 => Ok(Self::Keyring32),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ItemSlot {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Head => "HEAD",
            Self::Neck => "NECK",
            Self::Shoulders => "SHOULDERS",
            Self::Shirt => "SHIRT",
            Self::Chest => "CHEST",
            Self::Waist => "WAIST",
            Self::Legs => "LEGS",
            Self::Boots => "BOOTS",
            Self::Wrist => "WRIST",
            Self::Hands => "HANDS",
            Self::Ring1 => "RING1",
            Self::Ring2 => "RING2",
            Self::Trinket1 => "TRINKET1",
            Self::Trinket2 => "TRINKET2",
            Self::Back => "BACK",
            Self::MainHand => "MAIN_HAND",
            Self::OffHand => "OFF_HAND",
            Self::RangedOrRelic => "RANGED_OR_RELIC",
            Self::Tabard => "TABARD",
            Self::Bag1 => "BAG1",
            Self::Bag2 => "BAG2",
            Self::Bag3 => "BAG3",
            Self::Bag4 => "BAG4",
            Self::Inventory0 => "INVENTORY_0",
            Self::Inventory1 => "INVENTORY_1",
            Self::Inventory2 => "INVENTORY_2",
            Self::Inventory3 => "INVENTORY_3",
            Self::Inventory4 => "INVENTORY_4",
            Self::Inventory5 => "INVENTORY_5",
            Self::Inventory6 => "INVENTORY_6",
            Self::Inventory7 => "INVENTORY_7",
            Self::Inventory8 => "INVENTORY_8",
            Self::Inventory9 => "INVENTORY_9",
            Self::Inventory10 => "INVENTORY_10",
            Self::Inventory11 => "INVENTORY_11",
            Self::Inventory12 => "INVENTORY_12",
            Self::Inventory13 => "INVENTORY_13",
            Self::Inventory14 => "INVENTORY_14",
            Self::Inventory15 => "INVENTORY_15",
            Self::Bank1 => "BANK_1",
            Self::Bank2 => "BANK_2",
            Self::Bank3 => "BANK_3",
            Self::Bank4 => "BANK_4",
            Self::Bank5 => "BANK_5",
            Self::Bank6 => "BANK_6",
            Self::Bank7 => "BANK_7",
            Self::Bank8 => "BANK_8",
            Self::Bank9 => "BANK_9",
            Self::Bank10 => "BANK_10",
            Self::Bank11 => "BANK_11",
            Self::Bank12 => "BANK_12",
            Self::Bank13 => "BANK_13",
            Self::Bank14 => "BANK_14",
            Self::Bank15 => "BANK_15",
            Self::Bank16 => "BANK_16",
            Self::Bank17 => "BANK_17",
            Self::Bank18 => "BANK_18",
            Self::Bank19 => "BANK_19",
            Self::Bank20 => "BANK_20",
            Self::Bank21 => "BANK_21",
            Self::Bank22 => "BANK_22",
            Self::Bank23 => "BANK_23",
            Self::Bank24 => "BANK_24",
            Self::BankBagSlot1 => "BANK_BAG_SLOT_1",
            Self::BankBagSlot2 => "BANK_BAG_SLOT_2",
            Self::BankBagSlot3 => "BANK_BAG_SLOT_3",
            Self::BankBagSlot4 => "BANK_BAG_SLOT_4",
            Self::BankBagSlot5 => "BANK_BAG_SLOT_5",
            Self::BankBagSlot6 => "BANK_BAG_SLOT_6",
            Self::VendorBuyback1 => "VENDOR_BUYBACK_1",
            Self::VendorBuyback2 => "VENDOR_BUYBACK_2",
            Self::VendorBuyback3 => "VENDOR_BUYBACK_3",
            Self::VendorBuyback4 => "VENDOR_BUYBACK_4",
            Self::VendorBuyback5 => "VENDOR_BUYBACK_5",
            Self::VendorBuyback6 => "VENDOR_BUYBACK_6",
            Self::VendorBuyback7 => "VENDOR_BUYBACK_7",
            Self::VendorBuyback8 => "VENDOR_BUYBACK_8",
            Self::VendorBuyback9 => "VENDOR_BUYBACK_9",
            Self::VendorBuyback10 => "VENDOR_BUYBACK_10",
            Self::VendorBuyback11 => "VENDOR_BUYBACK_11",
            Self::VendorBuyback12 => "VENDOR_BUYBACK_12",
            Self::Keyring1 => "KEYRING_1",
            Self::Keyring2 => "KEYRING_2",
            Self::Keyring3 => "KEYRING_3",
            Self::Keyring4 => "KEYRING_4",
            Self::Keyring5 => "KEYRING_5",
            Self::Keyring6 => "KEYRING_6",
            Self::Keyring7 => "KEYRING_7",
            Self::Keyring8 => "KEYRING_8",
            Self::Keyring9 => "KEYRING_9",
            Self::Keyring10 => "KEYRING_10",
            Self::Keyring11 => "KEYRING_11",
            Self::Keyring12 => "KEYRING_12",
            Self::Keyring13 => "KEYRING_13",
            Self::Keyring14 => "KEYRING_14",
            Self::Keyring15 => "KEYRING_15",
            Self::Keyring16 => "KEYRING_16",
            Self::Keyring17 => "KEYRING_17",
            Self::Keyring18 => "KEYRING_18",
            Self::Keyring19 => "KEYRING_19",
            Self::Keyring20 => "KEYRING_20",
            Self::Keyring21 => "KEYRING_21",
            Self::Keyring22 => "KEYRING_22",
            Self::Keyring23 => "KEYRING_23",
            Self::Keyring24 => "KEYRING_24",
            Self::Keyring25 => "KEYRING_25",
            Self::Keyring26 => "KEYRING_26",
            Self::Keyring27 => "KEYRING_27",
            Self::Keyring28 => "KEYRING_28",
            Self::Keyring29 => "KEYRING_29",
            Self::Keyring30 => "KEYRING_30",
            Self::Keyring31 => "KEYRING_31",
            Self::Keyring32 => "KEYRING_32",
        }
    }

}

const NAME: &str = "ItemSlot";

impl Default for ItemSlot {
    fn default() -> Self {
        Self::Head
    }
}

impl std::fmt::Display for ItemSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Head => "Head",
            Self::Neck => "Neck",
            Self::Shoulders => "Shoulders",
            Self::Shirt => "Shirt",
            Self::Chest => "Chest",
            Self::Waist => "Waist",
            Self::Legs => "Legs",
            Self::Boots => "Boots",
            Self::Wrist => "Wrist",
            Self::Hands => "Hands",
            Self::Ring1 => "Ring1",
            Self::Ring2 => "Ring2",
            Self::Trinket1 => "Trinket1",
            Self::Trinket2 => "Trinket2",
            Self::Back => "Back",
            Self::MainHand => "MainHand",
            Self::OffHand => "OffHand",
            Self::RangedOrRelic => "RangedOrRelic",
            Self::Tabard => "Tabard",
            Self::Bag1 => "Bag1",
            Self::Bag2 => "Bag2",
            Self::Bag3 => "Bag3",
            Self::Bag4 => "Bag4",
            Self::Inventory0 => "Inventory0",
            Self::Inventory1 => "Inventory1",
            Self::Inventory2 => "Inventory2",
            Self::Inventory3 => "Inventory3",
            Self::Inventory4 => "Inventory4",
            Self::Inventory5 => "Inventory5",
            Self::Inventory6 => "Inventory6",
            Self::Inventory7 => "Inventory7",
            Self::Inventory8 => "Inventory8",
            Self::Inventory9 => "Inventory9",
            Self::Inventory10 => "Inventory10",
            Self::Inventory11 => "Inventory11",
            Self::Inventory12 => "Inventory12",
            Self::Inventory13 => "Inventory13",
            Self::Inventory14 => "Inventory14",
            Self::Inventory15 => "Inventory15",
            Self::Bank1 => "Bank1",
            Self::Bank2 => "Bank2",
            Self::Bank3 => "Bank3",
            Self::Bank4 => "Bank4",
            Self::Bank5 => "Bank5",
            Self::Bank6 => "Bank6",
            Self::Bank7 => "Bank7",
            Self::Bank8 => "Bank8",
            Self::Bank9 => "Bank9",
            Self::Bank10 => "Bank10",
            Self::Bank11 => "Bank11",
            Self::Bank12 => "Bank12",
            Self::Bank13 => "Bank13",
            Self::Bank14 => "Bank14",
            Self::Bank15 => "Bank15",
            Self::Bank16 => "Bank16",
            Self::Bank17 => "Bank17",
            Self::Bank18 => "Bank18",
            Self::Bank19 => "Bank19",
            Self::Bank20 => "Bank20",
            Self::Bank21 => "Bank21",
            Self::Bank22 => "Bank22",
            Self::Bank23 => "Bank23",
            Self::Bank24 => "Bank24",
            Self::BankBagSlot1 => "BankBagSlot1",
            Self::BankBagSlot2 => "BankBagSlot2",
            Self::BankBagSlot3 => "BankBagSlot3",
            Self::BankBagSlot4 => "BankBagSlot4",
            Self::BankBagSlot5 => "BankBagSlot5",
            Self::BankBagSlot6 => "BankBagSlot6",
            Self::VendorBuyback1 => "VendorBuyback1",
            Self::VendorBuyback2 => "VendorBuyback2",
            Self::VendorBuyback3 => "VendorBuyback3",
            Self::VendorBuyback4 => "VendorBuyback4",
            Self::VendorBuyback5 => "VendorBuyback5",
            Self::VendorBuyback6 => "VendorBuyback6",
            Self::VendorBuyback7 => "VendorBuyback7",
            Self::VendorBuyback8 => "VendorBuyback8",
            Self::VendorBuyback9 => "VendorBuyback9",
            Self::VendorBuyback10 => "VendorBuyback10",
            Self::VendorBuyback11 => "VendorBuyback11",
            Self::VendorBuyback12 => "VendorBuyback12",
            Self::Keyring1 => "Keyring1",
            Self::Keyring2 => "Keyring2",
            Self::Keyring3 => "Keyring3",
            Self::Keyring4 => "Keyring4",
            Self::Keyring5 => "Keyring5",
            Self::Keyring6 => "Keyring6",
            Self::Keyring7 => "Keyring7",
            Self::Keyring8 => "Keyring8",
            Self::Keyring9 => "Keyring9",
            Self::Keyring10 => "Keyring10",
            Self::Keyring11 => "Keyring11",
            Self::Keyring12 => "Keyring12",
            Self::Keyring13 => "Keyring13",
            Self::Keyring14 => "Keyring14",
            Self::Keyring15 => "Keyring15",
            Self::Keyring16 => "Keyring16",
            Self::Keyring17 => "Keyring17",
            Self::Keyring18 => "Keyring18",
            Self::Keyring19 => "Keyring19",
            Self::Keyring20 => "Keyring20",
            Self::Keyring21 => "Keyring21",
            Self::Keyring22 => "Keyring22",
            Self::Keyring23 => "Keyring23",
            Self::Keyring24 => "Keyring24",
            Self::Keyring25 => "Keyring25",
            Self::Keyring26 => "Keyring26",
            Self::Keyring27 => "Keyring27",
            Self::Keyring28 => "Keyring28",
            Self::Keyring29 => "Keyring29",
            Self::Keyring30 => "Keyring30",
            Self::Keyring31 => "Keyring31",
            Self::Keyring32 => "Keyring32",
        })
    }
}

impl TryFrom<u8> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ItemSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

