use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_petition_rename.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_petition_rename.wowm#L3):
/// ```text
/// msg MSG_PETITION_RENAME = 0x02C1 {
///     Guid petition;
///     CString new_name;
/// }
/// ```
pub struct MSG_PETITION_RENAME {
    pub petition: Guid,
    pub new_name: String,
}

impl crate::Message for MSG_PETITION_RENAME {
    const OPCODE: u32 = 0x02c1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        // new_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.new_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `new_name` must not be null-terminated.");
        w.write_all(self.new_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C1, size: body_size as u32 });
        }

        // petition: Guid
        let petition = Guid::read(r)?;

        // new_name: CString
        let new_name = crate::util::read_c_string_to_vec(r)?;
        let new_name = String::from_utf8(new_name)?;

        Ok(Self {
            petition,
            new_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for MSG_PETITION_RENAME {}

#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_PETITION_RENAME {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for MSG_PETITION_RENAME {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_PETITION_RENAME {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_PETITION_RENAME {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_PETITION_RENAME {}

impl MSG_PETITION_RENAME {
    pub(crate) fn size(&self) -> usize {
        8 // petition: Guid
        + self.new_name.len() + 1 // new_name: CString
    }
}

