/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm#L1):
/// ```text
/// enum AuctionHouse : u32 {
///     STORMWIND = 1;
///     ALLIANCE = 2;
///     DARNASSUS = 3;
///     UNDERCITY = 4;
///     THUNDER_BLUFF = 5;
///     HORDE = 6;
///     GOBLIN = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AuctionHouse {
    Stormwind,
    Alliance,
    Darnassus,
    Undercity,
    ThunderBluff,
    Horde,
    Goblin,
}

impl AuctionHouse {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Stormwind => 0x1,
            Self::Alliance => 0x2,
            Self::Darnassus => 0x3,
            Self::Undercity => 0x4,
            Self::ThunderBluff => 0x5,
            Self::Horde => 0x6,
            Self::Goblin => 0x7,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl AuctionHouse {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Stormwind => "STORMWIND",
            Self::Alliance => "ALLIANCE",
            Self::Darnassus => "DARNASSUS",
            Self::Undercity => "UNDERCITY",
            Self::ThunderBluff => "THUNDER_BLUFF",
            Self::Horde => "HORDE",
            Self::Goblin => "GOBLIN",
        }
    }

}

impl Default for AuctionHouse {
    fn default() -> Self {
        Self::Stormwind
    }
}

impl std::fmt::Display for AuctionHouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stormwind => f.write_str("Stormwind"),
            Self::Alliance => f.write_str("Alliance"),
            Self::Darnassus => f.write_str("Darnassus"),
            Self::Undercity => f.write_str("Undercity"),
            Self::ThunderBluff => f.write_str("ThunderBluff"),
            Self::Horde => f.write_str("Horde"),
            Self::Goblin => f.write_str("Goblin"),
        }
    }
}

impl TryFrom<u32> for AuctionHouse {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Stormwind),
            2 => Ok(Self::Alliance),
            3 => Ok(Self::Darnassus),
            4 => Ok(Self::Undercity),
            5 => Ok(Self::ThunderBluff),
            6 => Ok(Self::Horde),
            7 => Ok(Self::Goblin),
            v => Err(crate::errors::EnumError::new("AuctionHouse", v.into()),)
        }
    }
}

