use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::PetFeedback;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_action_feedback.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_action_feedback.wowm#L10):
/// ```text
/// smsg SMSG_PET_ACTION_FEEDBACK = 0x02C6 {
///     PetFeedback feedback;
/// }
/// ```
pub struct SMSG_PET_ACTION_FEEDBACK {
    pub feedback: PetFeedback,
}

impl ServerMessage for SMSG_PET_ACTION_FEEDBACK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // feedback: PetFeedback
        w.write_all(&(self.feedback.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02c6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // feedback: PetFeedback
        let feedback: PetFeedback = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            feedback,
        })
    }

}

