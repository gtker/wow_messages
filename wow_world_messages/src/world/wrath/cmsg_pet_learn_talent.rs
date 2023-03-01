use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_pet_learn_talent.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_pet_learn_talent.wowm#L1):
/// ```text
/// cmsg CMSG_PET_LEARN_TALENT = 0x047A {
///     Guid pet;
///     u32 talent;
///     u32 rank;
/// }
/// ```
pub struct CMSG_PET_LEARN_TALENT {
    pub pet: Guid,
    pub talent: u32,
    pub rank: u32,
}

impl crate::Message for CMSG_PET_LEARN_TALENT {
    const OPCODE: u32 = 0x047a;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // talent: u32
        w.write_all(&self.talent.to_le_bytes())?;

        // rank: u32
        w.write_all(&self.rank.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x047A, size: body_size as u32 });
        }

        // pet: Guid
        let pet = Guid::read(&mut r)?;

        // talent: u32
        let talent = crate::util::read_u32_le(&mut r)?;

        // rank: u32
        let rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            pet,
            talent,
            rank,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_LEARN_TALENT {}

