use crate::ReadableAndWritable;

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
use crate::logon::version_2::{CMD_AUTH_LOGON_CHALLENGE_Server, CMD_AUTH_LOGON_CHALLENGE_ServerError};
use crate::logon::version_2::{CMD_AUTH_LOGON_PROOF_Server, CMD_AUTH_LOGON_PROOF_ServerError};
use crate::logon::version_2::{CMD_AUTH_RECONNECT_CHALLENGE_Server, CMD_AUTH_RECONNECT_CHALLENGE_ServerError};
use crate::logon::version_2::{CMD_AUTH_RECONNECT_PROOF_Server, CMD_AUTH_RECONNECT_PROOF_ServerError};
use crate::logon::version_2::{CMD_REALM_LIST_Server, CMD_REALM_LIST_ServerError};

#[derive(Debug)]
pub enum ServerOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server),
    CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server),
    CMD_REALM_LIST(CMD_REALM_LIST_Server),
}

impl ReadableAndWritable for ServerOpcodeMessage {
    type Error = ServerOpcodeMessageError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let opcode = ServerOpcode::read(r)?;
        match opcode {
            ServerOpcode::CMD_AUTH_LOGON_CHALLENGE => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::read(r)?)),
            ServerOpcode::CMD_AUTH_LOGON_PROOF => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::read(r)?)),
            ServerOpcode::CMD_AUTH_RECONNECT_CHALLENGE => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::read(r)?)),
            ServerOpcode::CMD_AUTH_RECONNECT_PROOF => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::read(r)?)),
            ServerOpcode::CMD_REALM_LIST => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::read(r)?)),
        }
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.write(w)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => e.write(w)?,
            Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.write(w)?,
            Self::CMD_AUTH_RECONNECT_PROOF(e) => e.write(w)?,
            Self::CMD_REALM_LIST(e) => e.write(w)?,
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let opcode = ServerOpcode::tokio_read(r).await?;
            match opcode {
                ServerOpcode::CMD_AUTH_LOGON_CHALLENGE => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::tokio_read(r).await?)),
                ServerOpcode::CMD_AUTH_LOGON_PROOF => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::tokio_read(r).await?)),
                ServerOpcode::CMD_AUTH_RECONNECT_CHALLENGE => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::tokio_read(r).await?)),
                ServerOpcode::CMD_AUTH_RECONNECT_PROOF => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::tokio_read(r).await?)),
                ServerOpcode::CMD_REALM_LIST => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::tokio_read(r).await?)),
            }
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            match self {
                Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.tokio_write(w).await?,
                Self::CMD_AUTH_LOGON_PROOF(e) => e.tokio_write(w).await?,
                Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.tokio_write(w).await?,
                Self::CMD_AUTH_RECONNECT_PROOF(e) => e.tokio_write(w).await?,
                Self::CMD_REALM_LIST(e) => e.tokio_write(w).await?,
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let opcode = ServerOpcode::astd_read(r).await?;
            match opcode {
                ServerOpcode::CMD_AUTH_LOGON_CHALLENGE => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::astd_read(r).await?)),
                ServerOpcode::CMD_AUTH_LOGON_PROOF => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::astd_read(r).await?)),
                ServerOpcode::CMD_AUTH_RECONNECT_CHALLENGE => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Server::astd_read(r).await?)),
                ServerOpcode::CMD_AUTH_RECONNECT_PROOF => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Server::astd_read(r).await?)),
                ServerOpcode::CMD_REALM_LIST => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::astd_read(r).await?)),
            }
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            match self {
                Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.astd_write(w).await?,
                Self::CMD_AUTH_LOGON_PROOF(e) => e.astd_write(w).await?,
                Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.astd_write(w).await?,
                Self::CMD_AUTH_RECONNECT_PROOF(e) => e.astd_write(w).await?,
                Self::CMD_REALM_LIST(e) => e.astd_write(w).await?,
            }

            Ok(())
        })
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

impl From<ServerOpcodeError> for ServerOpcodeMessageError {
    fn from(e: ServerOpcodeError) -> Self {
        match e {
            ServerOpcodeError::Io(i) => Self::Io(i),
            ServerOpcodeError::InvalidOpcode(i) => Self::InvalidOpcode(i),
        }
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

#[derive(Debug)]
pub enum ServerOpcode {
    CMD_AUTH_LOGON_CHALLENGE,
    CMD_AUTH_LOGON_PROOF,
    CMD_AUTH_RECONNECT_CHALLENGE,
    CMD_AUTH_RECONNECT_PROOF,
    CMD_REALM_LIST,
}

impl ServerOpcode {
    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE => 0x00,
            Self::CMD_AUTH_LOGON_PROOF => 0x01,
            Self::CMD_AUTH_RECONNECT_CHALLENGE => 0x02,
            Self::CMD_AUTH_RECONNECT_PROOF => 0x03,
            Self::CMD_REALM_LIST => 0x10,
        }
    }

}

