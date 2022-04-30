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

pub trait ServerMessageWrite: MessageBody {
    fn write_unencrypted_server<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, self.size_without_size_or_opcode_fields() + 2)?;
        crate::util::write_u16_le(w, <Self as MessageBody>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(
            w,
            self.size_without_size_or_opcode_fields() + 2,
            <Self as MessageBody>::OPCODE,
        )?;

        self.write_body(w)?;
        Ok(())
    }
}

pub trait ClientMessageWrite: MessageBody {
    fn write_unencrypted_client<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size_without_size_or_opcode_fields() + 4))?;
        crate::util::write_u32_le(w, <Self as MessageBody>::OPCODE as u32)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(
            w,
            (self.size_without_size_or_opcode_fields() + 4),
            <Self as MessageBody>::OPCODE as u32,
        )?;

        self.write_body(w)?;
        Ok(())
    }
}

pub trait MessageBody: Sized {
    const OPCODE: u16;

    fn size_without_size_or_opcode_fields(&self) -> u16;

    type Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> Result<Self, Self::Error>;

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;
}

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
