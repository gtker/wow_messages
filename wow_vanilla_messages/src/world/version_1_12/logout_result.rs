use std::convert::{TryFrom, TryInto};

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
pub enum LogoutResult {
    SUCCESS,
    FAILURE_IN_COMBAT,
    /// vmangos checks for aura 9454. Has FIXME - Need the correct value.
    ///
    FAILURE_FROZEN_BY_GM,
    FAILURE_JUMPING_OR_FALLING,
}

impl LogoutResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SUCCESS => 0x0,
            Self::FAILURE_IN_COMBAT => 0x1,
            Self::FAILURE_FROZEN_BY_GM => 0x2,
            Self::FAILURE_JUMPING_OR_FALLING => 0x3,
        }
    }

}

impl Default for LogoutResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for LogoutResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::FAILURE_IN_COMBAT => f.write_str("FAILURE_IN_COMBAT"),
            Self::FAILURE_FROZEN_BY_GM => f.write_str("FAILURE_FROZEN_BY_GM"),
            Self::FAILURE_JUMPING_OR_FALLING => f.write_str("FAILURE_JUMPING_OR_FALLING"),
        }
    }
}

impl TryFrom<u32> for LogoutResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            1 => Ok(Self::FAILURE_IN_COMBAT),
            2 => Ok(Self::FAILURE_FROZEN_BY_GM),
            3 => Ok(Self::FAILURE_JUMPING_OR_FALLING),
            v => Err(crate::errors::EnumError::new("LogoutResult", v as u32),)
        }
    }
}

