use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_ACTIVATETAXI {
    pub guid: Guid,
    pub nodes: [u32; 2],
}

impl ClientMessageWrite for CMSG_ACTIVATETAXI {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_ACTIVATETAXI {
    const OPCODE: u16 = 0x01ad;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // nodes: u32[2]
        let mut nodes = [u32::default(); 2];
        for i in 0..2 {
            nodes[i] = crate::util::read_u32_le(r)?;
        }

        Ok(Self {
            guid,
            nodes,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // nodes: u32[2]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // nodes: u32[2]
        let mut nodes = [u32::default(); 2];
        for i in 0..2 {
            nodes[i] = crate::util::tokio_read_u32_le(r).await?;
        }

        Ok(Self {
            guid,
            nodes,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // nodes: u32[2]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // nodes: u32[2]
        let mut nodes = [u32::default(); 2];
        for i in 0..2 {
            nodes[i] = crate::util::astd_read_u32_le(r).await?;
        }

        Ok(Self {
            guid,
            nodes,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // nodes: u32[2]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }
}

impl ConstantSized for CMSG_ACTIVATETAXI {}

impl MaximumPossibleSized for CMSG_ACTIVATETAXI {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 2 * core::mem::size_of::<u32>() // nodes: u32[2]
    }
}

