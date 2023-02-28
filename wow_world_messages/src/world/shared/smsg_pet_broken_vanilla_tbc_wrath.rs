use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Not implemented in any Wrath emulator.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_broken.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_broken.wowm#L3):
/// ```text
/// smsg SMSG_PET_BROKEN = 0x02AF {
/// }
/// ```
pub struct SMSG_PET_BROKEN {
}

impl crate::Message for SMSG_PET_BROKEN {
    const OPCODE: u32 = 0x02af;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02AF, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_BROKEN {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_BROKEN {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_BROKEN {}

