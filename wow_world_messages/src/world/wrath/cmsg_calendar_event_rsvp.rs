use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::CalendarStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_event_rsvp.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_event_rsvp.wowm#L16):
/// ```text
/// cmsg CMSG_CALENDAR_EVENT_RSVP = 0x0432 {
///     Guid event;
///     Guid invite_id;
///     (u32)CalendarStatus status;
/// }
/// ```
pub struct CMSG_CALENDAR_EVENT_RSVP {
    pub event: Guid,
    pub invite_id: Guid,
    pub status: CalendarStatus,
}

impl crate::private::Sealed for CMSG_CALENDAR_EVENT_RSVP {}
impl CMSG_CALENDAR_EVENT_RSVP {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 20 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // event: Guid
        let event = crate::util::read_guid(&mut r)?;

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

        // status: CalendarStatus
        let status = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            event,
            invite_id,
            status,
        })
    }

}

impl crate::Message for CMSG_CALENDAR_EVENT_RSVP {
    const OPCODE: u32 = 0x0432;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_CALENDAR_EVENT_RSVP"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CALENDAR_EVENT_RSVP {{").unwrap();
        // Members
        writeln!(s, "    event = {};", self.event.guid()).unwrap();
        writeln!(s, "    invite_id = {};", self.invite_id.guid()).unwrap();
        writeln!(s, "    status = {};", self.status.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 24_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1074_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "event", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "status", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // status: CalendarStatus
        w.write_all(&u32::from(self.status.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1074, "CMSG_CALENDAR_EVENT_RSVP", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_EVENT_RSVP {}

