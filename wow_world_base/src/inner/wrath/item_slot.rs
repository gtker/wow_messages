/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm:267`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm#L267):
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
///     BANK_25 = 63;
///     BANK_26 = 64;
///     BANK_27 = 65;
///     BANK_28 = 66;
///     BANK_BAG_SLOT_1 = 67;
///     BANK_BAG_SLOT_2 = 68;
///     BANK_BAG_SLOT_3 = 69;
///     BANK_BAG_SLOT_4 = 70;
///     BANK_BAG_SLOT_5 = 71;
///     BANK_BAG_SLOT_6 = 72;
///     BANK_BAG_SLOT_7 = 73;
///     VENDOR_BUYBACK_1 = 74;
///     VENDOR_BUYBACK_2 = 75;
///     VENDOR_BUYBACK_3 = 76;
///     VENDOR_BUYBACK_4 = 77;
///     VENDOR_BUYBACK_5 = 78;
///     VENDOR_BUYBACK_6 = 79;
///     VENDOR_BUYBACK_7 = 80;
///     VENDOR_BUYBACK_8 = 81;
///     VENDOR_BUYBACK_9 = 82;
///     VENDOR_BUYBACK_10 = 83;
///     VENDOR_BUYBACK_11 = 84;
///     VENDOR_BUYBACK_12 = 85;
///     KEYRING_1 = 86;
///     KEYRING_2 = 87;
///     KEYRING_3 = 88;
///     KEYRING_4 = 89;
///     KEYRING_5 = 90;
///     KEYRING_6 = 91;
///     KEYRING_7 = 92;
///     KEYRING_8 = 93;
///     KEYRING_9 = 94;
///     KEYRING_10 = 95;
///     KEYRING_11 = 96;
///     KEYRING_12 = 97;
///     KEYRING_13 = 98;
///     KEYRING_14 = 99;
///     KEYRING_15 = 100;
///     KEYRING_16 = 101;
///     KEYRING_17 = 102;
///     KEYRING_18 = 103;
///     KEYRING_19 = 104;
///     KEYRING_20 = 105;
///     KEYRING_21 = 106;
///     KEYRING_22 = 107;
///     KEYRING_23 = 108;
///     KEYRING_24 = 109;
///     KEYRING_25 = 110;
///     KEYRING_26 = 111;
///     KEYRING_27 = 112;
///     KEYRING_28 = 113;
///     KEYRING_29 = 114;
///     KEYRING_30 = 115;
///     KEYRING_31 = 116;
///     KEYRING_32 = 117;
///     CURRENCY_TOKEN_1 = 118;
///     CURRENCY_TOKEN_2 = 119;
///     CURRENCY_TOKEN_3 = 120;
///     CURRENCY_TOKEN_4 = 121;
///     CURRENCY_TOKEN_5 = 122;
///     CURRENCY_TOKEN_6 = 123;
///     CURRENCY_TOKEN_7 = 124;
///     CURRENCY_TOKEN_8 = 125;
///     CURRENCY_TOKEN_9 = 126;
///     CURRENCY_TOKEN_10 = 127;
///     CURRENCY_TOKEN_11 = 128;
///     CURRENCY_TOKEN_12 = 129;
///     CURRENCY_TOKEN_13 = 130;
///     CURRENCY_TOKEN_14 = 131;
///     CURRENCY_TOKEN_15 = 132;
///     CURRENCY_TOKEN_16 = 133;
///     CURRENCY_TOKEN_17 = 134;
///     CURRENCY_TOKEN_18 = 135;
///     CURRENCY_TOKEN_19 = 136;
///     CURRENCY_TOKEN_20 = 137;
///     CURRENCY_TOKEN_21 = 138;
///     CURRENCY_TOKEN_22 = 139;
///     CURRENCY_TOKEN_23 = 140;
///     CURRENCY_TOKEN_24 = 141;
///     CURRENCY_TOKEN_25 = 142;
///     CURRENCY_TOKEN_26 = 143;
///     CURRENCY_TOKEN_27 = 144;
///     CURRENCY_TOKEN_28 = 145;
///     CURRENCY_TOKEN_29 = 146;
///     CURRENCY_TOKEN_30 = 147;
///     CURRENCY_TOKEN_31 = 148;
///     CURRENCY_TOKEN_32 = 149;
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
    Bank25,
    Bank26,
    Bank27,
    Bank28,
    BankBagSlot1,
    BankBagSlot2,
    BankBagSlot3,
    BankBagSlot4,
    BankBagSlot5,
    BankBagSlot6,
    BankBagSlot7,
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
    CurrencyToken1,
    CurrencyToken2,
    CurrencyToken3,
    CurrencyToken4,
    CurrencyToken5,
    CurrencyToken6,
    CurrencyToken7,
    CurrencyToken8,
    CurrencyToken9,
    CurrencyToken10,
    CurrencyToken11,
    CurrencyToken12,
    CurrencyToken13,
    CurrencyToken14,
    CurrencyToken15,
    CurrencyToken16,
    CurrencyToken17,
    CurrencyToken18,
    CurrencyToken19,
    CurrencyToken20,
    CurrencyToken21,
    CurrencyToken22,
    CurrencyToken23,
    CurrencyToken24,
    CurrencyToken25,
    CurrencyToken26,
    CurrencyToken27,
    CurrencyToken28,
    CurrencyToken29,
    CurrencyToken30,
    CurrencyToken31,
    CurrencyToken32,
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
            Self::Bank25 => 0x3f,
            Self::Bank26 => 0x40,
            Self::Bank27 => 0x41,
            Self::Bank28 => 0x42,
            Self::BankBagSlot1 => 0x43,
            Self::BankBagSlot2 => 0x44,
            Self::BankBagSlot3 => 0x45,
            Self::BankBagSlot4 => 0x46,
            Self::BankBagSlot5 => 0x47,
            Self::BankBagSlot6 => 0x48,
            Self::BankBagSlot7 => 0x49,
            Self::VendorBuyback1 => 0x4a,
            Self::VendorBuyback2 => 0x4b,
            Self::VendorBuyback3 => 0x4c,
            Self::VendorBuyback4 => 0x4d,
            Self::VendorBuyback5 => 0x4e,
            Self::VendorBuyback6 => 0x4f,
            Self::VendorBuyback7 => 0x50,
            Self::VendorBuyback8 => 0x51,
            Self::VendorBuyback9 => 0x52,
            Self::VendorBuyback10 => 0x53,
            Self::VendorBuyback11 => 0x54,
            Self::VendorBuyback12 => 0x55,
            Self::Keyring1 => 0x56,
            Self::Keyring2 => 0x57,
            Self::Keyring3 => 0x58,
            Self::Keyring4 => 0x59,
            Self::Keyring5 => 0x5a,
            Self::Keyring6 => 0x5b,
            Self::Keyring7 => 0x5c,
            Self::Keyring8 => 0x5d,
            Self::Keyring9 => 0x5e,
            Self::Keyring10 => 0x5f,
            Self::Keyring11 => 0x60,
            Self::Keyring12 => 0x61,
            Self::Keyring13 => 0x62,
            Self::Keyring14 => 0x63,
            Self::Keyring15 => 0x64,
            Self::Keyring16 => 0x65,
            Self::Keyring17 => 0x66,
            Self::Keyring18 => 0x67,
            Self::Keyring19 => 0x68,
            Self::Keyring20 => 0x69,
            Self::Keyring21 => 0x6a,
            Self::Keyring22 => 0x6b,
            Self::Keyring23 => 0x6c,
            Self::Keyring24 => 0x6d,
            Self::Keyring25 => 0x6e,
            Self::Keyring26 => 0x6f,
            Self::Keyring27 => 0x70,
            Self::Keyring28 => 0x71,
            Self::Keyring29 => 0x72,
            Self::Keyring30 => 0x73,
            Self::Keyring31 => 0x74,
            Self::Keyring32 => 0x75,
            Self::CurrencyToken1 => 0x76,
            Self::CurrencyToken2 => 0x77,
            Self::CurrencyToken3 => 0x78,
            Self::CurrencyToken4 => 0x79,
            Self::CurrencyToken5 => 0x7a,
            Self::CurrencyToken6 => 0x7b,
            Self::CurrencyToken7 => 0x7c,
            Self::CurrencyToken8 => 0x7d,
            Self::CurrencyToken9 => 0x7e,
            Self::CurrencyToken10 => 0x7f,
            Self::CurrencyToken11 => 0x80,
            Self::CurrencyToken12 => 0x81,
            Self::CurrencyToken13 => 0x82,
            Self::CurrencyToken14 => 0x83,
            Self::CurrencyToken15 => 0x84,
            Self::CurrencyToken16 => 0x85,
            Self::CurrencyToken17 => 0x86,
            Self::CurrencyToken18 => 0x87,
            Self::CurrencyToken19 => 0x88,
            Self::CurrencyToken20 => 0x89,
            Self::CurrencyToken21 => 0x8a,
            Self::CurrencyToken22 => 0x8b,
            Self::CurrencyToken23 => 0x8c,
            Self::CurrencyToken24 => 0x8d,
            Self::CurrencyToken25 => 0x8e,
            Self::CurrencyToken26 => 0x8f,
            Self::CurrencyToken27 => 0x90,
            Self::CurrencyToken28 => 0x91,
            Self::CurrencyToken29 => 0x92,
            Self::CurrencyToken30 => 0x93,
            Self::CurrencyToken31 => 0x94,
            Self::CurrencyToken32 => 0x95,
        }
    }

    pub const fn variants() -> [Self; 150] {
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
            Self::Bank25,
            Self::Bank26,
            Self::Bank27,
            Self::Bank28,
            Self::BankBagSlot1,
            Self::BankBagSlot2,
            Self::BankBagSlot3,
            Self::BankBagSlot4,
            Self::BankBagSlot5,
            Self::BankBagSlot6,
            Self::BankBagSlot7,
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
            Self::CurrencyToken1,
            Self::CurrencyToken2,
            Self::CurrencyToken3,
            Self::CurrencyToken4,
            Self::CurrencyToken5,
            Self::CurrencyToken6,
            Self::CurrencyToken7,
            Self::CurrencyToken8,
            Self::CurrencyToken9,
            Self::CurrencyToken10,
            Self::CurrencyToken11,
            Self::CurrencyToken12,
            Self::CurrencyToken13,
            Self::CurrencyToken14,
            Self::CurrencyToken15,
            Self::CurrencyToken16,
            Self::CurrencyToken17,
            Self::CurrencyToken18,
            Self::CurrencyToken19,
            Self::CurrencyToken20,
            Self::CurrencyToken21,
            Self::CurrencyToken22,
            Self::CurrencyToken23,
            Self::CurrencyToken24,
            Self::CurrencyToken25,
            Self::CurrencyToken26,
            Self::CurrencyToken27,
            Self::CurrencyToken28,
            Self::CurrencyToken29,
            Self::CurrencyToken30,
            Self::CurrencyToken31,
            Self::CurrencyToken32,
        ]
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
            Self::Bank25 => "BANK_25",
            Self::Bank26 => "BANK_26",
            Self::Bank27 => "BANK_27",
            Self::Bank28 => "BANK_28",
            Self::BankBagSlot1 => "BANK_BAG_SLOT_1",
            Self::BankBagSlot2 => "BANK_BAG_SLOT_2",
            Self::BankBagSlot3 => "BANK_BAG_SLOT_3",
            Self::BankBagSlot4 => "BANK_BAG_SLOT_4",
            Self::BankBagSlot5 => "BANK_BAG_SLOT_5",
            Self::BankBagSlot6 => "BANK_BAG_SLOT_6",
            Self::BankBagSlot7 => "BANK_BAG_SLOT_7",
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
            Self::CurrencyToken1 => "CURRENCY_TOKEN_1",
            Self::CurrencyToken2 => "CURRENCY_TOKEN_2",
            Self::CurrencyToken3 => "CURRENCY_TOKEN_3",
            Self::CurrencyToken4 => "CURRENCY_TOKEN_4",
            Self::CurrencyToken5 => "CURRENCY_TOKEN_5",
            Self::CurrencyToken6 => "CURRENCY_TOKEN_6",
            Self::CurrencyToken7 => "CURRENCY_TOKEN_7",
            Self::CurrencyToken8 => "CURRENCY_TOKEN_8",
            Self::CurrencyToken9 => "CURRENCY_TOKEN_9",
            Self::CurrencyToken10 => "CURRENCY_TOKEN_10",
            Self::CurrencyToken11 => "CURRENCY_TOKEN_11",
            Self::CurrencyToken12 => "CURRENCY_TOKEN_12",
            Self::CurrencyToken13 => "CURRENCY_TOKEN_13",
            Self::CurrencyToken14 => "CURRENCY_TOKEN_14",
            Self::CurrencyToken15 => "CURRENCY_TOKEN_15",
            Self::CurrencyToken16 => "CURRENCY_TOKEN_16",
            Self::CurrencyToken17 => "CURRENCY_TOKEN_17",
            Self::CurrencyToken18 => "CURRENCY_TOKEN_18",
            Self::CurrencyToken19 => "CURRENCY_TOKEN_19",
            Self::CurrencyToken20 => "CURRENCY_TOKEN_20",
            Self::CurrencyToken21 => "CURRENCY_TOKEN_21",
            Self::CurrencyToken22 => "CURRENCY_TOKEN_22",
            Self::CurrencyToken23 => "CURRENCY_TOKEN_23",
            Self::CurrencyToken24 => "CURRENCY_TOKEN_24",
            Self::CurrencyToken25 => "CURRENCY_TOKEN_25",
            Self::CurrencyToken26 => "CURRENCY_TOKEN_26",
            Self::CurrencyToken27 => "CURRENCY_TOKEN_27",
            Self::CurrencyToken28 => "CURRENCY_TOKEN_28",
            Self::CurrencyToken29 => "CURRENCY_TOKEN_29",
            Self::CurrencyToken30 => "CURRENCY_TOKEN_30",
            Self::CurrencyToken31 => "CURRENCY_TOKEN_31",
            Self::CurrencyToken32 => "CURRENCY_TOKEN_32",
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
            Self::Bank1 => f.write_str("Bank1"),
            Self::Bank2 => f.write_str("Bank2"),
            Self::Bank3 => f.write_str("Bank3"),
            Self::Bank4 => f.write_str("Bank4"),
            Self::Bank5 => f.write_str("Bank5"),
            Self::Bank6 => f.write_str("Bank6"),
            Self::Bank7 => f.write_str("Bank7"),
            Self::Bank8 => f.write_str("Bank8"),
            Self::Bank9 => f.write_str("Bank9"),
            Self::Bank10 => f.write_str("Bank10"),
            Self::Bank11 => f.write_str("Bank11"),
            Self::Bank12 => f.write_str("Bank12"),
            Self::Bank13 => f.write_str("Bank13"),
            Self::Bank14 => f.write_str("Bank14"),
            Self::Bank15 => f.write_str("Bank15"),
            Self::Bank16 => f.write_str("Bank16"),
            Self::Bank17 => f.write_str("Bank17"),
            Self::Bank18 => f.write_str("Bank18"),
            Self::Bank19 => f.write_str("Bank19"),
            Self::Bank20 => f.write_str("Bank20"),
            Self::Bank21 => f.write_str("Bank21"),
            Self::Bank22 => f.write_str("Bank22"),
            Self::Bank23 => f.write_str("Bank23"),
            Self::Bank24 => f.write_str("Bank24"),
            Self::Bank25 => f.write_str("Bank25"),
            Self::Bank26 => f.write_str("Bank26"),
            Self::Bank27 => f.write_str("Bank27"),
            Self::Bank28 => f.write_str("Bank28"),
            Self::BankBagSlot1 => f.write_str("BankBagSlot1"),
            Self::BankBagSlot2 => f.write_str("BankBagSlot2"),
            Self::BankBagSlot3 => f.write_str("BankBagSlot3"),
            Self::BankBagSlot4 => f.write_str("BankBagSlot4"),
            Self::BankBagSlot5 => f.write_str("BankBagSlot5"),
            Self::BankBagSlot6 => f.write_str("BankBagSlot6"),
            Self::BankBagSlot7 => f.write_str("BankBagSlot7"),
            Self::VendorBuyback1 => f.write_str("VendorBuyback1"),
            Self::VendorBuyback2 => f.write_str("VendorBuyback2"),
            Self::VendorBuyback3 => f.write_str("VendorBuyback3"),
            Self::VendorBuyback4 => f.write_str("VendorBuyback4"),
            Self::VendorBuyback5 => f.write_str("VendorBuyback5"),
            Self::VendorBuyback6 => f.write_str("VendorBuyback6"),
            Self::VendorBuyback7 => f.write_str("VendorBuyback7"),
            Self::VendorBuyback8 => f.write_str("VendorBuyback8"),
            Self::VendorBuyback9 => f.write_str("VendorBuyback9"),
            Self::VendorBuyback10 => f.write_str("VendorBuyback10"),
            Self::VendorBuyback11 => f.write_str("VendorBuyback11"),
            Self::VendorBuyback12 => f.write_str("VendorBuyback12"),
            Self::Keyring1 => f.write_str("Keyring1"),
            Self::Keyring2 => f.write_str("Keyring2"),
            Self::Keyring3 => f.write_str("Keyring3"),
            Self::Keyring4 => f.write_str("Keyring4"),
            Self::Keyring5 => f.write_str("Keyring5"),
            Self::Keyring6 => f.write_str("Keyring6"),
            Self::Keyring7 => f.write_str("Keyring7"),
            Self::Keyring8 => f.write_str("Keyring8"),
            Self::Keyring9 => f.write_str("Keyring9"),
            Self::Keyring10 => f.write_str("Keyring10"),
            Self::Keyring11 => f.write_str("Keyring11"),
            Self::Keyring12 => f.write_str("Keyring12"),
            Self::Keyring13 => f.write_str("Keyring13"),
            Self::Keyring14 => f.write_str("Keyring14"),
            Self::Keyring15 => f.write_str("Keyring15"),
            Self::Keyring16 => f.write_str("Keyring16"),
            Self::Keyring17 => f.write_str("Keyring17"),
            Self::Keyring18 => f.write_str("Keyring18"),
            Self::Keyring19 => f.write_str("Keyring19"),
            Self::Keyring20 => f.write_str("Keyring20"),
            Self::Keyring21 => f.write_str("Keyring21"),
            Self::Keyring22 => f.write_str("Keyring22"),
            Self::Keyring23 => f.write_str("Keyring23"),
            Self::Keyring24 => f.write_str("Keyring24"),
            Self::Keyring25 => f.write_str("Keyring25"),
            Self::Keyring26 => f.write_str("Keyring26"),
            Self::Keyring27 => f.write_str("Keyring27"),
            Self::Keyring28 => f.write_str("Keyring28"),
            Self::Keyring29 => f.write_str("Keyring29"),
            Self::Keyring30 => f.write_str("Keyring30"),
            Self::Keyring31 => f.write_str("Keyring31"),
            Self::Keyring32 => f.write_str("Keyring32"),
            Self::CurrencyToken1 => f.write_str("CurrencyToken1"),
            Self::CurrencyToken2 => f.write_str("CurrencyToken2"),
            Self::CurrencyToken3 => f.write_str("CurrencyToken3"),
            Self::CurrencyToken4 => f.write_str("CurrencyToken4"),
            Self::CurrencyToken5 => f.write_str("CurrencyToken5"),
            Self::CurrencyToken6 => f.write_str("CurrencyToken6"),
            Self::CurrencyToken7 => f.write_str("CurrencyToken7"),
            Self::CurrencyToken8 => f.write_str("CurrencyToken8"),
            Self::CurrencyToken9 => f.write_str("CurrencyToken9"),
            Self::CurrencyToken10 => f.write_str("CurrencyToken10"),
            Self::CurrencyToken11 => f.write_str("CurrencyToken11"),
            Self::CurrencyToken12 => f.write_str("CurrencyToken12"),
            Self::CurrencyToken13 => f.write_str("CurrencyToken13"),
            Self::CurrencyToken14 => f.write_str("CurrencyToken14"),
            Self::CurrencyToken15 => f.write_str("CurrencyToken15"),
            Self::CurrencyToken16 => f.write_str("CurrencyToken16"),
            Self::CurrencyToken17 => f.write_str("CurrencyToken17"),
            Self::CurrencyToken18 => f.write_str("CurrencyToken18"),
            Self::CurrencyToken19 => f.write_str("CurrencyToken19"),
            Self::CurrencyToken20 => f.write_str("CurrencyToken20"),
            Self::CurrencyToken21 => f.write_str("CurrencyToken21"),
            Self::CurrencyToken22 => f.write_str("CurrencyToken22"),
            Self::CurrencyToken23 => f.write_str("CurrencyToken23"),
            Self::CurrencyToken24 => f.write_str("CurrencyToken24"),
            Self::CurrencyToken25 => f.write_str("CurrencyToken25"),
            Self::CurrencyToken26 => f.write_str("CurrencyToken26"),
            Self::CurrencyToken27 => f.write_str("CurrencyToken27"),
            Self::CurrencyToken28 => f.write_str("CurrencyToken28"),
            Self::CurrencyToken29 => f.write_str("CurrencyToken29"),
            Self::CurrencyToken30 => f.write_str("CurrencyToken30"),
            Self::CurrencyToken31 => f.write_str("CurrencyToken31"),
            Self::CurrencyToken32 => f.write_str("CurrencyToken32"),
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
            63 => Ok(Self::Bank25),
            64 => Ok(Self::Bank26),
            65 => Ok(Self::Bank27),
            66 => Ok(Self::Bank28),
            67 => Ok(Self::BankBagSlot1),
            68 => Ok(Self::BankBagSlot2),
            69 => Ok(Self::BankBagSlot3),
            70 => Ok(Self::BankBagSlot4),
            71 => Ok(Self::BankBagSlot5),
            72 => Ok(Self::BankBagSlot6),
            73 => Ok(Self::BankBagSlot7),
            74 => Ok(Self::VendorBuyback1),
            75 => Ok(Self::VendorBuyback2),
            76 => Ok(Self::VendorBuyback3),
            77 => Ok(Self::VendorBuyback4),
            78 => Ok(Self::VendorBuyback5),
            79 => Ok(Self::VendorBuyback6),
            80 => Ok(Self::VendorBuyback7),
            81 => Ok(Self::VendorBuyback8),
            82 => Ok(Self::VendorBuyback9),
            83 => Ok(Self::VendorBuyback10),
            84 => Ok(Self::VendorBuyback11),
            85 => Ok(Self::VendorBuyback12),
            86 => Ok(Self::Keyring1),
            87 => Ok(Self::Keyring2),
            88 => Ok(Self::Keyring3),
            89 => Ok(Self::Keyring4),
            90 => Ok(Self::Keyring5),
            91 => Ok(Self::Keyring6),
            92 => Ok(Self::Keyring7),
            93 => Ok(Self::Keyring8),
            94 => Ok(Self::Keyring9),
            95 => Ok(Self::Keyring10),
            96 => Ok(Self::Keyring11),
            97 => Ok(Self::Keyring12),
            98 => Ok(Self::Keyring13),
            99 => Ok(Self::Keyring14),
            100 => Ok(Self::Keyring15),
            101 => Ok(Self::Keyring16),
            102 => Ok(Self::Keyring17),
            103 => Ok(Self::Keyring18),
            104 => Ok(Self::Keyring19),
            105 => Ok(Self::Keyring20),
            106 => Ok(Self::Keyring21),
            107 => Ok(Self::Keyring22),
            108 => Ok(Self::Keyring23),
            109 => Ok(Self::Keyring24),
            110 => Ok(Self::Keyring25),
            111 => Ok(Self::Keyring26),
            112 => Ok(Self::Keyring27),
            113 => Ok(Self::Keyring28),
            114 => Ok(Self::Keyring29),
            115 => Ok(Self::Keyring30),
            116 => Ok(Self::Keyring31),
            117 => Ok(Self::Keyring32),
            118 => Ok(Self::CurrencyToken1),
            119 => Ok(Self::CurrencyToken2),
            120 => Ok(Self::CurrencyToken3),
            121 => Ok(Self::CurrencyToken4),
            122 => Ok(Self::CurrencyToken5),
            123 => Ok(Self::CurrencyToken6),
            124 => Ok(Self::CurrencyToken7),
            125 => Ok(Self::CurrencyToken8),
            126 => Ok(Self::CurrencyToken9),
            127 => Ok(Self::CurrencyToken10),
            128 => Ok(Self::CurrencyToken11),
            129 => Ok(Self::CurrencyToken12),
            130 => Ok(Self::CurrencyToken13),
            131 => Ok(Self::CurrencyToken14),
            132 => Ok(Self::CurrencyToken15),
            133 => Ok(Self::CurrencyToken16),
            134 => Ok(Self::CurrencyToken17),
            135 => Ok(Self::CurrencyToken18),
            136 => Ok(Self::CurrencyToken19),
            137 => Ok(Self::CurrencyToken20),
            138 => Ok(Self::CurrencyToken21),
            139 => Ok(Self::CurrencyToken22),
            140 => Ok(Self::CurrencyToken23),
            141 => Ok(Self::CurrencyToken24),
            142 => Ok(Self::CurrencyToken25),
            143 => Ok(Self::CurrencyToken26),
            144 => Ok(Self::CurrencyToken27),
            145 => Ok(Self::CurrencyToken28),
            146 => Ok(Self::CurrencyToken29),
            147 => Ok(Self::CurrencyToken30),
            148 => Ok(Self::CurrencyToken31),
            149 => Ok(Self::CurrencyToken32),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
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
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
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

