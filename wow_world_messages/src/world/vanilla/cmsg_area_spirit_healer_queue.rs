use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/cmsg_area_spirit_healer_queue.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/cmsg_area_spirit_healer_queue.wowm#L3):
/// ```text
/// cmsg CMSG_AREA_SPIRIT_HEALER_QUEUE = 0x02E3 {
///     Guid guid;
/// }
/// ```
pub struct CMSG_AREA_SPIRIT_HEALER_QUEUE {
    pub guid: Guid,
}

impl crate::Message for CMSG_AREA_SPIRIT_HEALER_QUEUE {
    const OPCODE: u32 = 0x02e3;

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
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_AREA_SPIRIT_HEALER_QUEUE {}

