use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_corpse_map_position_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_corpse_map_position_query.wowm#L1):
/// ```text
/// cmsg CMSG_CORPSE_MAP_POSITION_QUERY = 0x04B6 {
///     u32 unknown;
/// }
/// ```
pub struct CMSG_CORPSE_MAP_POSITION_QUERY {
    pub unknown: u32,
}

impl crate::Message for CMSG_CORPSE_MAP_POSITION_QUERY {
    const OPCODE: u32 = 0x04b6;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04B6, size: body_size as u32 });
        }

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CORPSE_MAP_POSITION_QUERY {}

