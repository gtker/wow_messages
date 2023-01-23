use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_request_party_member_stats.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_request_party_member_stats.wowm#L3):
/// ```text
/// cmsg CMSG_REQUEST_PARTY_MEMBER_STATS = 0x027F {
///     Guid guid;
/// }
/// ```
pub struct CMSG_REQUEST_PARTY_MEMBER_STATS {
    pub guid: Guid,
}

impl crate::Message for CMSG_REQUEST_PARTY_MEMBER_STATS {
    const OPCODE: u32 = 0x027f;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x027F, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_REQUEST_PARTY_MEMBER_STATS {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_REQUEST_PARTY_MEMBER_STATS {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_REQUEST_PARTY_MEMBER_STATS {}

