/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm#L1):
/// ```text
/// enum AuctionCommandAction : u32 {
///     STARTED = 0;
///     REMOVED = 1;
///     BID_PLACED = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AuctionCommandAction {
    /// ERR_AUCTION_STARTED
    Started,
    /// ERR_AUCTION_REMOVED
    Removed,
    /// ERR_AUCTION_BID_PLACED
    BidPlaced,
}

impl AuctionCommandAction {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Started => 0x0,
            Self::Removed => 0x1,
            Self::BidPlaced => 0x2,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl AuctionCommandAction {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Started => "STARTED",
            Self::Removed => "REMOVED",
            Self::BidPlaced => "BID_PLACED",
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
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Started),
            1 => Ok(Self::Removed),
            2 => Ok(Self::BidPlaced),
            v => Err(crate::errors::EnumError::new("AuctionCommandAction", v.into()),)
        }
    }
}

