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
pub struct SMSG_DUEL_COMPLETE {
    pub ended_without_interruption: u8,
}

impl ServerMessage for SMSG_DUEL_COMPLETE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // ended_without_interruption: u8
        w.write_all(&self.ended_without_interruption.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x016a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // ended_without_interruption: u8
        let ended_without_interruption = crate::util::read_u8_le(r)?;

        Ok(Self {
            ended_without_interruption,
        })
    }

}

