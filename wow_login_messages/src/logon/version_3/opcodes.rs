use crate::{ServerMessage, ClientMessage};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::logon::version_3::CMD_AUTH_LOGON_CHALLENGE_Server;
use crate::logon::version_2::CMD_AUTH_LOGON_PROOF_Server;
use crate::logon::version_2::CMD_REALM_LIST_Server;
use crate::logon::version_3::CMD_XFER_INITIATE;
use crate::logon::version_3::CMD_XFER_DATA;

#[derive(Debug)]
pub enum ServerOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server),
    CMD_REALM_LIST(CMD_REALM_LIST_Server),
    CMD_XFER_INITIATE(CMD_XFER_INITIATE),
    CMD_XFER_DATA(CMD_XFER_DATA),
}

impl ServerOpcodeMessage {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => e.write_into_vec(w)?,
            Self::CMD_REALM_LIST(e) => e.write_into_vec(w)?,
            Self::CMD_XFER_INITIATE(e) => e.write_into_vec(w)?,
            Self::CMD_XFER_DATA(e) => e.write_into_vec(w)?,
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
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::read(r)?)),
            0x30 => Ok(Self::CMD_XFER_INITIATE(CMD_XFER_INITIATE::read(r)?)),
            0x31 => Ok(Self::CMD_XFER_DATA(CMD_XFER_DATA::read(r)?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::tokio_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::tokio_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::tokio_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::tokio_read(r).await?)),
            0x30 => Ok(Self::CMD_XFER_INITIATE(CMD_XFER_INITIATE::tokio_read(r).await?)),
            0x31 => Ok(Self::CMD_XFER_DATA(CMD_XFER_DATA::tokio_read(r).await?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::astd_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::astd_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::astd_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::astd_read(r).await?)),
            0x30 => Ok(Self::CMD_XFER_INITIATE(CMD_XFER_INITIATE::astd_read(r).await?)),
            0x31 => Ok(Self::CMD_XFER_DATA(CMD_XFER_DATA::astd_read(r).await?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

}

impl std::fmt::Display for ServerOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(_) => "CMD_AUTH_LOGON_CHALLENGE_Server",
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(_) => "CMD_AUTH_LOGON_PROOF_Server",
            ServerOpcodeMessage::CMD_REALM_LIST(_) => "CMD_REALM_LIST_Server",
            ServerOpcodeMessage::CMD_XFER_INITIATE(_) => "CMD_XFER_INITIATE",
            ServerOpcodeMessage::CMD_XFER_DATA(_) => "CMD_XFER_DATA",
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

impl From<CMD_REALM_LIST_Server> for ServerOpcodeMessage {
    fn from(c: CMD_REALM_LIST_Server) -> Self {
        Self::CMD_REALM_LIST(c)
    }
}

impl From<CMD_XFER_INITIATE> for ServerOpcodeMessage {
    fn from(c: CMD_XFER_INITIATE) -> Self {
        Self::CMD_XFER_INITIATE(c)
    }
}

impl From<CMD_XFER_DATA> for ServerOpcodeMessage {
    fn from(c: CMD_XFER_DATA) -> Self {
        Self::CMD_XFER_DATA(c)
    }
}

use crate::logon::all::CMD_AUTH_LOGON_CHALLENGE_Client;
use crate::logon::version_3::CMD_AUTH_LOGON_PROOF_Client;
use crate::logon::all::CMD_AUTH_RECONNECT_CHALLENGE_Client;
use crate::logon::version_3::CMD_SURVEY_RESULT;
use crate::logon::version_2::CMD_REALM_LIST_Client;
use crate::logon::version_3::CMD_XFER_ACCEPT;
use crate::logon::version_3::CMD_XFER_RESUME;
use crate::logon::version_3::CMD_XFER_CANCEL;

#[derive(Debug)]
pub enum ClientOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client),
    CMD_SURVEY_RESULT(CMD_SURVEY_RESULT),
    CMD_REALM_LIST(CMD_REALM_LIST_Client),
    CMD_XFER_ACCEPT(CMD_XFER_ACCEPT),
    CMD_XFER_RESUME(CMD_XFER_RESUME),
    CMD_XFER_CANCEL(CMD_XFER_CANCEL),
}

