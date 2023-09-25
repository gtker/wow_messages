/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_command_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_command_result.wowm#L1):
/// ```text
/// enum PartyResult : u8 {
///     SUCCESS = 0;
///     BAD_PLAYER_NAME = 1;
///     TARGET_NOT_IN_GROUP = 2;
///     GROUP_FULL = 3;
///     ALREADY_IN_GROUP = 4;
///     NOT_IN_GROUP = 5;
///     NOT_LEADER = 6;
///     PLAYER_WRONG_FACTION = 7;
///     IGNORING_YOU = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PartyResult {
    Success,
    BadPlayerName,
    TargetNotInGroup,
    GroupFull,
    AlreadyInGroup,
    NotInGroup,
    NotLeader,
    PlayerWrongFaction,
    IgnoringYou,
}

impl PartyResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Success => 0x0,
            Self::BadPlayerName => 0x1,
            Self::TargetNotInGroup => 0x2,
            Self::GroupFull => 0x3,
            Self::AlreadyInGroup => 0x4,
            Self::NotInGroup => 0x5,
            Self::NotLeader => 0x6,
            Self::PlayerWrongFaction => 0x7,
            Self::IgnoringYou => 0x8,
        }
    }

    pub const fn variants() -> [Self; 9] {
        [
            Self::Success,
            Self::BadPlayerName,
            Self::TargetNotInGroup,
            Self::GroupFull,
            Self::AlreadyInGroup,
            Self::NotInGroup,
            Self::NotLeader,
            Self::PlayerWrongFaction,
            Self::IgnoringYou,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Success),
            1 => Ok(Self::BadPlayerName),
            2 => Ok(Self::TargetNotInGroup),
            3 => Ok(Self::GroupFull),
            4 => Ok(Self::AlreadyInGroup),
            5 => Ok(Self::NotInGroup),
            6 => Ok(Self::NotLeader),
            7 => Ok(Self::PlayerWrongFaction),
            8 => Ok(Self::IgnoringYou),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl PartyResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::BadPlayerName => "BAD_PLAYER_NAME",
            Self::TargetNotInGroup => "TARGET_NOT_IN_GROUP",
            Self::GroupFull => "GROUP_FULL",
            Self::AlreadyInGroup => "ALREADY_IN_GROUP",
            Self::NotInGroup => "NOT_IN_GROUP",
            Self::NotLeader => "NOT_LEADER",
            Self::PlayerWrongFaction => "PLAYER_WRONG_FACTION",
            Self::IgnoringYou => "IGNORING_YOU",
        }
    }

}

const NAME: &str = "PartyResult";

impl Default for PartyResult {
    fn default() -> Self {
        Self::Success
    }
}

impl std::fmt::Display for PartyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.write_str("Success"),
            Self::BadPlayerName => f.write_str("BadPlayerName"),
            Self::TargetNotInGroup => f.write_str("TargetNotInGroup"),
            Self::GroupFull => f.write_str("GroupFull"),
            Self::AlreadyInGroup => f.write_str("AlreadyInGroup"),
            Self::NotInGroup => f.write_str("NotInGroup"),
            Self::NotLeader => f.write_str("NotLeader"),
            Self::PlayerWrongFaction => f.write_str("PlayerWrongFaction"),
            Self::IgnoringYou => f.write_str("IgnoringYou"),
        }
    }
}

impl TryFrom<u8> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

