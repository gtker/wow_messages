use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/cmsg_arena_team_roster.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/cmsg_arena_team_roster.wowm#L1):
/// ```text
/// cmsg CMSG_ARENA_TEAM_ROSTER = 0x034D {
///     u32 arena_team;
/// }
/// ```
pub struct CMSG_ARENA_TEAM_ROSTER {
    pub arena_team: u32,
}

impl crate::Message for CMSG_ARENA_TEAM_ROSTER {
    const OPCODE: u32 = 0x034d;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x034D, size: body_size as u32 });
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(r)?;

        Ok(Self {
            arena_team,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_ARENA_TEAM_ROSTER {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_ARENA_TEAM_ROSTER {}

