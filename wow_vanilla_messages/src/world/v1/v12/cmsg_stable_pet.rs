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
pub struct CMSG_STABLE_PET {
    pub npc_guid: Guid,
}

impl CMSG_STABLE_PET {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc_guid: Guid
        w.write_all(&self.npc_guid.guid().to_le_bytes())?;

        Ok(())
    }
}

impl ClientMessage for CMSG_STABLE_PET {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc_guid: Guid
        w.write_all(&self.npc_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0270;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::read(r)?;

        Ok(Self {
            npc_guid,
        })
    }

}

