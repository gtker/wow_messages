/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_buy_bank_slot_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_buy_bank_slot_result.wowm#L3):
/// ```text
/// enum BuyBankSlotResult : u32 {
///     FAILED_TOO_MANY = 0;
///     INSUFFICIENT_FUNDS = 1;
///     NOT_BANKER = 2;
///     OK = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BuyBankSlotResult {
    FailedTooMany,
    InsufficientFunds,
    NotBanker,
    Ok,
}

impl BuyBankSlotResult {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::FailedTooMany => 0x0,
            Self::InsufficientFunds => 0x1,
            Self::NotBanker => 0x2,
            Self::Ok => 0x3,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl BuyBankSlotResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::FailedTooMany => "FAILED_TOO_MANY",
            Self::InsufficientFunds => "INSUFFICIENT_FUNDS",
            Self::NotBanker => "NOT_BANKER",
            Self::Ok => "OK",
        }
    }

}

impl Default for BuyBankSlotResult {
    fn default() -> Self {
        Self::FailedTooMany
    }
}

impl std::fmt::Display for BuyBankSlotResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedTooMany => f.write_str("FailedTooMany"),
            Self::InsufficientFunds => f.write_str("InsufficientFunds"),
            Self::NotBanker => f.write_str("NotBanker"),
            Self::Ok => f.write_str("Ok"),
        }
    }
}

impl TryFrom<u32> for BuyBankSlotResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FailedTooMany),
            1 => Ok(Self::InsufficientFunds),
            2 => Ok(Self::NotBanker),
            3 => Ok(Self::Ok),
            v => Err(crate::errors::EnumError::new("BuyBankSlotResult", v as u64),)
        }
    }
}

