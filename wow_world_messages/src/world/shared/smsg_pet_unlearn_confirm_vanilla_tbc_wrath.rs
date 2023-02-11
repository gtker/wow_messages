use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_unlearn_confirm.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_unlearn_confirm.wowm#L3):
/// ```text
/// smsg SMSG_PET_UNLEARN_CONFIRM = 0x02F1 {
///     Guid pet;
///     u32 talent_reset_cost;
/// }
/// ```
pub struct SMSG_PET_UNLEARN_CONFIRM {
    pub pet: Guid,
    pub talent_reset_cost: u32,
}

impl crate::Message for SMSG_PET_UNLEARN_CONFIRM {
    const OPCODE: u32 = 0x02f1;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // talent_reset_cost: u32
        w.write_all(&self.talent_reset_cost.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02F1, size: body_size as u32 });
        }

        // pet: Guid
        let pet = Guid::read(r)?;

        // talent_reset_cost: u32
        let talent_reset_cost = crate::util::read_u32_le(r)?;

        Ok(Self {
            pet,
            talent_reset_cost,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_UNLEARN_CONFIRM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_UNLEARN_CONFIRM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_UNLEARN_CONFIRM {}

