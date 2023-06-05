/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_refer_a_friend_failure.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_refer_a_friend_failure.wowm#L1):
/// ```text
/// enum ReferAFriendError : u8 {
///     NONE = 0x00;
///     NOT_REFERRED_BY = 0x01;
///     TARGET_TOO_HIGH = 0x02;
///     INSUFFICIENT_GRANTABLE_LEVELS = 0x03;
///     TOO_FAR = 0x04;
///     DIFFERENT_FACTION = 0x05;
///     NOT_NOW = 0x06;
///     GRANT_LEVEL_MAX = 0x07;
///     NO_TARGET = 0x08;
///     NOT_IN_GROUP = 0x09;
///     SUMMON_LEVEL_MAX = 0x0A;
///     SUMMON_COOLDOWN = 0x0B;
///     INSUFFICIENT_EXPANSION_LEVEL = 0x0C;
///     SUMMON_OFFLINE = 0x0D;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ReferAFriendError {
    None,
    NotReferredBy,
    TargetTooHigh,
    InsufficientGrantableLevels,
    TooFar,
    DifferentFaction,
    NotNow,
    GrantLevelMax,
    NoTarget,
    NotInGroup,
    SummonLevelMax,
    SummonCooldown,
    InsufficientExpansionLevel,
    SummonOffline,
}

impl ReferAFriendError {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::NotReferredBy => 0x1,
            Self::TargetTooHigh => 0x2,
            Self::InsufficientGrantableLevels => 0x3,
            Self::TooFar => 0x4,
            Self::DifferentFaction => 0x5,
            Self::NotNow => 0x6,
            Self::GrantLevelMax => 0x7,
            Self::NoTarget => 0x8,
            Self::NotInGroup => 0x9,
            Self::SummonLevelMax => 0xa,
            Self::SummonCooldown => 0xb,
            Self::InsufficientExpansionLevel => 0xc,
            Self::SummonOffline => 0xd,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl ReferAFriendError {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::NotReferredBy => "NOT_REFERRED_BY",
            Self::TargetTooHigh => "TARGET_TOO_HIGH",
            Self::InsufficientGrantableLevels => "INSUFFICIENT_GRANTABLE_LEVELS",
            Self::TooFar => "TOO_FAR",
            Self::DifferentFaction => "DIFFERENT_FACTION",
            Self::NotNow => "NOT_NOW",
            Self::GrantLevelMax => "GRANT_LEVEL_MAX",
            Self::NoTarget => "NO_TARGET",
            Self::NotInGroup => "NOT_IN_GROUP",
            Self::SummonLevelMax => "SUMMON_LEVEL_MAX",
            Self::SummonCooldown => "SUMMON_COOLDOWN",
            Self::InsufficientExpansionLevel => "INSUFFICIENT_EXPANSION_LEVEL",
            Self::SummonOffline => "SUMMON_OFFLINE",
        }
    }

}

impl Default for ReferAFriendError {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for ReferAFriendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::NotReferredBy => f.write_str("NotReferredBy"),
            Self::TargetTooHigh => f.write_str("TargetTooHigh"),
            Self::InsufficientGrantableLevels => f.write_str("InsufficientGrantableLevels"),
            Self::TooFar => f.write_str("TooFar"),
            Self::DifferentFaction => f.write_str("DifferentFaction"),
            Self::NotNow => f.write_str("NotNow"),
            Self::GrantLevelMax => f.write_str("GrantLevelMax"),
            Self::NoTarget => f.write_str("NoTarget"),
            Self::NotInGroup => f.write_str("NotInGroup"),
            Self::SummonLevelMax => f.write_str("SummonLevelMax"),
            Self::SummonCooldown => f.write_str("SummonCooldown"),
            Self::InsufficientExpansionLevel => f.write_str("InsufficientExpansionLevel"),
            Self::SummonOffline => f.write_str("SummonOffline"),
        }
    }
}

impl TryFrom<u8> for ReferAFriendError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::NotReferredBy),
            2 => Ok(Self::TargetTooHigh),
            3 => Ok(Self::InsufficientGrantableLevels),
            4 => Ok(Self::TooFar),
            5 => Ok(Self::DifferentFaction),
            6 => Ok(Self::NotNow),
            7 => Ok(Self::GrantLevelMax),
            8 => Ok(Self::NoTarget),
            9 => Ok(Self::NotInGroup),
            10 => Ok(Self::SummonLevelMax),
            11 => Ok(Self::SummonCooldown),
            12 => Ok(Self::InsufficientExpansionLevel),
            13 => Ok(Self::SummonOffline),
            v => Err(crate::errors::EnumError::new("ReferAFriendError", v as u64),)
        }
    }
}

