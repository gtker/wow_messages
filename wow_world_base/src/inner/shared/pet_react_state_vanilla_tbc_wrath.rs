/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/pet_common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/pet_common.wowm#L1):
/// ```text
/// enum PetReactState : u8 {
///     PASSIVE = 0;
///     DEFENSIVE = 1;
///     AGGRESSIVE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PetReactState {
    Passive,
    Defensive,
    Aggressive,
}

impl PetReactState {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Passive => 0x0,
            Self::Defensive => 0x1,
            Self::Aggressive => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Passive,
            Self::Defensive,
            Self::Aggressive,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl PetReactState {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Passive => "PASSIVE",
            Self::Defensive => "DEFENSIVE",
            Self::Aggressive => "AGGRESSIVE",
        }
    }

}

const NAME: &str = "PetReactState";

impl Default for PetReactState {
    fn default() -> Self {
        Self::Passive
    }
}

impl std::fmt::Display for PetReactState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Passive => f.write_str("Passive"),
            Self::Defensive => f.write_str("Defensive"),
            Self::Aggressive => f.write_str("Aggressive"),
        }
    }
}

impl TryFrom<u8> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Passive),
            1 => Ok(Self::Defensive),
            2 => Ok(Self::Aggressive),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

