use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            id,
        })
    }

}
impl ClientMessage for CMSG_PET_CANCEL_AURA {}

