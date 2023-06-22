/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_action_feedback.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_action_feedback.wowm#L3):
/// ```text
/// enum PetFeedback : u8 {
///     PET_DEAD = 1;
///     NOTHING_TO_ATTACK = 2;
///     CANT_ATTACK_TARGET = 3;
///     NO_PATH_TO = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PetFeedback {
    PetDead,
    NothingToAttack,
    CantAttackTarget,
    NoPathTo,
}

impl PetFeedback {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::PetDead => 0x1,
            Self::NothingToAttack => 0x2,
            Self::CantAttackTarget => 0x3,
            Self::NoPathTo => 0x4,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl PetFeedback {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::PetDead => "PET_DEAD",
            Self::NothingToAttack => "NOTHING_TO_ATTACK",
            Self::CantAttackTarget => "CANT_ATTACK_TARGET",
            Self::NoPathTo => "NO_PATH_TO",
        }
    }

}

const NAME: &str = "PetFeedback";

impl Default for PetFeedback {
    fn default() -> Self {
        Self::PetDead
    }
}

impl std::fmt::Display for PetFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PetDead => f.write_str("PetDead"),
            Self::NothingToAttack => f.write_str("NothingToAttack"),
            Self::CantAttackTarget => f.write_str("CantAttackTarget"),
            Self::NoPathTo => f.write_str("NoPathTo"),
        }
    }
}

impl TryFrom<u8> for PetFeedback {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::PetDead),
            2 => Ok(Self::NothingToAttack),
            3 => Ok(Self::CantAttackTarget),
            4 => Ok(Self::NoPathTo),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for PetFeedback {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for PetFeedback {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PetFeedback {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PetFeedback {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for PetFeedback {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PetFeedback {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PetFeedback {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PetFeedback {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

