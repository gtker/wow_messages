use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::ListInventoryItem;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_LIST_INVENTORY {
    pub vendor: Guid,
    pub items: Vec<ListInventoryItem>,
}

impl ServerMessageWrite for SMSG_LIST_INVENTORY {}

impl MessageBody for SMSG_LIST_INVENTORY {
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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor: Guid
        self.vendor.write(w)?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: ListInventoryItem[amount_of_items]
        for i in self.items.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_LIST_INVENTORY {
    fn size(&self) -> usize {
        8 // vendor: Guid
        + 1 // amount_of_items: u8
        + self.items.iter().fold(0, |acc, x| acc + ListInventoryItem::size()) // items: ListInventoryItem[amount_of_items]
    }
}

impl MaximumPossibleSized for SMSG_LIST_INVENTORY {
    fn maximum_possible_size() -> usize {
        8 // vendor: Guid
        + 1 // amount_of_items: u8
        + 255 * ListInventoryItem::maximum_possible_size() // items: ListInventoryItem[amount_of_items]
    }
}

