use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_removed_alert.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_removed_alert.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_REMOVED_ALERT = 0x0443 {
///     Bool show_alert;
///     Guid event_id;
///     DateTime event_time;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CALENDAR_EVENT_REMOVED_ALERT {
    pub show_alert: bool,
    pub event_id: Guid,
    pub event_time: DateTime,
}

impl crate::private::Sealed for SMSG_CALENDAR_EVENT_REMOVED_ALERT {}
impl SMSG_CALENDAR_EVENT_REMOVED_ALERT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 13 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // show_alert: Bool
        let show_alert = crate::util::read_bool_u8(&mut r)?;

        // event_id: Guid
        let event_id = crate::util::read_guid(&mut r)?;

        // event_time: DateTime
        let event_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        Ok(Self {
            show_alert,
            event_id,
            event_time,
        })
    }

}

impl crate::Message for SMSG_CALENDAR_EVENT_REMOVED_ALERT {
    const OPCODE: u32 = 0x0443;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CALENDAR_EVENT_REMOVED_ALERT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_EVENT_REMOVED_ALERT {{").unwrap();
        // Members
        writeln!(s, "    show_alert = {};", if self.show_alert { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    event_id = {};", self.event_id.guid()).unwrap();
        writeln!(s, "    event_time = {};", self.event_time.as_int()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 15_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1091_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "show_alert", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "event_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "event_time", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // show_alert: Bool
        w.write_all(u8::from(self.show_alert).to_le_bytes().as_slice())?;

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // event_time: DateTime
        w.write_all(&self.event_time.as_int().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1091, "SMSG_CALENDAR_EVENT_REMOVED_ALERT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_REMOVED_ALERT {}

