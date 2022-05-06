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
pub struct SMSG_ATTACKSTOP {
    pub player: Guid,
    pub enemy: Guid,
    pub unknown1: u32,
}

impl ServerMessageWrite for SMSG_ATTACKSTOP {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_ATTACKSTOP {
    const OPCODE: u16 = 0x0144;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // enemy: PackedGuid
        let enemy = Guid::read_packed(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            player,
            enemy,
            unknown1,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed(w)?;

        // enemy: PackedGuid
        self.enemy.write_packed(w)?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player: PackedGuid
        let player = Guid::tokio_read_packed(r).await?;

        // enemy: PackedGuid
        let enemy = Guid::tokio_read_packed(r).await?;

        // unknown1: u32
        let unknown1 = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            player,
            enemy,
            unknown1,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.tokio_write_packed(w).await?;

        // enemy: PackedGuid
        self.enemy.tokio_write_packed(w).await?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player: PackedGuid
        let player = Guid::astd_read_packed(r).await?;

        // enemy: PackedGuid
        let enemy = Guid::astd_read_packed(r).await?;

        // unknown1: u32
        let unknown1 = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            player,
            enemy,
            unknown1,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.astd_write_packed(w).await?;

        // enemy: PackedGuid
        self.enemy.astd_write_packed(w).await?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes()).await?;

        Ok(())
    }

}

impl VariableSized for SMSG_ATTACKSTOP {
    fn size(&self) -> usize {
        0
        + self.player.size() // player: Guid
        + self.enemy.size() // enemy: Guid
        + 4 // unknown1: u32
    }
}

impl MaximumPossibleSized for SMSG_ATTACKSTOP {
    fn maximum_possible_size() -> usize {
        0
        + 9 // player: Guid
        + 9 // enemy: Guid
        + 4 // unknown1: u32
    }
}

