use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_arena_team.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_arena_team.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_ARENA_TEAM = 0x042C {
///     u32 arena_team;
/// }
/// ```
pub struct CMSG_CALENDAR_ARENA_TEAM {
    pub arena_team: u32,
}

impl crate::private::Sealed for CMSG_CALENDAR_ARENA_TEAM {}
impl crate::Message for CMSG_CALENDAR_ARENA_TEAM {
    const OPCODE: u32 = 0x042c;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x042C, size: body_size });
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            arena_team,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_ARENA_TEAM {}

