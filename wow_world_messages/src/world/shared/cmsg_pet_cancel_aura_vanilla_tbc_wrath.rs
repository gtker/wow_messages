use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_cancel_aura.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_cancel_aura.wowm#L3):
/// ```text
/// cmsg CMSG_PET_CANCEL_AURA = 0x026B {
///     Guid guid;
///     u32 id;
/// }
/// ```
pub struct CMSG_PET_CANCEL_AURA {
    pub guid: Guid,
    pub id: u32,
}

impl crate::Message for CMSG_PET_CANCEL_AURA {
    const OPCODE: u32 = 0x026b;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x026B, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_CANCEL_AURA {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PET_CANCEL_AURA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_CANCEL_AURA {}

