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
pub struct SMSG_NOTIFICATION {
    pub notification: String,
}

impl ServerMessageWrite for SMSG_NOTIFICATION {}

impl MessageBody for SMSG_NOTIFICATION {
    const OPCODE: u16 = 0x01cb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_NOTIFICATIONError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // notification: CString
        let notification = crate::util::read_c_string_to_vec(r)?;
        let notification = String::from_utf8(notification)?;

        Ok(Self {
            notification,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // notification: CString
        w.write_all(self.notification.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_NOTIFICATION {
    fn size(&self) -> usize {
        self.notification.len() + 1 // notification: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_NOTIFICATION {
    fn maximum_possible_size() -> usize {
        256 // notification: CString
    }
}

#[derive(Debug)]
pub enum SMSG_NOTIFICATIONError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_NOTIFICATIONError {}
impl std::fmt::Display for SMSG_NOTIFICATIONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_NOTIFICATIONError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_NOTIFICATIONError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

