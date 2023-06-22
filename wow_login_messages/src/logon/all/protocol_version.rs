/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm#L3):
/// ```text
/// enum ProtocolVersion : u8 {
///     TWO = 2;
///     THREE = 3;
///     FIVE = 5;
///     SIX = 6;
///     SEVEN = 7;
///     EIGHT = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ProtocolVersion {
    /// Used for login by `1.1.2.4125`.
    /// Used for reconnect by `1.1.2.4125`, `1.12.1.5875`, `2.0.0.6080`, and 2.0.1.6180`.
    Two,
    /// Used for login by `1.12.1.5875`, `2.0.0.6080`, and `2.0.1.6180`.
    Three,
    /// Used for login and reconnect by `2.0.3.6299`.
    Five,
    /// Used for login and reconnect by `2.0.5.6320`, `2.0.7.6383`, `2.0.8.6403`, `2.0.10.6448`, `2.0.12.6546`, `2.1.0.6692`, `2.1.0.6729`, `2.1.1.6739`, `2.1.2.6803`, `2.1.3.6898`, `2.2.0.7272`, `2.2.2.7318`, `2.2.2.7318`, and `2.2.3.7359`.
    Six,
    /// Used for login and reconnect by `2.3.0.7561`, `2.3.2.7741`, and `2.3.3.7799`.
    Seven,
    /// Used for login and reconnect by `2.4.0.8089`, `2.4.1.8125`, `2.4.2.8278`, `2.4.3.8606`, and `3.3.5.12340`.
    Eight,
}

impl ProtocolVersion {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Two => 0x2,
            Self::Three => 0x3,
            Self::Five => 0x5,
            Self::Six => 0x6,
            Self::Seven => 0x7,
            Self::Eight => 0x8,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl ProtocolVersion {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Two => "TWO",
            Self::Three => "THREE",
            Self::Five => "FIVE",
            Self::Six => "SIX",
            Self::Seven => "SEVEN",
            Self::Eight => "EIGHT",
        }
    }

}

const NAME: &str = "ProtocolVersion";

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::Two
    }
}

impl std::fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Two => f.write_str("Two"),
            Self::Three => f.write_str("Three"),
            Self::Five => f.write_str("Five"),
            Self::Six => f.write_str("Six"),
            Self::Seven => f.write_str("Seven"),
            Self::Eight => f.write_str("Eight"),
        }
    }
}

impl TryFrom<u8> for ProtocolVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for ProtocolVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ProtocolVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ProtocolVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ProtocolVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ProtocolVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ProtocolVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ProtocolVersion {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

