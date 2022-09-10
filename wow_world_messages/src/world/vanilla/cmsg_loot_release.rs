use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_loot_release.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_loot_release.wowm#L3):
/// ```text
/// cmsg CMSG_LOOT_RELEASE = 0x015F {
///     Guid guid;
/// }
/// ```
pub struct CMSG_LOOT_RELEASE {
    pub guid: Guid,
}

impl crate::Message for CMSG_LOOT_RELEASE {
    const OPCODE: u32 = 0x015f;

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
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for CMSG_LOOT_RELEASE {}

