use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum UnitStandState {
    STAND,
    SIT,
    SIT_CHAIR,
    SLEEP,
    SIT_LOW_CHAIR,
    SIT_MEDIUM_CHAIR,
    SIT_HIGH_CHAIR,
    DEAD,
    KNEEL,
    CUSTOM,
}

impl UnitStandState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::STAND => 0x0,
            Self::SIT => 0x1,
            Self::SIT_CHAIR => 0x2,
            Self::SLEEP => 0x3,
            Self::SIT_LOW_CHAIR => 0x4,
            Self::SIT_MEDIUM_CHAIR => 0x5,
            Self::SIT_HIGH_CHAIR => 0x6,
            Self::DEAD => 0x7,
            Self::KNEEL => 0x8,
            Self::CUSTOM => 0x9,
        }
    }

}

impl ConstantSized for UnitStandState {}

impl MaximumPossibleSized for UnitStandState {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for UnitStandState {
    fn default() -> Self {
        Self::STAND
    }
}

impl std::fmt::Display for UnitStandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STAND => f.write_str("STAND"),
            Self::SIT => f.write_str("SIT"),
            Self::SIT_CHAIR => f.write_str("SIT_CHAIR"),
            Self::SLEEP => f.write_str("SLEEP"),
            Self::SIT_LOW_CHAIR => f.write_str("SIT_LOW_CHAIR"),
            Self::SIT_MEDIUM_CHAIR => f.write_str("SIT_MEDIUM_CHAIR"),
            Self::SIT_HIGH_CHAIR => f.write_str("SIT_HIGH_CHAIR"),
            Self::DEAD => f.write_str("DEAD"),
            Self::KNEEL => f.write_str("KNEEL"),
            Self::CUSTOM => f.write_str("CUSTOM"),
        }
    }
}

impl TryFrom<u8> for UnitStandState {
    type Error = UnitStandStateError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::STAND),
            1 => Ok(Self::SIT),
            2 => Ok(Self::SIT_CHAIR),
            3 => Ok(Self::SLEEP),
            4 => Ok(Self::SIT_LOW_CHAIR),
            5 => Ok(Self::SIT_MEDIUM_CHAIR),
            6 => Ok(Self::SIT_HIGH_CHAIR),
            7 => Ok(Self::DEAD),
            8 => Ok(Self::KNEEL),
            9 => Ok(Self::CUSTOM),
            _ => Err(UnitStandStateError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct UnitStandStateError {
    value: u8,
}

impl UnitStandStateError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for UnitStandStateError {}
impl std::fmt::Display for UnitStandStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'UnitStandState': '{}'", self.value))
    }
}

