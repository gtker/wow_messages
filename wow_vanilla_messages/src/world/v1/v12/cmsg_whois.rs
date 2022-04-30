use std::convert::{TryFrom, TryInto};
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
pub struct CMSG_WHOIS {
    pub character: String,
}

impl ClientMessageWrite for CMSG_WHOIS {}

impl MessageBody for CMSG_WHOIS {
    const OPCODE: u16 = 0x0064;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_WHOISError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // character: CString
        let character = crate::util::read_c_string_to_vec(r)?;
        let character = String::from_utf8(character)?;

        Ok(Self {
            character,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // character: CString
        w.write_all(self.character.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_WHOIS {
    fn size(&self) -> usize {
        self.character.len() + 1 // character: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_WHOIS {
    fn maximum_possible_size() -> usize {
        256 // character: CString
    }
}

#[derive(Debug)]
pub enum CMSG_WHOISError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_WHOISError {}
impl std::fmt::Display for CMSG_WHOISError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_WHOISError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_WHOISError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

