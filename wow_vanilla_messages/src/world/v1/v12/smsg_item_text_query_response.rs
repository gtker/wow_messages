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
pub struct SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub item_text_id: u32,
    pub text: String,
}

impl ServerMessageWrite for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    const OPCODE: u16 = 0x244;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    type Error = SMSG_ITEM_TEXT_QUERY_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(r)?;

        // text: CString
        let text = crate::util::read_c_string_to_vec(r)?;
        let text = String::from_utf8(text)?;

        Ok(Self {
            item_text_id,
            text,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // text: CString
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    fn size(&self) -> usize {
        4 // item_text_id: u32
        + self.text.len() + 1 // text: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        4 // item_text_id: u32
        + 256 // text: CString
    }
}

#[derive(Debug)]
pub enum SMSG_ITEM_TEXT_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_ITEM_TEXT_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_ITEM_TEXT_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ITEM_TEXT_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_ITEM_TEXT_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

