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
pub struct CMSG_ADD_IGNORE {
    pub ignore_name: String,
}

impl ClientMessageWrite for CMSG_ADD_IGNORE {
    const OPCODE: u32 = 0x6c;

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
impl MessageBody for CMSG_ADD_IGNORE {
    type Error = CMSG_ADD_IGNOREError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // ignore_name: CString
        let ignore_name = crate::util::read_c_string_to_vec(r)?;
        let ignore_name = String::from_utf8(ignore_name)?;

        Ok(Self {
            ignore_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // ignore_name: CString
        w.write_all(self.ignore_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_ADD_IGNORE {
    fn size(&self) -> usize {
        self.ignore_name.len() + 1 // ignore_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_ADD_IGNORE {
    fn maximum_possible_size() -> usize {
        256 // ignore_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_ADD_IGNOREError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_ADD_IGNOREError {}
impl std::fmt::Display for CMSG_ADD_IGNOREError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_ADD_IGNOREError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_ADD_IGNOREError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

