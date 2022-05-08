use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PvpRank, PvpRankError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PVP_CREDIT {
    pub honor_points: u32,
    pub victim: Guid,
    pub rank: PvpRank,
}

impl ServerMessageWrite for SMSG_PVP_CREDIT {}

impl MessageBody for SMSG_PVP_CREDIT {
    const OPCODE: u16 = 0x028c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PVP_CREDITError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // honor_points: u32
        let honor_points = crate::util::read_u32_le(r)?;

        // victim: Guid
        let victim = Guid::read(r)?;

        // rank: PvpRank
        let rank = PvpRank::read_u32_le(r)?;

        Ok(Self {
            honor_points,
            victim,
            rank,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // honor_points: u32
        w.write_all(&self.honor_points.to_le_bytes())?;

        // victim: Guid
        self.victim.write(w)?;

        // rank: PvpRank
        self.rank.write_u32_le(w)?;

        Ok(())
    }

    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // honor_points: u32
            let honor_points = crate::util::tokio_read_u32_le(r).await?;

            // victim: Guid
            let victim = Guid::tokio_read(r).await?;

            // rank: PvpRank
            let rank = PvpRank::tokio_read_u32_le(r).await?;

            Ok(Self {
                honor_points,
                victim,
                rank,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
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
            // honor_points: u32
            w.write_all(&self.honor_points.to_le_bytes()).await?;

            // victim: Guid
            self.victim.tokio_write(w).await?;

            // rank: PvpRank
            self.rank.tokio_write_u32_le(w).await?;

            Ok(())
        })
    }

    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // honor_points: u32
            let honor_points = crate::util::astd_read_u32_le(r).await?;

            // victim: Guid
            let victim = Guid::astd_read(r).await?;

            // rank: PvpRank
            let rank = PvpRank::astd_read_u32_le(r).await?;

            Ok(Self {
                honor_points,
                victim,
                rank,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
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
            // honor_points: u32
            w.write_all(&self.honor_points.to_le_bytes()).await?;

            // victim: Guid
            self.victim.astd_write(w).await?;

            // rank: PvpRank
            self.rank.astd_write_u32_le(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_PVP_CREDIT {}

impl MaximumPossibleSized for SMSG_PVP_CREDIT {
    fn maximum_possible_size() -> usize {
        0
        + 4 // honor_points: u32
        + 8 // victim: Guid
        + 1 // rank: PvpRank
    }
}

#[derive(Debug)]
pub enum SMSG_PVP_CREDITError {
    Io(std::io::Error),
    PvpRank(PvpRankError),
}

impl std::error::Error for SMSG_PVP_CREDITError {}
impl std::fmt::Display for SMSG_PVP_CREDITError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PvpRank(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PVP_CREDITError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PvpRankError> for SMSG_PVP_CREDITError {
    fn from(e: PvpRankError) -> Self {
        Self::PvpRank(e)
    }
}

