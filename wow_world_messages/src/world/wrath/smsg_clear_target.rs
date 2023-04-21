use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_clear_target.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_clear_target.wowm#L7):
/// ```text
/// smsg SMSG_CLEAR_TARGET = 0x03BF {
///     Guid target;
/// }
/// ```
pub struct SMSG_CLEAR_TARGET {
    pub target: Guid,
}

impl crate::private::Sealed for SMSG_CLEAR_TARGET {}
impl crate::Message for SMSG_CLEAR_TARGET {
    const OPCODE: u32 = 0x03bf;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03BF, size: body_size as u32 });
        }

        // target: Guid
        let target = Guid::read(&mut r)?;

        Ok(Self {
            target,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CLEAR_TARGET {}

