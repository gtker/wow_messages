use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::PetTameFailureReason;
use crate::world::vanilla::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm#L24):
/// ```text
/// smsg SMSG_PET_TAME_FAILURE = 0x0173 {
///     PetTameFailureReason reason;
/// }
/// ```
pub struct SMSG_PET_TAME_FAILURE {
    pub reason: PetTameFailureReason,
}

impl crate::Message for SMSG_PET_TAME_FAILURE {
    const OPCODE: u32 = 0x0173;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reason: PetTameFailureReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // reason: PetTameFailureReason
        let reason: PetTameFailureReason = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            reason,
        })
    }

}
impl ServerMessage for SMSG_PET_TAME_FAILURE {}

