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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LfgTeleportError {
    PlayerDead,
    Falling,
    InVehicle,
    Fatigue,
    InvalidLocation,
    /// azerothcore: It can be 7 or 8
    Combat,
}

impl LfgTeleportError {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::PlayerDead => 0x1,
            Self::Falling => 0x2,
            Self::InVehicle => 0x3,
            Self::Fatigue => 0x4,
            Self::InvalidLocation => 0x6,
            Self::Combat => 0x8,
        }
    }

    pub const fn variants() -> [Self; 6] {
        [
            Self::PlayerDead,
            Self::Falling,
            Self::InVehicle,
            Self::Fatigue,
            Self::InvalidLocation,
            Self::Combat,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl LfgTeleportError {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::PlayerDead => "PLAYER_DEAD",
            Self::Falling => "FALLING",
            Self::InVehicle => "IN_VEHICLE",
            Self::Fatigue => "FATIGUE",
            Self::InvalidLocation => "INVALID_LOCATION",
            Self::Combat => "COMBAT",
        }
    }

}

const NAME: &str = "LfgTeleportError";

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
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::PlayerDead),
            2 => Ok(Self::Falling),
            3 => Ok(Self::InVehicle),
            4 => Ok(Self::Fatigue),
            6 => Ok(Self::InvalidLocation),
            8 => Ok(Self::Combat),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for LfgTeleportError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for LfgTeleportError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for LfgTeleportError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for LfgTeleportError {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for LfgTeleportError {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for LfgTeleportError {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for LfgTeleportError {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for LfgTeleportError {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

