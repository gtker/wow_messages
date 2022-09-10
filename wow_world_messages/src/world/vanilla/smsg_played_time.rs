use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_played_time.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_played_time.wowm#L3):
/// ```text
/// smsg SMSG_PLAYED_TIME = 0x01CD {
///     u32 total_played_time;
///     u32 level_played_time;
/// }
/// ```
pub struct SMSG_PLAYED_TIME {
    pub total_played_time: u32,
    pub level_played_time: u32,
}

impl crate::Message for SMSG_PLAYED_TIME {
    const OPCODE: u32 = 0x01cd;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // total_played_time: u32
        w.write_all(&self.total_played_time.to_le_bytes())?;

        // level_played_time: u32
        w.write_all(&self.level_played_time.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // total_played_time: u32
        let total_played_time = crate::util::read_u32_le(r)?;

        // level_played_time: u32
        let level_played_time = crate::util::read_u32_le(r)?;

        Ok(Self {
            total_played_time,
            level_played_time,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_PLAYED_TIME {}

