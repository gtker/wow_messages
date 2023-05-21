use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_event_signup.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_event_signup.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_EVENT_SIGNUP = 0x04BA {
///     Guid event_id;
///     Bool tentative;
/// }
/// ```
pub struct CMSG_CALENDAR_EVENT_SIGNUP {
    pub event_id: Guid,
    pub tentative: bool,
}

impl crate::private::Sealed for CMSG_CALENDAR_EVENT_SIGNUP {}
impl CMSG_CALENDAR_EVENT_SIGNUP {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04BA, size: body_size });
        }

        // event_id: Guid
        let event_id = crate::util::read_guid(&mut r)?;

        // tentative: Bool
        let tentative = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            event_id,
            tentative,
        })
    }

}

impl crate::Message for CMSG_CALENDAR_EVENT_SIGNUP {
    const OPCODE: u32 = 0x04ba;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CALENDAR_EVENT_SIGNUP {{").unwrap();
        // Members
        writeln!(s, "    event_id = {};", self.event_id.guid()).unwrap();
        writeln!(s, "    tentative = {};", if self.tentative { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 13_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1210_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "event_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "tentative", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // tentative: Bool
        w.write_all(u8::from(self.tentative).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_EVENT_SIGNUP {}

