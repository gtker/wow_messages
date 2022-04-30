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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_AREA_TRIGGER_MESSAGE {
    pub length: u32,
    pub message: String,
}

impl ServerMessageWrite for SMSG_AREA_TRIGGER_MESSAGE {}

impl MessageBody for SMSG_AREA_TRIGGER_MESSAGE {
    const OPCODE: u16 = 0x02b8;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_AREA_TRIGGER_MESSAGEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // length: u32
        let length = crate::util::read_u32_le(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            length,
            message,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // length: u32
        w.write_all(&self.length.to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_AREA_TRIGGER_MESSAGE {
    fn size(&self) -> usize {
        4 // length: u32
        + self.message.len() + 1 // message: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_AREA_TRIGGER_MESSAGE {
    fn maximum_possible_size() -> usize {
        4 // length: u32
        + 256 // message: CString
    }
}

#[derive(Debug)]
pub enum SMSG_AREA_TRIGGER_MESSAGEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_AREA_TRIGGER_MESSAGEError {}
impl std::fmt::Display for SMSG_AREA_TRIGGER_MESSAGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_AREA_TRIGGER_MESSAGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_AREA_TRIGGER_MESSAGEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

