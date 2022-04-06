use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{PetTalkReason, PetTalkReasonError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_pet_action_sound.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_pet_action_sound.wowm#L8):
/// ```text
/// smsg SMSG_PET_ACTION_SOUND = 0x324 {
///     Guid guid;
///     PetTalkReason reason;
/// }
/// ```
pub struct SMSG_PET_ACTION_SOUND {
    pub guid: Guid,
    pub reason: PetTalkReason,
}

impl WorldServerMessageWrite for SMSG_PET_ACTION_SOUND {
    const OPCODE: u16 = 0x324;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_PET_ACTION_SOUND {
    type Error = SMSG_PET_ACTION_SOUNDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // reason: PetTalkReason
        let reason = PetTalkReason::read(r)?;

        Ok(Self {
            guid,
            reason,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // reason: PetTalkReason
        self.reason.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PET_ACTION_SOUND {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PET_ACTION_SOUND {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + PetTalkReason::size() // reason: PetTalkReason
    }
}

#[derive(Debug)]
pub enum SMSG_PET_ACTION_SOUNDError {
    Io(std::io::Error),
    PetTalkReason(PetTalkReasonError),
}

impl std::error::Error for SMSG_PET_ACTION_SOUNDError {}
impl std::fmt::Display for SMSG_PET_ACTION_SOUNDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetTalkReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_ACTION_SOUNDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetTalkReasonError> for SMSG_PET_ACTION_SOUNDError {
    fn from(e: PetTalkReasonError) -> Self {
        Self::PetTalkReason(e)
    }
}

