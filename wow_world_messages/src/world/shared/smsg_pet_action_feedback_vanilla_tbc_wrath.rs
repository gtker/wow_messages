use std::io::{Read, Write};

use wow_world_base::shared::pet_feedback_vanilla_tbc_wrath::PetFeedback;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_action_feedback.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_action_feedback.wowm#L10):
/// ```text
/// smsg SMSG_PET_ACTION_FEEDBACK = 0x02C6 {
///     PetFeedback feedback;
/// }
/// ```
pub struct SMSG_PET_ACTION_FEEDBACK {
    pub feedback: PetFeedback,
}

impl crate::private::Sealed for SMSG_PET_ACTION_FEEDBACK {}
impl crate::Message for SMSG_PET_ACTION_FEEDBACK {
    const OPCODE: u32 = 0x02c6;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // feedback: PetFeedback
        w.write_all(&(self.feedback.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C6, size: body_size });
        }

        // feedback: PetFeedback
        let feedback: PetFeedback = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            feedback,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_ACTION_FEEDBACK {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_ACTION_FEEDBACK {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_ACTION_FEEDBACK {}

