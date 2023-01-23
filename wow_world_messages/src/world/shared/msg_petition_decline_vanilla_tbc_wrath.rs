use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_petition_decline.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_petition_decline.wowm#L3):
/// ```text
/// msg MSG_PETITION_DECLINE = 0x01C2 {
///     Guid petition;
/// }
/// ```
pub struct MSG_PETITION_DECLINE {
    pub petition: Guid,
}

impl crate::Message for MSG_PETITION_DECLINE {
    const OPCODE: u32 = 0x01c2;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C2, size: body_size as u32 });
        }

        // petition: Guid
        let petition = Guid::read(r)?;

        Ok(Self {
            petition,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_PETITION_DECLINE {}

