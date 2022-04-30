use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, ServerMessageWrite, MessageBody};
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
pub struct MSG_PETITION_DECLINE {
    pub petition: Guid,
}

impl ClientMessageWrite for MSG_PETITION_DECLINE {}

impl ServerMessageWrite for MSG_PETITION_DECLINE {}

impl MessageBody for MSG_PETITION_DECLINE {
    const OPCODE: u16 = 0x01c2;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition: Guid
        let petition = Guid::read(r)?;

        Ok(Self {
            petition,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition: Guid
        self.petition.write(w)?;

        Ok(())
    }
}

impl ConstantSized for MSG_PETITION_DECLINE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MSG_PETITION_DECLINE {
    fn maximum_possible_size() -> usize {
        8 // petition: Guid
    }
}

