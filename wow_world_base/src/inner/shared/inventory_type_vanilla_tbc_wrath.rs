/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/inventory_type.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/inventory_type.wowm#L3):
/// ```text
/// enum InventoryType : u8 {
///     NON_EQUIP = 0;
///     HEAD = 1;
///     NECK = 2;
///     SHOULDERS = 3;
///     BODY = 4;
///     CHEST = 5;
///     WAIST = 6;
///     LEGS = 7;
///     FEET = 8;
///     WRISTS = 9;
///     HANDS = 10;
///     FINGER = 11;
///     TRINKET = 12;
///     WEAPON = 13;
///     SHIELD = 14;
///     RANGED = 15;
///     CLOAK = 16;
///     TWO_HANDED_WEAPON = 17;
///     BAG = 18;
///     TABARD = 19;
///     ROBE = 20;
///     WEAPON_MAIN_HAND = 21;
///     WEAPON_OFF_HAND = 22;
///     HOLDABLE = 23;
///     AMMO = 24;
///     THROWN = 25;
///     RANGED_RIGHT = 26;
///     QUIVER = 27;
///     RELIC = 28;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum InventoryType {
    NonEquip,
    Head,
    Neck,
    Shoulders,
    Body,
    Chest,
    Waist,
    Legs,
    Feet,
    Wrists,
    Hands,
    Finger,
    Trinket,
    Weapon,
    Shield,
    Ranged,
    Cloak,
    TwoHandedWeapon,
    Bag,
    Tabard,
    Robe,
    WeaponMainHand,
    WeaponOffHand,
    Holdable,
    Ammo,
    Thrown,
    RangedRight,
    Quiver,
    Relic,
}

impl InventoryType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NonEquip => 0x0,
            Self::Head => 0x1,
            Self::Neck => 0x2,
            Self::Shoulders => 0x3,
            Self::Body => 0x4,
            Self::Chest => 0x5,
            Self::Waist => 0x6,
            Self::Legs => 0x7,
            Self::Feet => 0x8,
            Self::Wrists => 0x9,
            Self::Hands => 0xa,
            Self::Finger => 0xb,
            Self::Trinket => 0xc,
            Self::Weapon => 0xd,
            Self::Shield => 0xe,
            Self::Ranged => 0xf,
            Self::Cloak => 0x10,
            Self::TwoHandedWeapon => 0x11,
            Self::Bag => 0x12,
            Self::Tabard => 0x13,
            Self::Robe => 0x14,
            Self::WeaponMainHand => 0x15,
            Self::WeaponOffHand => 0x16,
            Self::Holdable => 0x17,
            Self::Ammo => 0x18,
            Self::Thrown => 0x19,
            Self::RangedRight => 0x1a,
            Self::Quiver => 0x1b,
            Self::Relic => 0x1c,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl InventoryType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NonEquip => "NON_EQUIP",
            Self::Head => "HEAD",
            Self::Neck => "NECK",
            Self::Shoulders => "SHOULDERS",
            Self::Body => "BODY",
            Self::Chest => "CHEST",
            Self::Waist => "WAIST",
            Self::Legs => "LEGS",
            Self::Feet => "FEET",
            Self::Wrists => "WRISTS",
            Self::Hands => "HANDS",
            Self::Finger => "FINGER",
            Self::Trinket => "TRINKET",
            Self::Weapon => "WEAPON",
            Self::Shield => "SHIELD",
            Self::Ranged => "RANGED",
            Self::Cloak => "CLOAK",
            Self::TwoHandedWeapon => "TWO_HANDED_WEAPON",
            Self::Bag => "BAG",
            Self::Tabard => "TABARD",
            Self::Robe => "ROBE",
            Self::WeaponMainHand => "WEAPON_MAIN_HAND",
            Self::WeaponOffHand => "WEAPON_OFF_HAND",
            Self::Holdable => "HOLDABLE",
            Self::Ammo => "AMMO",
            Self::Thrown => "THROWN",
            Self::RangedRight => "RANGED_RIGHT",
            Self::Quiver => "QUIVER",
            Self::Relic => "RELIC",
        }
    }

}

impl Default for InventoryType {
    fn default() -> Self {
        Self::NonEquip
    }
}

impl std::fmt::Display for InventoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonEquip => f.write_str("NonEquip"),
            Self::Head => f.write_str("Head"),
            Self::Neck => f.write_str("Neck"),
            Self::Shoulders => f.write_str("Shoulders"),
            Self::Body => f.write_str("Body"),
            Self::Chest => f.write_str("Chest"),
            Self::Waist => f.write_str("Waist"),
            Self::Legs => f.write_str("Legs"),
            Self::Feet => f.write_str("Feet"),
            Self::Wrists => f.write_str("Wrists"),
            Self::Hands => f.write_str("Hands"),
            Self::Finger => f.write_str("Finger"),
            Self::Trinket => f.write_str("Trinket"),
            Self::Weapon => f.write_str("Weapon"),
            Self::Shield => f.write_str("Shield"),
            Self::Ranged => f.write_str("Ranged"),
            Self::Cloak => f.write_str("Cloak"),
            Self::TwoHandedWeapon => f.write_str("TwoHandedWeapon"),
            Self::Bag => f.write_str("Bag"),
            Self::Tabard => f.write_str("Tabard"),
            Self::Robe => f.write_str("Robe"),
            Self::WeaponMainHand => f.write_str("WeaponMainHand"),
            Self::WeaponOffHand => f.write_str("WeaponOffHand"),
            Self::Holdable => f.write_str("Holdable"),
            Self::Ammo => f.write_str("Ammo"),
            Self::Thrown => f.write_str("Thrown"),
            Self::RangedRight => f.write_str("RangedRight"),
            Self::Quiver => f.write_str("Quiver"),
            Self::Relic => f.write_str("Relic"),
        }
    }
}

impl TryFrom<u8> for InventoryType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NonEquip),
            1 => Ok(Self::Head),
            2 => Ok(Self::Neck),
            3 => Ok(Self::Shoulders),
            4 => Ok(Self::Body),
            5 => Ok(Self::Chest),
            6 => Ok(Self::Waist),
            7 => Ok(Self::Legs),
            8 => Ok(Self::Feet),
            9 => Ok(Self::Wrists),
            10 => Ok(Self::Hands),
            11 => Ok(Self::Finger),
            12 => Ok(Self::Trinket),
            13 => Ok(Self::Weapon),
            14 => Ok(Self::Shield),
            15 => Ok(Self::Ranged),
            16 => Ok(Self::Cloak),
            17 => Ok(Self::TwoHandedWeapon),
            18 => Ok(Self::Bag),
            19 => Ok(Self::Tabard),
            20 => Ok(Self::Robe),
            21 => Ok(Self::WeaponMainHand),
            22 => Ok(Self::WeaponOffHand),
            23 => Ok(Self::Holdable),
            24 => Ok(Self::Ammo),
            25 => Ok(Self::Thrown),
            26 => Ok(Self::RangedRight),
            27 => Ok(Self::Quiver),
            28 => Ok(Self::Relic),
            v => Err(crate::errors::EnumError::new("InventoryType", v as u64),)
        }
    }
}

