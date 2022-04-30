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
pub struct CMSG_GROUP_CHANGE_SUB_GROUP {
    pub name: String,
    pub group_number: u8,
}

impl ClientMessageWrite for CMSG_GROUP_CHANGE_SUB_GROUP {
    const OPCODE: u32 = 0x27e;

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
impl MessageBody for CMSG_GROUP_CHANGE_SUB_GROUP {
    type Error = CMSG_GROUP_CHANGE_SUB_GROUPError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // group_number: u8
        let group_number = crate::util::read_u8_le(r)?;

        Ok(Self {
            name,
            group_number,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // group_number: u8
        w.write_all(&self.group_number.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for CMSG_GROUP_CHANGE_SUB_GROUP {
    fn size(&self) -> usize {
        self.name.len() + 1 // name: CString and Null Terminator
        + 1 // group_number: u8
    }
}

impl MaximumPossibleSized for CMSG_GROUP_CHANGE_SUB_GROUP {
    fn maximum_possible_size() -> usize {
        256 // name: CString
        + 1 // group_number: u8
    }
}

#[derive(Debug)]
pub enum CMSG_GROUP_CHANGE_SUB_GROUPError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GROUP_CHANGE_SUB_GROUPError {}
impl std::fmt::Display for CMSG_GROUP_CHANGE_SUB_GROUPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GROUP_CHANGE_SUB_GROUPError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GROUP_CHANGE_SUB_GROUPError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

