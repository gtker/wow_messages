use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_pet_learn_talent.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_pet_learn_talent.wowm#L1):
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // talent: u32
        w.write_all(&self.talent.to_le_bytes())?;

        // rank: u32
        w.write_all(&self.rank.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x047A, size: body_size as u32 });
        }

        // pet: Guid
        let pet = Guid::read(r)?;

        // talent: u32
        let talent = crate::util::read_u32_le(r)?;

        // rank: u32
        let rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            pet,
            talent,
            rank,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_PET_LEARN_TALENT {}

