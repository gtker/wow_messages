use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{RaidInfo, RaidInfoError};
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
pub struct SMSG_RAID_INSTANCE_INFO {
    pub raid_infos: Vec<RaidInfo>,
}

impl ServerMessageWrite for SMSG_RAID_INSTANCE_INFO {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_RAID_INSTANCE_INFO {
    const OPCODE: u16 = 0x02cc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_RAID_INSTANCE_INFOError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_raid_infos: u32
        let amount_of_raid_infos = crate::util::read_u32_le(r)?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        let mut raid_infos = Vec::with_capacity(amount_of_raid_infos as usize);
        for i in 0..amount_of_raid_infos {
            raid_infos.push(RaidInfo::read(r)?);
        }

        Ok(Self {
            raid_infos,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_raid_infos: u32
        w.write_all(&(self.raid_infos.len() as u32).to_le_bytes())?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        for i in self.raid_infos.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_raid_infos: u32
        let amount_of_raid_infos = crate::util::tokio_read_u32_le(r).await?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        let mut raid_infos = Vec::with_capacity(amount_of_raid_infos as usize);
        for i in 0..amount_of_raid_infos {
            raid_infos.push(RaidInfo::tokio_read(r).await?);
        }

        Ok(Self {
            raid_infos,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_raid_infos: u32
        w.write_all(&(self.raid_infos.len() as u32).to_le_bytes()).await?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        for i in self.raid_infos.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_raid_infos: u32
        let amount_of_raid_infos = crate::util::astd_read_u32_le(r).await?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        let mut raid_infos = Vec::with_capacity(amount_of_raid_infos as usize);
        for i in 0..amount_of_raid_infos {
            raid_infos.push(RaidInfo::astd_read(r).await?);
        }

        Ok(Self {
            raid_infos,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_raid_infos: u32
        w.write_all(&(self.raid_infos.len() as u32).to_le_bytes()).await?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        for i in self.raid_infos.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_RAID_INSTANCE_INFO {
    fn size(&self) -> usize {
        4 // amount_of_raid_infos: u32
        + self.raid_infos.iter().fold(0, |acc, x| acc + RaidInfo::size()) // raid_infos: RaidInfo[amount_of_raid_infos]
    }
}

impl MaximumPossibleSized for SMSG_RAID_INSTANCE_INFO {
    fn maximum_possible_size() -> usize {
        4 // amount_of_raid_infos: u32
        + 4294967295 * RaidInfo::maximum_possible_size() // raid_infos: RaidInfo[amount_of_raid_infos]
    }
}

#[derive(Debug)]
pub enum SMSG_RAID_INSTANCE_INFOError {
    Io(std::io::Error),
    RaidInfo(RaidInfoError),
}

impl std::error::Error for SMSG_RAID_INSTANCE_INFOError {}
impl std::fmt::Display for SMSG_RAID_INSTANCE_INFOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RaidInfo(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_RAID_INSTANCE_INFOError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RaidInfoError> for SMSG_RAID_INSTANCE_INFOError {
    fn from(e: RaidInfoError) -> Self {
        Self::RaidInfo(e)
    }
}

