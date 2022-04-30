use crate::MessageBody;
use crate::OpcodeMessage;
use crate::{WorldServerMessageWrite, WorldClientMessageWrite};
use wow_srp::header_crypto::{Decrypter, Encrypter};

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::v1::v2::CMSG_CHAR_ENUM;

#[derive(Debug)]
pub enum WorldClientOpcode {
    CMSG_CHAR_ENUM,
}

impl WorldClientOpcode {
    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::CMSG_CHAR_ENUM => 0x37,
        }
    }

}

impl WorldClientOpcode {
    pub fn new(opcode: u32) -> std::result::Result<Self, WorldClientOpcodeError> {
        match opcode {
            0x37 => Ok(Self::CMSG_CHAR_ENUM),
            opcode => Err(WorldClientOpcodeError::InvalidOpcode(opcode)),
        }
    }

}

impl From<&WorldClientOpcodeMessage> for WorldClientOpcode {
    fn from(e: &WorldClientOpcodeMessage) -> Self {
        match *e {
            WorldClientOpcodeMessage::CMSG_CHAR_ENUM(_) => Self::CMSG_CHAR_ENUM,
        }
    }
}

#[derive(Debug)]
pub enum WorldClientOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u32),
}

impl std::error::Error for WorldClientOpcodeError {
}

impl std::fmt::Display for WorldClientOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for WorldClient: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for WorldClientOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub enum WorldClientOpcodeMessage {
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
}

impl OpcodeMessage for WorldClientOpcodeMessage {
    type Error = WorldClientOpcodeMessageError;
    fn write_unencrypted<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        match self {
            Self::CMSG_CHAR_ENUM(i) => i.write_body(w)?,
        }
        Ok(())
    }

    fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let size = crate::util::read_u16_be(r)?;
        let opcode = crate::util::read_u32_le(r)?;
        match opcode {
            0x37 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::read_body(r, (size - 4) as u32)?)),
            _ => Err(Self::Error::InvalidOpcode(opcode)),
        }
    }

    fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, Self::Error> {
        let header = d.read_and_decrypt_client_header(r)?;
        match header.opcode {
            0x37 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::read_body(r, (header.size - 4) as u32)?)),
            _ => Err(Self::Error::InvalidOpcode(header.opcode)),
        }
    }

    fn write_encrypted<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        match self {
            Self::CMSG_CHAR_ENUM(i) => i.write_encrypted_client(w, e)?,
        }
        Ok(())
    }

}

#[derive(Debug)]
pub enum WorldClientOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u32),
}

impl std::error::Error for WorldClientOpcodeMessageError {}
impl std::fmt::Display for WorldClientOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for WorldClientMessage: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for WorldClientOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<WorldClientOpcodeError> for WorldClientOpcodeMessageError {
    fn from(e: WorldClientOpcodeError) -> Self {
        match e {
            WorldClientOpcodeError::Io(i) => Self::Io(i),
            WorldClientOpcodeError::InvalidOpcode(i) => Self::InvalidOpcode(i),
        }
    }
}

use crate::world::v1::v2::SMSG_AUTH_CHALLENGE;
use crate::world::v1::v2::{SMSG_AUTH_RESPONSE, SMSG_AUTH_RESPONSEError};

#[derive(Debug)]
pub enum WorldServerOpcode {
    SMSG_AUTH_CHALLENGE,
    SMSG_AUTH_RESPONSE,
}

impl WorldServerOpcode {
    pub const fn as_u16(&self) -> u16 {
        match self {
            Self::SMSG_AUTH_CHALLENGE => 0x1ec,
            Self::SMSG_AUTH_RESPONSE => 0x1ee,
        }
    }

}

impl WorldServerOpcode {
    pub fn new(opcode: u16) -> std::result::Result<Self, WorldServerOpcodeError> {
        match opcode {
            0x1ec => Ok(Self::SMSG_AUTH_CHALLENGE),
            0x1ee => Ok(Self::SMSG_AUTH_RESPONSE),
            opcode => Err(WorldServerOpcodeError::InvalidOpcode(opcode)),
        }
    }

}

impl From<&WorldServerOpcodeMessage> for WorldServerOpcode {
    fn from(e: &WorldServerOpcodeMessage) -> Self {
        match *e {
            WorldServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => Self::SMSG_AUTH_CHALLENGE,
            WorldServerOpcodeMessage::SMSG_AUTH_RESPONSE(_) => Self::SMSG_AUTH_RESPONSE,
        }
    }
}

#[derive(Debug)]
pub enum WorldServerOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u16),
}

impl std::error::Error for WorldServerOpcodeError {
}

impl std::fmt::Display for WorldServerOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for WorldServer: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for WorldServerOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub enum WorldServerOpcodeMessage {
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
}

impl OpcodeMessage for WorldServerOpcodeMessage {
    type Error = WorldServerOpcodeMessageError;
    fn write_unencrypted<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        match self {
            Self::SMSG_AUTH_CHALLENGE(i) => i.write_body(w)?,
            Self::SMSG_AUTH_RESPONSE(i) => i.write_body(w)?,
        }
        Ok(())
    }

    fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let size = crate::util::read_u16_be(r)?;
        let opcode = crate::util::read_u16_le(r)?;
        match opcode {
            0x1ec => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::read_body(r, (size - 2) as u32)?)),
            0x1ee => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::read_body(r, (size - 2) as u32)?)),
            _ => Err(Self::Error::InvalidOpcode(opcode)),
        }
    }

    fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, Self::Error> {
        let header = d.read_and_decrypt_server_header(r)?;
        match header.opcode {
            0x1ec => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::read_body(r, (header.size - 2) as u32)?)),
            0x1ee => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::read_body(r, (header.size - 2) as u32)?)),
            _ => Err(Self::Error::InvalidOpcode(header.opcode)),
        }
    }

    fn write_encrypted<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        match self {
            Self::SMSG_AUTH_CHALLENGE(i) => i.write_encrypted_server(w, e)?,
            Self::SMSG_AUTH_RESPONSE(i) => i.write_encrypted_server(w, e)?,
        }
        Ok(())
    }

}

#[derive(Debug)]
pub enum WorldServerOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u16),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSEError),
}

impl std::error::Error for WorldServerOpcodeMessageError {}
impl std::fmt::Display for WorldServerOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for WorldServerMessage: '{}'", i)),
            Self::SMSG_AUTH_RESPONSE(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for WorldServerOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<WorldServerOpcodeError> for WorldServerOpcodeMessageError {
    fn from(e: WorldServerOpcodeError) -> Self {
        match e {
            WorldServerOpcodeError::Io(i) => Self::Io(i),
            WorldServerOpcodeError::InvalidOpcode(i) => Self::InvalidOpcode(i),
        }
    }
}

impl From<SMSG_AUTH_RESPONSEError> for WorldServerOpcodeMessageError {
    fn from(e: SMSG_AUTH_RESPONSEError) -> Self {
        match e {
            SMSG_AUTH_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_AUTH_RESPONSE(e),
        }
    }
}

