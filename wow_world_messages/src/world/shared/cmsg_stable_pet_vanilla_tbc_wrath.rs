use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_stable_pet.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_stable_pet.wowm#L3):
/// ```text
/// cmsg CMSG_STABLE_PET = 0x0270 {
///     Guid stable_master;
/// }
/// ```
pub struct CMSG_STABLE_PET {
    pub stable_master: Guid,
}

impl crate::Message for CMSG_STABLE_PET {
    const OPCODE: u32 = 0x0270;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // stable_master: Guid
        w.write_all(&self.stable_master.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0270, size: body_size as u32 });
        }

        // stable_master: Guid
        let stable_master = Guid::read(r)?;

        Ok(Self {
            stable_master,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_STABLE_PET {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_STABLE_PET {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_STABLE_PET {}

