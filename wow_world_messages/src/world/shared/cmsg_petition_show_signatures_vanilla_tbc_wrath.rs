use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_petition_show_signatures.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_petition_show_signatures.wowm#L3):
/// ```text
/// cmsg CMSG_PETITION_SHOW_SIGNATURES = 0x01BE {
///     Guid item;
/// }
/// ```
pub struct CMSG_PETITION_SHOW_SIGNATURES {
    pub item: Guid,
}

impl crate::Message for CMSG_PETITION_SHOW_SIGNATURES {
    const OPCODE: u32 = 0x01be;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01BE, size: body_size as u32 });
        }

        // item: Guid
        let item = Guid::read(r)?;

        Ok(Self {
            item,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PETITION_SHOW_SIGNATURES {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PETITION_SHOW_SIGNATURES {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PETITION_SHOW_SIGNATURES {}

