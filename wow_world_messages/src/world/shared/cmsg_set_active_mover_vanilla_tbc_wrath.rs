use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_set_active_mover.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_set_active_mover.wowm#L3):
/// ```text
/// cmsg CMSG_SET_ACTIVE_MOVER = 0x026A {
///     Guid guid;
/// }
/// ```
pub struct CMSG_SET_ACTIVE_MOVER {
    pub guid: Guid,
}

impl crate::Message for CMSG_SET_ACTIVE_MOVER {
    const OPCODE: u32 = 0x026a;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x026A, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SET_ACTIVE_MOVER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_ACTIVE_MOVER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_ACTIVE_MOVER {}

