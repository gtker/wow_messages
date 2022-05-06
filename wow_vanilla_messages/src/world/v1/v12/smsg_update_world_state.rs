use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::WorldState;
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
pub struct SMSG_UPDATE_WORLD_STATE {
    pub state: WorldState,
}

impl ServerMessageWrite for SMSG_UPDATE_WORLD_STATE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_UPDATE_WORLD_STATE {
    const OPCODE: u16 = 0x02c3;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // state: WorldState
        let state = WorldState::read(r)?;

        Ok(Self {
            state,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: WorldState
        self.state.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // state: WorldState
        let state = WorldState::tokio_read(r).await?;

        Ok(Self {
            state,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: WorldState
        self.state.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // state: WorldState
        let state = WorldState::astd_read(r).await?;

        Ok(Self {
            state,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: WorldState
        self.state.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_UPDATE_WORLD_STATE {}

impl MaximumPossibleSized for SMSG_UPDATE_WORLD_STATE {
    fn maximum_possible_size() -> usize {
        0
        + 8 // state: WorldState
    }
}

