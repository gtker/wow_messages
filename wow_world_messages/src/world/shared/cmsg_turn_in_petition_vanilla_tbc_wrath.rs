use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_turn_in_petition.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_turn_in_petition.wowm#L3):
/// ```text
/// cmsg CMSG_TURN_IN_PETITION = 0x01C4 {
///     Guid petition;
/// }
/// ```
pub struct CMSG_TURN_IN_PETITION {
    pub petition: Guid,
}

impl crate::Message for CMSG_TURN_IN_PETITION {
    const OPCODE: u32 = 0x01c4;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C4, size: body_size as u32 });
        }

        // petition: Guid
        let petition = Guid::read(&mut r)?;

        Ok(Self {
            petition,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TURN_IN_PETITION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TURN_IN_PETITION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TURN_IN_PETITION {}

