/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_sell_item.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_sell_item.wowm#L30):
/// ```text
/// enum SellItemResult : u8 {
///     ERR_CANT_FIND_ITEM = 1;
///     ERR_CANT_SELL_ITEM = 2;
///     ERR_CANT_FIND_VENDOR = 3;
///     ERR_YOU_DONT_OWN_THAT_ITEM = 4;
///     ERR_UNK = 5;
///     ERR_ONLY_EMPTY_BAG = 6;
///     ERR_CANT_SELL_TO_THIS_MERCHANT = 7;
///     ERR_MUST_REPAIR_ITEM_DURABILITY_TO_USE = 8;
///     INTERNAL_BAG_ERROR = 9;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SellItemResult {
    /// The item was not found.
    ///
    ErrCantFindItem,
    /// The merchant doesn't want that item.
    ///
    ErrCantSellItem,
    /// The merchant doesn't like you.
    ///
    ErrCantFindVendor,
    /// You don't own that item.
    ///
    ErrYouDontOwnThatItem,
    /// Nothing appears...
    ///
    ErrUnk,
    /// You can only do that with empty bags.
    ///
    ErrOnlyEmptyBag,
    /// You cannot sell items to this merchant.
    ///
    ErrCantSellToThisMerchant,
    /// You must repair that item's durability to use it.
    ///
    ErrMustRepairItemDurabilityToUse,
    /// Internal Bag Error
    ///
    InternalBagError,
}

impl SellItemResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::ErrCantFindItem => 0x1,
            Self::ErrCantSellItem => 0x2,
            Self::ErrCantFindVendor => 0x3,
            Self::ErrYouDontOwnThatItem => 0x4,
            Self::ErrUnk => 0x5,
            Self::ErrOnlyEmptyBag => 0x6,
            Self::ErrCantSellToThisMerchant => 0x7,
            Self::ErrMustRepairItemDurabilityToUse => 0x8,
            Self::InternalBagError => 0x9,
        }
    }

}

impl Default for SellItemResult {
    fn default() -> Self {
        Self::ErrCantFindItem
    }
}

impl std::fmt::Display for SellItemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErrCantFindItem => f.write_str("ErrCantFindItem"),
            Self::ErrCantSellItem => f.write_str("ErrCantSellItem"),
            Self::ErrCantFindVendor => f.write_str("ErrCantFindVendor"),
            Self::ErrYouDontOwnThatItem => f.write_str("ErrYouDontOwnThatItem"),
            Self::ErrUnk => f.write_str("ErrUnk"),
            Self::ErrOnlyEmptyBag => f.write_str("ErrOnlyEmptyBag"),
            Self::ErrCantSellToThisMerchant => f.write_str("ErrCantSellToThisMerchant"),
            Self::ErrMustRepairItemDurabilityToUse => f.write_str("ErrMustRepairItemDurabilityToUse"),
            Self::InternalBagError => f.write_str("InternalBagError"),
        }
    }
}

impl TryFrom<u8> for SellItemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ErrCantFindItem),
            2 => Ok(Self::ErrCantSellItem),
            3 => Ok(Self::ErrCantFindVendor),
            4 => Ok(Self::ErrYouDontOwnThatItem),
            5 => Ok(Self::ErrUnk),
            6 => Ok(Self::ErrOnlyEmptyBag),
            7 => Ok(Self::ErrCantSellToThisMerchant),
            8 => Ok(Self::ErrMustRepairItemDurabilityToUse),
            9 => Ok(Self::InternalBagError),
            v => Err(crate::errors::EnumError::new("SellItemResult", v as u64),)
        }
    }
}

