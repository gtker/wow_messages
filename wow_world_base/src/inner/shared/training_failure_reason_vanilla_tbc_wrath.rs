/// These errors are only printed in client console.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_buy_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_buy_failed.wowm#L3):
/// ```text
/// enum TrainingFailureReason : u32 {
///     UNAVAILABLE = 0;
///     NOT_ENOUGH_MONEY = 1;
///     NOT_ENOUGH_SKILL = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TrainingFailureReason {
    /// Trainer service %d unavailable.
    Unavailable,
    /// Not enough money for trainer service %d.
    NotEnoughMoney,
    /// Not enough skill points for trainer service %d.
    NotEnoughSkill,
}

impl TrainingFailureReason {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Unavailable => 0x0,
            Self::NotEnoughMoney => 0x1,
            Self::NotEnoughSkill => 0x2,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl TrainingFailureReason {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Unavailable => "UNAVAILABLE",
            Self::NotEnoughMoney => "NOT_ENOUGH_MONEY",
            Self::NotEnoughSkill => "NOT_ENOUGH_SKILL",
        }
    }

}

const NAME: &str = "TrainingFailureReason";

impl Default for TrainingFailureReason {
    fn default() -> Self {
        Self::Unavailable
    }
}

impl std::fmt::Display for TrainingFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable => f.write_str("Unavailable"),
            Self::NotEnoughMoney => f.write_str("NotEnoughMoney"),
            Self::NotEnoughSkill => f.write_str("NotEnoughSkill"),
        }
    }
}

impl TryFrom<u32> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unavailable),
            1 => Ok(Self::NotEnoughMoney),
            2 => Ok(Self::NotEnoughSkill),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

