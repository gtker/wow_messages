use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm#L8):
/// ```text
/// cmsg CMSG_OFFER_PETITION = 0x01C3 {
///     u32 unknown0;
///     Guid petition_guid;
///     Guid target_guid;
/// }
/// ```
pub struct CMSG_OFFER_PETITION {
    pub unknown0: u32,
    pub petition_guid: Guid,
    pub target_guid: Guid,
}

impl crate::Message for CMSG_OFFER_PETITION {
    const OPCODE: u32 = 0x01c3;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown0: u32
        w.write_all(&self.unknown0.to_le_bytes())?;

        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C3, size: body_size as u32 });
        }

        // unknown0: u32
        let unknown0 = crate::util::read_u32_le(r)?;

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            unknown0,
            petition_guid,
            target_guid,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_OFFER_PETITION {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_OFFER_PETITION {}

