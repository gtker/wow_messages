use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_arena_team_roster.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_arena_team_roster.wowm#L10):
/// ```text
/// enum ArenaTeamRole : u8 {
///     CAPTAIN = 0;
///     MEMBER = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ArenaTeamRole {
    Captain,
    Member,
}

impl ArenaTeamRole {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Captain => 0x0,
            Self::Member => 0x1,
        }
    }

}

impl Default for ArenaTeamRole {
    fn default() -> Self {
        Self::Captain
    }
}

impl std::fmt::Display for ArenaTeamRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Captain => f.write_str("Captain"),
            Self::Member => f.write_str("Member"),
        }
    }
}

impl TryFrom<u8> for ArenaTeamRole {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Captain),
            1 => Ok(Self::Member),
            v => Err(crate::errors::EnumError::new("ArenaTeamRole", v as u32),)
        }
    }
}

