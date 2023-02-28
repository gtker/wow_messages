use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_clear_target.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_clear_target.wowm#L1):
/// ```text
/// smsg SMSG_CLEAR_TARGET = 0x03BE {
///     Guid target;
/// }
/// ```
pub struct SMSG_CLEAR_TARGET {
    pub target: Guid,
}

impl crate::Message for SMSG_CLEAR_TARGET {
    const OPCODE: u32 = 0x03be;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03BE, size: body_size as u32 });
        }

        // target: Guid
        let target = Guid::read(r)?;

        Ok(Self {
            target,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CLEAR_TARGET {}

