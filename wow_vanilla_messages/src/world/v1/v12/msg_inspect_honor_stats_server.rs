use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PvpRank, PvpRankError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_INSPECT_HONOR_STATS_Server {
    pub guid: Guid,
    pub highest_rank: PvpRank,
    pub today_honorable_and_dishonorable: u32,
    pub yesterday_honorable: u16,
    pub unknown1: u16,
    pub last_week_honorable: u16,
    pub unknown2: u16,
    pub this_week_honorable: u16,
    pub unknown3: u16,
    pub lifetime_honorable: u32,
    pub lifetime_dishonorable: u32,
    pub yesterday_honor: u32,
    pub last_week_honor: u32,
    pub this_week_honor: u32,
    pub last_week_standing: PvpRank,
    pub rank_progress_bar: u8,
}

impl ServerMessageWrite for MSG_INSPECT_HONOR_STATS_Server {}

impl MessageBody for MSG_INSPECT_HONOR_STATS_Server {
    const OPCODE: u16 = 0x02d6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = MSG_INSPECT_HONOR_STATS_ServerError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // highest_rank: PvpRank
        let highest_rank: PvpRank = crate::util::read_u8_le(r)?.try_into()?;

        // today_honorable_and_dishonorable: u32
        let today_honorable_and_dishonorable = crate::util::read_u32_le(r)?;

        // yesterday_honorable: u16
        let yesterday_honorable = crate::util::read_u16_le(r)?;

        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(r)?;

        // last_week_honorable: u16
        let last_week_honorable = crate::util::read_u16_le(r)?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(r)?;

        // this_week_honorable: u16
        let this_week_honorable = crate::util::read_u16_le(r)?;

        // unknown3: u16
        let unknown3 = crate::util::read_u16_le(r)?;

        // lifetime_honorable: u32
        let lifetime_honorable = crate::util::read_u32_le(r)?;

        // lifetime_dishonorable: u32
        let lifetime_dishonorable = crate::util::read_u32_le(r)?;

        // yesterday_honor: u32
        let yesterday_honor = crate::util::read_u32_le(r)?;

        // last_week_honor: u32
        let last_week_honor = crate::util::read_u32_le(r)?;

        // this_week_honor: u32
        let this_week_honor = crate::util::read_u32_le(r)?;

        // last_week_standing: PvpRank
        let last_week_standing: PvpRank = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // rank_progress_bar: u8
        let rank_progress_bar = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            highest_rank,
            today_honorable_and_dishonorable,
            yesterday_honorable,
            unknown1,
            last_week_honorable,
            unknown2,
            this_week_honorable,
            unknown3,
            lifetime_honorable,
            lifetime_dishonorable,
            yesterday_honor,
            last_week_honor,
            this_week_honor,
            last_week_standing,
            rank_progress_bar,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // highest_rank: PvpRank
        w.write_all(&(self.highest_rank.as_int() as u8).to_le_bytes())?;

        // today_honorable_and_dishonorable: u32
        w.write_all(&self.today_honorable_and_dishonorable.to_le_bytes())?;

        // yesterday_honorable: u16
        w.write_all(&self.yesterday_honorable.to_le_bytes())?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        // last_week_honorable: u16
        w.write_all(&self.last_week_honorable.to_le_bytes())?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

        // this_week_honorable: u16
        w.write_all(&self.this_week_honorable.to_le_bytes())?;

        // unknown3: u16
        w.write_all(&self.unknown3.to_le_bytes())?;

        // lifetime_honorable: u32
        w.write_all(&self.lifetime_honorable.to_le_bytes())?;

        // lifetime_dishonorable: u32
        w.write_all(&self.lifetime_dishonorable.to_le_bytes())?;

        // yesterday_honor: u32
        w.write_all(&self.yesterday_honor.to_le_bytes())?;

        // last_week_honor: u32
        w.write_all(&self.last_week_honor.to_le_bytes())?;

        // this_week_honor: u32
        w.write_all(&self.this_week_honor.to_le_bytes())?;

        // last_week_standing: PvpRank
        w.write_all(&(self.last_week_standing.as_int() as u32).to_le_bytes())?;