impl ClientOpcodeMessage {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_SURVEY_RESULT(e) => e.write_into_vec(w)?,
            Self::CMD_REALM_LIST(e) => e.write_into_vec(w)?,
            Self::CMD_XFER_ACCEPT(e) => e.write_into_vec(w)?,
            Self::CMD_XFER_RESUME(e) => e.write_into_vec(w)?,
            Self::CMD_XFER_CANCEL(e) => e.write_into_vec(w)?,
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
            0x04 => Ok(Self::CMD_SURVEY_RESULT(CMD_SURVEY_RESULT::read(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::read(r)?)),
            0x32 => Ok(Self::CMD_XFER_ACCEPT(CMD_XFER_ACCEPT::read(r)?)),
            0x33 => Ok(Self::CMD_XFER_RESUME(CMD_XFER_RESUME::read(r)?)),
            0x34 => Ok(Self::CMD_XFER_CANCEL(CMD_XFER_CANCEL::read(r)?)),
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
            0x04 => Ok(Self::CMD_SURVEY_RESULT(CMD_SURVEY_RESULT::tokio_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::tokio_read(r).await?)),
            0x32 => Ok(Self::CMD_XFER_ACCEPT(CMD_XFER_ACCEPT::tokio_read(r).await?)),
            0x33 => Ok(Self::CMD_XFER_RESUME(CMD_XFER_RESUME::tokio_read(r).await?)),
            0x34 => Ok(Self::CMD_XFER_CANCEL(CMD_XFER_CANCEL::tokio_read(r).await?)),
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
            0x04 => Ok(Self::CMD_SURVEY_RESULT(CMD_SURVEY_RESULT::astd_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::astd_read(r).await?)),
            0x32 => Ok(Self::CMD_XFER_ACCEPT(CMD_XFER_ACCEPT::astd_read(r).await?)),
            0x33 => Ok(Self::CMD_XFER_RESUME(CMD_XFER_RESUME::astd_read(r).await?)),
            0x34 => Ok(Self::CMD_XFER_CANCEL(CMD_XFER_CANCEL::astd_read(r).await?)),
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
            ClientOpcodeMessage::CMD_SURVEY_RESULT(_) => "CMD_SURVEY_RESULT",
            ClientOpcodeMessage::CMD_REALM_LIST(_) => "CMD_REALM_LIST_Client",
            ClientOpcodeMessage::CMD_XFER_ACCEPT(_) => "CMD_XFER_ACCEPT",
            ClientOpcodeMessage::CMD_XFER_RESUME(_) => "CMD_XFER_RESUME",
            ClientOpcodeMessage::CMD_XFER_CANCEL(_) => "CMD_XFER_CANCEL",
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

impl From<CMD_SURVEY_RESULT> for ClientOpcodeMessage {
    fn from(c: CMD_SURVEY_RESULT) -> Self {
        Self::CMD_SURVEY_RESULT(c)
    }
}

impl From<CMD_REALM_LIST_Client> for ClientOpcodeMessage {
    fn from(c: CMD_REALM_LIST_Client) -> Self {
        Self::CMD_REALM_LIST(c)
    }
}

impl From<CMD_XFER_ACCEPT> for ClientOpcodeMessage {
    fn from(c: CMD_XFER_ACCEPT) -> Self {
        Self::CMD_XFER_ACCEPT(c)
    }
}

impl From<CMD_XFER_RESUME> for ClientOpcodeMessage {
    fn from(c: CMD_XFER_RESUME) -> Self {
        Self::CMD_XFER_RESUME(c)
    }
}

impl From<CMD_XFER_CANCEL> for ClientOpcodeMessage {
    fn from(c: CMD_XFER_CANCEL) -> Self {
        Self::CMD_XFER_CANCEL(c)
    }
}

