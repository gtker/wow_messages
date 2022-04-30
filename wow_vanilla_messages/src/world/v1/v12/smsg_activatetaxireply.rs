use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ActivateTaxiReply, ActivateTaxiReplyError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_ACTIVATETAXIREPLY {
    pub reply: ActivateTaxiReply,
}

impl WorldServerMessageWrite for SMSG_ACTIVATETAXIREPLY {
    const OPCODE: u16 = 0x1ae;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_ACTIVATETAXIREPLY {
    type Error = SMSG_ACTIVATETAXIREPLYError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reply: ActivateTaxiReply
        let reply = ActivateTaxiReply::read(r)?;

        Ok(Self {
            reply,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reply: ActivateTaxiReply
        self.reply.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_ACTIVATETAXIREPLY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_ACTIVATETAXIREPLY {
    fn maximum_possible_size() -> usize {
        ActivateTaxiReply::size() // reply: ActivateTaxiReply
    }
}

#[derive(Debug)]
pub enum SMSG_ACTIVATETAXIREPLYError {
    Io(std::io::Error),
    ActivateTaxiReply(ActivateTaxiReplyError),
}

impl std::error::Error for SMSG_ACTIVATETAXIREPLYError {}
impl std::fmt::Display for SMSG_ACTIVATETAXIREPLYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::ActivateTaxiReply(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ACTIVATETAXIREPLYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ActivateTaxiReplyError> for SMSG_ACTIVATETAXIREPLYError {
    fn from(e: ActivateTaxiReplyError) -> Self {
        Self::ActivateTaxiReply(e)
    }
}

