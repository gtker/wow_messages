use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm#L3):
/// ```text
/// cmsg CMSG_OFFER_PETITION = 0x01C3 {
///     Guid petition_guid;
///     Guid target_guid;
/// }
/// ```
pub struct CMSG_OFFER_PETITION {
    pub petition_guid: Guid,
    pub target_guid: Guid,
}

impl crate::Message for CMSG_OFFER_PETITION {
    const OPCODE: u32 = 0x01c3;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            petition_guid,
            target_guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_OFFER_PETITION {}

