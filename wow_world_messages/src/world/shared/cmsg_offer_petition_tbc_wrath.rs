use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm#L8):
/// ```text
/// cmsg CMSG_OFFER_PETITION = 0x01C3 {
///     u32 unknown0;
///     Guid petition;
///     Guid target;
/// }
/// ```
pub struct CMSG_OFFER_PETITION {
    pub unknown0: u32,
    pub petition: Guid,
    pub target: Guid,
}

impl crate::private::Sealed for CMSG_OFFER_PETITION {}
impl crate::Message for CMSG_OFFER_PETITION {
    const OPCODE: u32 = 0x01c3;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown0: u32
        w.write_all(&self.unknown0.to_le_bytes())?;

        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C3, size: body_size });
        }

        // unknown0: u32
        let unknown0 = crate::util::read_u32_le(&mut r)?;

        // petition: Guid
        let petition = crate::util::read_guid(&mut r)?;

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        Ok(Self {
            unknown0,
            petition,
            target,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_OFFER_PETITION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_OFFER_PETITION {}

