use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_loot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_loot.wowm#L3):
/// ```text
/// cmsg CMSG_LOOT = 0x015D {
///     Guid guid;
/// }
/// ```
pub struct CMSG_LOOT {
    pub guid: Guid,
}

impl crate::Message for CMSG_LOOT {
    const OPCODE: u32 = 0x015d;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x015D, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LOOT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LOOT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LOOT {}

