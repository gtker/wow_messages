/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L23):
/// ```text
/// enum LootMethodError : u8 {
///     DIDNT_KILL = 0;
///     TOO_FAR = 4;
///     BAD_FACING = 5;
///     LOCKED = 6;
///     NOTSTANDING = 8;
///     STUNNED = 9;
///     PLAYER_NOT_FOUND = 10;
///     PLAY_TIME_EXCEEDED = 11;
///     MASTER_INV_FULL = 12;
///     MASTER_UNIQUE_ITEM = 13;
///     MASTER_OTHER = 14;
///     ALREADY_PICKPOCKETED = 15;
///     NOT_WHILE_SHAPESHIFTED = 16;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LootMethodError {
    /// You don't have permission to loot that corpse.
    DidntKill,
    /// You are too far away to loot that corpse.
    TooFar,
    /// You must be facing the corpse to loot it.
    BadFacing,
    /// Someone is already looting that corpse.
    Locked,
    /// You need to be standing up to loot something!
    Notstanding,
    /// You can't loot anything while stunned!
    Stunned,
    /// Player not found
    PlayerNotFound,
    /// Maximum play time exceeded
    PlayTimeExceeded,
    /// That player's inventory is full
    MasterInvFull,
    /// Player has too many of that item already
    MasterUniqueItem,
    /// Can't assign item to that player
    MasterOther,
    /// Your target has already had its pockets picked
    AlreadyPickpocketed,
    /// You can't do that while shapeshifted.
    NotWhileShapeshifted,
}

impl LootMethodError {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::DidntKill => 0x0,
            Self::TooFar => 0x4,
            Self::BadFacing => 0x5,
            Self::Locked => 0x6,
            Self::Notstanding => 0x8,
            Self::Stunned => 0x9,
            Self::PlayerNotFound => 0xa,
            Self::PlayTimeExceeded => 0xb,
            Self::MasterInvFull => 0xc,
            Self::MasterUniqueItem => 0xd,
            Self::MasterOther => 0xe,
            Self::AlreadyPickpocketed => 0xf,
            Self::NotWhileShapeshifted => 0x10,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl LootMethodError {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::DidntKill => "DIDNT_KILL",
            Self::TooFar => "TOO_FAR",
            Self::BadFacing => "BAD_FACING",
            Self::Locked => "LOCKED",
            Self::Notstanding => "NOTSTANDING",
            Self::Stunned => "STUNNED",
            Self::PlayerNotFound => "PLAYER_NOT_FOUND",
            Self::PlayTimeExceeded => "PLAY_TIME_EXCEEDED",
            Self::MasterInvFull => "MASTER_INV_FULL",
            Self::MasterUniqueItem => "MASTER_UNIQUE_ITEM",
            Self::MasterOther => "MASTER_OTHER",
            Self::AlreadyPickpocketed => "ALREADY_PICKPOCKETED",
            Self::NotWhileShapeshifted => "NOT_WHILE_SHAPESHIFTED",
        }
    }

}

const NAME: &str = "LootMethodError";

impl Default for LootMethodError {
    fn default() -> Self {
        Self::DidntKill
    }
}

impl std::fmt::Display for LootMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DidntKill => f.write_str("DidntKill"),
            Self::TooFar => f.write_str("TooFar"),
            Self::BadFacing => f.write_str("BadFacing"),
            Self::Locked => f.write_str("Locked"),
            Self::Notstanding => f.write_str("Notstanding"),
            Self::Stunned => f.write_str("Stunned"),
            Self::PlayerNotFound => f.write_str("PlayerNotFound"),
            Self::PlayTimeExceeded => f.write_str("PlayTimeExceeded"),
            Self::MasterInvFull => f.write_str("MasterInvFull"),
            Self::MasterUniqueItem => f.write_str("MasterUniqueItem"),
            Self::MasterOther => f.write_str("MasterOther"),
            Self::AlreadyPickpocketed => f.write_str("AlreadyPickpocketed"),
            Self::NotWhileShapeshifted => f.write_str("NotWhileShapeshifted"),
        }
    }
}

impl TryFrom<u8> for LootMethodError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DidntKill),
            4 => Ok(Self::TooFar),
            5 => Ok(Self::BadFacing),
            6 => Ok(Self::Locked),
            8 => Ok(Self::Notstanding),
            9 => Ok(Self::Stunned),
            10 => Ok(Self::PlayerNotFound),
            11 => Ok(Self::PlayTimeExceeded),
            12 => Ok(Self::MasterInvFull),
            13 => Ok(Self::MasterUniqueItem),
            14 => Ok(Self::MasterOther),
            15 => Ok(Self::AlreadyPickpocketed),
            16 => Ok(Self::NotWhileShapeshifted),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for LootMethodError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for LootMethodError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for LootMethodError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for LootMethodError {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for LootMethodError {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for LootMethodError {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for LootMethodError {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for LootMethodError {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

