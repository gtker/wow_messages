use crate::{ServerMessage, ClientMessage};
use wow_srp::header_crypto::{Decrypter, Encrypter};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::v1::v2::CMSG_CHAR_ENUM;

#[derive(Debug)]
pub enum ClientOpcodeMessage {
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
}

impl ClientOpcodeMessage {

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let size = (crate::util::read_u16_be(r)? - 4) as u32;
        let opcode = crate::util::read_u32_le(r)?;
        match opcode {
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as ClientMessage>::read_body(r, size)?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let mut header = [0u8; 6];
        r.read_exact(&mut header)?;
        let header = d.decrypt_client_header(header);
        let header_size = (header.size - 4) as u32;
        match header.opcode {
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as ClientMessage>::read_body(r, header_size)?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }


    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let size = (crate::util::tokio_read_u16_be(r).await? - 4) as u32;
        let opcode = crate::util::tokio_read_u32_le(r).await?;
        match opcode {
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as ClientMessage>::tokio_read_body(r, size).await?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: AsyncReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let mut header = [0u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let header_size = (header.size - 4) as u32;
        match header.opcode {
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as ClientMessage>::tokio_read_body(r, header_size).await?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }


    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let size = (crate::util::astd_read_u16_be(r).await? - 4) as u32;
        let opcode = crate::util::astd_read_u32_le(r).await?;
        match opcode {
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as ClientMessage>::astd_read_body(r, size).await?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: ReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let mut header = [0u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let header_size = (header.size - 4) as u32;
        match header.opcode {
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as ClientMessage>::astd_read_body(r, header_size).await?)),
            _ => Err(ClientOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }

}

#[derive(Debug)]
pub enum ClientOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u32),
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

use crate::world::v1::v2::SMSG_AUTH_CHALLENGE;
use crate::world::v1::v2::SMSG_AUTH_RESPONSE;

#[derive(Debug)]
pub enum ServerOpcodeMessage {
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
}

impl ServerOpcodeMessage {

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let size = (crate::util::read_u16_be(r)? - 2) as u32;
        let opcode = crate::util::read_u16_le(r)?;
        match opcode {
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as ServerMessage>::read_body(r, size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as ServerMessage>::read_body(r, size)?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let mut header = [0u8; 4];
        r.read_exact(&mut header)?;
        let header = d.decrypt_server_header(header);
        let header_size = (header.size - 2) as u32;
        match header.opcode {
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as ServerMessage>::read_body(r, header_size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as ServerMessage>::read_body(r, header_size)?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }


    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let size = (crate::util::tokio_read_u16_be(r).await? - 2) as u32;
        let opcode = crate::util::tokio_read_u16_le(r).await?;
        match opcode {
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as ServerMessage>::tokio_read_body(r, size).await?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as ServerMessage>::tokio_read_body(r, size).await?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: AsyncReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let mut header = [0u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let header_size = (header.size - 2) as u32;
        match header.opcode {
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as ServerMessage>::tokio_read_body(r, header_size).await?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as ServerMessage>::tokio_read_body(r, header_size).await?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }


    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let size = (crate::util::astd_read_u16_be(r).await? - 2) as u32;
        let opcode = crate::util::astd_read_u16_le(r).await?;
        match opcode {
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as ServerMessage>::astd_read_body(r, size).await?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as ServerMessage>::astd_read_body(r, size).await?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: ReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let mut header = [0u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let header_size = (header.size - 2) as u32;
        match header.opcode {
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as ServerMessage>::astd_read_body(r, header_size).await?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as ServerMessage>::astd_read_body(r, header_size).await?)),
            _ => Err(ServerOpcodeMessageError::InvalidOpcode(header.opcode)),
        }
    }

}

#[derive(Debug)]
pub enum ServerOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u16),
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

