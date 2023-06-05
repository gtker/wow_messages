use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_removed_alert.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_removed_alert.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_REMOVED_ALERT = 0x0443 {
///     Bool show_alert;
///     Guid event_id;
///     DateTime event_time;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_REMOVED_ALERT {
    pub show_alert: bool,
    pub event_id: Guid,
    pub event_time: DateTime,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CALENDAR_EVENT_REMOVED_ALERT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_EVENT_REMOVED_ALERT {{").unwrap();
        // Members
        writeln!(s, "    show_alert = {};", if self.show_alert { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    event_id = {};", self.event_id.guid()).unwrap();
        writeln!(s, "    event_time = {};", self.event_time.as_int()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 17_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1091_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "show_alert");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_CALENDAR_EVENT_REMOVED_ALERT {}
impl crate::Message for SMSG_CALENDAR_EVENT_REMOVED_ALERT {
    const OPCODE: u32 = 0x0443;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0443, size: body_size });
        }

        // show_alert: Bool
        let show_alert = crate::util::read_u8_le(&mut r)? != 0;

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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_REMOVED_ALERT {}

