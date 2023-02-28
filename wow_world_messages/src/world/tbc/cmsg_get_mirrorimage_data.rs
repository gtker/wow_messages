use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_get_mirrorimage_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_get_mirrorimage_data.wowm#L1):
/// ```text
/// cmsg CMSG_GET_MIRRORIMAGE_DATA = 0x0400 {
///     Guid target;
/// }
/// ```
pub struct CMSG_GET_MIRRORIMAGE_DATA {
    pub target: Guid,
}

impl crate::Message for CMSG_GET_MIRRORIMAGE_DATA {
    const OPCODE: u32 = 0x0400;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0400, size: body_size as u32 });
        }

        // target: Guid
        let target = Guid::read(r)?;

        Ok(Self {
            target,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GET_MIRRORIMAGE_DATA {}

