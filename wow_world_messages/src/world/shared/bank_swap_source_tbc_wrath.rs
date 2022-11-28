use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_guild_bank_swap_items.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_guild_bank_swap_items.wowm#L1):
/// ```text
/// enum BankSwapSource : u8 {
///     INVENTORY = 0;
///     BANK = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BankSwapSource {
    Inventory,
    Bank,
}

impl BankSwapSource {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Inventory => 0x0,
            Self::Bank => 0x1,
        }
    }

}

impl Default for BankSwapSource {
    fn default() -> Self {
        Self::Inventory
    }
}

impl std::fmt::Display for BankSwapSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inventory => f.write_str("Inventory"),
            Self::Bank => f.write_str("Bank"),
        }
    }
}

impl TryFrom<u8> for BankSwapSource {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Inventory),
            1 => Ok(Self::Bank),
            v => Err(crate::errors::EnumError::new("BankSwapSource", v as u32),)
        }
    }
}

