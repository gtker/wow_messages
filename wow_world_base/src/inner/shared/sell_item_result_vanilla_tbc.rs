/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_sell_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_sell_item.wowm#L1):
/// ```text
/// enum SellItemResult : u8 {
///     CANT_FIND_ITEM = 1;
///     CANT_SELL_ITEM = 2;
///     CANT_FIND_VENDOR = 3;
///     YOU_DONT_OWN_THAT_ITEM = 4;
///     UNK = 5;
///     ONLY_EMPTY_BAG = 6;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SellItemResult {
    CantFindItem,
    /// cmangos/vmangos/mangoszero: merchant doesn't like that item
    CantSellItem,
    /// cmangos/vmangos/mangoszero: merchant doesn't like you
    CantFindVendor,
    /// cmangos/vmangos/mangoszero: you don't own that item
    YouDontOwnThatItem,
    /// cmangos/vmangos/mangoszero: nothing appears...
    Unk,
    /// cmangos/vmangos/mangoszero: can only do with empty bags
    OnlyEmptyBag,
}

impl SellItemResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::CantFindItem => 0x1,
            Self::CantSellItem => 0x2,
            Self::CantFindVendor => 0x3,
            Self::YouDontOwnThatItem => 0x4,
            Self::Unk => 0x5,
            Self::OnlyEmptyBag => 0x6,
        }
    }

    pub const fn variants() -> [Self; 6] {
        [
            Self::CantFindItem,
            Self::CantSellItem,
            Self::CantFindVendor,
            Self::YouDontOwnThatItem,
            Self::Unk,
            Self::OnlyEmptyBag,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl SellItemResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::CantFindItem => "CANT_FIND_ITEM",
            Self::CantSellItem => "CANT_SELL_ITEM",
            Self::CantFindVendor => "CANT_FIND_VENDOR",
            Self::YouDontOwnThatItem => "YOU_DONT_OWN_THAT_ITEM",
            Self::Unk => "UNK",
            Self::OnlyEmptyBag => "ONLY_EMPTY_BAG",
        }
    }

}

const NAME: &str = "SellItemResult";

impl Default for SellItemResult {
    fn default() -> Self {
        Self::CantFindItem
    }
}

impl std::fmt::Display for SellItemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CantFindItem => f.write_str("CantFindItem"),
            Self::CantSellItem => f.write_str("CantSellItem"),
            Self::CantFindVendor => f.write_str("CantFindVendor"),
            Self::YouDontOwnThatItem => f.write_str("YouDontOwnThatItem"),
            Self::Unk => f.write_str("Unk"),
            Self::OnlyEmptyBag => f.write_str("OnlyEmptyBag"),
        }
    }
}

impl TryFrom<u8> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::CantFindItem),
            2 => Ok(Self::CantSellItem),
            3 => Ok(Self::CantFindVendor),
            4 => Ok(Self::YouDontOwnThatItem),
            5 => Ok(Self::Unk),
            6 => Ok(Self::OnlyEmptyBag),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

