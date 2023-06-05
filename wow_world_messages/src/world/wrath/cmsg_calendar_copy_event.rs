use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_copy_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_copy_event.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_COPY_EVENT = 0x0430 {
///     Guid event;
///     Guid invite_id;
///     DateTime time;
/// }
/// ```
pub struct CMSG_CALENDAR_COPY_EVENT {
    pub event: Guid,
    pub invite_id: Guid,
    pub time: DateTime,
}

impl crate::private::Sealed for CMSG_CALENDAR_COPY_EVENT {}
impl crate::Message for CMSG_CALENDAR_COPY_EVENT {
    const OPCODE: u32 = 0x0430;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // time: DateTime
        w.write_all(&self.time.as_int().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0430, size: body_size });
        }

        // event: Guid
        let event = crate::util::read_guid(&mut r)?;

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

        // time: DateTime
        let time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        Ok(Self {
            event,
            invite_id,
            time,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_COPY_EVENT {}

