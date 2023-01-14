use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_barber_shop_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_barber_shop_result.wowm#L1):
/// ```text
/// enum BarberShopResult : u8 {
///     OK = 0;
///     NOT_ENOUGH_MONEY = 1;
///     MUST_BE_SEATED_IN_BARBER_CHAIR = 2;
///     NOT_ENOUGH_MONEY2 = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BarberShopResult {
    Ok,
    NotEnoughMoney,
    MustBeSeatedInBarberChair,
    NotEnoughMoney2,
}

impl BarberShopResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Ok => 0x0,
            Self::NotEnoughMoney => 0x1,
            Self::MustBeSeatedInBarberChair => 0x2,
            Self::NotEnoughMoney2 => 0x3,
        }
    }

}

impl Default for BarberShopResult {
    fn default() -> Self {
        Self::Ok
    }
}

impl std::fmt::Display for BarberShopResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::NotEnoughMoney => f.write_str("NotEnoughMoney"),
            Self::MustBeSeatedInBarberChair => f.write_str("MustBeSeatedInBarberChair"),
            Self::NotEnoughMoney2 => f.write_str("NotEnoughMoney2"),
        }
    }
}

impl TryFrom<u8> for BarberShopResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::NotEnoughMoney),
            2 => Ok(Self::MustBeSeatedInBarberChair),
            3 => Ok(Self::NotEnoughMoney2),
            v => Err(crate::errors::EnumError::new("BarberShopResult", v as u64),)
        }
    }
}

