use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_server_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_server_message.wowm#L3):
/// ```text
/// enum ServerMessageType : u32 {
///     SHUTDOWN_TIME = 1;
///     RESTART_TIME = 2;
///     CUSTOM = 3;
///     SHUTDOWN_CANCELLED = 4;
///     RESTART_CANCELLED = 5;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ServerMessageType {
    SHUTDOWN_TIME,
    RESTART_TIME,
    CUSTOM,
    SHUTDOWN_CANCELLED,
    RESTART_CANCELLED,
}

impl ServerMessageType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SHUTDOWN_TIME => 0x1,
            Self::RESTART_TIME => 0x2,
            Self::CUSTOM => 0x3,
            Self::SHUTDOWN_CANCELLED => 0x4,
            Self::RESTART_CANCELLED => 0x5,
        }
    }

}

impl Default for ServerMessageType {
    fn default() -> Self {
        Self::SHUTDOWN_TIME
    }
}

impl std::fmt::Display for ServerMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SHUTDOWN_TIME => f.write_str("SHUTDOWN_TIME"),
            Self::RESTART_TIME => f.write_str("RESTART_TIME"),
            Self::CUSTOM => f.write_str("CUSTOM"),
            Self::SHUTDOWN_CANCELLED => f.write_str("SHUTDOWN_CANCELLED"),
            Self::RESTART_CANCELLED => f.write_str("RESTART_CANCELLED"),
        }
    }
}

impl TryFrom<u32> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::SHUTDOWN_TIME),
            2 => Ok(Self::RESTART_TIME),
            3 => Ok(Self::CUSTOM),
            4 => Ok(Self::SHUTDOWN_CANCELLED),
            5 => Ok(Self::RESTART_CANCELLED),
            v => Err(crate::errors::EnumError::new("ServerMessageType", v as u32),)
        }
    }
}

