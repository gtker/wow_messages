use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/cmsg_area_spirit_healer_queue.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/cmsg_area_spirit_healer_queue.wowm#L3):
/// ```text
/// cmsg CMSG_AREA_SPIRIT_HEALER_QUEUE = 0x02E3 {
///     Guid guid;
/// }
/// ```
pub struct CMSG_AREA_SPIRIT_HEALER_QUEUE {
    pub guid: Guid,
}

impl crate::private::Sealed for CMSG_AREA_SPIRIT_HEALER_QUEUE {}
impl crate::Message for CMSG_AREA_SPIRIT_HEALER_QUEUE {
    const OPCODE: u32 = 0x02e3;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E3, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AREA_SPIRIT_HEALER_QUEUE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AREA_SPIRIT_HEALER_QUEUE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AREA_SPIRIT_HEALER_QUEUE {}

