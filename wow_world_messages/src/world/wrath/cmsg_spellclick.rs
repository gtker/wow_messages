use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_spellclick.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_spellclick.wowm#L7):
/// ```text
/// cmsg CMSG_SPELLCLICK = 0x03F8 {
///     Guid target;
/// }
/// ```
pub struct CMSG_SPELLCLICK {
    pub target: Guid,
}

impl crate::Message for CMSG_SPELLCLICK {
    const OPCODE: u32 = 0x03f8;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F8, size: body_size as u32 });
        }

        // target: Guid
        let target = Guid::read(r)?;

        Ok(Self {
            target,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SPELLCLICK {}

