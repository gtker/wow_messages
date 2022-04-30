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
pub struct SMSG_LOOT_MONEY_NOTIFY {
    pub amount: u32,
}

impl ServerMessageWrite for SMSG_LOOT_MONEY_NOTIFY {
    const OPCODE: u16 = 0x163;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_LOOT_MONEY_NOTIFY {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount: u32
        let amount = crate::util::read_u32_le(r)?;

        Ok(Self {
            amount,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_LOOT_MONEY_NOTIFY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LOOT_MONEY_NOTIFY {
    fn maximum_possible_size() -> usize {
        4 // amount: u32
    }
}

