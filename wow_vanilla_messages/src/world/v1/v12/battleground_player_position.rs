use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct BattlegroundPlayerPosition {
    pub player: Guid,
    pub position_x: f32,
    pub position_y: f32,
}

impl ReadableAndWritable for BattlegroundPlayerPosition {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
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
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: Guid
        self.player.write(w)?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
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
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // player: Guid
            self.player.tokio_write(w).await?;

            // position_x: f32
            w.write_all(&self.position_x.to_le_bytes()).await?;

            // position_y: f32
            w.write_all(&self.position_y.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
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
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // player: Guid
            self.player.astd_write(w).await?;

            // position_x: f32
            w.write_all(&self.position_x.to_le_bytes()).await?;

            // position_y: f32
            w.write_all(&self.position_y.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for BattlegroundPlayerPosition {}

impl MaximumPossibleSized for BattlegroundPlayerPosition {
    fn maximum_possible_size() -> usize {
        0
        + 8 // player: Guid
        + 4 // position_x: f32
        + 4 // position_y: f32
    }
}

