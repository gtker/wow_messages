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
pub struct CMSG_AUTOEQUIP_ITEM_SLOT {
    pub guid: Guid,
    pub destination_slot: u8,
}

impl CMSG_AUTOEQUIP_ITEM_SLOT {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 9], std::io::Error> {
        let mut array_w = [0u8; 9];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_AUTOEQUIP_ITEM_SLOT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x010f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // destination_slot: u8
        let destination_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            destination_slot,
        })
    }

}

