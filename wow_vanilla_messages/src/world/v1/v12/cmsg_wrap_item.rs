use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_WRAP_ITEM {
    pub gift_bag_index: u8,
    pub gift_slot: u8,
    pub item_bag_index: u8,
    pub item_slot: u8,
}

impl ClientMessageWrite for CMSG_WRAP_ITEM {}

impl MessageBody for CMSG_WRAP_ITEM {
    const OPCODE: u16 = 0x01d3;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // gift_bag_index: u8
        let gift_bag_index = crate::util::read_u8_le(r)?;

        // gift_slot: u8
        let gift_slot = crate::util::read_u8_le(r)?;

        // item_bag_index: u8
        let item_bag_index = crate::util::read_u8_le(r)?;

        // item_slot: u8
        let item_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            gift_bag_index,
            gift_slot,
            item_bag_index,
            item_slot,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // gift_bag_index: u8
        w.write_all(&self.gift_bag_index.to_le_bytes())?;

        // gift_slot: u8
        w.write_all(&self.gift_slot.to_le_bytes())?;

        // item_bag_index: u8
        w.write_all(&self.item_bag_index.to_le_bytes())?;

        // item_slot: u8
        w.write_all(&self.item_slot.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_WRAP_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_WRAP_ITEM {
    fn maximum_possible_size() -> usize {
        1 // gift_bag_index: u8
        + 1 // gift_slot: u8
        + 1 // item_bag_index: u8
        + 1 // item_slot: u8
    }
}

