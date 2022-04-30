use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct CMSG_GET_MAIL_LIST {
    pub mailbox_guid: Guid,
}

impl ClientMessageWrite for CMSG_GET_MAIL_LIST {}

impl MessageBody for CMSG_GET_MAIL_LIST {
    const OPCODE: u16 = 0x023a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox_guid: Guid
        let mailbox_guid = Guid::read(r)?;

        Ok(Self {
            mailbox_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox_guid: Guid
        self.mailbox_guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_GET_MAIL_LIST {}

impl MaximumPossibleSized for CMSG_GET_MAIL_LIST {
    fn maximum_possible_size() -> usize {
        8 // mailbox_guid: Guid
    }
}

