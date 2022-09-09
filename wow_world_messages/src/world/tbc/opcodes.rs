use crate::tbc::{ServerMessage, ClientMessage};
use wow_srp::vanilla_header::{DecrypterHalf, EncrypterHalf};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::tbc::CMSG_CHAR_CREATE;
use crate::world::tbc::CMSG_CHAR_ENUM;
use crate::world::tbc::CMSG_CHAR_DELETE;
use crate::world::tbc::CMSG_PLAYER_LOGIN;
use crate::world::tbc::CMSG_CHAR_RENAME;

#[derive(Debug, Clone, PartialEq)]
pub enum ClientOpcodeMessage {
    CMSG_CHAR_CREATE(CMSG_CHAR_CREATE),
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
    CMSG_CHAR_DELETE(CMSG_CHAR_DELETE),
    CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN),
    CMSG_CHAR_RENAME(CMSG_CHAR_RENAME),
}

impl ClientOpcodeMessage {
    fn read_opcodes(opcode: u32, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(<CMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size)?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(<CMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(<CMSG_PLAYER_LOGIN as crate::Message>::read_body(&mut r, body_size)?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(<CMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size)?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode, size: body_size }),
        }
    }

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::read_u16_be(r)?.saturating_sub(4)) as u32;
        let opcode = crate::util::read_u32_le(r)?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header)?;
        let header = d.decrypt_client_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::tokio_read_u16_be(r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::tokio_read_u32_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::astd_read_u16_be(r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::astd_read_u32_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "sync")]
    pub fn write_encrypted_client<W: std::io::Write>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::CMSG_CHAR_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_ENUM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_DELETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_RENAME(c) => c.write_encrypted_client(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::CMSG_CHAR_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_ENUM(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_DELETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_RENAME(c) => c.write_unencrypted_client(w),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_encrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_encrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_client(w).await,
        }
    }

}

impl std::fmt::Display for ClientOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ClientOpcodeMessage::CMSG_CHAR_CREATE(_) => "CMSG_CHAR_CREATE",
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => "CMSG_CHAR_ENUM",
            ClientOpcodeMessage::CMSG_CHAR_DELETE(_) => "CMSG_CHAR_DELETE",
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => "CMSG_PLAYER_LOGIN",
            ClientOpcodeMessage::CMSG_CHAR_RENAME(_) => "CMSG_CHAR_RENAME",
        })
    }
}

impl From<CMSG_CHAR_CREATE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_CREATE) -> Self {
        Self::CMSG_CHAR_CREATE(c)
    }
}

impl From<CMSG_CHAR_ENUM> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_ENUM) -> Self {
        Self::CMSG_CHAR_ENUM(c)
    }
}

impl From<CMSG_CHAR_DELETE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_DELETE) -> Self {
        Self::CMSG_CHAR_DELETE(c)
    }
}

impl From<CMSG_PLAYER_LOGIN> for ClientOpcodeMessage {
    fn from(c: CMSG_PLAYER_LOGIN) -> Self {
        Self::CMSG_PLAYER_LOGIN(c)
    }
}

impl From<CMSG_CHAR_RENAME> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_RENAME) -> Self {
        Self::CMSG_CHAR_RENAME(c)
    }
}

use crate::world::tbc::SMSG_CHAR_CREATE;
use crate::world::tbc::SMSG_CHAR_ENUM;
use crate::world::tbc::SMSG_CHAR_DELETE;
use crate::world::tbc::SMSG_TUTORIAL_FLAGS;
use crate::world::tbc::SMSG_AUTH_CHALLENGE;
use crate::world::tbc::SMSG_ACCOUNT_DATA_TIMES;
use crate::world::tbc::SMSG_LOGIN_VERIFY_WORLD;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerOpcodeMessage {
    SMSG_CHAR_CREATE(SMSG_CHAR_CREATE),
    SMSG_CHAR_ENUM(SMSG_CHAR_ENUM),
    SMSG_CHAR_DELETE(SMSG_CHAR_DELETE),
    SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS),
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD),
}

impl ServerOpcodeMessage {
    fn read_opcodes(opcode: u16, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x003A => Ok(Self::SMSG_CHAR_CREATE(<SMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(<SMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size)?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(<SMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(<SMSG_TUTORIAL_FLAGS as crate::Message>::read_body(&mut r, body_size)?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(<SMSG_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size)?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(<SMSG_LOGIN_VERIFY_WORLD as crate::Message>::read_body(&mut r, body_size)?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode: opcode.into(), size: body_size }),
        }
    }

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::read_u16_be(r)?.saturating_sub(2)) as u32;
        let opcode = crate::util::read_u16_le(r)?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header)?;
        let header = d.decrypt_server_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::tokio_read_u16_be(r).await?.saturating_sub(2)) as u32;
        let opcode = crate::util::tokio_read_u16_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::astd_read_u16_be(r).await?.saturating_sub(2)) as u32;
        let opcode = crate::util::astd_read_u16_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "sync")]
    pub fn write_encrypted_server<W: std::io::Write>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::SMSG_CHAR_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_ENUM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_DELETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_encrypted_server(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::SMSG_CHAR_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_ENUM(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_DELETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_unencrypted_server(w),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_encrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_encrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_unencrypted_server(w).await,
        }
    }

}

impl std::fmt::Display for ServerOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServerOpcodeMessage::SMSG_CHAR_CREATE(_) => "SMSG_CHAR_CREATE",
            ServerOpcodeMessage::SMSG_CHAR_ENUM(_) => "SMSG_CHAR_ENUM",
            ServerOpcodeMessage::SMSG_CHAR_DELETE(_) => "SMSG_CHAR_DELETE",
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(_) => "SMSG_TUTORIAL_FLAGS",
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => "SMSG_AUTH_CHALLENGE",
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(_) => "SMSG_ACCOUNT_DATA_TIMES",
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(_) => "SMSG_LOGIN_VERIFY_WORLD",
        })
    }
}

impl From<SMSG_CHAR_CREATE> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_CREATE) -> Self {
        Self::SMSG_CHAR_CREATE(c)
    }
}

impl From<SMSG_CHAR_ENUM> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_ENUM) -> Self {
        Self::SMSG_CHAR_ENUM(c)
    }
}

impl From<SMSG_CHAR_DELETE> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_DELETE) -> Self {
        Self::SMSG_CHAR_DELETE(c)
    }
}

impl From<SMSG_TUTORIAL_FLAGS> for ServerOpcodeMessage {
    fn from(c: SMSG_TUTORIAL_FLAGS) -> Self {
        Self::SMSG_TUTORIAL_FLAGS(c)
    }
}

impl From<SMSG_AUTH_CHALLENGE> for ServerOpcodeMessage {
    fn from(c: SMSG_AUTH_CHALLENGE) -> Self {
        Self::SMSG_AUTH_CHALLENGE(c)
    }
}

impl From<SMSG_ACCOUNT_DATA_TIMES> for ServerOpcodeMessage {
    fn from(c: SMSG_ACCOUNT_DATA_TIMES) -> Self {
        Self::SMSG_ACCOUNT_DATA_TIMES(c)
    }
}

impl From<SMSG_LOGIN_VERIFY_WORLD> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGIN_VERIFY_WORLD) -> Self {
        Self::SMSG_LOGIN_VERIFY_WORLD(c)
    }
}

