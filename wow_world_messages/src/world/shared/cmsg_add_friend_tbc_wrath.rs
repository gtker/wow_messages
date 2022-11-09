use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_add_friend.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_add_friend.wowm#L7):
/// ```text
/// cmsg CMSG_ADD_FRIEND = 0x0069 {
///     CString friend_name;
///     CString friend_note;
/// }
/// ```
pub struct CMSG_ADD_FRIEND {
    pub friend_name: String,
    pub friend_note: String,
}

impl crate::Message for CMSG_ADD_FRIEND {
    const OPCODE: u32 = 0x0069;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // friend_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.friend_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `friend_name` must not be null-terminated.");
        w.write_all(self.friend_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // friend_note: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.friend_note.as_bytes().iter().rev().next(), Some(&0_u8), "String `friend_note` must not be null-terminated.");
        w.write_all(self.friend_note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=512).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0069, size: body_size as u32 });
        }

        // friend_name: CString
        let friend_name = crate::util::read_c_string_to_vec(r)?;
        let friend_name = String::from_utf8(friend_name)?;

        // friend_note: CString
        let friend_note = crate::util::read_c_string_to_vec(r)?;
        let friend_note = String::from_utf8(friend_note)?;

        Ok(Self {
            friend_name,
            friend_note,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_ADD_FRIEND {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_ADD_FRIEND {}

impl CMSG_ADD_FRIEND {
    pub(crate) fn size(&self) -> usize {
        self.friend_name.len() + 1 // friend_name: CString
        + self.friend_note.len() + 1 // friend_note: CString
    }
}

