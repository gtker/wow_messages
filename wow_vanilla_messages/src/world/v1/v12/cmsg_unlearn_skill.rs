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
pub struct CMSG_UNLEARN_SKILL {
    pub skill_id: u32,
}

impl ClientMessage for CMSG_UNLEARN_SKILL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // skill_id: u32
        w.write_all(&self.skill_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0202;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // skill_id: u32
        let skill_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            skill_id,
        })
    }

}

