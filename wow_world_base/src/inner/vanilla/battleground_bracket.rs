/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L1):
/// ```text
/// enum BattlegroundBracket : u8 {
///     TENS = 0;
///     TWENTIES = 1;
///     THIRTIES = 2;
///     FORTIES = 3;
///     FIFTIES = 4;
///     SIXTY = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BattlegroundBracket {
    /// 10-19
    Tens,
    /// 20-29
    Twenties,
    /// 30-39
    Thirties,
    /// 40-49
    Forties,
    /// 50-59
    Fifties,
    /// 60
    Sixty,
}

impl BattlegroundBracket {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Tens => 0x0,
            Self::Twenties => 0x1,
            Self::Thirties => 0x2,
            Self::Forties => 0x3,
            Self::Fifties => 0x4,
            Self::Sixty => 0x5,
        }
    }

    pub const fn variants() -> [Self; 6] {
        [
            Self::Tens,
            Self::Twenties,
            Self::Thirties,
            Self::Forties,
            Self::Fifties,
            Self::Sixty,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Tens),
            1 => Ok(Self::Twenties),
            2 => Ok(Self::Thirties),
            3 => Ok(Self::Forties),
            4 => Ok(Self::Fifties),
            5 => Ok(Self::Sixty),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl BattlegroundBracket {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Tens => "TENS",
            Self::Twenties => "TWENTIES",
            Self::Thirties => "THIRTIES",
            Self::Forties => "FORTIES",
            Self::Fifties => "FIFTIES",
            Self::Sixty => "SIXTY",
        }
    }

}

const NAME: &str = "BattlegroundBracket";

impl Default for BattlegroundBracket {
    fn default() -> Self {
        Self::Tens
    }
}

impl std::fmt::Display for BattlegroundBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tens => f.write_str("Tens"),
            Self::Twenties => f.write_str("Twenties"),
            Self::Thirties => f.write_str("Thirties"),
            Self::Forties => f.write_str("Forties"),
            Self::Fifties => f.write_str("Fifties"),
            Self::Sixty => f.write_str("Sixty"),
        }
    }
}

impl TryFrom<u8> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