impl ReadableAndWritable for ServerOpcode {
    type Error = ServerOpcodeError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let opcode = crate::util::read_u8_le(r)?;

        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF),
            0x10 => Ok(Self::CMD_REALM_LIST),
            opcode => Err(ServerOpcodeError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u8_le(w, self.as_u8())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let opcode = crate::util::tokio_read_u8_le(r).await?;

            match opcode {
                0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE),
                0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF),
                0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE),
                0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF),
                0x10 => Ok(Self::CMD_REALM_LIST),
                opcode => Err(ServerOpcodeError::InvalidOpcode(opcode)),
            }
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            crate::util::tokio_write_u8_le(w, self.as_u8()).await?;
            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let opcode = crate::util::astd_read_u8_le(r).await?;

            match opcode {
                0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE),
                0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF),
                0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE),
                0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF),
                0x10 => Ok(Self::CMD_REALM_LIST),
                opcode => Err(ServerOpcodeError::InvalidOpcode(opcode)),
            }
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            crate::util::astd_write_u8_le(w, self.as_u8()).await?;
            Ok(())
        })
    }

}

impl From<&ServerOpcodeMessage> for ServerOpcode {
    fn from(e: &ServerOpcodeMessage) -> Self {
        match *e {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(_) => Self::CMD_AUTH_LOGON_CHALLENGE,
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(_) => Self::CMD_AUTH_LOGON_PROOF,
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(_) => Self::CMD_AUTH_RECONNECT_CHALLENGE,
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(_) => Self::CMD_AUTH_RECONNECT_PROOF,
            ServerOpcodeMessage::CMD_REALM_LIST(_) => Self::CMD_REALM_LIST,
        }
    }
}

#[derive(Debug)]
pub enum ServerOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u8),
}

impl std::fmt::Display for ServerOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for Server: '{}'", i)),
        }
    }
}

impl std::error::Error for ServerOpcodeError {
}

impl From<std::io::Error> for ServerOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

use crate::logon::all::{CMD_AUTH_LOGON_CHALLENGE_Client, CMD_AUTH_LOGON_CHALLENGE_ClientError};
use crate::logon::version_2::CMD_AUTH_LOGON_PROOF_Client;
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

impl ReadableAndWritable for ClientOpcodeMessage {
    type Error = ClientOpcodeMessageError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let opcode = ClientOpcode::read(r)?;
        match opcode {
            ClientOpcode::CMD_AUTH_LOGON_CHALLENGE => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::read(r)?)),
            ClientOpcode::CMD_AUTH_LOGON_PROOF => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::read(r)?)),
            ClientOpcode::CMD_AUTH_RECONNECT_CHALLENGE => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::read(r)?)),
            ClientOpcode::CMD_AUTH_RECONNECT_PROOF => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::read(r)?)),
            ClientOpcode::CMD_REALM_LIST => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::read(r)?)),
        }
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.write(w)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => e.write(w)?,
            Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.write(w)?,
            Self::CMD_AUTH_RECONNECT_PROOF(e) => e.write(w)?,
            Self::CMD_REALM_LIST(e) => e.write(w)?,
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let opcode = ClientOpcode::tokio_read(r).await?;
            match opcode {
                ClientOpcode::CMD_AUTH_LOGON_CHALLENGE => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::tokio_read(r).await?)),
                ClientOpcode::CMD_AUTH_LOGON_PROOF => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::tokio_read(r).await?)),
                ClientOpcode::CMD_AUTH_RECONNECT_CHALLENGE => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::tokio_read(r).await?)),
                ClientOpcode::CMD_AUTH_RECONNECT_PROOF => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::tokio_read(r).await?)),
                ClientOpcode::CMD_REALM_LIST => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::tokio_read(r).await?)),
            }
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            match self {
                Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.tokio_write(w).await?,
                Self::CMD_AUTH_LOGON_PROOF(e) => e.tokio_write(w).await?,
                Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.tokio_write(w).await?,
                Self::CMD_AUTH_RECONNECT_PROOF(e) => e.tokio_write(w).await?,
                Self::CMD_REALM_LIST(e) => e.tokio_write(w).await?,
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let opcode = ClientOpcode::astd_read(r).await?;
            match opcode {
                ClientOpcode::CMD_AUTH_LOGON_CHALLENGE => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::astd_read(r).await?)),
                ClientOpcode::CMD_AUTH_LOGON_PROOF => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::astd_read(r).await?)),
                ClientOpcode::CMD_AUTH_RECONNECT_CHALLENGE => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::astd_read(r).await?)),
                ClientOpcode::CMD_AUTH_RECONNECT_PROOF => Ok(Self::CMD_AUTH_RECONNECT_PROOF(CMD_AUTH_RECONNECT_PROOF_Client::astd_read(r).await?)),
                ClientOpcode::CMD_REALM_LIST => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::astd_read(r).await?)),
            }
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            match self {
                Self::CMD_AUTH_LOGON_CHALLENGE(e) => e.astd_write(w).await?,
                Self::CMD_AUTH_LOGON_PROOF(e) => e.astd_write(w).await?,
                Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => e.astd_write(w).await?,
                Self::CMD_AUTH_RECONNECT_PROOF(e) => e.astd_write(w).await?,
                Self::CMD_REALM_LIST(e) => e.astd_write(w).await?,
            }

            Ok(())
        })
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

