use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// This message only exists as a coment in trinitycore.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_notes.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_notes.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_INVITE_NOTES = 0x0460 {
///     PackedGuid invitee;
///     Guid invite_id;
///     CString text;
///     Bool unknown;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_INVITE_NOTES {
    pub invitee: Guid,
    pub invite_id: Guid,
    pub text: String,
    pub unknown: bool,
}

impl crate::private::Sealed for SMSG_CALENDAR_EVENT_INVITE_NOTES {}
impl crate::Message for SMSG_CALENDAR_EVENT_INVITE_NOTES {
    const OPCODE: u32 = 0x0460;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // invitee: PackedGuid
        self.invitee.write_packed_guid_into_vec(&mut w)?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown: Bool
        w.write_all(u8::from(self.unknown).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(12..=274).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0460, size: body_size });
        }

        // invitee: PackedGuid
        let invitee = Guid::read_packed(&mut r)?;

        // invite_id: Guid
        let invite_id = Guid::read(&mut r)?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(text)?
        };

        // unknown: Bool
        let unknown = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            invitee,
            invite_id,
            text,
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_INVITE_NOTES {}

impl SMSG_CALENDAR_EVENT_INVITE_NOTES {
    pub(crate) fn size(&self) -> usize {
        self.invitee.size() // invitee: PackedGuid
        + 8 // invite_id: Guid
        + self.text.len() + 1 // text: CString
        + 1 // unknown: Bool
    }
}

