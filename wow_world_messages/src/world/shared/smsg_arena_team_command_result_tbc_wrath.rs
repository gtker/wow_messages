use std::io::{Read, Write};

use wow_world_base::shared::arena_team_command_error_tbc_wrath::ArenaTeamCommandError;
use wow_world_base::shared::arena_team_command_tbc_wrath::ArenaTeamCommand;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_command_result.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_command_result.wowm#L32):
/// ```text
/// smsg SMSG_ARENA_TEAM_COMMAND_RESULT = 0x0349 {
///     ArenaTeamCommand command;
///     CString team;
///     CString player;
///     ArenaTeamCommandError error;
/// }
/// ```
pub struct SMSG_ARENA_TEAM_COMMAND_RESULT {
    pub command: ArenaTeamCommand,
    pub team: String,
    pub player: String,
    pub error: ArenaTeamCommandError,
}

impl crate::private::Sealed for SMSG_ARENA_TEAM_COMMAND_RESULT {}
impl crate::Message for SMSG_ARENA_TEAM_COMMAND_RESULT {
    const OPCODE: u32 = 0x0349;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // command: ArenaTeamCommand
        w.write_all(&u32::from(self.command.as_int()).to_le_bytes())?;

        // team: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.team.as_bytes().iter().rev().next(), Some(&0_u8), "String `team` must not be null-terminated.");
        w.write_all(self.team.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // player: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player.as_bytes().iter().rev().next(), Some(&0_u8), "String `player` must not be null-terminated.");
        w.write_all(self.player.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // error: ArenaTeamCommandError
        w.write_all(&u32::from(self.error.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=520).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0349, size: body_size as u32 });
        }

        // command: ArenaTeamCommand
        let command: ArenaTeamCommand = crate::util::read_u32_le(&mut r)?.try_into()?;

        // team: CString
        let team = {
            let team = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(team)?
        };

        // player: CString
        let player = {
            let player = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player)?
        };

        // error: ArenaTeamCommandError
        let error: ArenaTeamCommandError = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            command,
            team,
            player,
            error,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_TEAM_COMMAND_RESULT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_TEAM_COMMAND_RESULT {}

impl SMSG_ARENA_TEAM_COMMAND_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // command: ArenaTeamCommand
        + self.team.len() + 1 // team: CString
        + self.player.len() + 1 // player: CString
        + 4 // error: ArenaTeamCommandError
    }
}

