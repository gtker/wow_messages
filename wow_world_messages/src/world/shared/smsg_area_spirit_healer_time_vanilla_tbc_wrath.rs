use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // next_resurrect_time: u32
        w.write_all(&self.next_resurrect_time.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E4, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // next_resurrect_time: u32
        let next_resurrect_time = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            next_resurrect_time,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AREA_SPIRIT_HEALER_TIME {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AREA_SPIRIT_HEALER_TIME {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AREA_SPIRIT_HEALER_TIME {}

