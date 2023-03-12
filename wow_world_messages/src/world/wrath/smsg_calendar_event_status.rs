use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_status.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_status.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_STATUS = 0x043C {
///     PackedGuid invitee;
///     Guid event_id;
///     DateTime event_time;
///     u32 flags;
///     u8 status;
///     u8 rank;
///     DateTime status_time;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_STATUS {
    pub invitee: Guid,
    pub event_id: Guid,
    pub event_time: DateTime,
    pub flags: u32,
    pub status: u8,
    pub rank: u8,
    pub status_time: DateTime,
}

impl crate::Message for SMSG_CALENDAR_EVENT_STATUS {
    const OPCODE: u32 = 0x043c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // invitee: PackedGuid
        self.invitee.write_packed_guid_into_vec(&mut w)?;

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // event_time: DateTime
        w.write_all(&self.event_time.as_int().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // status: u8
        w.write_all(&self.status.to_le_bytes())?;

        // rank: u8
        w.write_all(&self.rank.to_le_bytes())?;

        // status_time: DateTime
        w.write_all(&self.status_time.as_int().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(24..=31).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x043C, size: body_size as u32 });
        }

        // invitee: PackedGuid
        let invitee = Guid::read_packed(&mut r)?;

        // event_id: Guid
        let event_id = Guid::read(&mut r)?;

        // event_time: DateTime
        let event_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // status: u8
        let status = crate::util::read_u8_le(&mut r)?;

        // rank: u8
        let rank = crate::util::read_u8_le(&mut r)?;

        // status_time: DateTime
        let status_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            invitee,
            event_id,
            event_time,
            flags,
            status,
            rank,
            status_time,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_STATUS {}

impl SMSG_CALENDAR_EVENT_STATUS {
    pub(crate) fn size(&self) -> usize {
        self.invitee.size() // invitee: PackedGuid
        + 8 // event_id: Guid
        + 4 // event_time: DateTime
        + 4 // flags: u32
        + 1 // status: u8
        + 1 // rank: u8
        + 4 // status_time: DateTime
    }
}

