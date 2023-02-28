use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_get_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_get_event.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_GET_EVENT = 0x042A {
///     Guid event;
/// }
/// ```
pub struct CMSG_CALENDAR_GET_EVENT {
    pub event: Guid,
}

impl crate::Message for CMSG_CALENDAR_GET_EVENT {
    const OPCODE: u32 = 0x042a;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x042A, size: body_size as u32 });
        }

        // event: Guid
        let event = Guid::read(r)?;

        Ok(Self {
            event,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_GET_EVENT {}

