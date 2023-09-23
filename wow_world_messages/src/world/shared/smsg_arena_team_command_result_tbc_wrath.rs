use std::io::{Read, Write};

use wow_world_base::shared::arena_team_command_error_tbc_wrath::ArenaTeamCommandError;
use wow_world_base::shared::arena_team_command_tbc_wrath::ArenaTeamCommand;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_command_result.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_command_result.wowm#L31):
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
impl SMSG_ARENA_TEAM_COMMAND_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(10..=520).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // command: ArenaTeamCommand
        let command = crate::util::read_u32_le(&mut r)?.try_into()?;

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
        let error = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            command,
            team,
            player,
            error,
        })
    }

}

impl crate::Message for SMSG_ARENA_TEAM_COMMAND_RESULT {
    const OPCODE: u32 = 0x0349;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ARENA_TEAM_COMMAND_RESULT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ARENA_TEAM_COMMAND_RESULT {{").unwrap();
        // Members
        writeln!(s, "    command = {};", self.command.as_test_case_value()).unwrap();
        writeln!(s, "    team = \"{}\";", self.team).unwrap();
        writeln!(s, "    player = \"{}\";", self.player).unwrap();
        writeln!(s, "    error = {};", self.error.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 841_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "command", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.team.len() + 1, "team", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.player.len() + 1, "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "error", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // command: ArenaTeamCommand
        w.write_all(&(self.command.as_int().to_le_bytes()))?;

        // team: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.team.as_bytes().iter().next_back(), Some(&0_u8), "String `team` must not be null-terminated.");
        w.write_all(self.team.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // player: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player.as_bytes().iter().next_back(), Some(&0_u8), "String `player` must not be null-terminated.");
        w.write_all(self.player.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // error: ArenaTeamCommandError
        w.write_all(&(self.error.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(841, "SMSG_ARENA_TEAM_COMMAND_RESULT", body_size, a))
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

