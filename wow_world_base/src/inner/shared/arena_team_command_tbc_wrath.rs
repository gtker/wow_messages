/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_command_result.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_command_result.wowm#L22):
/// ```text
/// enum ArenaTeamCommand : u32 {
///     TEAM_CREATE_S = 0x00;
///     TEAM_INVITE_SS = 0x01;
///     TEAM_QUIT_S = 0x03;
///     TEAM_FOUNDER_S = 0x0E;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ArenaTeamCommand {
    TeamCreateS,
    TeamInviteSs,
    TeamQuitS,
    TeamFounderS,
}

impl ArenaTeamCommand {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::TeamCreateS => 0x0,
            Self::TeamInviteSs => 0x1,
            Self::TeamQuitS => 0x3,
            Self::TeamFounderS => 0xe,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::TeamCreateS,
            Self::TeamInviteSs,
            Self::TeamQuitS,
            Self::TeamFounderS,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::TeamCreateS),
            1 => Ok(Self::TeamInviteSs),
            3 => Ok(Self::TeamQuitS),
            14 => Ok(Self::TeamFounderS),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ArenaTeamCommand {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::TeamCreateS => "TEAM_CREATE_S",
            Self::TeamInviteSs => "TEAM_INVITE_SS",
            Self::TeamQuitS => "TEAM_QUIT_S",
            Self::TeamFounderS => "TEAM_FOUNDER_S",
        }
    }

}

const NAME: &str = "ArenaTeamCommand";

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
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for ArenaTeamCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for ArenaTeamCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ArenaTeamCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ArenaTeamCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ArenaTeamCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ArenaTeamCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ArenaTeamCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ArenaTeamCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

