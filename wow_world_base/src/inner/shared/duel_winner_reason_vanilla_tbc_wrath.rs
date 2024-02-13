/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_winner.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_winner.wowm#L3):
/// ```text
/// enum DuelWinnerReason : u8 {
///     WON = 0;
///     FLED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum DuelWinnerReason {
    Won,
    Fled,
}

impl DuelWinnerReason {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Won => 0x0,
            Self::Fled => 0x1,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
            Self::Won,
            Self::Fled,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Won),
            1 => Ok(Self::Fled),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl DuelWinnerReason {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Won => "WON",
            Self::Fled => "FLED",
        }
    }

}

const NAME: &str = "DuelWinnerReason";

impl Default for DuelWinnerReason {
    fn default() -> Self {
        Self::Won
    }
}

impl std::fmt::Display for DuelWinnerReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Won => f.write_str("Won"),
            Self::Fled => f.write_str("Fled"),
        }
    }
}

impl TryFrom<u8> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

