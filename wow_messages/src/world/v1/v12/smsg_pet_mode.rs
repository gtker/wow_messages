use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PetCommandState, PetCommandStateError};
use crate::world::v1::v12::{PetReactState, PetReactStateError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_pet_mode.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_pet_mode.wowm#L3):
/// ```text
/// smsg SMSG_PET_MODE = 0x17A {
///     u64 guid;
///     PetReactState react_state;
///     PetCommandState command_state;
///     u8 unknown1;
///     u8 pet_enabled;
/// }
/// ```
pub struct SMSG_PET_MODE {
    pub guid: u64,
    pub react_state: PetReactState,
    pub command_state: PetCommandState,
    pub unknown1: u8,
    pub pet_enabled: u8,
}

impl WorldServerMessageWrite for SMSG_PET_MODE {
    const OPCODE: u16 = 0x17a;

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
impl WorldMessageBody for SMSG_PET_MODE {
    type Error = SMSG_PET_MODEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // react_state: PetReactState
        let react_state = PetReactState::read(r)?;

        // command_state: PetCommandState
        let command_state = PetCommandState::read(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // pet_enabled: u8
        let pet_enabled = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            react_state,
            command_state,
            unknown1,
            pet_enabled,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // react_state: PetReactState
        self.react_state.write(w)?;

        // command_state: PetCommandState
        self.command_state.write(w)?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // pet_enabled: u8
        w.write_all(&self.pet_enabled.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PET_MODE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PET_MODE {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + PetReactState::size() // react_state: PetReactState
        + PetCommandState::size() // command_state: PetCommandState
        + 1 // unknown1: u8
        + 1 // pet_enabled: u8
    }
}

#[derive(Debug)]
pub enum SMSG_PET_MODEError {
    Io(std::io::Error),
    PetCommandState(PetCommandStateError),
    PetReactState(PetReactStateError),
}

impl std::error::Error for SMSG_PET_MODEError {}
impl std::fmt::Display for SMSG_PET_MODEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetCommandState(i) => i.fmt(f),
            Self::PetReactState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_MODEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetCommandStateError> for SMSG_PET_MODEError {
    fn from(e: PetCommandStateError) -> Self {
        Self::PetCommandState(e)
    }
}

impl From<PetReactStateError> for SMSG_PET_MODEError {
    fn from(e: PetReactStateError) -> Self {
        Self::PetReactState(e)
    }
}

