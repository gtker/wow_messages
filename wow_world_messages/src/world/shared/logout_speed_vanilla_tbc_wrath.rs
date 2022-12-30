use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm#L12):
/// ```text
/// enum LogoutSpeed : u8 {
///     DELAYED = 0;
///     INSTANT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LogoutSpeed {
    Delayed,
    Instant,
}

impl LogoutSpeed {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Delayed => 0x0,
            Self::Instant => 0x1,
        }
    }

}

impl Default for LogoutSpeed {
    fn default() -> Self {
        Self::Delayed
    }
}

impl std::fmt::Display for LogoutSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Delayed => f.write_str("Delayed"),
            Self::Instant => f.write_str("Instant"),
        }
    }
}

impl TryFrom<u8> for LogoutSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Delayed),
            1 => Ok(Self::Instant),
            v => Err(crate::errors::EnumError::new("LogoutSpeed", v as u64),)
        }
    }
}

