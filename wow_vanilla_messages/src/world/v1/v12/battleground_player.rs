use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PvpRank, PvpRankError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct BattlegroundPlayer {
    pub player: Guid,
    pub rank: PvpRank,
    pub killing_blows: u32,
    pub honorable_kills: u32,
    pub deaths: u32,
    pub bonus_honor: u32,
    pub fields: Vec<u32>,
}

impl BattlegroundPlayer {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundPlayerError> {
        // player: Guid
        let player = Guid::read(r)?;

        // rank: PvpRank
        let rank: PvpRank = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // killing_blows: u32
        let killing_blows = crate::util::read_u32_le(r)?;

        // honorable_kills: u32
        let honorable_kills = crate::util::read_u32_le(r)?;

        // deaths: u32
        let deaths = crate::util::read_u32_le(r)?;

        // bonus_honor: u32
        let bonus_honor = crate::util::read_u32_le(r)?;

        // amount_of_extra_fields: u32
        let amount_of_extra_fields = crate::util::read_u32_le(r)?;

        // fields: u32[amount_of_extra_fields]
        let mut fields = Vec::with_capacity(amount_of_extra_fields as usize);
        for i in 0..amount_of_extra_fields {
            fields.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            player,
            rank,
            killing_blows,
            honorable_kills,
            deaths,
            bonus_honor,
            fields,
        })
    }

    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: Guid
        self.player.write(w)?;

        // rank: PvpRank
        crate::util::write_u32_le(w, self.rank.as_int() as u32)?;

        // killing_blows: u32
        w.write_all(&self.killing_blows.to_le_bytes())?;

        // honorable_kills: u32
        w.write_all(&self.honorable_kills.to_le_bytes())?;

        // deaths: u32
        w.write_all(&self.deaths.to_le_bytes())?;

        // bonus_honor: u32
        w.write_all(&self.bonus_honor.to_le_bytes())?;

        // amount_of_extra_fields: u32
        w.write_all(&(self.fields.len() as u32).to_le_bytes())?;

        // fields: u32[amount_of_extra_fields]
        for i in self.fields.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, BattlegroundPlayerError> {
        // player: Guid
        let player = Guid::tokio_read(r).await?;

        // rank: PvpRank
        let rank: PvpRank = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

        // killing_blows: u32
        let killing_blows = crate::util::tokio_read_u32_le(r).await?;

        // honorable_kills: u32
        let honorable_kills = crate::util::tokio_read_u32_le(r).await?;

        // deaths: u32
        let deaths = crate::util::tokio_read_u32_le(r).await?;

        // bonus_honor: u32
        let bonus_honor = crate::util::tokio_read_u32_le(r).await?;

        // amount_of_extra_fields: u32
        let amount_of_extra_fields = crate::util::tokio_read_u32_le(r).await?;

        // fields: u32[amount_of_extra_fields]
        let mut fields = Vec::with_capacity(amount_of_extra_fields as usize);
        for i in 0..amount_of_extra_fields {
            fields.push(crate::util::tokio_read_u32_le(r).await?);
        }

        Ok(Self {
            player,
            rank,
            killing_blows,
            honorable_kills,
            deaths,
            bonus_honor,
            fields,
        })
    }

    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: Guid
        self.player.tokio_write(w).await?;

        // rank: PvpRank
        crate::util::tokio_write_u32_le(w, self.rank.as_int() as u32).await?;

        // killing_blows: u32
        w.write_all(&self.killing_blows.to_le_bytes()).await?;

        // honorable_kills: u32
        w.write_all(&self.honorable_kills.to_le_bytes()).await?;

        // deaths: u32
        w.write_all(&self.deaths.to_le_bytes()).await?;

        // bonus_honor: u32
        w.write_all(&self.bonus_honor.to_le_bytes()).await?;

        // amount_of_extra_fields: u32
        w.write_all(&(self.fields.len() as u32).to_le_bytes()).await?;

        // fields: u32[amount_of_extra_fields]
        for i in self.fields.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, BattlegroundPlayerError> {
        // player: Guid
        let player = Guid::astd_read(r).await?;

        // rank: PvpRank
        let rank: PvpRank = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

        // killing_blows: u32
        let killing_blows = crate::util::astd_read_u32_le(r).await?;

        // honorable_kills: u32
        let honorable_kills = crate::util::astd_read_u32_le(r).await?;

        // deaths: u32
        let deaths = crate::util::astd_read_u32_le(r).await?;

        // bonus_honor: u32
        let bonus_honor = crate::util::astd_read_u32_le(r).await?;

        // amount_of_extra_fields: u32
        let amount_of_extra_fields = crate::util::astd_read_u32_le(r).await?;

        // fields: u32[amount_of_extra_fields]
        let mut fields = Vec::with_capacity(amount_of_extra_fields as usize);
        for i in 0..amount_of_extra_fields {
            fields.push(crate::util::astd_read_u32_le(r).await?);
        }

        Ok(Self {
            player,
            rank,
            killing_blows,
            honorable_kills,
            deaths,
            bonus_honor,
            fields,
        })
    }

    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: Guid
        self.player.astd_write(w).await?;

        // rank: PvpRank
        crate::util::astd_write_u32_le(w, self.rank.as_int() as u32).await?;

        // killing_blows: u32
        w.write_all(&self.killing_blows.to_le_bytes()).await?;

        // honorable_kills: u32
        w.write_all(&self.honorable_kills.to_le_bytes()).await?;

        // deaths: u32
        w.write_all(&self.deaths.to_le_bytes()).await?;

        // bonus_honor: u32
        w.write_all(&self.bonus_honor.to_le_bytes()).await?;

        // amount_of_extra_fields: u32
        w.write_all(&(self.fields.len() as u32).to_le_bytes()).await?;

        // fields: u32[amount_of_extra_fields]
        for i in self.fields.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

}

impl VariableSized for BattlegroundPlayer {
    fn size(&self) -> usize {
        0
        + 8 // player: Guid
        + 4 // rank: PvpRank
        + 4 // killing_blows: u32
        + 4 // honorable_kills: u32
        + 4 // deaths: u32
        + 4 // bonus_honor: u32
        + 4 // amount_of_extra_fields: u32
        + self.fields.len() * core::mem::size_of::<u32>() // fields: u32[amount_of_extra_fields]
    }
}

impl MaximumPossibleSized for BattlegroundPlayer {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[derive(Debug)]
pub enum BattlegroundPlayerError {
    Io(std::io::Error),
    PvpRank(PvpRankError),
}

impl std::error::Error for BattlegroundPlayerError {}
impl std::fmt::Display for BattlegroundPlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PvpRank(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for BattlegroundPlayerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PvpRankError> for BattlegroundPlayerError {
    fn from(e: PvpRankError) -> Self {
        Self::PvpRank(e)
    }
}

