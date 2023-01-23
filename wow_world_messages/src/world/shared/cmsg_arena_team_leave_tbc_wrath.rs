use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/cmsg_arena_team_leave.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/cmsg_arena_team_leave.wowm#L1):
/// ```text
/// cmsg CMSG_ARENA_TEAM_LEAVE = 0x0353 {
///     u32 arena_team;
/// }
/// ```
pub struct CMSG_ARENA_TEAM_LEAVE {
    pub arena_team: u32,
}

impl crate::Message for CMSG_ARENA_TEAM_LEAVE {
    const OPCODE: u32 = 0x0353;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0353, size: body_size as u32 });
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(r)?;

        Ok(Self {
            arena_team,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_ARENA_TEAM_LEAVE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_ARENA_TEAM_LEAVE {}

