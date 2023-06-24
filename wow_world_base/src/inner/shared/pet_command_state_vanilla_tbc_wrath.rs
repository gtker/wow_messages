/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/pet_common.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/pet_common.wowm#L9):
/// ```text
/// enum PetCommandState : u8 {
///     STAY = 0;
///     FOLLOW = 1;
///     ATTACK = 2;
///     DISMISS = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PetCommandState {
    Stay,
    Follow,
    Attack,
    Dismiss,
}

impl PetCommandState {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Stay => 0x0,
            Self::Follow => 0x1,
            Self::Attack => 0x2,
            Self::Dismiss => 0x3,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::Stay,
            Self::Follow,
            Self::Attack,
            Self::Dismiss,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl PetCommandState {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Stay => "STAY",
            Self::Follow => "FOLLOW",
            Self::Attack => "ATTACK",
            Self::Dismiss => "DISMISS",
        }
    }

}

const NAME: &str = "PetCommandState";

impl Default for PetCommandState {
    fn default() -> Self {
        Self::Stay
    }
}

impl std::fmt::Display for PetCommandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stay => f.write_str("Stay"),
            Self::Follow => f.write_str("Follow"),
            Self::Attack => f.write_str("Attack"),
            Self::Dismiss => f.write_str("Dismiss"),
        }
    }
}

impl TryFrom<u8> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Stay),
            1 => Ok(Self::Follow),
            2 => Ok(Self::Attack),
            3 => Ok(Self::Dismiss),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

