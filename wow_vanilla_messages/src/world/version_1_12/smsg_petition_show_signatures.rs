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
pub struct SMSG_PETITION_SHOW_SIGNATURES {
    pub item_guid: Guid,
    pub owner_guid: Guid,
    pub petition_guid: Guid,
    pub amount_of_signatures: u8,
}

impl ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // owner_guid: Guid
        w.write_all(&self.owner_guid.guid().to_le_bytes())?;

        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // amount_of_signatures: u8
        w.write_all(&self.amount_of_signatures.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01bf;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        25
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // owner_guid: Guid
        let owner_guid = Guid::read(r)?;

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // amount_of_signatures: u8
        let amount_of_signatures = crate::util::read_u8_le(r)?;

        Ok(Self {
            item_guid,
            owner_guid,
            petition_guid,
            amount_of_signatures,
        })
    }

}

