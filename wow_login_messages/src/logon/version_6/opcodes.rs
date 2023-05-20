use crate::{ServerMessage, ClientMessage};
use std::io::{Read, Write};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use super::*;
use crate::all::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server),
    CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server),
    CMD_REALM_LIST(CMD_REALM_LIST_Server),
}

impl ServerOpcodeMessage {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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
    pub fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::read_u8_le(&mut r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::read::<R, crate::private::Internal>(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::read::<R, crate::private::Internal>(r)?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::read::<R, crate::private::Internal>(r)?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::read::<R, crate::private::Internal>(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::read::<R, crate::private::Internal>(r)?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::tokio_read_u8_le(&mut r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::tokio_read::<R, crate::private::Internal>(r).await?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::astd_read_u8_le(&mut r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::astd_read::<R, crate::private::Internal>(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::astd_read::<R, crate::private::Internal>(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::astd_read::<R, crate::private::Internal>(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::astd_read::<R, crate::private::Internal>(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::astd_read::<R, crate::private::Internal>(r).await?)),
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


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client),
    CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client),
    CMD_REALM_LIST(CMD_REALM_LIST_Client),
}

impl ClientOpcodeMessage {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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
    pub fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::read_u8_le(&mut r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::read::<R, crate::private::Internal>(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::read::<R, crate::private::Internal>(r)?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::read::<R, crate::private::Internal>(r)?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::read::<R, crate::private::Internal>(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::read::<R, crate::private::Internal>(r)?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::tokio_read_u8_le(&mut r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::tokio_read::<R, crate::private::Internal>(r).await?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::astd_read_u8_le(&mut r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::astd_read::<R, crate::private::Internal>(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::astd_read::<R, crate::private::Internal>(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::astd_read::<R, crate::private::Internal>(r).await?)),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::astd_read::<R, crate::private::Internal>(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::astd_read::<R, crate::private::Internal>(r).await?)),
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

