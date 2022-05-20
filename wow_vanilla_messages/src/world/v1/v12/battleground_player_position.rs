use std::convert::{TryFrom, TryInto};
use crate::Guid;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct BattlegroundPlayerPosition {
    pub player: Guid,
    pub position_x: f32,
    pub position_y: f32,
}

impl BattlegroundPlayerPosition {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // player: Guid
        let player = Guid::read(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        Ok(Self {
            player,
            position_x,
            position_y,
        })
    }

    #[cfg(feature = "sync")]
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // player: Guid
        let player = Guid::tokio_read(r).await?;

        // position_x: f32
        let position_x = crate::util::tokio_read_f32_le(r).await?;
        // position_y: f32
        let position_y = crate::util::tokio_read_f32_le(r).await?;
        Ok(Self {
            player,
            position_x,
            position_y,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes()).await?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes()).await?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // player: Guid
        let player = Guid::astd_read(r).await?;

        // position_x: f32
        let position_x = crate::util::astd_read_f32_le(r).await?;
        // position_y: f32
        let position_y = crate::util::astd_read_f32_le(r).await?;
        Ok(Self {
            player,
            position_x,
            position_y,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes()).await?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes()).await?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes()).await?;

        Ok(())
    }

}

impl BattlegroundPlayerPosition {
    pub(crate) fn size() -> usize {
        0
        + 8 // player: Guid
        + 4 // position_x: f32
        + 4 // position_y: f32
    }
}

