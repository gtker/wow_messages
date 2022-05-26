use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_ATTACKSTART {
    pub attacker_guid: Guid,
    pub victim_guid: Guid,
}

impl SMSG_ATTACKSTART {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 16], std::io::Error> {
        let mut array_w = [0u8; 16];
        let mut w = array_w.as_mut_slice();
        // attacker_guid: Guid
        w.write_all(&self.attacker_guid.guid().to_le_bytes())?;

        // victim_guid: Guid
        w.write_all(&self.victim_guid.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_ATTACKSTART {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // attacker_guid: Guid
        w.write_all(&self.attacker_guid.guid().to_le_bytes())?;

        // victim_guid: Guid
        w.write_all(&self.victim_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0143;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // attacker_guid: Guid
        let attacker_guid = Guid::read(r)?;

        // victim_guid: Guid
        let victim_guid = Guid::read(r)?;

        Ok(Self {
            attacker_guid,
            victim_guid,
        })
    }

}

