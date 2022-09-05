use crate::{ServerMessage, ClientMessage};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::logon::version_2::CMD_AUTH_LOGON_CHALLENGE_Server;
use crate::logon::version_2::CMD_AUTH_LOGON_PROOF_Server;
use crate::logon::version_2::CMD_AUTH_RECONNECT_CHALLENGE_Server;
use crate::logon::version_2::CMD_AUTH_RECONNECT_PROOF_Server;
use crate::logon::version_2::CMD_REALM_LIST_Server;

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
    pub fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::read_u8_le(r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::read(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::read(r)?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::read(r)?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::read(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::read(r)?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::tokio_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::tokio_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::tokio_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::tokio_read(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::tokio_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::tokio_read(r).await?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::astd_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::astd_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::astd_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::astd_read(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::astd_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::astd_read(r).await?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

}

impl std::fmt::Display for ServerOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(_) => "CMD_AUTH_LOGON_CHALLENGE_Server",
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(_) => "CMD_AUTH_LOGON_PROOF_Server",
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(_) => "CMD_AUTH_RECONNECT_CHALLENGE_Server",
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(_) => "CMD_AUTH_RECONNECT_PROOF_Server",
            ServerOpcodeMessage::CMD_REALM_LIST(_) => "CMD_REALM_LIST_Server",
        })
    }
}

impl From<CMD_AUTH_LOGON_CHALLENGE_Server> for ServerOpcodeMessage {
    fn from(c: CMD_AUTH_LOGON_CHALLENGE_Server) -> Self {
        Self::CMD_AUTH_LOGON_CHALLENGE(c)
    }
}

impl From<CMD_AUTH_LOGON_PROOF_Server> for ServerOpcodeMessage {
    fn from(c: CMD_AUTH_LOGON_PROOF_Server) -> Self {
        Self::CMD_AUTH_LOGON_PROOF(c)
    }
}

impl From<CMD_AUTH_RECONNECT_CHALLENGE_Server> for ServerOpcodeMessage {
    fn from(c: CMD_AUTH_RECONNECT_CHALLENGE_Server) -> Self {
        Self::CMD_AUTH_RECONNECT_CHALLENGE(c)
    }
}

impl From<CMD_AUTH_RECONNECT_PROOF_Server> for ServerOpcodeMessage {
    fn from(c: CMD_AUTH_RECONNECT_PROOF_Server) -> Self {
        Self::CMD_AUTH_RECONNECT_PROOF(c)
    }
}

impl From<CMD_REALM_LIST_Server> for ServerOpcodeMessage {
    fn from(c: CMD_REALM_LIST_Server) -> Self {
        Self::CMD_REALM_LIST(c)
    }
}

use crate::logon::all::CMD_AUTH_LOGON_CHALLENGE_Client;
use crate::logon::version_2::CMD_AUTH_LOGON_PROOF_Client;
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
    pub fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::read_u8_le(r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::read(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::read(r)?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::read(r)?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::read(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::read(r)?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::tokio_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::tokio_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::tokio_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::tokio_read(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::tokio_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::tokio_read(r).await?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::astd_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::astd_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::astd_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::astd_read(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::astd_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::astd_read(r).await?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

}

impl std::fmt::Display for ClientOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ClientOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(_) => "CMD_AUTH_LOGON_CHALLENGE_Client",
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(_) => "CMD_AUTH_LOGON_PROOF_Client",
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(_) => "CMD_AUTH_RECONNECT_CHALLENGE_Client",
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(_) => "CMD_AUTH_RECONNECT_PROOF_Client",
            ClientOpcodeMessage::CMD_REALM_LIST(_) => "CMD_REALM_LIST_Client",
        })
    }
}

impl From<CMD_AUTH_LOGON_CHALLENGE_Client> for ClientOpcodeMessage {
    fn from(c: CMD_AUTH_LOGON_CHALLENGE_Client) -> Self {
        Self::CMD_AUTH_LOGON_CHALLENGE(c)
    }
}

impl From<CMD_AUTH_LOGON_PROOF_Client> for ClientOpcodeMessage {
    fn from(c: CMD_AUTH_LOGON_PROOF_Client) -> Self {
        Self::CMD_AUTH_LOGON_PROOF(c)
    }
}

impl From<CMD_AUTH_RECONNECT_CHALLENGE_Client> for ClientOpcodeMessage {
    fn from(c: CMD_AUTH_RECONNECT_CHALLENGE_Client) -> Self {
        Self::CMD_AUTH_RECONNECT_CHALLENGE(c)
    }
}

impl From<CMD_AUTH_RECONNECT_PROOF_Client> for ClientOpcodeMessage {
    fn from(c: CMD_AUTH_RECONNECT_PROOF_Client) -> Self {
        Self::CMD_AUTH_RECONNECT_PROOF(c)
    }
}

impl From<CMD_REALM_LIST_Client> for ClientOpcodeMessage {
    fn from(c: CMD_REALM_LIST_Client) -> Self {
        Self::CMD_REALM_LIST(c)
    }
}

