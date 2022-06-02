use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::PetCommandState;
use crate::world::version_1_12::PetReactState;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_mode.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_mode.wowm#L3):
/// ```text
/// smsg SMSG_PET_MODE = 0x017A {
///     Guid guid;
///     PetReactState react_state;
///     PetCommandState command_state;
///     u8 unknown1;
///     u8 pet_enabled;
/// }
/// ```
pub struct SMSG_PET_MODE {
    pub guid: Guid,
    pub react_state: PetReactState,
    pub command_state: PetCommandState,
    pub unknown1: u8,
    pub pet_enabled: u8,
}

impl ServerMessage for SMSG_PET_MODE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // react_state: PetReactState
        w.write_all(&(self.react_state.as_int() as u8).to_le_bytes())?;

        // command_state: PetCommandState
        w.write_all(&(self.command_state.as_int() as u8).to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // pet_enabled: u8
        w.write_all(&self.pet_enabled.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x017a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // react_state: PetReactState
        let react_state: PetReactState = crate::util::read_u8_le(r)?.try_into()?;

        // command_state: PetCommandState
        let command_state: PetCommandState = crate::util::read_u8_le(r)?.try_into()?;

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

}

