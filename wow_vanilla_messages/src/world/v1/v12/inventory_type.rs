use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum InventoryType {
    NON_EQUIP,
    HEAD,
    NECK_OR_RELIC,
    SHOULDERS,
    BODY,
    CHEST,
    WAIST,
    LEGS,
    FEET,
    WRISTS,
    HANDS,
    FINGER,
    TRINKET,
    WEAPON,
    SHIELD,
    RANGED,
    CLOAK,
    TWO_HANDED_WEAPON,
    BAG,
    TABARD,
    ROBE,
    WEAPON_MAIN_HAND,
    WEAPON_OFF_HAND,
    HOLDABLE,
    AMMO,
    THROWN,
    RANGED_RIGHT,
    QUIVER,
}

impl InventoryType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NON_EQUIP => 0x0,
            Self::HEAD => 0x1,
            Self::NECK_OR_RELIC => 0x2,
            Self::SHOULDERS => 0x3,
            Self::BODY => 0x4,
            Self::CHEST => 0x5,
            Self::WAIST => 0x6,
            Self::LEGS => 0x7,
            Self::FEET => 0x8,
            Self::WRISTS => 0x9,
            Self::HANDS => 0xa,
            Self::FINGER => 0xb,
            Self::TRINKET => 0xc,
            Self::WEAPON => 0xd,
            Self::SHIELD => 0xe,
            Self::RANGED => 0xf,
            Self::CLOAK => 0x10,
            Self::TWO_HANDED_WEAPON => 0x11,
            Self::BAG => 0x12,
            Self::TABARD => 0x13,
            Self::ROBE => 0x14,
            Self::WEAPON_MAIN_HAND => 0x15,
            Self::WEAPON_OFF_HAND => 0x16,
            Self::HOLDABLE => 0x17,
            Self::AMMO => 0x18,
            Self::THROWN => 0x19,
            Self::RANGED_RIGHT => 0x1a,
            Self::QUIVER => 0x1b,
        }
    }

}

impl Default for InventoryType {
    fn default() -> Self {
        Self::NON_EQUIP
    }
}

impl std::fmt::Display for InventoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NON_EQUIP => f.write_str("NON_EQUIP"),
            Self::HEAD => f.write_str("HEAD"),
            Self::NECK_OR_RELIC => f.write_str("NECK_OR_RELIC"),
            Self::SHOULDERS => f.write_str("SHOULDERS"),
            Self::BODY => f.write_str("BODY"),
            Self::CHEST => f.write_str("CHEST"),
            Self::WAIST => f.write_str("WAIST"),
            Self::LEGS => f.write_str("LEGS"),
            Self::FEET => f.write_str("FEET"),
            Self::WRISTS => f.write_str("WRISTS"),
            Self::HANDS => f.write_str("HANDS"),
            Self::FINGER => f.write_str("FINGER"),
            Self::TRINKET => f.write_str("TRINKET"),
            Self::WEAPON => f.write_str("WEAPON"),
            Self::SHIELD => f.write_str("SHIELD"),
            Self::RANGED => f.write_str("RANGED"),
            Self::CLOAK => f.write_str("CLOAK"),
            Self::TWO_HANDED_WEAPON => f.write_str("TWO_HANDED_WEAPON"),
            Self::BAG => f.write_str("BAG"),
            Self::TABARD => f.write_str("TABARD"),
            Self::ROBE => f.write_str("ROBE"),
            Self::WEAPON_MAIN_HAND => f.write_str("WEAPON_MAIN_HAND"),
            Self::WEAPON_OFF_HAND => f.write_str("WEAPON_OFF_HAND"),
            Self::HOLDABLE => f.write_str("HOLDABLE"),
            Self::AMMO => f.write_str("AMMO"),
            Self::THROWN => f.write_str("THROWN"),
            Self::RANGED_RIGHT => f.write_str("RANGED_RIGHT"),
            Self::QUIVER => f.write_str("QUIVER"),
        }
    }
}

impl TryFrom<u8> for InventoryType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NON_EQUIP),
            1 => Ok(Self::HEAD),
            2 => Ok(Self::NECK_OR_RELIC),
            3 => Ok(Self::SHOULDERS),
            4 => Ok(Self::BODY),
            5 => Ok(Self::CHEST),
            6 => Ok(Self::WAIST),
            7 => Ok(Self::LEGS),
            8 => Ok(Self::FEET),
            9 => Ok(Self::WRISTS),
            10 => Ok(Self::HANDS),
            11 => Ok(Self::FINGER),
            12 => Ok(Self::TRINKET),
            13 => Ok(Self::WEAPON),
            14 => Ok(Self::SHIELD),
            15 => Ok(Self::RANGED),
            16 => Ok(Self::CLOAK),
            17 => Ok(Self::TWO_HANDED_WEAPON),
            18 => Ok(Self::BAG),
            19 => Ok(Self::TABARD),
            20 => Ok(Self::ROBE),
            21 => Ok(Self::WEAPON_MAIN_HAND),
            22 => Ok(Self::WEAPON_OFF_HAND),
            23 => Ok(Self::HOLDABLE),
            24 => Ok(Self::AMMO),
            25 => Ok(Self::THROWN),
            26 => Ok(Self::RANGED_RIGHT),
            27 => Ok(Self::QUIVER),
            v => Err(crate::errors::EnumError::new("InventoryType", v as u32),)
        }
    }
}

