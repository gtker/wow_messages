use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_clientcache_version.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_clientcache_version.wowm#L1):
/// ```text
/// smsg SMSG_CLIENTCACHE_VERSION = 0x04AB {
///     u32 version;
/// }
/// ```
pub struct SMSG_CLIENTCACHE_VERSION {
    pub version: u32,
}

impl crate::Message for SMSG_CLIENTCACHE_VERSION {
    const OPCODE: u32 = 0x04ab;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // version: u32
        w.write_all(&self.version.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // version: u32
        let version = crate::util::read_u32_le(r)?;

        Ok(Self {
            version,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CLIENTCACHE_VERSION {}

