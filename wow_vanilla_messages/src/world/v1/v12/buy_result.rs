use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BuyResult {
    CANT_FIND_ITEM,
    ITEM_ALREADY_SOLD,
    NOT_ENOUGHT_MONEY,
    SELLER_DONT_LIKE_YOU,
    DISTANCE_TOO_FAR,
    ITEM_SOLD_OUT,
    CANT_CARRY_MORE,
    RANK_REQUIRE,
    REPUTATION_REQUIRE,
}

impl BuyResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::CANT_FIND_ITEM => 0x0,
            Self::ITEM_ALREADY_SOLD => 0x1,
            Self::NOT_ENOUGHT_MONEY => 0x2,
            Self::SELLER_DONT_LIKE_YOU => 0x4,
            Self::DISTANCE_TOO_FAR => 0x5,
            Self::ITEM_SOLD_OUT => 0x7,
            Self::CANT_CARRY_MORE => 0x8,
            Self::RANK_REQUIRE => 0xb,
            Self::REPUTATION_REQUIRE => 0xc,
        }
    }

}

impl Default for BuyResult {
    fn default() -> Self {
        Self::CANT_FIND_ITEM
    }
}

impl std::fmt::Display for BuyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CANT_FIND_ITEM => f.write_str("CANT_FIND_ITEM"),
            Self::ITEM_ALREADY_SOLD => f.write_str("ITEM_ALREADY_SOLD"),
            Self::NOT_ENOUGHT_MONEY => f.write_str("NOT_ENOUGHT_MONEY"),
            Self::SELLER_DONT_LIKE_YOU => f.write_str("SELLER_DONT_LIKE_YOU"),
            Self::DISTANCE_TOO_FAR => f.write_str("DISTANCE_TOO_FAR"),
            Self::ITEM_SOLD_OUT => f.write_str("ITEM_SOLD_OUT"),
            Self::CANT_CARRY_MORE => f.write_str("CANT_CARRY_MORE"),
            Self::RANK_REQUIRE => f.write_str("RANK_REQUIRE"),
            Self::REPUTATION_REQUIRE => f.write_str("REPUTATION_REQUIRE"),
        }
    }
}

impl TryFrom<u8> for BuyResult {
    type Error = BuyResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CANT_FIND_ITEM),
            1 => Ok(Self::ITEM_ALREADY_SOLD),
            2 => Ok(Self::NOT_ENOUGHT_MONEY),
            4 => Ok(Self::SELLER_DONT_LIKE_YOU),
            5 => Ok(Self::DISTANCE_TOO_FAR),
            7 => Ok(Self::ITEM_SOLD_OUT),
            8 => Ok(Self::CANT_CARRY_MORE),
            11 => Ok(Self::RANK_REQUIRE),
            12 => Ok(Self::REPUTATION_REQUIRE),
            _ => Err(BuyResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct BuyResultError {
    pub value: u8,
}

impl BuyResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for BuyResultError {}
impl std::fmt::Display for BuyResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BuyResult': '{}'", self.value))
    }
}

