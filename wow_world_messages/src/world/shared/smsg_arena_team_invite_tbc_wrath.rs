use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_invite.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_TEAM_INVITE = 0x0350 {
///     CString player_name;
///     CString team_name;
/// }
/// ```
pub struct SMSG_ARENA_TEAM_INVITE {
    pub player_name: String,
    pub team_name: String,
}

impl crate::Message for SMSG_ARENA_TEAM_INVITE {
    const OPCODE: u32 = 0x0350;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `player_name` must not be null-terminated.");
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // team_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.team_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `team_name` must not be null-terminated.");
        w.write_all(self.team_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=512).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0350, size: body_size as u32 });
        }

        // player_name: CString
        let player_name = {
            let player_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player_name)?
        };

        // team_name: CString
        let team_name = {
            let team_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(team_name)?
        };

        Ok(Self {
            player_name,
            team_name,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_TEAM_INVITE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_TEAM_INVITE {}

impl SMSG_ARENA_TEAM_INVITE {
    pub(crate) fn size(&self) -> usize {
        self.player_name.len() + 1 // player_name: CString
        + self.team_name.len() + 1 // team_name: CString
    }
}

