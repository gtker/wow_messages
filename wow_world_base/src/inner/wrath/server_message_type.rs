/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_server_message.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_server_message.wowm#L18):
/// ```text
/// enum ServerMessageType : u32 {
///     SHUTDOWN_TIME = 1;
///     RESTART_TIME = 2;
///     CUSTOM = 3;
///     SHUTDOWN_CANCELLED = 4;
///     RESTART_CANCELLED = 5;
///     BATTLEGROUND_SHUTDOWN = 6;
///     BATTLEGROUND_RESTART = 7;
///     INSTANCE_SHUTDOWN = 8;
///     INSTANCE_RESTART = 9;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ServerMessageType {
    ShutdownTime,
    RestartTime,
    Custom,
    ShutdownCancelled,
    RestartCancelled,
    BattlegroundShutdown,
    BattlegroundRestart,
    InstanceShutdown,
    InstanceRestart,
}

impl ServerMessageType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::ShutdownTime => 0x1,
            Self::RestartTime => 0x2,
            Self::Custom => 0x3,
            Self::ShutdownCancelled => 0x4,
            Self::RestartCancelled => 0x5,
            Self::BattlegroundShutdown => 0x6,
            Self::BattlegroundRestart => 0x7,
            Self::InstanceShutdown => 0x8,
            Self::InstanceRestart => 0x9,
        }
    }

    pub const fn variants() -> [Self; 9] {
        [
            Self::ShutdownTime,
            Self::RestartTime,
            Self::Custom,
            Self::ShutdownCancelled,
            Self::RestartCancelled,
            Self::BattlegroundShutdown,
            Self::BattlegroundRestart,
            Self::InstanceShutdown,
            Self::InstanceRestart,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl ServerMessageType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::ShutdownTime => "SHUTDOWN_TIME",
            Self::RestartTime => "RESTART_TIME",
            Self::Custom => "CUSTOM",
            Self::ShutdownCancelled => "SHUTDOWN_CANCELLED",
            Self::RestartCancelled => "RESTART_CANCELLED",
            Self::BattlegroundShutdown => "BATTLEGROUND_SHUTDOWN",
            Self::BattlegroundRestart => "BATTLEGROUND_RESTART",
            Self::InstanceShutdown => "INSTANCE_SHUTDOWN",
            Self::InstanceRestart => "INSTANCE_RESTART",
        }
    }

}

const NAME: &str = "ServerMessageType";

impl Default for ServerMessageType {
    fn default() -> Self {
        Self::ShutdownTime
    }
}

impl std::fmt::Display for ServerMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShutdownTime => f.write_str("ShutdownTime"),
            Self::RestartTime => f.write_str("RestartTime"),
            Self::Custom => f.write_str("Custom"),
            Self::ShutdownCancelled => f.write_str("ShutdownCancelled"),
            Self::RestartCancelled => f.write_str("RestartCancelled"),
            Self::BattlegroundShutdown => f.write_str("BattlegroundShutdown"),
            Self::BattlegroundRestart => f.write_str("BattlegroundRestart"),
            Self::InstanceShutdown => f.write_str("InstanceShutdown"),
            Self::InstanceRestart => f.write_str("InstanceRestart"),
        }
    }
}

impl TryFrom<u32> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ShutdownTime),
            2 => Ok(Self::RestartTime),
            3 => Ok(Self::Custom),
            4 => Ok(Self::ShutdownCancelled),
            5 => Ok(Self::RestartCancelled),
            6 => Ok(Self::BattlegroundShutdown),
            7 => Ok(Self::BattlegroundRestart),
            8 => Ok(Self::InstanceShutdown),
            9 => Ok(Self::InstanceRestart),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ServerMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

