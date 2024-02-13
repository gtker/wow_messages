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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
    pub const fn as_int(&self) -> u8 {
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

    pub const fn variants() -> [Self; 9] {
        [
            Self::CantFindItem,
            Self::ItemAlreadySold,
            Self::NotEnoughtMoney,
            Self::SellerDontLikeYou,
            Self::DistanceTooFar,
            Self::ItemSoldOut,
            Self::CantCarryMore,
            Self::RankRequire,
            Self::ReputationRequire,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
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
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl BuyResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::CantFindItem => "CANT_FIND_ITEM",
            Self::ItemAlreadySold => "ITEM_ALREADY_SOLD",
            Self::NotEnoughtMoney => "NOT_ENOUGHT_MONEY",
            Self::SellerDontLikeYou => "SELLER_DONT_LIKE_YOU",
            Self::DistanceTooFar => "DISTANCE_TOO_FAR",
            Self::ItemSoldOut => "ITEM_SOLD_OUT",
            Self::CantCarryMore => "CANT_CARRY_MORE",
            Self::RankRequire => "RANK_REQUIRE",
            Self::ReputationRequire => "REPUTATION_REQUIRE",
        }
    }

}

const NAME: &str = "BuyResult";

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
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for BuyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for BuyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for BuyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for BuyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for BuyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for BuyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for BuyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for BuyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

