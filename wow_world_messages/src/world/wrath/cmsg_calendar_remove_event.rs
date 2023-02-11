use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_remove_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_remove_event.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_REMOVE_EVENT = 0x042F {
///     Guid event;
///     Guid invite_id;
///     u32 flags;
/// }
/// ```
pub struct CMSG_CALENDAR_REMOVE_EVENT {
    pub event: Guid,
    pub invite_id: Guid,
    pub flags: u32,
}

impl crate::Message for CMSG_CALENDAR_REMOVE_EVENT {
    const OPCODE: u32 = 0x042f;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x042F, size: body_size as u32 });
        }

        // event: Guid
        let event = Guid::read(r)?;

        // invite_id: Guid
        let invite_id = Guid::read(r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        Ok(Self {
            event,
            invite_id,
            flags,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_REMOVE_EVENT {}

