use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::ListInventoryItem;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_LIST_INVENTORY {
    pub vendor: Guid,
    pub items: Vec<ListInventoryItem>,
}

impl ServerMessage for SMSG_LIST_INVENTORY {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: ListInventoryItem[amount_of_items]
        for i in self.items.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x019f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // vendor: Guid
        let vendor = Guid::read(r)?;

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(r)?;

        // items: ListInventoryItem[amount_of_items]
        let mut items = Vec::with_capacity(amount_of_items as usize);
        for i in 0..amount_of_items {
            items.push(ListInventoryItem::read(r)?);
        }

        Ok(Self {
            vendor,
            items,
        })
    }

}

impl SMSG_LIST_INVENTORY {
    pub(crate) fn size(&self) -> usize {
        0
        + 8 // vendor: Guid
        + 1 // amount_of_items: u8
        + self.items.len() * 28 // items: ListInventoryItem[amount_of_items]
    }
}

