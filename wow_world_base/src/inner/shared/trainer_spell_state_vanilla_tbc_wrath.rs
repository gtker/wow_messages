/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm#L1):
/// ```text
/// enum TrainerSpellState : u8 {
///     GREEN = 0;
///     RED = 1;
///     GRAY = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TrainerSpellState {
    Green,
    Red,
    Gray,
}

impl TrainerSpellState {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Green => 0x0,
            Self::Red => 0x1,
            Self::Gray => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Green,
            Self::Red,
            Self::Gray,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Green),
            1 => Ok(Self::Red),
            2 => Ok(Self::Gray),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl TrainerSpellState {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Green => "GREEN",
            Self::Red => "RED",
            Self::Gray => "GRAY",
        }
    }

}

const NAME: &str = "TrainerSpellState";

impl Default for TrainerSpellState {
    fn default() -> Self {
        Self::Green
    }
}

impl std::fmt::Display for TrainerSpellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Green => f.write_str("Green"),
            Self::Red => f.write_str("Red"),
            Self::Gray => f.write_str("Gray"),
        }
    }
}

impl TryFrom<u8> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

