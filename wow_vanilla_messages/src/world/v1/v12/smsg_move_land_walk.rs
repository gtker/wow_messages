use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_MOVE_LAND_WALK {
    pub guid: Guid,
    pub counter: u32,
}

impl ServerMessageWrite for SMSG_MOVE_LAND_WALK {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_MOVE_LAND_WALK {
    const OPCODE: u16 = 0x00df;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            counter,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::tokio_read_packed(r).await?;

        // counter: u32
        let counter = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            guid,
            counter,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.tokio_write_packed(w).await?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::astd_read_packed(r).await?;

        // counter: u32
        let counter = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            guid,
            counter,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.astd_write_packed(w).await?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes()).await?;

        Ok(())
    }
}

impl VariableSized for SMSG_MOVE_LAND_WALK {
    fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 4 // counter: u32
    }
}

impl MaximumPossibleSized for SMSG_MOVE_LAND_WALK {
    fn maximum_possible_size() -> usize {
        9 // guid: PackedGuid
        + 4 // counter: u32
    }
}

