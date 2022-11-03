use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_TIME_SKIPPED = 0x02CE {
///     Guid guid;
///     u32 lag;
/// }
/// ```
pub struct CMSG_MOVE_TIME_SKIPPED {
    pub guid: Guid,
    pub lag: u32,
}

impl crate::Message for CMSG_MOVE_TIME_SKIPPED {
    const OPCODE: u32 = 0x02ce;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // lag: u32
        w.write_all(&self.lag.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02CE, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // lag: u32
        let lag = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            lag,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_MOVE_TIME_SKIPPED {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_MOVE_TIME_SKIPPED {}

