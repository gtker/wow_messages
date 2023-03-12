use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/cmsg_gameobj_use.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/cmsg_gameobj_use.wowm#L3):
/// ```text
/// cmsg CMSG_GAMEOBJ_USE = 0x00B1 {
///     Guid guid;
/// }
/// ```
pub struct CMSG_GAMEOBJ_USE {
    pub guid: Guid,
}

impl crate::Message for CMSG_GAMEOBJ_USE {
    const OPCODE: u32 = 0x00b1;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00B1, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GAMEOBJ_USE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GAMEOBJ_USE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GAMEOBJ_USE {}

