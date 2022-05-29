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
pub struct CMSG_SELL_ITEM {
    pub vendor_guid: Guid,
    pub item_guid: Guid,
    pub amount: u8,
}

impl ClientMessage for CMSG_SELL_ITEM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // vendor_guid: Guid
        w.write_all(&self.vendor_guid.guid().to_le_bytes())?;

        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01a0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        17
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // vendor_guid: Guid
        let vendor_guid = Guid::read(r)?;

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_guid,
            amount,
        })
    }

}

