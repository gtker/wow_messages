use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ItemClass {
    CONSUMABLE,
    CONTAINER,
    WEAPON,
    RESERVED_1,
    ARMOR,
    REAGENT,
    PROJECTILE,
    TRADE_GOODS,
    RESERVED_2,
    RECIPE,
    RESERVED_3,
    QUIVER,
    QUEST,
    KEY,
    RESERVED_4,
    MISC,
}

impl ItemClass {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::CONSUMABLE => 0x0,
            Self::CONTAINER => 0x1,
            Self::WEAPON => 0x2,
            Self::RESERVED_1 => 0x3,
            Self::ARMOR => 0x4,
            Self::REAGENT => 0x5,
            Self::PROJECTILE => 0x6,
            Self::TRADE_GOODS => 0x7,
            Self::RESERVED_2 => 0x8,
            Self::RECIPE => 0x9,
            Self::RESERVED_3 => 0xa,
            Self::QUIVER => 0xb,
            Self::QUEST => 0xc,
            Self::KEY => 0xd,
            Self::RESERVED_4 => 0xe,
            Self::MISC => 0xf,
        }
    }

}

impl Default for ItemClass {
    fn default() -> Self {
        Self::CONSUMABLE
    }
}

impl std::fmt::Display for ItemClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CONSUMABLE => f.write_str("CONSUMABLE"),
            Self::CONTAINER => f.write_str("CONTAINER"),
            Self::WEAPON => f.write_str("WEAPON"),
            Self::RESERVED_1 => f.write_str("RESERVED_1"),
            Self::ARMOR => f.write_str("ARMOR"),
            Self::REAGENT => f.write_str("REAGENT"),
            Self::PROJECTILE => f.write_str("PROJECTILE"),
            Self::TRADE_GOODS => f.write_str("TRADE_GOODS"),
            Self::RESERVED_2 => f.write_str("RESERVED_2"),
            Self::RECIPE => f.write_str("RECIPE"),
            Self::RESERVED_3 => f.write_str("RESERVED_3"),
            Self::QUIVER => f.write_str("QUIVER"),
            Self::QUEST => f.write_str("QUEST"),
            Self::KEY => f.write_str("KEY"),
            Self::RESERVED_4 => f.write_str("RESERVED_4"),
            Self::MISC => f.write_str("MISC"),
        }
    }
}

impl TryFrom<u8> for ItemClass {
    type Error = ItemClassError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CONSUMABLE),
            1 => Ok(Self::CONTAINER),
            2 => Ok(Self::WEAPON),
            3 => Ok(Self::RESERVED_1),
            4 => Ok(Self::ARMOR),
            5 => Ok(Self::REAGENT),
            6 => Ok(Self::PROJECTILE),
            7 => Ok(Self::TRADE_GOODS),
            8 => Ok(Self::RESERVED_2),
            9 => Ok(Self::RECIPE),
            10 => Ok(Self::RESERVED_3),
            11 => Ok(Self::QUIVER),
            12 => Ok(Self::QUEST),
            13 => Ok(Self::KEY),
            14 => Ok(Self::RESERVED_4),
            15 => Ok(Self::MISC),
            _ => Err(ItemClassError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct ItemClassError {
    pub value: u8,
}

impl ItemClassError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for ItemClassError {}
impl std::fmt::Display for ItemClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ItemClass': '{}'", self.value))
    }
}

