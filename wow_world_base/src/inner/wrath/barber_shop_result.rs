/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_barber_shop_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_barber_shop_result.wowm#L1):
/// ```text
/// enum BarberShopResult : u8 {
///     OK = 0;
///     NOT_ENOUGH_MONEY = 1;
///     MUST_BE_SEATED_IN_BARBER_CHAIR = 2;
///     NOT_ENOUGH_MONEY2 = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BarberShopResult {
    Ok,
    NotEnoughMoney,
    MustBeSeatedInBarberChair,
    NotEnoughMoney2,
}

impl BarberShopResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Ok => 0x0,
            Self::NotEnoughMoney => 0x1,
            Self::MustBeSeatedInBarberChair => 0x2,
            Self::NotEnoughMoney2 => 0x3,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::Ok,
            Self::NotEnoughMoney,
            Self::MustBeSeatedInBarberChair,
            Self::NotEnoughMoney2,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::NotEnoughMoney),
            2 => Ok(Self::MustBeSeatedInBarberChair),
            3 => Ok(Self::NotEnoughMoney2),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl BarberShopResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::NotEnoughMoney => "NOT_ENOUGH_MONEY",
            Self::MustBeSeatedInBarberChair => "MUST_BE_SEATED_IN_BARBER_CHAIR",
            Self::NotEnoughMoney2 => "NOT_ENOUGH_MONEY2",
        }
    }

}

const NAME: &str = "BarberShopResult";

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
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for BarberShopResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for BarberShopResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for BarberShopResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for BarberShopResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for BarberShopResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for BarberShopResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for BarberShopResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for BarberShopResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

