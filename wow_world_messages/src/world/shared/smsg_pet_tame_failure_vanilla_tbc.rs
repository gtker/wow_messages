use std::io::{Read, Write};
use wow_world_base::shared::pet_tame_failure_reason_vanilla_tbc::PetTameFailureReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // reason: PetTameFailureReason
        w.write_all(&u8::from(self.reason.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0173, size: body_size as u32 });
        }

        // reason: PetTameFailureReason
        let reason: PetTameFailureReason = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            reason,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_TAME_FAILURE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_TAME_FAILURE {}

