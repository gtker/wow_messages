use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/cmsg_arena_team_remove.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/cmsg_arena_team_remove.wowm#L1):
/// ```text
/// cmsg CMSG_ARENA_TEAM_REMOVE = 0x0354 {
///     u32 arena_team;
///     CString player;
/// }
/// ```
pub struct CMSG_ARENA_TEAM_REMOVE {
    pub arena_team: u32,
    pub player: String,
}

impl crate::Message for CMSG_ARENA_TEAM_REMOVE {
    const OPCODE: u32 = 0x0354;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        // player: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player.as_bytes().iter().rev().next(), Some(&0_u8), "String `player` must not be null-terminated.");
        w.write_all(self.player.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0354, size: body_size as u32 });
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(&mut r)?;

        // player: CString
        let player = {
            let player = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player)?
        };

        Ok(Self {
            arena_team,
            player,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ARENA_TEAM_REMOVE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ARENA_TEAM_REMOVE {}

impl CMSG_ARENA_TEAM_REMOVE {
    pub(crate) fn size(&self) -> usize {
        4 // arena_team: u32
        + self.player.len() + 1 // player: CString
    }
}

