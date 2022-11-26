use std::convert::{TryFrom, TryInto};
use crate::world::shared::arena_team_command_tbc_wrath::ArenaTeamCommand;
use crate::world::shared::arena_team_command_error_tbc_wrath::ArenaTeamCommandError;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_arena_team_command_result.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_arena_team_command_result.wowm#L32):
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

impl crate::Message for SMSG_ARENA_TEAM_COMMAND_RESULT {
    const OPCODE: u32 = 0x0349;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // command: ArenaTeamCommand
        w.write_all(&(self.command.as_int() as u32).to_le_bytes())?;

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
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=520).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0349, size: body_size as u32 });
        }

        // command: ArenaTeamCommand
        let command: ArenaTeamCommand = crate::util::read_u32_le(r)?.try_into()?;

        // team: CString
        let team = crate::util::read_c_string_to_vec(r)?;
        let team = String::from_utf8(team)?;

        // player: CString
        let player = crate::util::read_c_string_to_vec(r)?;
        let player = String::from_utf8(player)?;

        // error: ArenaTeamCommandError
        let error: ArenaTeamCommandError = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            command,
            team,
            player,
            error,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_ARENA_TEAM_COMMAND_RESULT {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ARENA_TEAM_COMMAND_RESULT {}

impl SMSG_ARENA_TEAM_COMMAND_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // command: ArenaTeamCommand
        + self.team.len() + 1 // team: CString
        + self.player.len() + 1 // player: CString
        + 4 // error: ArenaTeamCommandError
    }
}

