/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm:76`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm#L76):
/// ```text
/// enum AuctionCommandResultTwo : u32 {
///     OK = 0;
///     ERR_INVENTORY = 1;
///     ERR_DATABASE = 2;
///     ERR_NOT_ENOUGH_MONEY = 3;
///     ERR_ITEM_NOT_FOUND = 4;
///     ERR_HIGHER_BID = 5;
///     ERR_BID_INCREMENT = 7;
///     ERR_BID_OWN = 10;
///     ERR_RESTRICTED_ACCOUNT = 13;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AuctionCommandResultTwo {
    /// depends on enum AuctionAction
    ///
    Ok,
    /// depends on enum `InventoryChangeResult`
    ///
    ErrInventory,
    /// ERR_AUCTION_DATABASE_ERROR (default)
    ///
    ErrDatabase,
    /// ERR_NOT_ENOUGH_MONEY
    ///
    ErrNotEnoughMoney,
    /// ERR_ITEM_NOT_FOUND
    ///
    ErrItemNotFound,
    /// ERR_AUCTION_HIGHER_BID
    ///
    ErrHigherBid,
    /// ERR_AUCTION_BID_INCREMENT
    ///
    ErrBidIncrement,
    /// ERR_AUCTION_BID_OWN
    ///
    ErrBidOwn,
    /// ERR_RESTRICTED_ACCOUNT
    ///
    ErrRestrictedAccount,
}

impl AuctionCommandResultTwo {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Ok => 0x0,
            Self::ErrInventory => 0x1,
            Self::ErrDatabase => 0x2,
            Self::ErrNotEnoughMoney => 0x3,
            Self::ErrItemNotFound => 0x4,
            Self::ErrHigherBid => 0x5,
            Self::ErrBidIncrement => 0x7,
            Self::ErrBidOwn => 0xa,
            Self::ErrRestrictedAccount => 0xd,
        }
    }

}

impl Default for AuctionCommandResultTwo {
    fn default() -> Self {
        Self::Ok
    }
}

impl std::fmt::Display for AuctionCommandResultTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::ErrInventory => f.write_str("ErrInventory"),
            Self::ErrDatabase => f.write_str("ErrDatabase"),
            Self::ErrNotEnoughMoney => f.write_str("ErrNotEnoughMoney"),
            Self::ErrItemNotFound => f.write_str("ErrItemNotFound"),
            Self::ErrHigherBid => f.write_str("ErrHigherBid"),
            Self::ErrBidIncrement => f.write_str("ErrBidIncrement"),
            Self::ErrBidOwn => f.write_str("ErrBidOwn"),
            Self::ErrRestrictedAccount => f.write_str("ErrRestrictedAccount"),
        }
    }
}

impl TryFrom<u32> for AuctionCommandResultTwo {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::ErrInventory),
            2 => Ok(Self::ErrDatabase),
            3 => Ok(Self::ErrNotEnoughMoney),
            4 => Ok(Self::ErrItemNotFound),
            5 => Ok(Self::ErrHigherBid),
            7 => Ok(Self::ErrBidIncrement),
            10 => Ok(Self::ErrBidOwn),
            13 => Ok(Self::ErrRestrictedAccount),
            v => Err(crate::errors::EnumError::new("AuctionCommandResultTwo", v as u64),)
        }
    }
}

