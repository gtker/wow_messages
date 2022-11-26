use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_stable_revive_pet.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_stable_revive_pet.wowm#L1):
/// ```text
/// cmsg CMSG_STABLE_REVIVE_PET = 0x0274 {
/// }
/// ```
pub struct CMSG_STABLE_REVIVE_PET {
}

impl crate::Message for CMSG_STABLE_REVIVE_PET {
    const OPCODE: u32 = 0x0274;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0274, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_STABLE_REVIVE_PET {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_STABLE_REVIVE_PET {}

