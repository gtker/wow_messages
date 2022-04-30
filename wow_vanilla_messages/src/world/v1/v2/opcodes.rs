use crate::MessageBody;
use crate::OpcodeMessage;
use crate::{ServerMessageWrite, ClientMessageWrite};
use wow_srp::header_crypto::{Decrypter, Encrypter};

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::v1::v2::CMSG_CHAR_ENUM;

#[derive(Debug)]
pub enum ClientOpcode {
    CMSG_CHAR_ENUM,
}

impl ClientOpcode {
    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::CMSG_CHAR_ENUM => 0x37,
        }
    }

}

impl ClientOpcode {
    pub fn new(opcode: u32) -> std::result::Result<Self, ClientOpcodeError> {
        match opcode {
            0x37 => Ok(Self::CMSG_CHAR_ENUM),
            opcode => Err(ClientOpcodeError::InvalidOpcode(opcode)),
        }
    }

}

impl From<&ClientOpcodeMessage> for ClientOpcode {
    fn from(e: &ClientOpcodeMessage) -> Self {
        match *e {
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => Self::CMSG_CHAR_ENUM,
        }
    }
}

#[derive(Debug)]
pub enum ClientOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u32),
}

impl std::error::Error for ClientOpcodeError {
}

impl std::fmt::Display for ClientOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for Client: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for ClientOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub enum ClientOpcodeMessage {
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
}

impl OpcodeMessage for ClientOpcodeMessage {
    type Error = ClientOpcodeMessageError;
    fn write_unencrypted<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        match self {
            Self::CMSG_CHAR_ENUM(i) => i.write_body(w)?,
        }
        Ok(())
    }

    fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let size = (crate::util::read_u16_be(r)? - 4) as u32;
        let opcode = crate::util::read_u32_le(r)?;
        match opcode {
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::read_body(r, size)?)),
            _ => Err(Self::Error::InvalidOpcode(opcode)),
        }
    }

    fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, Self::Error> {
        let header = d.read_and_decrypt_client_header(r)?;
        let header_size = (header.size - 4) as u32;
        match header.opcode {
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(CMSG_CHAR_ENUM::read_body(r, header_size)?)),
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
pub enum ClientOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u32),
}

impl std::error::Error for ClientOpcodeMessageError {}
impl std::fmt::Display for ClientOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ClientMessage: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for ClientOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ClientOpcodeError> for ClientOpcodeMessageError {
    fn from(e: ClientOpcodeError) -> Self {
        match e {
            ClientOpcodeError::Io(i) => Self::Io(i),
            ClientOpcodeError::InvalidOpcode(i) => Self::InvalidOpcode(i),
        }
    }
}

use crate::world::v1::v2::SMSG_AUTH_CHALLENGE;
use crate::world::v1::v2::{SMSG_AUTH_RESPONSE, SMSG_AUTH_RESPONSEError};

#[derive(Debug)]
pub enum ServerOpcode {
    SMSG_AUTH_CHALLENGE,
    SMSG_AUTH_RESPONSE,
}

impl ServerOpcode {
    pub const fn as_u16(&self) -> u16 {
        match self {
            Self::SMSG_AUTH_CHALLENGE => 0x1ec,
            Self::SMSG_AUTH_RESPONSE => 0x1ee,
        }
    }

}

impl ServerOpcode {
    pub fn new(opcode: u16) -> std::result::Result<Self, ServerOpcodeError> {
        match opcode {
            0x1ec => Ok(Self::SMSG_AUTH_CHALLENGE),
            0x1ee => Ok(Self::SMSG_AUTH_RESPONSE),
            opcode => Err(ServerOpcodeError::InvalidOpcode(opcode)),
        }
    }

}

impl From<&ServerOpcodeMessage> for ServerOpcode {
    fn from(e: &ServerOpcodeMessage) -> Self {
        match *e {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => Self::SMSG_AUTH_CHALLENGE,
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(_) => Self::SMSG_AUTH_RESPONSE,
        }
    }
}

#[derive(Debug)]
pub enum ServerOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u16),
}

impl std::error::Error for ServerOpcodeError {
}

impl std::fmt::Display for ServerOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for Server: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for ServerOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub enum ServerOpcodeMessage {
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
}

impl OpcodeMessage for ServerOpcodeMessage {
    type Error = ServerOpcodeMessageError;
    fn write_unencrypted<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        match self {
            Self::SMSG_AUTH_CHALLENGE(i) => i.write_body(w)?,
            Self::SMSG_AUTH_RESPONSE(i) => i.write_body(w)?,
        }
        Ok(())
    }

    fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let size = (crate::util::read_u16_be(r)? - 2) as u32;
        let opcode = crate::util::read_u16_le(r)?;
        match opcode {
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::read_body(r, size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::read_body(r, size)?)),
            _ => Err(Self::Error::InvalidOpcode(opcode)),
        }
    }

    fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, Self::Error> {
        let header = d.read_and_decrypt_server_header(r)?;
        let header_size = (header.size - 2) as u32;
        match header.opcode {
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE::read_body(r, header_size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE::read_body(r, header_size)?)),
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
pub enum ServerOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u16),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSEError),
}

impl std::error::Error for ServerOpcodeMessageError {}
impl std::fmt::Display for ServerOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ServerMessage: '{}'", i)),
            Self::SMSG_AUTH_RESPONSE(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for ServerOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ServerOpcodeError> for ServerOpcodeMessageError {
    fn from(e: ServerOpcodeError) -> Self {
        match e {
            ServerOpcodeError::Io(i) => Self::Io(i),
            ServerOpcodeError::InvalidOpcode(i) => Self::InvalidOpcode(i),
        }
    }
}

impl From<SMSG_AUTH_RESPONSEError> for ServerOpcodeMessageError {
    fn from(e: SMSG_AUTH_RESPONSEError) -> Self {
        match e {
            SMSG_AUTH_RESPONSEError::Io(i) => Self::Io(i),
            _ => Self::SMSG_AUTH_RESPONSE(e),
        }
    }
}