impl From<ClientOpcodeError> for ClientOpcodeMessageError {
    fn from(e: ClientOpcodeError) -> Self {
        match e {
            ClientOpcodeError::Io(i) => Self::Io(i),
            ClientOpcodeError::InvalidOpcode(i) => Self::InvalidOpcode(i),
        }
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

#[derive(Debug)]
pub enum ClientOpcode {
    CMD_AUTH_LOGON_CHALLENGE,
    CMD_AUTH_LOGON_PROOF,
    CMD_AUTH_RECONNECT_CHALLENGE,
    CMD_AUTH_RECONNECT_PROOF,
    CMD_REALM_LIST,
}

impl ClientOpcode {
    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE => 0x00,
            Self::CMD_AUTH_LOGON_PROOF => 0x01,
            Self::CMD_AUTH_RECONNECT_CHALLENGE => 0x02,
            Self::CMD_AUTH_RECONNECT_PROOF => 0x03,
            Self::CMD_REALM_LIST => 0x10,
        }
    }

}

impl ReadableAndWritable for ClientOpcode {
    type Error = ClientOpcodeError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let opcode = crate::util::read_u8_le(r)?;

        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE),
            0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF),
            0x10 => Ok(Self::CMD_REALM_LIST),
            opcode => Err(ClientOpcodeError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u8_le(w, self.as_u8())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let opcode = crate::util::tokio_read_u8_le(r).await?;

            match opcode {
                0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE),
                0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF),
                0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE),
                0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF),
                0x10 => Ok(Self::CMD_REALM_LIST),
                opcode => Err(ClientOpcodeError::InvalidOpcode(opcode)),
            }
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            crate::util::tokio_write_u8_le(w, self.as_u8()).await?;
            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let opcode = crate::util::astd_read_u8_le(r).await?;

            match opcode {
                0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE),
                0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF),
                0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE),
                0x03 => Ok(Self::CMD_AUTH_RECONNECT_PROOF),
                0x10 => Ok(Self::CMD_REALM_LIST),
                opcode => Err(ClientOpcodeError::InvalidOpcode(opcode)),
            }
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            crate::util::astd_write_u8_le(w, self.as_u8()).await?;
            Ok(())
        })
    }

}

impl From<&ClientOpcodeMessage> for ClientOpcode {
    fn from(e: &ClientOpcodeMessage) -> Self {
        match *e {
            ClientOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(_) => Self::CMD_AUTH_LOGON_CHALLENGE,
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(_) => Self::CMD_AUTH_LOGON_PROOF,
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(_) => Self::CMD_AUTH_RECONNECT_CHALLENGE,
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(_) => Self::CMD_AUTH_RECONNECT_PROOF,
            ClientOpcodeMessage::CMD_REALM_LIST(_) => Self::CMD_REALM_LIST,
        }
    }
}

#[derive(Debug)]
pub enum ClientOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u8),
}

impl std::fmt::Display for ClientOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for Client: '{}'", i)),
        }
    }
}

impl std::error::Error for ClientOpcodeError {
}

impl From<std::io::Error> for ClientOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

