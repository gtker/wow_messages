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

#[cfg(feature = "print-testcase")]
impl CMSG_CALENDAR_EVENT_MODERATOR_STATUS {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CALENDAR_EVENT_MODERATOR_STATUS {{").unwrap();
        // Members
        writeln!(s, "    event = {};", self.event.guid()).unwrap();
        writeln!(s, "    invite_id = {};", self.invite_id.guid()).unwrap();
        writeln!(s, "    sender_invite_id = {};", self.sender_invite_id.guid()).unwrap();
        writeln!(s, "    rank = {};", self.rank.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 29_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1077_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "event", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "sender_invite_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "rank", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_CALENDAR_EVENT_MODERATOR_STATUS {}
impl crate::Message for CMSG_CALENDAR_EVENT_MODERATOR_STATUS {
    const OPCODE: u32 = 0x0435;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_CALENDAR_EVENT_MODERATOR_STATUS::to_test_case_string(self)
    }

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

        // rank: CalendarModeratorRank
        w.write_all(&(self.rank.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 25 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0435, size: body_size });
        }

        // event: Guid
        let event = crate::util::read_guid(&mut r)?;

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

        // sender_invite_id: Guid
        let sender_invite_id = crate::util::read_guid(&mut r)?;

        // rank: CalendarModeratorRank
        let rank = crate::util::read_u8_le(&mut r)?.try_into()?;

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

