/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update_player.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update_player.wowm#L1):
/// ```text
/// enum LfgUpdateType : u8 {
///     DEFAULT = 0;
///     LEADER_LEAVE = 1;
///     ROLECHECK_ABORTED = 4;
///     JOIN = 5;
///     ROLECHECK_FAILED = 6;
///     LEAVE = 7;
///     PROPOSAL_FAILED = 8;
///     PROPOSAL_DECLINED = 9;
///     GROUP_FOUND = 10;
///     ADDED_TO_QUEUE = 12;
///     PROPOSAL_BEGIN = 13;
///     STATUS = 14;
///     GROUP_MEMBER_OFFLINE = 15;
///     GROUP_DISBAND = 16;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LfgUpdateType {
    Default,
    LeaderLeave,
    RolecheckAborted,
    Join,
    RolecheckFailed,
    Leave,
    ProposalFailed,
    ProposalDeclined,
    GroupFound,
    AddedToQueue,
    ProposalBegin,
    Status,
    GroupMemberOffline,
    GroupDisband,
}

impl LfgUpdateType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Default => 0x0,
            Self::LeaderLeave => 0x1,
            Self::RolecheckAborted => 0x4,
            Self::Join => 0x5,
            Self::RolecheckFailed => 0x6,
            Self::Leave => 0x7,
            Self::ProposalFailed => 0x8,
            Self::ProposalDeclined => 0x9,
            Self::GroupFound => 0xa,
            Self::AddedToQueue => 0xc,
            Self::ProposalBegin => 0xd,
            Self::Status => 0xe,
            Self::GroupMemberOffline => 0xf,
            Self::GroupDisband => 0x10,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl LfgUpdateType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Default => "DEFAULT",
            Self::LeaderLeave => "LEADER_LEAVE",
            Self::RolecheckAborted => "ROLECHECK_ABORTED",
            Self::Join => "JOIN",
            Self::RolecheckFailed => "ROLECHECK_FAILED",
            Self::Leave => "LEAVE",
            Self::ProposalFailed => "PROPOSAL_FAILED",
            Self::ProposalDeclined => "PROPOSAL_DECLINED",
            Self::GroupFound => "GROUP_FOUND",
            Self::AddedToQueue => "ADDED_TO_QUEUE",
            Self::ProposalBegin => "PROPOSAL_BEGIN",
            Self::Status => "STATUS",
            Self::GroupMemberOffline => "GROUP_MEMBER_OFFLINE",
            Self::GroupDisband => "GROUP_DISBAND",
        }
    }

}

const NAME: &str = "LfgUpdateType";

impl Default for LfgUpdateType {
    fn default() -> Self {
        Self::Default
    }
}

impl std::fmt::Display for LfgUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => f.write_str("Default"),
            Self::LeaderLeave => f.write_str("LeaderLeave"),
            Self::RolecheckAborted => f.write_str("RolecheckAborted"),
            Self::Join => f.write_str("Join"),
            Self::RolecheckFailed => f.write_str("RolecheckFailed"),
            Self::Leave => f.write_str("Leave"),
            Self::ProposalFailed => f.write_str("ProposalFailed"),
            Self::ProposalDeclined => f.write_str("ProposalDeclined"),
            Self::GroupFound => f.write_str("GroupFound"),
            Self::AddedToQueue => f.write_str("AddedToQueue"),
            Self::ProposalBegin => f.write_str("ProposalBegin"),
            Self::Status => f.write_str("Status"),
            Self::GroupMemberOffline => f.write_str("GroupMemberOffline"),
            Self::GroupDisband => f.write_str("GroupDisband"),
        }
    }
}

impl TryFrom<u8> for LfgUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::LeaderLeave),
            4 => Ok(Self::RolecheckAborted),
            5 => Ok(Self::Join),
            6 => Ok(Self::RolecheckFailed),
            7 => Ok(Self::Leave),
            8 => Ok(Self::ProposalFailed),
            9 => Ok(Self::ProposalDeclined),
            10 => Ok(Self::GroupFound),
            12 => Ok(Self::AddedToQueue),
            13 => Ok(Self::ProposalBegin),
            14 => Ok(Self::Status),
            15 => Ok(Self::GroupMemberOffline),
            16 => Ok(Self::GroupDisband),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for LfgUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for LfgUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for LfgUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for LfgUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for LfgUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for LfgUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for LfgUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for LfgUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

