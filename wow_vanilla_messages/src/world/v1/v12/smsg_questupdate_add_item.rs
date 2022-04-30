use std::convert::{TryFrom, TryInto};
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
#[derive(Copy)]
pub struct SMSG_QUESTUPDATE_ADD_ITEM {
    pub required_item_id: u32,
    pub items_required: u32,
}

impl ServerMessageWrite for SMSG_QUESTUPDATE_ADD_ITEM {}

impl MessageBody for SMSG_QUESTUPDATE_ADD_ITEM {
    const OPCODE: u16 = 0x019a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // required_item_id: u32
        let required_item_id = crate::util::read_u32_le(r)?;

        // items_required: u32
        let items_required = crate::util::read_u32_le(r)?;

        Ok(Self {
            required_item_id,
            items_required,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // required_item_id: u32
        w.write_all(&self.required_item_id.to_le_bytes())?;

        // items_required: u32
        w.write_all(&self.items_required.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_QUESTUPDATE_ADD_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_QUESTUPDATE_ADD_ITEM {
    fn maximum_possible_size() -> usize {
        4 // required_item_id: u32
        + 4 // items_required: u32
    }
}

