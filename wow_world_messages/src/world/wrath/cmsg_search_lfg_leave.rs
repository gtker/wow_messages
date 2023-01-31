use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_search_lfg_leave.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_search_lfg_leave.wowm#L1):
/// ```text
/// cmsg CMSG_SEARCH_LFG_LEAVE = 0x035F {
///     u32 dungeon_id;
/// }
/// ```
pub struct CMSG_SEARCH_LFG_LEAVE {
    pub dungeon_id: u32,
}

impl crate::Message for CMSG_SEARCH_LFG_LEAVE {
    const OPCODE: u32 = 0x035f;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x035F, size: body_size as u32 });
        }

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            dungeon_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SEARCH_LFG_LEAVE {}

