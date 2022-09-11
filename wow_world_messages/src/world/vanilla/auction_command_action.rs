use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/unimplemented/vanilla/smsg_auction_command_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/unimplemented/vanilla/smsg_auction_command_result.wowm#L3):
/// ```text
/// enum AuctionCommandAction : u32 {
///     STARTED = 0;
///     REMOVED = 1;
///     BID_PLACED = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum AuctionCommandAction {
    /// ERR_AUCTION_STARTED
    ///
    Started,
    /// ERR_AUCTION_REMOVED
    ///
    Removed,
    /// ERR_AUCTION_BID_PLACED
    ///
    BidPlaced,
}

impl AuctionCommandAction {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Started => 0x0,
            Self::Removed => 0x1,
            Self::BidPlaced => 0x2,
        }
    }

}

impl Default for AuctionCommandAction {
    fn default() -> Self {
        Self::Started
    }
}

impl std::fmt::Display for AuctionCommandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Started => f.write_str("Started"),
            Self::Removed => f.write_str("Removed"),
            Self::BidPlaced => f.write_str("BidPlaced"),
        }
    }
}

impl TryFrom<u32> for AuctionCommandAction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Started),
            1 => Ok(Self::Removed),
            2 => Ok(Self::BidPlaced),
            v => Err(crate::errors::EnumError::new("AuctionCommandAction", v as u32),)
        }
    }
}

