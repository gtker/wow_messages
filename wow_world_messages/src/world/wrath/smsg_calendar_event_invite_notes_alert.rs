use crate::Guid;
use std::io::{Write, Read};

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

impl crate::Message for SMSG_CALENDAR_EVENT_INVITE_NOTES_ALERT {
    const OPCODE: u32 = 0x0461;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
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
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0461, size: body_size as u32 });
        }

        // invite_id: Guid
        let invite_id = Guid::read(r)?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(r)?;
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

