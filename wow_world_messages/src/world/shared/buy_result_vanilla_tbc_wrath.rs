/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_buy_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_buy_failed.wowm#L3):
/// ```text
/// enum BuyResult : u8 {
///     CANT_FIND_ITEM = 0;
///     ITEM_ALREADY_SOLD = 1;
///     NOT_ENOUGHT_MONEY = 2;
///     SELLER_DONT_LIKE_YOU = 4;
///     DISTANCE_TOO_FAR = 5;
///     ITEM_SOLD_OUT = 7;
///     CANT_CARRY_MORE = 8;
///     RANK_REQUIRE = 11;
///     REPUTATION_REQUIRE = 12;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BuyResult {
    CantFindItem,
    ItemAlreadySold,
    NotEnoughtMoney,
    SellerDontLikeYou,
    DistanceTooFar,
    ItemSoldOut,
    CantCarryMore,
    RankRequire,
    ReputationRequire,
}

impl BuyResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::CantFindItem => 0x0,
            Self::ItemAlreadySold => 0x1,
            Self::NotEnoughtMoney => 0x2,
            Self::SellerDontLikeYou => 0x4,
            Self::DistanceTooFar => 0x5,
            Self::ItemSoldOut => 0x7,
            Self::CantCarryMore => 0x8,
            Self::RankRequire => 0xb,
            Self::ReputationRequire => 0xc,
        }
    }

}

impl Default for BuyResult {
    fn default() -> Self {
        Self::CantFindItem
    }
}

impl std::fmt::Display for BuyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CantFindItem => f.write_str("CantFindItem"),
            Self::ItemAlreadySold => f.write_str("ItemAlreadySold"),
            Self::NotEnoughtMoney => f.write_str("NotEnoughtMoney"),
            Self::SellerDontLikeYou => f.write_str("SellerDontLikeYou"),
            Self::DistanceTooFar => f.write_str("DistanceTooFar"),
            Self::ItemSoldOut => f.write_str("ItemSoldOut"),
            Self::CantCarryMore => f.write_str("CantCarryMore"),
            Self::RankRequire => f.write_str("RankRequire"),
            Self::ReputationRequire => f.write_str("ReputationRequire"),
        }
    }
}

impl TryFrom<u8> for BuyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CantFindItem),
            1 => Ok(Self::ItemAlreadySold),
            2 => Ok(Self::NotEnoughtMoney),
            4 => Ok(Self::SellerDontLikeYou),
            5 => Ok(Self::DistanceTooFar),
            7 => Ok(Self::ItemSoldOut),
            8 => Ok(Self::CantCarryMore),
            11 => Ok(Self::RankRequire),
            12 => Ok(Self::ReputationRequire),
            v => Err(crate::errors::EnumError::new("BuyResult", v as u64),)
        }
    }
}

