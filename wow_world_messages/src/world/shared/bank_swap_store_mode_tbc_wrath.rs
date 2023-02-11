/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_swap_items.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_swap_items.wowm#L8):
/// ```text
/// enum BankSwapStoreMode : u8 {
///     MANUAL = 0;
///     AUTOMATIC = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BankSwapStoreMode {
    Manual,
    Automatic,
}

impl BankSwapStoreMode {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Manual => 0x0,
            Self::Automatic => 0x1,
        }
    }

}

impl Default for BankSwapStoreMode {
    fn default() -> Self {
        Self::Manual
    }
}

impl std::fmt::Display for BankSwapStoreMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manual => f.write_str("Manual"),
            Self::Automatic => f.write_str("Automatic"),
        }
    }
}

impl TryFrom<u8> for BankSwapStoreMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Manual),
            1 => Ok(Self::Automatic),
            v => Err(crate::errors::EnumError::new("BankSwapStoreMode", v as u64),)
        }
    }
}

