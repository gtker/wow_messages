use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::CalendarModeratorRank;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_event_moderator_status.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_event_moderator_status.wowm#L9):
/// ```text
/// cmsg CMSG_CALENDAR_EVENT_MODERATOR_STATUS = 0x0435 {
///     Guid event;
///     Guid invite_id;
///     Guid sender_invite_id;
///     CalendarModeratorRank rank;
/// }
/// ```
pub struct CMSG_CALENDAR_EVENT_MODERATOR_STATUS {
    pub event: Guid,
    pub invite_id: Guid,
    pub sender_invite_id: Guid,
    pub rank: CalendarModeratorRank,
}

impl crate::private::Sealed for CMSG_CALENDAR_EVENT_MODERATOR_STATUS {}
impl crate::Message for CMSG_CALENDAR_EVENT_MODERATOR_STATUS {
    const OPCODE: u32 = 0x0435;

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // sender_invite_id: Guid
        w.write_all(&self.sender_invite_id.guid().to_le_bytes())?;

        // rank: CalendarModeratorRank
        w.write_all(&u8::from(self.rank.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 25 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0435, size: body_size as u32 });
        }

        // event: Guid
        let event = Guid::read(&mut r)?;

        // invite_id: Guid
        let invite_id = Guid::read(&mut r)?;

        // sender_invite_id: Guid
        let sender_invite_id = Guid::read(&mut r)?;

        // rank: CalendarModeratorRank
        let rank: CalendarModeratorRank = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            event,
            invite_id,
            sender_invite_id,
            rank,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_EVENT_MODERATOR_STATUS {}

