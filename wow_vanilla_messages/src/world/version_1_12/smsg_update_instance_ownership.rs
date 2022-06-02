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
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_INSTANCE_OWNERSHIP = 0x032B {
///     u32 has_been_saved;
/// }
/// ```
pub struct SMSG_UPDATE_INSTANCE_OWNERSHIP {
    pub has_been_saved: u32,
}

impl ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // has_been_saved: u32
        w.write_all(&self.has_been_saved.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x032b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // has_been_saved: u32
        let has_been_saved = crate::util::read_u32_le(r)?;

        Ok(Self {
            has_been_saved,
        })
    }

}

