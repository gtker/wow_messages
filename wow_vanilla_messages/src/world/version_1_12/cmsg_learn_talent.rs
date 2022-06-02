use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm#L3):
/// ```text
/// cmsg CMSG_LEARN_TALENT = 0x0251 {
///     u32 talent_id;
///     u32 requested_rank;
/// }
/// ```
pub struct CMSG_LEARN_TALENT {
    pub talent_id: u32,
    pub requested_rank: u32,
}

impl ClientMessage for CMSG_LEARN_TALENT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // talent_id: u32
        w.write_all(&self.talent_id.to_le_bytes())?;

        // requested_rank: u32
        w.write_all(&self.requested_rank.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0251;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // talent_id: u32
        let talent_id = crate::util::read_u32_le(r)?;

        // requested_rank: u32
        let requested_rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            talent_id,
            requested_rank,
        })
    }

}

