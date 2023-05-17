use std::io::{Read, Write};

use crate::wrath::PetTameFailureReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm:54`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm#L54):
/// ```text
/// smsg SMSG_PET_TAME_FAILURE = 0x0173 {
///     PetTameFailureReason reason;
/// }
/// ```
pub struct SMSG_PET_TAME_FAILURE {
    pub reason: PetTameFailureReason,
}

impl crate::private::Sealed for SMSG_PET_TAME_FAILURE {}
impl crate::Message for SMSG_PET_TAME_FAILURE {
    const OPCODE: u32 = 0x0173;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // reason: PetTameFailureReason
        w.write_all(&(self.reason.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0173, size: body_size });
        }

        // reason: PetTameFailureReason
        let reason: PetTameFailureReason = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            reason,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_TAME_FAILURE {}

