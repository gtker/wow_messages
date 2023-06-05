/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_command_result.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_command_result.wowm#L15):
/// ```text
/// enum PartyResult : u8 {
///     SUCCESS = 0;
///     BAD_PLAYER_NAME = 1;
///     TARGET_NOT_IN_GROUP = 2;
///     TARGET_NOT_IN_INSTANCE = 3;
///     GROUP_FULL = 4;
///     ALREADY_IN_GROUP = 5;
///     NOT_IN_GROUP = 6;
///     NOT_LEADER = 7;
///     PLAYER_WRONG_FACTION = 8;
///     IGNORING_YOU = 9;
///     LFG_PENDING = 12;
///     INVITE_RESTRICTED = 13;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PartyResult {
    Success,
    BadPlayerName,
    TargetNotInGroup,
    TargetNotInInstance,
    GroupFull,
    AlreadyInGroup,
    NotInGroup,
    NotLeader,
    PlayerWrongFaction,
    IgnoringYou,
    LfgPending,
    InviteRestricted,
}

impl PartyResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Success => 0x0,
            Self::BadPlayerName => 0x1,
            Self::TargetNotInGroup => 0x2,
            Self::TargetNotInInstance => 0x3,
            Self::GroupFull => 0x4,
            Self::AlreadyInGroup => 0x5,
            Self::NotInGroup => 0x6,
            Self::NotLeader => 0x7,
            Self::PlayerWrongFaction => 0x8,
            Self::IgnoringYou => 0x9,
            Self::LfgPending => 0xc,
            Self::InviteRestricted => 0xd,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl PartyResult {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::BadPlayerName => "BAD_PLAYER_NAME",
            Self::TargetNotInGroup => "TARGET_NOT_IN_GROUP",
            Self::TargetNotInInstance => "TARGET_NOT_IN_INSTANCE",
            Self::GroupFull => "GROUP_FULL",
            Self::AlreadyInGroup => "ALREADY_IN_GROUP",
            Self::NotInGroup => "NOT_IN_GROUP",
            Self::NotLeader => "NOT_LEADER",
            Self::PlayerWrongFaction => "PLAYER_WRONG_FACTION",
            Self::IgnoringYou => "IGNORING_YOU",
            Self::LfgPending => "LFG_PENDING",
            Self::InviteRestricted => "INVITE_RESTRICTED",
        }
    }

}

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
            Self::TargetNotInInstance => f.write_str("TargetNotInInstance"),
            Self::GroupFull => f.write_str("GroupFull"),
            Self::AlreadyInGroup => f.write_str("AlreadyInGroup"),
            Self::NotInGroup => f.write_str("NotInGroup"),
            Self::NotLeader => f.write_str("NotLeader"),
            Self::PlayerWrongFaction => f.write_str("PlayerWrongFaction"),
            Self::IgnoringYou => f.write_str("IgnoringYou"),
            Self::LfgPending => f.write_str("LfgPending"),
            Self::InviteRestricted => f.write_str("InviteRestricted"),
        }
    }
}

impl TryFrom<u8> for PartyResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            1 => Ok(Self::BadPlayerName),
            2 => Ok(Self::TargetNotInGroup),
            3 => Ok(Self::TargetNotInInstance),
            4 => Ok(Self::GroupFull),
            5 => Ok(Self::AlreadyInGroup),
            6 => Ok(Self::NotInGroup),
            7 => Ok(Self::NotLeader),
            8 => Ok(Self::PlayerWrongFaction),
            9 => Ok(Self::IgnoringYou),
            12 => Ok(Self::LfgPending),
            13 => Ok(Self::InviteRestricted),
            v => Err(crate::errors::EnumError::new("PartyResult", v as u64),)
        }
    }
}

