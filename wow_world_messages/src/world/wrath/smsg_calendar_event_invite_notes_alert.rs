use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// This message only exists as a comment in trinitycore.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_notes_alert.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_notes_alert.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_INVITE_NOTES_ALERT = 0x0461 {
///     Guid invite_id;
///     CString text;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_INVITE_NOTES_ALERT {
    pub invite_id: Guid,
    pub text: String,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CALENDAR_EVENT_INVITE_NOTES_ALERT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_EVENT_INVITE_NOTES_ALERT {{").unwrap();
        // Members
        writeln!(s, "    invite_id = {};", self.invite_id.guid()).unwrap();
        writeln!(s, "    text = \"{}\";", self.text).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1121_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id");
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

impl crate::private::Sealed for SMSG_CALENDAR_EVENT_INVITE_NOTES_ALERT {}
impl crate::Message for SMSG_CALENDAR_EVENT_INVITE_NOTES_ALERT {
    const OPCODE: u32 = 0x0461;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0461, size: body_size });
        }

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(text)?
        };

        Ok(Self {
            invite_id,
            text,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_INVITE_NOTES_ALERT {}

impl SMSG_CALENDAR_EVENT_INVITE_NOTES_ALERT {
    pub(crate) fn size(&self) -> usize {
        8 // invite_id: Guid
        + self.text.len() + 1 // text: CString
    }
}

