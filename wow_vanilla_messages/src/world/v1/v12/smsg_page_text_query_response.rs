use std::convert::{TryFrom, TryInto};
use crate::{WorldServerMessageWrite, WorldMessageBody};
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
pub struct SMSG_PAGE_TEXT_QUERY_RESPONSE {
    pub page_id: u32,
    pub text: String,
    pub next_page_id: u32,
}

impl WorldServerMessageWrite for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    const OPCODE: u16 = 0x5b;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    type Error = SMSG_PAGE_TEXT_QUERY_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // page_id: u32
        let page_id = crate::util::read_u32_le(r)?;

        // text: CString
        let text = crate::util::read_c_string_to_vec(r)?;
        let text = String::from_utf8(text)?;

        // next_page_id: u32
        let next_page_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            page_id,
            text,
            next_page_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // page_id: u32
        w.write_all(&self.page_id.to_le_bytes())?;

        // text: CString
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // next_page_id: u32
        w.write_all(&self.next_page_id.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    fn size(&self) -> usize {
        4 // page_id: u32
        + self.text.len() + 1 // text: CString and Null Terminator
        + 4 // next_page_id: u32
    }
}

impl MaximumPossibleSized for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        4 // page_id: u32
        + 256 // text: CString
        + 4 // next_page_id: u32
    }
}

#[derive(Debug)]
pub enum SMSG_PAGE_TEXT_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_PAGE_TEXT_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_PAGE_TEXT_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PAGE_TEXT_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_PAGE_TEXT_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

