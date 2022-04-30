use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone)]
#[derive(Copy)]
pub struct SMSG_ACTION_BUTTONS {
    pub data: [u32; 120],
}

impl ServerMessageWrite for SMSG_ACTION_BUTTONS {}

impl MessageBody for SMSG_ACTION_BUTTONS {
    const OPCODE: u16 = 0x0129;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // data: u32[120]
        let mut data = [u32::default(); 120];
        for i in 0..120 {
            data[i] = crate::util::read_u32_le(r)?;
        }

        Ok(Self {
            data,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // data: u32[120]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl ConstantSized for SMSG_ACTION_BUTTONS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_ACTION_BUTTONS {
    fn maximum_possible_size() -> usize {
        120 * core::mem::size_of::<u32>() // data: u32[120]
    }
}

