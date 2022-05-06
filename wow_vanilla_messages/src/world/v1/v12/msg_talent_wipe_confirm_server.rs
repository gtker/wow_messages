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
#[derive(Copy)]
pub struct MSG_TALENT_WIPE_CONFIRM_Server {
    pub wiping_npc: Guid,
    pub cost_in_copper: u32,
}

impl ServerMessageWrite for MSG_TALENT_WIPE_CONFIRM_Server {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for MSG_TALENT_WIPE_CONFIRM_Server {
    const OPCODE: u16 = 0x02aa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // wiping_npc: Guid
        let wiping_npc = Guid::read(r)?;

        // cost_in_copper: u32
        let cost_in_copper = crate::util::read_u32_le(r)?;

        Ok(Self {
            wiping_npc,
            cost_in_copper,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // wiping_npc: Guid
        self.wiping_npc.write(w)?;

        // cost_in_copper: u32
        w.write_all(&self.cost_in_copper.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // wiping_npc: Guid
        let wiping_npc = Guid::tokio_read(r).await?;

        // cost_in_copper: u32
        let cost_in_copper = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            wiping_npc,
            cost_in_copper,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // wiping_npc: Guid
        self.wiping_npc.tokio_write(w).await?;

        // cost_in_copper: u32
        w.write_all(&self.cost_in_copper.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // wiping_npc: Guid
        let wiping_npc = Guid::astd_read(r).await?;

        // cost_in_copper: u32
        let cost_in_copper = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            wiping_npc,
            cost_in_copper,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // wiping_npc: Guid
        self.wiping_npc.astd_write(w).await?;

        // cost_in_copper: u32
        w.write_all(&self.cost_in_copper.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for MSG_TALENT_WIPE_CONFIRM_Server {}

impl MaximumPossibleSized for MSG_TALENT_WIPE_CONFIRM_Server {
    fn maximum_possible_size() -> usize {
        0
        + 8 // wiping_npc: Guid
        + 4 // cost_in_copper: u32
    }
}

