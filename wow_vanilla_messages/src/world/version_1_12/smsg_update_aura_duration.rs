use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_update_aura_duration.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_update_aura_duration.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_AURA_DURATION = 0x0137 {
///     u8 aura_slot;
///     u32 aura_duration;
/// }
/// ```
pub struct SMSG_UPDATE_AURA_DURATION {
    pub aura_slot: u8,
    pub aura_duration: u32,
}

impl ServerMessage for SMSG_UPDATE_AURA_DURATION {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // aura_slot: u8
        w.write_all(&self.aura_slot.to_le_bytes())?;

        // aura_duration: u32
        w.write_all(&self.aura_duration.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0137;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // aura_slot: u8
        let aura_slot = crate::util::read_u8_le(r)?;

        // aura_duration: u32
        let aura_duration = crate::util::read_u32_le(r)?;

        Ok(Self {
            aura_slot,
            aura_duration,
        })
    }

}

