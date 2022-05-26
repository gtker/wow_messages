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
pub struct CMSG_ZONEUPDATE {
    pub zone_id: u32,
}

impl CMSG_ZONEUPDATE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_ZONEUPDATE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01f4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            zone_id,
        })
    }

}

