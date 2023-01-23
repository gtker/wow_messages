use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Emus just set all values to 0.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_corpse_map_position_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_corpse_map_position_query_response.wowm#L1):
/// ```text
/// smsg SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE = 0x04B7 {
///     f32 unknown1;
///     f32 unknown2;
///     f32 unknown3;
///     f32 unknown4;
/// }
/// ```
pub struct SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {
    pub unknown1: f32,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown4: f32,
}

impl crate::Message for SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {
    const OPCODE: u32 = 0x04b7;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: f32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: f32
        w.write_all(&self.unknown4.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04B7, size: body_size as u32 });
        }

        // unknown1: f32
        let unknown1 = crate::util::read_f32_le(r)?;
        // unknown2: f32
        let unknown2 = crate::util::read_f32_le(r)?;
        // unknown3: f32
        let unknown3 = crate::util::read_f32_le(r)?;
        // unknown4: f32
        let unknown4 = crate::util::read_f32_le(r)?;
        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            unknown4,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {}

