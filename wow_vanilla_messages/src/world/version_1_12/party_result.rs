use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_command_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_command_result.wowm#L3):
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
pub enum PartyResult {
    SUCCESS,
    BAD_PLAYER_NAME,
    TARGET_NOT_IN_GROUP,
    GROUP_FULL,
    ALREADY_IN_GROUP,
    NOT_IN_GROUP,
    NOT_LEADER,
    PLAYER_WRONG_FACTION,
    IGNORING_YOU,
}

impl PartyResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SUCCESS => 0x0,
            Self::BAD_PLAYER_NAME => 0x1,
            Self::TARGET_NOT_IN_GROUP => 0x2,
            Self::GROUP_FULL => 0x3,
            Self::ALREADY_IN_GROUP => 0x4,
            Self::NOT_IN_GROUP => 0x5,
            Self::NOT_LEADER => 0x6,
            Self::PLAYER_WRONG_FACTION => 0x7,
            Self::IGNORING_YOU => 0x8,
        }
    }

}

impl Default for PartyResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for PartyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::BAD_PLAYER_NAME => f.write_str("BAD_PLAYER_NAME"),
            Self::TARGET_NOT_IN_GROUP => f.write_str("TARGET_NOT_IN_GROUP"),
            Self::GROUP_FULL => f.write_str("GROUP_FULL"),
            Self::ALREADY_IN_GROUP => f.write_str("ALREADY_IN_GROUP"),
            Self::NOT_IN_GROUP => f.write_str("NOT_IN_GROUP"),
            Self::NOT_LEADER => f.write_str("NOT_LEADER"),
            Self::PLAYER_WRONG_FACTION => f.write_str("PLAYER_WRONG_FACTION"),
            Self::IGNORING_YOU => f.write_str("IGNORING_YOU"),
        }
    }
}

impl TryFrom<u8> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            1 => Ok(Self::BAD_PLAYER_NAME),
            2 => Ok(Self::TARGET_NOT_IN_GROUP),
            3 => Ok(Self::GROUP_FULL),
            4 => Ok(Self::ALREADY_IN_GROUP),
            5 => Ok(Self::NOT_IN_GROUP),
            6 => Ok(Self::NOT_LEADER),
            7 => Ok(Self::PLAYER_WRONG_FACTION),
            8 => Ok(Self::IGNORING_YOU),
            v => Err(crate::errors::EnumError::new("PartyResult", v as u32),)
        }
    }
}

