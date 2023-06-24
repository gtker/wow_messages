/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm#L12):
/// ```text
/// enum PetitionResult : u32 {
///     OK = 0;
///     ALREADY_SIGNED = 1;
///     ALREADY_IN_GUILD = 2;
///     CANT_SIGN_OWN = 3;
///     NOT_SERVER = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PetitionResult {
    Ok,
    AlreadySigned,
    AlreadyInGuild,
    CantSignOwn,
    NotServer,
}

impl PetitionResult {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Ok => 0x0,
            Self::AlreadySigned => 0x1,
            Self::AlreadyInGuild => 0x2,
            Self::CantSignOwn => 0x3,
            Self::NotServer => 0x4,
        }
    }

    pub const fn variants() -> [Self; 5] {
        [
            Self::Ok,
            Self::AlreadySigned,
            Self::AlreadyInGuild,
            Self::CantSignOwn,
            Self::NotServer,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl PetitionResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::AlreadySigned => "ALREADY_SIGNED",
            Self::AlreadyInGuild => "ALREADY_IN_GUILD",
            Self::CantSignOwn => "CANT_SIGN_OWN",
            Self::NotServer => "NOT_SERVER",
        }
    }

}

const NAME: &str = "PetitionResult";

impl Default for PetitionResult {
    fn default() -> Self {
        Self::Ok
    }
}

impl std::fmt::Display for PetitionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::AlreadySigned => f.write_str("AlreadySigned"),
            Self::AlreadyInGuild => f.write_str("AlreadyInGuild"),
            Self::CantSignOwn => f.write_str("CantSignOwn"),
            Self::NotServer => f.write_str("NotServer"),
        }
    }
}

impl TryFrom<u32> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::AlreadySigned),
            2 => Ok(Self::AlreadyInGuild),
            3 => Ok(Self::CantSignOwn),
            4 => Ok(Self::NotServer),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

