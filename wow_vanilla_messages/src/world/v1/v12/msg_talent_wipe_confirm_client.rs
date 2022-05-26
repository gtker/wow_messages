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

impl MSG_TALENT_WIPE_CONFIRM_Client {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // wiping_npc: Guid
        w.write_all(&self.wiping_npc.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for MSG_TALENT_WIPE_CONFIRM_Client {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // wiping_npc: Guid
        w.write_all(&self.wiping_npc.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02aa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // wiping_npc: Guid
        let wiping_npc = Guid::read(r)?;

        Ok(Self {
            wiping_npc,
        })
    }

}

