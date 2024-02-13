/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/loot_common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/loot_common.wowm#L1):
/// ```text
/// enum RollVote : u8 {
///     PASS = 0;
///     NEED = 1;
///     GREED = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum RollVote {
    Pass,
    Need,
    Greed,
}

impl RollVote {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Pass => 0x0,
            Self::Need => 0x1,
            Self::Greed => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Pass,
            Self::Need,
            Self::Greed,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Pass),
            1 => Ok(Self::Need),
            2 => Ok(Self::Greed),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl RollVote {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Need => "NEED",
            Self::Greed => "GREED",
        }
    }

}

const NAME: &str = "RollVote";

impl Default for RollVote {
    fn default() -> Self {
        Self::Pass
    }
}

impl std::fmt::Display for RollVote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pass => f.write_str("Pass"),
            Self::Need => f.write_str("Need"),
            Self::Greed => f.write_str("Greed"),
        }
    }
}

impl TryFrom<u8> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

