use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_set_contact_notes.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_set_contact_notes.wowm#L1):
/// ```text
/// cmsg CMSG_SET_CONTACT_NOTES = 0x006B {
///     Guid player;
///     CString note;
/// }
/// ```
pub struct CMSG_SET_CONTACT_NOTES {
    pub player: Guid,
    pub note: String,
}

impl crate::Message for CMSG_SET_CONTACT_NOTES {
    const OPCODE: u32 = 0x006b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // note: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.note.as_bytes().iter().rev().next(), Some(&0_u8), "String `note` must not be null-terminated.");
        w.write_all(self.note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x006B, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(r)?;

        // note: CString
        let note = crate::util::read_c_string_to_vec(r)?;
        let note = String::from_utf8(note)?;

        Ok(Self {
            player,
            note,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_CONTACT_NOTES {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_CONTACT_NOTES {}

impl CMSG_SET_CONTACT_NOTES {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + self.note.len() + 1 // note: CString
    }
}

