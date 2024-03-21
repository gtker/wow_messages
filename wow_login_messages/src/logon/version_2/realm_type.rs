/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L2):
/// ```text
/// enum RealmType : u8 {
///     PLAYER_VS_ENVIRONMENT = 0;
///     PLAYER_VS_PLAYER = 1;
///     ROLEPLAYING = 6;
///     ROLEPLAYING_PLAYER_VS_PLAYER = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RealmType {
    PlayerVsEnvironment,
    PlayerVsPlayer,
    Roleplaying,
    RoleplayingPlayerVsPlayer,
}

impl RealmType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PlayerVsEnvironment => 0x0,
            Self::PlayerVsPlayer => 0x1,
            Self::Roleplaying => 0x6,
            Self::RoleplayingPlayerVsPlayer => 0x8,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::PlayerVsEnvironment,
            Self::PlayerVsPlayer,
            Self::Roleplaying,
            Self::RoleplayingPlayerVsPlayer,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::PlayerVsEnvironment),
            1 => Ok(Self::PlayerVsPlayer),
            6 => Ok(Self::Roleplaying),
            8 => Ok(Self::RoleplayingPlayerVsPlayer),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

const NAME: &str = "RealmType";

impl Default for RealmType {
    fn default() -> Self {
        Self::PlayerVsEnvironment
    }
}

impl std::fmt::Display for RealmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlayerVsEnvironment => f.write_str("PlayerVsEnvironment"),
            Self::PlayerVsPlayer => f.write_str("PlayerVsPlayer"),
            Self::Roleplaying => f.write_str("Roleplaying"),
            Self::RoleplayingPlayerVsPlayer => f.write_str("RoleplayingPlayerVsPlayer"),
        }
    }
}

impl TryFrom<u8> for RealmType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for RealmType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for RealmType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for RealmType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for RealmType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for RealmType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for RealmType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for RealmType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

