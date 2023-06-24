/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm#L3):
/// ```text
/// enum LogoutResult : u32 {
///     SUCCESS = 0;
///     FAILURE_IN_COMBAT = 1;
///     FAILURE_FROZEN_BY_GM = 2;
///     FAILURE_JUMPING_OR_FALLING = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LogoutResult {
    Success,
    FailureInCombat,
    /// vmangos checks for aura 9454. Has FIXME - Need the correct value.
    FailureFrozenByGm,
    FailureJumpingOrFalling,
}

impl LogoutResult {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Success => 0x0,
            Self::FailureInCombat => 0x1,
            Self::FailureFrozenByGm => 0x2,
            Self::FailureJumpingOrFalling => 0x3,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::Success,
            Self::FailureInCombat,
            Self::FailureFrozenByGm,
            Self::FailureJumpingOrFalling,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl LogoutResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::FailureInCombat => "FAILURE_IN_COMBAT",
            Self::FailureFrozenByGm => "FAILURE_FROZEN_BY_GM",
            Self::FailureJumpingOrFalling => "FAILURE_JUMPING_OR_FALLING",
        }
    }

}

const NAME: &str = "LogoutResult";

impl Default for LogoutResult {
    fn default() -> Self {
        Self::Success
    }
}

impl std::fmt::Display for LogoutResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.write_str("Success"),
            Self::FailureInCombat => f.write_str("FailureInCombat"),
            Self::FailureFrozenByGm => f.write_str("FailureFrozenByGm"),
            Self::FailureJumpingOrFalling => f.write_str("FailureJumpingOrFalling"),
        }
    }
}

impl TryFrom<u32> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            1 => Ok(Self::FailureInCombat),
            2 => Ok(Self::FailureFrozenByGm),
            3 => Ok(Self::FailureJumpingOrFalling),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

