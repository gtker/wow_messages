use crate::{ServerMessage, ClientMessage};
use std::io::{Read, Write};
use super::*;
use crate::all::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server),
    CMD_REALM_LIST(CMD_REALM_LIST_Server),
    CMD_XFER_INITIATE,
    CMD_XFER_DATA(CMD_XFER_DATA),
}

impl ServerOpcodeMessage {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => e.write_into_vec(w)?,
            Self::CMD_REALM_LIST(e) => e.write_into_vec(w)?,
            Self::CMD_XFER_INITIATE => {}
            Self::CMD_XFER_DATA(e) => e.write_into_vec(w)?,
        }

        Ok(())
    }
}

impl ServerOpcodeMessage {
    #[cfg(feature = "sync")]
    pub fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::read_u8_le(&mut r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::read::<R, crate::private::Internal>(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::read::<R, crate::private::Internal>(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::read::<R, crate::private::Internal>(r)?)),
            0x30 => Ok(Self::CMD_XFER_INITIATE),
            0x31 => Ok(Self::CMD_XFER_DATA(CMD_XFER_DATA::read::<R, crate::private::Internal>(r)?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::tokio_read_u8_le(&mut r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x30 => Ok(Self::CMD_XFER_INITIATE),
            0x31 => Ok(Self::CMD_XFER_DATA(CMD_XFER_DATA::tokio_read::<R, crate::private::Internal>(r).await?)),
            opcode => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::astd_read_u8_le(&mut r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::astd_read::<R, crate::private::Internal>(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::astd_read::<R, crate::private::Internal>(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::astd_read::<R, crate::private::Internal>(r).await?)),
            0x30 => Ok(Self::CMD_XFER_INITIATE),
            0x31 => Ok(Self::CMD_XFER_DATA(CMD_XFER_DATA::astd_read::<R, crate::private::Internal>(r).await?)),
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
            ServerOpcodeMessage::CMD_XFER_INITIATE => "CMD_XFER_INITIATE",
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
    fn from(_: CMD_XFER_INITIATE) -> Self {
        Self::CMD_XFER_INITIATE
    }
}

impl From<CMD_XFER_DATA> for ServerOpcodeMessage {
    fn from(c: CMD_XFER_DATA) -> Self {
        Self::CMD_XFER_DATA(c)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client),
    CMD_SURVEY_RESULT(CMD_SURVEY_RESULT),
    CMD_REALM_LIST(CMD_REALM_LIST_Client),
    CMD_XFER_ACCEPT,
    CMD_XFER_RESUME(CMD_XFER_RESUME),
    CMD_XFER_CANCEL,
}

impl ClientOpcodeMessage {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => e.write_into_vec(w)?,
            Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.write_into_vec(w)?,
            Self::CMD_SURVEY_RESULT(e) => e.write_into_vec(w)?,
            Self::CMD_REALM_LIST(e) => e.write_into_vec(w)?,
            Self::CMD_XFER_ACCEPT => {}
            Self::CMD_XFER_RESUME(e) => e.write_into_vec(w)?,
            Self::CMD_XFER_CANCEL => {}
        }

        Ok(())
    }
}

impl ClientOpcodeMessage {
    #[cfg(feature = "sync")]
    pub fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ExpectedOpcodeError> {
        let opcode = crate::util::read_u8_le(&mut r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::read::<R, crate::private::Internal>(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::read::<R, crate::private::Internal>(r)?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::read::<R, crate::private::Internal>(r)?)),
            0x04 => Ok(Self::CMD_SURVEY_RESULT(CMD_SURVEY_RESULT::read::<R, crate::private::Internal>(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::read::<R, crate::private::Internal>(r)?)),
            0x32 => Ok(Self::CMD_XFER_ACCEPT),
            0x33 => Ok(Self::CMD_XFER_RESUME(CMD_XFER_RESUME::read::<R, crate::private::Internal>(r)?)),
            0x34 => Ok(Self::CMD_XFER_CANCEL),
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
            0x04 => Ok(Self::CMD_SURVEY_RESULT(CMD_SURVEY_RESULT::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x32 => Ok(Self::CMD_XFER_ACCEPT),
            0x33 => Ok(Self::CMD_XFER_RESUME(CMD_XFER_RESUME::tokio_read::<R, crate::private::Internal>(r).await?)),
            0x34 => Ok(Self::CMD_XFER_CANCEL),
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
            0x04 => Ok(Self::CMD_SURVEY_RESULT(CMD_SURVEY_RESULT::astd_read::<R, crate::private::Internal>(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::astd_read::<R, crate::private::Internal>(r).await?)),
            0x32 => Ok(Self::CMD_XFER_ACCEPT),
            0x33 => Ok(Self::CMD_XFER_RESUME(CMD_XFER_RESUME::astd_read::<R, crate::private::Internal>(r).await?)),
            0x34 => Ok(Self::CMD_XFER_CANCEL),
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
            ClientOpcodeMessage::CMD_XFER_ACCEPT => "CMD_XFER_ACCEPT",
            ClientOpcodeMessage::CMD_XFER_RESUME(_) => "CMD_XFER_RESUME",
            ClientOpcodeMessage::CMD_XFER_CANCEL => "CMD_XFER_CANCEL",
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
    fn from(_: CMD_XFER_ACCEPT) -> Self {
        Self::CMD_XFER_ACCEPT
    }
}

impl From<CMD_XFER_RESUME> for ClientOpcodeMessage {
    fn from(c: CMD_XFER_RESUME) -> Self {
        Self::CMD_XFER_RESUME(c)
    }
}

impl From<CMD_XFER_CANCEL> for ClientOpcodeMessage {
    fn from(_: CMD_XFER_CANCEL) -> Self {
        Self::CMD_XFER_CANCEL
    }
}