        // rank_progress_bar: u8
        w.write_all(&self.rank_progress_bar.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // highest_rank: PvpRank
            let highest_rank: PvpRank = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // today_honorable_and_dishonorable: u32
            let today_honorable_and_dishonorable = crate::util::tokio_read_u32_le(r).await?;

            // yesterday_honorable: u16
            let yesterday_honorable = crate::util::tokio_read_u16_le(r).await?;

            // unknown1: u16
            let unknown1 = crate::util::tokio_read_u16_le(r).await?;

            // last_week_honorable: u16
            let last_week_honorable = crate::util::tokio_read_u16_le(r).await?;

            // unknown2: u16
            let unknown2 = crate::util::tokio_read_u16_le(r).await?;

            // this_week_honorable: u16
            let this_week_honorable = crate::util::tokio_read_u16_le(r).await?;

            // unknown3: u16
            let unknown3 = crate::util::tokio_read_u16_le(r).await?;

            // lifetime_honorable: u32
            let lifetime_honorable = crate::util::tokio_read_u32_le(r).await?;

            // lifetime_dishonorable: u32
            let lifetime_dishonorable = crate::util::tokio_read_u32_le(r).await?;

            // yesterday_honor: u32
            let yesterday_honor = crate::util::tokio_read_u32_le(r).await?;

            // last_week_honor: u32
            let last_week_honor = crate::util::tokio_read_u32_le(r).await?;

            // this_week_honor: u32
            let this_week_honor = crate::util::tokio_read_u32_le(r).await?;

            // last_week_standing: PvpRank
            let last_week_standing: PvpRank = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            // rank_progress_bar: u8
            let rank_progress_bar = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                guid,
                highest_rank,
                today_honorable_and_dishonorable,
                yesterday_honorable,
                unknown1,
                last_week_honorable,
                unknown2,
                this_week_honorable,
                unknown3,
                lifetime_honorable,
                lifetime_dishonorable,
                yesterday_honor,
                last_week_honor,
                this_week_honor,
                last_week_standing,
                rank_progress_bar,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // highest_rank: PvpRank
            w.write_all(&(self.highest_rank.as_int() as u8).to_le_bytes()).await?;

            // today_honorable_and_dishonorable: u32
            w.write_all(&self.today_honorable_and_dishonorable.to_le_bytes()).await?;

            // yesterday_honorable: u16
            w.write_all(&self.yesterday_honorable.to_le_bytes()).await?;

            // unknown1: u16
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // last_week_honorable: u16
            w.write_all(&self.last_week_honorable.to_le_bytes()).await?;

            // unknown2: u16
            w.write_all(&self.unknown2.to_le_bytes()).await?;

            // this_week_honorable: u16
            w.write_all(&self.this_week_honorable.to_le_bytes()).await?;

            // unknown3: u16
            w.write_all(&self.unknown3.to_le_bytes()).await?;

            // lifetime_honorable: u32
            w.write_all(&self.lifetime_honorable.to_le_bytes()).await?;

            // lifetime_dishonorable: u32
            w.write_all(&self.lifetime_dishonorable.to_le_bytes()).await?;

            // yesterday_honor: u32
            w.write_all(&self.yesterday_honor.to_le_bytes()).await?;

            // last_week_honor: u32
            w.write_all(&self.last_week_honor.to_le_bytes()).await?;

            // this_week_honor: u32
            w.write_all(&self.this_week_honor.to_le_bytes()).await?;

            // last_week_standing: PvpRank
            w.write_all(&(self.last_week_standing.as_int() as u32).to_le_bytes()).await?;

            // rank_progress_bar: u8
            w.write_all(&self.rank_progress_bar.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // highest_rank: PvpRank
            let highest_rank: PvpRank = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // today_honorable_and_dishonorable: u32
            let today_honorable_and_dishonorable = crate::util::astd_read_u32_le(r).await?;

            // yesterday_honorable: u16
            let yesterday_honorable = crate::util::astd_read_u16_le(r).await?;

            // unknown1: u16
            let unknown1 = crate::util::astd_read_u16_le(r).await?;

            // last_week_honorable: u16
            let last_week_honorable = crate::util::astd_read_u16_le(r).await?;

            // unknown2: u16
            let unknown2 = crate::util::astd_read_u16_le(r).await?;

            // this_week_honorable: u16
            let this_week_honorable = crate::util::astd_read_u16_le(r).await?;

            // unknown3: u16
            let unknown3 = crate::util::astd_read_u16_le(r).await?;

            // lifetime_honorable: u32
            let lifetime_honorable = crate::util::astd_read_u32_le(r).await?;

            // lifetime_dishonorable: u32
            let lifetime_dishonorable = crate::util::astd_read_u32_le(r).await?;

            // yesterday_honor: u32
            let yesterday_honor = crate::util::astd_read_u32_le(r).await?;

            // last_week_honor: u32
            let last_week_honor = crate::util::astd_read_u32_le(r).await?;

            // this_week_honor: u32
            let this_week_honor = crate::util::astd_read_u32_le(r).await?;

            // last_week_standing: PvpRank
            let last_week_standing: PvpRank = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            // rank_progress_bar: u8
            let rank_progress_bar = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                guid,
                highest_rank,
                today_honorable_and_dishonorable,
                yesterday_honorable,
                unknown1,
                last_week_honorable,
                unknown2,
                this_week_honorable,
                unknown3,
                lifetime_honorable,
                lifetime_dishonorable,
                yesterday_honor,
                last_week_honor,
                this_week_honor,
                last_week_standing,
                rank_progress_bar,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // guid: Guid
            self.guid.astd_write(w).await?;

            // highest_rank: PvpRank
            w.write_all(&(self.highest_rank.as_int() as u8).to_le_bytes()).await?;

            // today_honorable_and_dishonorable: u32
            w.write_all(&self.today_honorable_and_dishonorable.to_le_bytes()).await?;

            // yesterday_honorable: u16
            w.write_all(&self.yesterday_honorable.to_le_bytes()).await?;

            // unknown1: u16
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // last_week_honorable: u16
            w.write_all(&self.last_week_honorable.to_le_bytes()).await?;

            // unknown2: u16
            w.write_all(&self.unknown2.to_le_bytes()).await?;

            // this_week_honorable: u16
            w.write_all(&self.this_week_honorable.to_le_bytes()).await?;

            // unknown3: u16
            w.write_all(&self.unknown3.to_le_bytes()).await?;

            // lifetime_honorable: u32
            w.write_all(&self.lifetime_honorable.to_le_bytes()).await?;

            // lifetime_dishonorable: u32
            w.write_all(&self.lifetime_dishonorable.to_le_bytes()).await?;

            // yesterday_honor: u32
            w.write_all(&self.yesterday_honor.to_le_bytes()).await?;

            // last_week_honor: u32
            w.write_all(&self.last_week_honor.to_le_bytes()).await?;

            // this_week_honor: u32
            w.write_all(&self.this_week_honor.to_le_bytes()).await?;

            // last_week_standing: PvpRank
            w.write_all(&(self.last_week_standing.as_int() as u32).to_le_bytes()).await?;

            // rank_progress_bar: u8
            w.write_all(&self.rank_progress_bar.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl MSG_INSPECT_HONOR_STATS_Server {
    pub(crate) fn size() -> usize {
        0
        + 8 // guid: Guid
        + 1 // highest_rank: PvpRank
        + 4 // today_honorable_and_dishonorable: u32
        + 2 // yesterday_honorable: u16
        + 2 // unknown1: u16
        + 2 // last_week_honorable: u16
        + 2 // unknown2: u16
        + 2 // this_week_honorable: u16
        + 2 // unknown3: u16
        + 4 // lifetime_honorable: u32
        + 4 // lifetime_dishonorable: u32
        + 4 // yesterday_honor: u32
        + 4 // last_week_honor: u32
        + 4 // this_week_honor: u32
        + 1 // last_week_standing: PvpRank
        + 1 // rank_progress_bar: u8
    }
}

#[derive(Debug)]
pub enum MSG_INSPECT_HONOR_STATS_ServerError {
    Io(std::io::Error),
    PvpRank(PvpRankError),
}

impl std::error::Error for MSG_INSPECT_HONOR_STATS_ServerError {}
impl std::fmt::Display for MSG_INSPECT_HONOR_STATS_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PvpRank(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_INSPECT_HONOR_STATS_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PvpRankError> for MSG_INSPECT_HONOR_STATS_ServerError {
    fn from(e: PvpRankError) -> Self {
        Self::PvpRank(e)
    }
}

