use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::CalendarStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_event_status.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_event_status.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_EVENT_STATUS = 0x0434 {
///     Guid event;
///     Guid invite_id;
///     Guid sender_invite_id;
///     CalendarStatus status;
/// }
/// ```
pub struct CMSG_CALENDAR_EVENT_STATUS {
    pub event: Guid,
    pub invite_id: Guid,
    pub sender_invite_id: Guid,
    pub status: CalendarStatus,
}

impl crate::private::Sealed for CMSG_CALENDAR_EVENT_STATUS {}
impl crate::Message for CMSG_CALENDAR_EVENT_STATUS {
    const OPCODE: u32 = 0x0434;

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // sender_invite_id: Guid
        w.write_all(&self.sender_invite_id.guid().to_le_bytes())?;

        // status: CalendarStatus
        w.write_all(&(self.status.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 25 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0434, size: body_size });
        }

        // event: Guid
        let event = crate::util::read_guid(&mut r)?;

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

        // sender_invite_id: Guid
        let sender_invite_id = crate::util::read_guid(&mut r)?;

        // status: CalendarStatus
        let status = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            event,
            invite_id,
            sender_invite_id,
            status,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_EVENT_STATUS {}

