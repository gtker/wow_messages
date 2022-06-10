use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_unlearn_confirm.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_unlearn_confirm.wowm#L3):
/// ```text
/// smsg SMSG_PET_UNLEARN_CONFIRM = 0x02F1 {
///     Guid pet_guid;
///     u32 talent_reset_cost;
/// }
/// ```
pub struct SMSG_PET_UNLEARN_CONFIRM {
    pub pet_guid: Guid,
    pub talent_reset_cost: u32,
}

impl ServerMessage for SMSG_PET_UNLEARN_CONFIRM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        // talent_reset_cost: u32
        w.write_all(&self.talent_reset_cost.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02f1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        // talent_reset_cost: u32
        let talent_reset_cost = crate::util::read_u32_le(r)?;

        Ok(Self {
            pet_guid,
            talent_reset_cost,
        })
    }

}

