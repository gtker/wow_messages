/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:25`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L25):
/// ```text
/// enum KeyVersion : u8 {
///     ZERO = 0;
///     ONE = 1;
///     TWO = 2;
///     THREE = 3;
///     FOUR = 4;
///     FIVE = 5;
///     SIX = 6;
///     SEVEN = 7;
///     EIGHT = 8;
///     NINE = 9;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum KeyVersion {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl KeyVersion {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Zero => 0x0,
            Self::One => 0x1,
            Self::Two => 0x2,
            Self::Three => 0x3,
            Self::Four => 0x4,
            Self::Five => 0x5,
            Self::Six => 0x6,
            Self::Seven => 0x7,
            Self::Eight => 0x8,
            Self::Nine => 0x9,
        }
    }

    pub const fn variants() -> [Self; 10] {
        [
            Self::Zero,
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl KeyVersion {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Zero => "ZERO",
            Self::One => "ONE",
            Self::Two => "TWO",
            Self::Three => "THREE",
            Self::Four => "FOUR",
            Self::Five => "FIVE",
            Self::Six => "SIX",
            Self::Seven => "SEVEN",
            Self::Eight => "EIGHT",
            Self::Nine => "NINE",
        }
    }

}

const NAME: &str = "KeyVersion";

impl Default for KeyVersion {
    fn default() -> Self {
        Self::Zero
    }
}

impl std::fmt::Display for KeyVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => f.write_str("Zero"),
            Self::One => f.write_str("One"),
            Self::Two => f.write_str("Two"),
            Self::Three => f.write_str("Three"),
            Self::Four => f.write_str("Four"),
            Self::Five => f.write_str("Five"),
            Self::Six => f.write_str("Six"),
            Self::Seven => f.write_str("Seven"),
            Self::Eight => f.write_str("Eight"),
            Self::Nine => f.write_str("Nine"),
        }
    }
}

impl TryFrom<u8> for KeyVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for KeyVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for KeyVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for KeyVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for KeyVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for KeyVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for KeyVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for KeyVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for KeyVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

