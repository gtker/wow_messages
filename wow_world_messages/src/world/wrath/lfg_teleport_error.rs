use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_teleport_denied.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_teleport_denied.wowm#L1):
/// ```text
/// enum LfgTeleportError : u32 {
///     PLAYER_DEAD = 1;
///     FALLING = 2;
///     IN_VEHICLE = 3;
///     FATIGUE = 4;
///     INVALID_LOCATION = 6;
///     COMBAT = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LfgTeleportError {
    PlayerDead,
    Falling,
    InVehicle,
    Fatigue,
    InvalidLocation,
    /// azerothcore: It can be 7 or 8
    ///
    Combat,
}

impl LfgTeleportError {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::PlayerDead => 0x1,
            Self::Falling => 0x2,
            Self::InVehicle => 0x3,
            Self::Fatigue => 0x4,
            Self::InvalidLocation => 0x6,
            Self::Combat => 0x8,
        }
    }

}

impl Default for LfgTeleportError {
    fn default() -> Self {
        Self::PlayerDead
    }
}

impl std::fmt::Display for LfgTeleportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlayerDead => f.write_str("PlayerDead"),
            Self::Falling => f.write_str("Falling"),
            Self::InVehicle => f.write_str("InVehicle"),
            Self::Fatigue => f.write_str("Fatigue"),
            Self::InvalidLocation => f.write_str("InvalidLocation"),
            Self::Combat => f.write_str("Combat"),
        }
    }
}

impl TryFrom<u32> for LfgTeleportError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::PlayerDead),
            2 => Ok(Self::Falling),
            3 => Ok(Self::InVehicle),
            4 => Ok(Self::Fatigue),
            6 => Ok(Self::InvalidLocation),
            8 => Ok(Self::Combat),
            v => Err(crate::errors::EnumError::new("LfgTeleportError", v as u64),)
        }
    }
}

