use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_petition_rename.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_petition_rename.wowm#L3):
/// ```text
/// msg MSG_PETITION_RENAME = 0x02C1 {
///     Guid petition_guid;
///     CString new_name;
/// }
/// ```
pub struct MSG_PETITION_RENAME {
    pub petition_guid: Guid,
    pub new_name: String,
}

impl crate::Message for MSG_PETITION_RENAME {
    const OPCODE: u32 = 0x02c1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // new_name: CString
        w.write_all(self.new_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // new_name: CString
        let new_name = crate::util::read_c_string_to_vec(r)?;
        let new_name = String::from_utf8(new_name)?;

        Ok(Self {
            petition_guid,
            new_name,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for MSG_PETITION_RENAME {}

#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for MSG_PETITION_RENAME {}

impl MSG_PETITION_RENAME {
    pub(crate) fn size(&self) -> usize {
        8 // petition_guid: Guid
        + self.new_name.len() + 1 // new_name: CString
    }
}

