use std::io::{Read, Write};

use crate::Guid;

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

impl crate::private::Sealed for MSG_PETITION_DECLINE {}
impl crate::Message for MSG_PETITION_DECLINE {
    const OPCODE: u32 = 0x01c2;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C2, size: body_size });
        }

        // petition: Guid
        let petition = Guid::read(&mut r)?;

        Ok(Self {
            petition,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_PETITION_DECLINE {}

