use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::PetFeedback;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_action_feedback.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_action_feedback.wowm#L10):
/// ```text
/// smsg SMSG_PET_ACTION_FEEDBACK = 0x02C6 {
///     PetFeedback feedback;
/// }
/// ```
pub struct SMSG_PET_ACTION_FEEDBACK {
    pub feedback: PetFeedback,
}

impl crate::Message for SMSG_PET_ACTION_FEEDBACK {
    const OPCODE: u32 = 0x02c6;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // feedback: PetFeedback
        w.write_all(&(self.feedback.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // feedback: PetFeedback
        let feedback: PetFeedback = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            feedback,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_PET_ACTION_FEEDBACK {}

