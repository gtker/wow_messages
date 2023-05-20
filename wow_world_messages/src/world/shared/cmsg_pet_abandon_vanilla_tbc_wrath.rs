use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_abandon.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_abandon.wowm#L3):
/// ```text
/// cmsg CMSG_PET_ABANDON = 0x0176 {
///     Guid pet;
/// }
/// ```
pub struct CMSG_PET_ABANDON {
    pub pet: Guid,
}

impl crate::private::Sealed for CMSG_PET_ABANDON {}
impl crate::Message for CMSG_PET_ABANDON {
    const OPCODE: u32 = 0x0176;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0176, size: body_size });
        }

        // pet: Guid
        let pet = Guid::read(&mut r)?;

        Ok(Self {
            pet,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_ABANDON {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PET_ABANDON {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_ABANDON {}

