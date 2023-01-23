use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_cancel_aura.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_cancel_aura.wowm#L3):
/// ```text
/// cmsg CMSG_CANCEL_AURA = 0x0136 {
///     u32 id;
/// }
/// ```
pub struct CMSG_CANCEL_AURA {
    pub id: u32,
}

impl crate::Message for CMSG_CANCEL_AURA {
    const OPCODE: u32 = 0x0136;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0136, size: body_size as u32 });
        }

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_CANCEL_AURA {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_CANCEL_AURA {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CANCEL_AURA {}

