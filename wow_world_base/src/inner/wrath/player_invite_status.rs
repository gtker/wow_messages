/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_invite.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_invite.wowm#L7):
/// ```text
/// enum PlayerInviteStatus : u8 {
///     ALREADY_IN_GROUP = 0;
///     NOT_IN_GROUP = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PlayerInviteStatus {
    AlreadyInGroup,
    NotInGroup,
}

impl PlayerInviteStatus {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::AlreadyInGroup => 0x0,
            Self::NotInGroup => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl PlayerInviteStatus {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::AlreadyInGroup => "ALREADY_IN_GROUP",
            Self::NotInGroup => "NOT_IN_GROUP",
        }
    }

}

impl Default for PlayerInviteStatus {
    fn default() -> Self {
        Self::AlreadyInGroup
    }
}

impl std::fmt::Display for PlayerInviteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyInGroup => f.write_str("AlreadyInGroup"),
            Self::NotInGroup => f.write_str("NotInGroup"),
        }
    }
}

impl TryFrom<u8> for PlayerInviteStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AlreadyInGroup),
            1 => Ok(Self::NotInGroup),
            v => Err(crate::errors::EnumError::new("PlayerInviteStatus", v.into()),)
        }
    }
}

