use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::SpellCastResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_cast_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_cast_failed.wowm#L3):
/// ```text
/// smsg SMSG_PET_CAST_FAILED = 0x0138 {
///     u32 id;
///     u8 unknown1;
///     SpellCastResult result;
/// }
/// ```
pub struct SMSG_PET_CAST_FAILED {
    pub id: u32,
    pub unknown1: u8,
    pub result: SpellCastResult,
}

impl ServerMessage for SMSG_PET_CAST_FAILED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0138;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        6
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // result: SpellCastResult
        let result: SpellCastResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            id,
            unknown1,
            result,
        })
    }

}

