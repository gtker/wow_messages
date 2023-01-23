use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_stop_attack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_stop_attack.wowm#L3):
/// ```text
/// cmsg CMSG_PET_STOP_ATTACK = 0x02EA {
///     Guid pet;
/// }
/// ```
pub struct CMSG_PET_STOP_ATTACK {
    pub pet: Guid,
}

impl crate::Message for CMSG_PET_STOP_ATTACK {
    const OPCODE: u32 = 0x02ea;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02EA, size: body_size as u32 });
        }

        // pet: Guid
        let pet = Guid::read(r)?;

        Ok(Self {
            pet,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_PET_STOP_ATTACK {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_PET_STOP_ATTACK {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_PET_STOP_ATTACK {}

