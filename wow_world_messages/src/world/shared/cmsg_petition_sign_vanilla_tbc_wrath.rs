use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_petition_sign.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_petition_sign.wowm#L3):
/// ```text
/// cmsg CMSG_PETITION_SIGN = 0x01C0 {
///     Guid petition;
///     u8 unknown1;
/// }
/// ```
pub struct CMSG_PETITION_SIGN {
    pub petition: Guid,
    pub unknown1: u8,
}

impl crate::Message for CMSG_PETITION_SIGN {
    const OPCODE: u32 = 0x01c0;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C0, size: body_size as u32 });
        }

        // petition: Guid
        let petition = Guid::read(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            petition,
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PETITION_SIGN {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PETITION_SIGN {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PETITION_SIGN {}

