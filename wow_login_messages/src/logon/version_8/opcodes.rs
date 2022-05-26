use crate::{ServerMessage, ClientMessage};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::logon::version_8::{CMD_AUTH_LOGON_CHALLENGE_Server, CMD_AUTH_LOGON_CHALLENGE_ServerError};
use crate::logon::version_8::{CMD_AUTH_LOGON_PROOF_Server, CMD_AUTH_LOGON_PROOF_ServerError};
use crate::logon::version_8::{CMD_AUTH_RECONNECT_CHALLENGE_Server, CMD_AUTH_RECONNECT_CHALLENGE_ServerError};
use crate::logon::version_8::{CMD_AUTH_RECONNECT_PROOF_Server, CMD_AUTH_RECONNECT_PROOF_ServerError};
use crate::logon::version_8::{CMD_REALM_LIST_Server, CMD_REALM_LIST_ServerError};

#[derive(Debug)]
pub enum ServerOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server),
    CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server),
    CMD_REALM_LIST(CMD_REALM_LIST_Server),
}

impl ServerOpcodeMessage {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_AUTH_RECONNECT_PROOF(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_REALM_LIST(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
        }

        Ok(w)
    }
}

impl ServerOpcodeMessage {
    #[cfg(feature = "sync")]
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
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_ServerError),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_ServerError),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_ServerError),
    CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_ServerError),
    CMD_REALM_LIST(CMD_REALM_LIST_ServerError),
}

impl std::error::Error for ServerOpcodeMessageError {}
impl std::fmt::Display for ServerOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ServerMessage: '{}'", i)),
            Self::CMD_AUTH_LOGON_CHALLENGE(i) => i.fmt(f),
            Self::CMD_AUTH_LOGON_PROOF(i) => i.fmt(f),
            Self::CMD_AUTH_RECONNECT_CHALLENGE(i) => i.fmt(f),
            Self::CMD_AUTH_RECONNECT_PROOF(i) => i.fmt(f),
            Self::CMD_REALM_LIST(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for ServerOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<CMD_AUTH_LOGON_CHALLENGE_ServerError> for ServerOpcodeMessageError {
    fn from(e: CMD_AUTH_LOGON_CHALLENGE_ServerError) -> Self {
        match e {
            CMD_AUTH_LOGON_CHALLENGE_ServerError::Io(i) => Self::Io(i),
            _ => Self::CMD_AUTH_LOGON_CHALLENGE(e),
        }
    }
}

impl From<CMD_AUTH_LOGON_PROOF_ServerError> for ServerOpcodeMessageError {
    fn from(e: CMD_AUTH_LOGON_PROOF_ServerError) -> Self {
        match e {
            CMD_AUTH_LOGON_PROOF_ServerError::Io(i) => Self::Io(i),
            _ => Self::CMD_AUTH_LOGON_PROOF(e),
        }
    }
}

impl From<CMD_AUTH_RECONNECT_CHALLENGE_ServerError> for ServerOpcodeMessageError {
    fn from(e: CMD_AUTH_RECONNECT_CHALLENGE_ServerError) -> Self {
        match e {
            CMD_AUTH_RECONNECT_CHALLENGE_ServerError::Io(i) => Self::Io(i),
            _ => Self::CMD_AUTH_RECONNECT_CHALLENGE(e),
        }
    }
}

impl From<CMD_AUTH_RECONNECT_PROOF_ServerError> for ServerOpcodeMessageError {
    fn from(e: CMD_AUTH_RECONNECT_PROOF_ServerError) -> Self {
        match e {
            CMD_AUTH_RECONNECT_PROOF_ServerError::Io(i) => Self::Io(i),
            _ => Self::CMD_AUTH_RECONNECT_PROOF(e),
        }
    }
}

impl From<CMD_REALM_LIST_ServerError> for ServerOpcodeMessageError {
    fn from(e: CMD_REALM_LIST_ServerError) -> Self {
        match e {
            CMD_REALM_LIST_ServerError::Io(i) => Self::Io(i),
            _ => Self::CMD_REALM_LIST(e),
        }
    }
}

use crate::logon::all::{CMD_AUTH_LOGON_CHALLENGE_Client, CMD_AUTH_LOGON_CHALLENGE_ClientError};
use crate::logon::version_8::CMD_AUTH_LOGON_PROOF_Client;
use crate::logon::all::{CMD_AUTH_RECONNECT_CHALLENGE_Client, CMD_AUTH_RECONNECT_CHALLENGE_ClientError};
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
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_AUTH_RECONNECT_PROOF(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_REALM_LIST(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
        }

        Ok(w)
    }
}

impl ClientOpcodeMessage {
    #[cfg(feature = "sync")]
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
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_ClientError),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_ClientError),
}

impl std::error::Error for ClientOpcodeMessageError {}
impl std::fmt::Display for ClientOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ClientMessage: '{}'", i)),
            Self::CMD_AUTH_LOGON_CHALLENGE(i) => i.fmt(f),
            Self::CMD_AUTH_RECONNECT_CHALLENGE(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for ClientOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<CMD_AUTH_LOGON_CHALLENGE_ClientError> for ClientOpcodeMessageError {
    fn from(e: CMD_AUTH_LOGON_CHALLENGE_ClientError) -> Self {
        match e {
            CMD_AUTH_LOGON_CHALLENGE_ClientError::Io(i) => Self::Io(i),
            _ => Self::CMD_AUTH_LOGON_CHALLENGE(e),
        }
    }
}

impl From<CMD_AUTH_RECONNECT_CHALLENGE_ClientError> for ClientOpcodeMessageError {
    fn from(e: CMD_AUTH_RECONNECT_CHALLENGE_ClientError) -> Self {
        match e {
            CMD_AUTH_RECONNECT_CHALLENGE_ClientError::Io(i) => Self::Io(i),
            _ => Self::CMD_AUTH_RECONNECT_CHALLENGE(e),
        }
    }
}

