#![allow(dead_code)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::util::read_u8_le;
use std::fmt::{Display, Formatter};
use std::io::{Error, Read, Write};

use wow_srp::header_crypto::{Decrypter, Encrypter};

pub mod helper;
pub(crate) mod util;
mod world;

pub use world::*;

pub use helper::aura_mask::AuraMask;
pub use helper::guid::Guid;
pub use helper::update_mask::UpdateMask;

#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub const DEFAULT_PORT: u16 = 8085;

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait ServerMessageWrite: MessageBody {
    const OPCODE_LENGTH: u16 = 2;

    fn write_unencrypted_server<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        crate::util::write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
        )?;
        crate::util::write_u16_le(w, <Self as MessageBody>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        e.write_encrypted_server_header(
            w,
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
            <Self as MessageBody>::OPCODE,
        )?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_unencrypted_server<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        crate::util::tokio_write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
        )
        .await?;
        crate::util::tokio_write_u16_le(w, <Self as MessageBody>::OPCODE).await?;

        self.tokio_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_encrypted_server<W: AsyncWriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let header = e.encrypt_server_header(
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
            <Self as MessageBody>::OPCODE,
        );
        w.write_all(&header);

        self.tokio_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_unencrypted_server<W: WriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        crate::util::astd_write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
        )
        .await?;
        crate::util::astd_write_u16_le(w, <Self as MessageBody>::OPCODE).await?;

        self.astd_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_encrypted_server<W: WriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let header = e.encrypt_server_header(
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
            <Self as MessageBody>::OPCODE,
        );
        w.write_all(&header).await;

        self.astd_write_body(w).await?;
        Ok(())
    }
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait ClientMessageWrite: MessageBody {
    const OPCODE_LENGTH: u16 = 4;

    fn write_unencrypted_client<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        crate::util::write_u16_be(
            w,
            (self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH),
        )?;
        crate::util::write_u32_le(w, <Self as MessageBody>::OPCODE as u32)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        e.write_encrypted_client_header(
            w,
            (self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH),
            <Self as MessageBody>::OPCODE as u32,
        )?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_unencrypted_client<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        crate::util::tokio_write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
        )
        .await?;
        crate::util::tokio_write_u32_le(w, <Self as MessageBody>::OPCODE as u32).await?;

        self.tokio_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_encrypted_client<W: AsyncWriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let header = e.encrypt_client_header(
            self.size_without_size_or_opcode_fields() + 4,
            <Self as MessageBody>::OPCODE as u32,
        );
        w.write_all(&header);

        self.tokio_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_unencrypted_client<W: WriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        crate::util::astd_write_u16_be(w, self.size_without_size_or_opcode_fields() + 4).await?;
        crate::util::astd_write_u32_le(w, <Self as MessageBody>::OPCODE as u32).await?;

        self.astd_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_encrypted_client<W: WriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let header = e.encrypt_client_header(
            self.size_without_size_or_opcode_fields() + 4,
            <Self as MessageBody>::OPCODE as u32,
        );
        w.write_all(&header).await;

        self.astd_write_body(w).await?;
        Ok(())
    }
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait MessageBody: Sized {
    const OPCODE: u16;

    fn size_without_size_or_opcode_fields(&self) -> u16;

    type Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> Result<Self, Self::Error>;

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(
        r: &mut R,
        body_size: u32,
    ) -> Result<Self, Self::Error>;
    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(
        r: &mut R,
        body_size: u32,
    ) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait OpcodeMessage: Sized {
    type Error;

    fn read_unencrypted<R: std::io::Read>(r: &mut R) -> Result<Self, Self::Error>;

    fn write_unencrypted<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    fn read_encrypted<R: std::io::Read, D: Decrypter>(
        r: &mut R,
        d: &mut D,
    ) -> std::result::Result<Self, Self::Error>;

    fn write_encrypted<W: std::io::Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> std::result::Result<(), std::io::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_unencrypted<R: AsyncReadExt + Unpin + Send>(
        r: &mut R,
    ) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_unencrypted<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_encrypted<R: AsyncReadExt + Unpin + Send, D: Decrypter + Send>(
        r: &mut R,
        d: &mut D,
    ) -> std::result::Result<Self, Self::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_encrypted<W: AsyncWriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> std::result::Result<(), std::io::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_read_unencrypted<R: ReadExt + Unpin + Send>(
        r: &mut R,
    ) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_write_unencrypted<W: WriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_read_encrypted<R: ReadExt + Unpin + Send, D: Decrypter + Send>(
        r: &mut R,
        d: &mut D,
    ) -> std::result::Result<Self, Self::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_write_encrypted<W: WriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> std::result::Result<(), std::io::Error>;
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait ReadableAndWritable: Sized {
    type Error;
    fn read<R: std::io::Read>(r: &mut R) -> Result<Self, Self::Error>;
    fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error>;
    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W)
        -> Result<(), std::io::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;
}

pub trait ConstantSized: MaximumPossibleSized {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

pub trait VariableSized: MaximumPossibleSized {
    fn size(&self) -> usize;
}

pub trait MaximumPossibleSized {
    fn maximum_possible_size() -> usize;
}
