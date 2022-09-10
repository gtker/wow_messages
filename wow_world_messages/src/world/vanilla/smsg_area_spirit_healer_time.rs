use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_area_spirit_healer_time.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_area_spirit_healer_time.wowm#L3):
/// ```text
/// smsg SMSG_AREA_SPIRIT_HEALER_TIME = 0x02E4 {
///     Guid guid;
///     u32 next_resurrect_time;
/// }
/// ```
pub struct SMSG_AREA_SPIRIT_HEALER_TIME {
    pub guid: Guid,
    pub next_resurrect_time: u32,
}

impl crate::Message for SMSG_AREA_SPIRIT_HEALER_TIME {
    const OPCODE: u32 = 0x02e4;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // next_resurrect_time: u32
        w.write_all(&self.next_resurrect_time.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // next_resurrect_time: u32
        let next_resurrect_time = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            next_resurrect_time,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_AREA_SPIRIT_HEALER_TIME {}

