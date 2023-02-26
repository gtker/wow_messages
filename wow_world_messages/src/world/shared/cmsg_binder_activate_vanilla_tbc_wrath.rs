use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_binder_activate.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_binder_activate.wowm#L3):
/// ```text
/// cmsg CMSG_BINDER_ACTIVATE = 0x01B5 {
///     Guid guid;
/// }
/// ```
pub struct CMSG_BINDER_ACTIVATE {
    pub guid: Guid,
}

impl crate::Message for CMSG_BINDER_ACTIVATE {
    const OPCODE: u32 = 0x01b5;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01B5, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BINDER_ACTIVATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BINDER_ACTIVATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BINDER_ACTIVATE {}

