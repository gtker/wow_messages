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
pub struct CMSG_PETITION_QUERY {
    pub guild_guid: u32,
    pub petition_guid: Guid,
}

impl CMSG_PETITION_QUERY {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guild_guid: u32
        w.write_all(&self.guild_guid.to_le_bytes())?;

        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        Ok(())
    }
}

impl ClientMessage for CMSG_PETITION_QUERY {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guild_guid: u32
        w.write_all(&self.guild_guid.to_le_bytes())?;

        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01c6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guild_guid: u32
        let guild_guid = crate::util::read_u32_le(r)?;

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        Ok(Self {
            guild_guid,
            petition_guid,
        })
    }

}

