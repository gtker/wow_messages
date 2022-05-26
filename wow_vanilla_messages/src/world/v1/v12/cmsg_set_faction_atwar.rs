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
pub struct CMSG_SET_FACTION_ATWAR {
    pub reputation_list_id: u32,
    pub flags: u8,
}

impl CMSG_SET_FACTION_ATWAR {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 5], std::io::Error> {
        let mut array_w = [0u8; 5];
        let mut w = array_w.as_mut_slice();
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_SET_FACTION_ATWAR {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0125;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(r)?;

        Ok(Self {
            reputation_list_id,
            flags,
        })
    }

}

