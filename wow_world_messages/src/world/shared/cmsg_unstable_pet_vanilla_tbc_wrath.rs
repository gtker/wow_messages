use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_unstable_pet.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_unstable_pet.wowm#L3):
/// ```text
/// cmsg CMSG_UNSTABLE_PET = 0x0271 {
///     Guid stable_master;
///     u32 pet_number;
/// }
/// ```
pub struct CMSG_UNSTABLE_PET {
    pub stable_master: Guid,
    pub pet_number: u32,
}

impl crate::private::Sealed for CMSG_UNSTABLE_PET {}
impl crate::Message for CMSG_UNSTABLE_PET {
    const OPCODE: u32 = 0x0271;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // stable_master: Guid
        w.write_all(&self.stable_master.guid().to_le_bytes())?;

        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0271, size: body_size });
        }

        // stable_master: Guid
        let stable_master = Guid::read(&mut r)?;

        // pet_number: u32
        let pet_number = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            stable_master,
            pet_number,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_UNSTABLE_PET {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_UNSTABLE_PET {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_UNSTABLE_PET {}

