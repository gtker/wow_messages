use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_arena_team_command_result.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_arena_team_command_result.wowm#L23):
/// ```text
/// enum ArenaTeamCommand : u32 {
///     TEAM_CREATE_S = 0x00;
///     TEAM_INVITE_SS = 0x01;
///     TEAM_QUIT_S = 0x03;
///     TEAM_FOUNDER_S = 0x0E;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ArenaTeamCommand {
    TeamCreateS,
    TeamInviteSs,
    TeamQuitS,
    TeamFounderS,
}

impl ArenaTeamCommand {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::TeamCreateS => 0x0,
            Self::TeamInviteSs => 0x1,
            Self::TeamQuitS => 0x3,
            Self::TeamFounderS => 0xe,
        }
    }

}

impl Default for ArenaTeamCommand {
    fn default() -> Self {
        Self::TeamCreateS
    }
}

impl std::fmt::Display for ArenaTeamCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TeamCreateS => f.write_str("TeamCreateS"),
            Self::TeamInviteSs => f.write_str("TeamInviteSs"),
            Self::TeamQuitS => f.write_str("TeamQuitS"),
            Self::TeamFounderS => f.write_str("TeamFounderS"),
        }
    }
}

impl TryFrom<u32> for ArenaTeamCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::TeamCreateS),
            1 => Ok(Self::TeamInviteSs),
            3 => Ok(Self::TeamQuitS),
            14 => Ok(Self::TeamFounderS),
            v => Err(crate::errors::EnumError::new("ArenaTeamCommand", v as u32),)
        }
    }
}

