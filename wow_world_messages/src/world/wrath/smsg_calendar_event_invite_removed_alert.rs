use crate:: {
    DateTime,
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_removed_alert.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_removed_alert.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_INVITE_REMOVED_ALERT = 0x0441 {
///     Guid event_id;
///     DateTime event_time;
///     u32 flags;
///     u8 status;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_INVITE_REMOVED_ALERT {
    pub event_id: Guid,
    pub event_time: DateTime,
    pub flags: u32,
    pub status: u8,
}

impl crate::Message for SMSG_CALENDAR_EVENT_INVITE_REMOVED_ALERT {
    const OPCODE: u32 = 0x0441;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // event_time: DateTime
        w.write_all(&self.event_time.as_int().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // status: u8
        w.write_all(&self.status.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0441, size: body_size as u32 });
        }

        // event_id: Guid
        let event_id = Guid::read(&mut r)?;

        // event_time: DateTime
        let event_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // status: u8
        let status = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            event_id,
            event_time,
            flags,
            status,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_INVITE_REMOVED_ALERT {}

