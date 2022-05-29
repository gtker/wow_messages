use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_TALENT_WIPE_CONFIRM_Client {
    pub wiping_npc: Guid,
}

impl ClientMessage for MSG_TALENT_WIPE_CONFIRM_Client {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // wiping_npc: Guid
        w.write_all(&self.wiping_npc.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02aa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // wiping_npc: Guid
        let wiping_npc = Guid::read(r)?;

        Ok(Self {
            wiping_npc,
        })
    }

}

