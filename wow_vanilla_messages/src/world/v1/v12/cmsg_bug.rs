use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_BUG {
    pub suggestion: u32,
    pub content_length: u32,
    pub content: String,
    pub type_length: u32,
    pub bug_type: String,
}

impl ClientMessageWrite for CMSG_BUG {
    const OPCODE: u32 = 0x1ca;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as ClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as ClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for CMSG_BUG {
    type Error = CMSG_BUGError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // suggestion: u32
        let suggestion = crate::util::read_u32_le(r)?;

        // content_length: u32
        let content_length = crate::util::read_u32_le(r)?;

        // content: CString
        let content = crate::util::read_c_string_to_vec(r)?;
        let content = String::from_utf8(content)?;

        // type_length: u32
        let type_length = crate::util::read_u32_le(r)?;

        // bug_type: CString
        let bug_type = crate::util::read_c_string_to_vec(r)?;
        let bug_type = String::from_utf8(bug_type)?;

        Ok(Self {
            suggestion,
            content_length,
            content,
            type_length,
            bug_type,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // suggestion: u32
        w.write_all(&self.suggestion.to_le_bytes())?;

        // content_length: u32
        w.write_all(&self.content_length.to_le_bytes())?;

        // content: CString
        w.write_all(self.content.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // type_length: u32
        w.write_all(&self.type_length.to_le_bytes())?;

        // bug_type: CString
        w.write_all(self.bug_type.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_BUG {
    fn size(&self) -> usize {
        4 // suggestion: u32
        + 4 // content_length: u32
        + self.content.len() + 1 // content: CString and Null Terminator
        + 4 // type_length: u32
        + self.bug_type.len() + 1 // bug_type: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_BUG {
    fn maximum_possible_size() -> usize {
        4 // suggestion: u32
        + 4 // content_length: u32
        + 256 // content: CString
        + 4 // type_length: u32
        + 256 // bug_type: CString
    }
}

#[derive(Debug)]
pub enum CMSG_BUGError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_BUGError {}
impl std::fmt::Display for CMSG_BUGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_BUGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_BUGError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

