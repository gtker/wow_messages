use crate::{ServerMessage, ClientMessage};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::logon::version_8::CMD_AUTH_LOGON_CHALLENGE_Server;
use crate::logon::version_8::CMD_AUTH_LOGON_PROOF_Server;
use crate::logon::version_8::CMD_AUTH_RECONNECT_CHALLENGE_Server;
use crate::logon::version_8::CMD_AUTH_RECONNECT_PROOF_Server;
use crate::logon::version_8::CMD_REALM_LIST_Server;

#[derive(Debug)]
pub enum ServerOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server),
    CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server),
    CMD_REALM_LIST(CMD_REALM_LIST_Server),
}

impl ServerOpcodeMessage {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_RECONNECT_PROOF(e) => e.write_into_vec(w)?,
            Self::CMD_REALM_LIST(e) => e.write_into_vec(w)?,
        }

        Ok(())
    }
}

impl ServerOpcodeMessage {
    pub fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let opcode = crate::util::read_u8_le(r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::read(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::read(r)?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::read(r)?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::read(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::read(r)?)),
            opcode => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let opcode = crate::util::tokio_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::tokio_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::tokio_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::tokio_read(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::tokio_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::tokio_read(r).await?)),
            opcode => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let opcode = crate::util::astd_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::astd_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::astd_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::astd_read(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::astd_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::astd_read(r).await?)),
            opcode => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

}

#[derive(Debug)]
pub enum ServerOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u8),
    String(std::string::FromUtf8Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for ServerOpcodeMessageError {}
impl std::fmt::Display for ServerOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Enum(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ServerMessage: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for ServerOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::ParseError> for ServerOpcodeMessageError {
    fn from(e: crate::errors::ParseError) -> Self {
        match e {
            crate::errors::ParseError::Io(i) => Self::Io(i),
            crate::errors::ParseError::Enum(i) => Self::Enum(i),
            crate::errors::ParseError::String(i) => Self::String(i),
        }
    }
}

use crate::logon::all::CMD_AUTH_LOGON_CHALLENGE_Client;
use crate::logon::version_8::CMD_AUTH_LOGON_PROOF_Client;
use crate::logon::all::CMD_AUTH_RECONNECT_CHALLENGE_Client;
use crate::logon::version_2::CMD_AUTH_RECONNECT_PROOF_Client;
use crate::logon::version_2::CMD_REALM_LIST_Client;

#[derive(Debug)]
pub enum ClientOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client),
    CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client),
    CMD_REALM_LIST(CMD_REALM_LIST_Client),
}

impl ClientOpcodeMessage {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_RECONNECT_PROOF(e) => e.write_into_vec(w)?,
            Self::CMD_REALM_LIST(e) => e.write_into_vec(w)?,
        }

        Ok(())
    }
}

impl ClientOpcodeMessage {
    pub fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let opcode = crate::util::read_u8_le(r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::read(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::read(r)?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::read(r)?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::read(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::read(r)?)),
            opcode => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let opcode = crate::util::tokio_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::tokio_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::tokio_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::tokio_read(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::tokio_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::tokio_read(r).await?)),
            opcode => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let opcode = crate::util::astd_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::astd_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::astd_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::astd_read(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::astd_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::astd_read(r).await?)),
            opcode => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

}

#[derive(Debug)]
pub enum ClientOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u8),
    String(std::string::FromUtf8Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for ClientOpcodeMessageError {}
impl std::fmt::Display for ClientOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Enum(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ClientMessage: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for ClientOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::ParseError> for ClientOpcodeMessageError {
    fn from(e: crate::errors::ParseError) -> Self {
        match e {
            crate::errors::ParseError::Io(i) => Self::Io(i),
            crate::errors::ParseError::Enum(i) => Self::Enum(i),
            crate::errors::ParseError::String(i) => Self::String(i),
        }
    }
}

