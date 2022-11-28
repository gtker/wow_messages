use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_accept_level_grant.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_accept_level_grant.wowm#L7):
/// ```text
/// cmsg CMSG_ACCEPT_LEVEL_GRANT = 0x0420 {
///     PackedGuid guid;
/// }
/// ```
pub struct CMSG_ACCEPT_LEVEL_GRANT {
    pub guid: Guid,
}

impl crate::Message for CMSG_ACCEPT_LEVEL_GRANT {
    const OPCODE: u32 = 0x0420;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0420, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_ACCEPT_LEVEL_GRANT {}

impl CMSG_ACCEPT_LEVEL_GRANT {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

