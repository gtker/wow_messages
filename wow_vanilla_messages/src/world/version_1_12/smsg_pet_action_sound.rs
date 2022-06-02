use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::PetTalkReason;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_action_sound.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_action_sound.wowm#L8):
/// ```text
/// smsg SMSG_PET_ACTION_SOUND = 0x0324 {
///     Guid guid;
///     PetTalkReason reason;
/// }
/// ```
pub struct SMSG_PET_ACTION_SOUND {
    pub guid: Guid,
    pub reason: PetTalkReason,
}

impl ServerMessage for SMSG_PET_ACTION_SOUND {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // reason: PetTalkReason
        w.write_all(&(self.reason.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0324;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // reason: PetTalkReason
        let reason: PetTalkReason = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            guid,
            reason,
        })
    }

}

