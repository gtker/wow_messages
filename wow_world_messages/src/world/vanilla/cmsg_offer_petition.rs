use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm#L1):
/// ```text
/// cmsg CMSG_OFFER_PETITION = 0x01C3 {
///     Guid petition;
///     Guid target;
/// }
/// ```
pub struct CMSG_OFFER_PETITION {
    pub petition: Guid,
    pub target: Guid,
}

impl crate::Message for CMSG_OFFER_PETITION {
    const OPCODE: u32 = 0x01c3;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C3, size: body_size as u32 });
        }

        // petition: Guid
        let petition = Guid::read(r)?;

        // target: Guid
        let target = Guid::read(r)?;

        Ok(Self {
            petition,
            target,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_OFFER_PETITION {}

