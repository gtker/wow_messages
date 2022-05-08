use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{RaidInfo, RaidInfoError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_RAID_INSTANCE_INFO {
    pub raid_infos: Vec<RaidInfo>,
}

impl ServerMessageWrite for SMSG_RAID_INSTANCE_INFO {}

impl MessageBody for SMSG_RAID_INSTANCE_INFO {
    const OPCODE: u16 = 0x02cc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_RAID_INSTANCE_INFOError;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_raid_infos: u32
        w.write_all(&(self.raid_infos.len() as u32).to_le_bytes())?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        for i in self.raid_infos.iter() {
            i.write(w)?;
        }

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
            // amount_of_raid_infos: u32
            w.write_all(&(self.raid_infos.len() as u32).to_le_bytes()).await?;

            // raid_infos: RaidInfo[amount_of_raid_infos]
            for i in self.raid_infos.iter() {
                i.tokio_write(w).await?;
            }

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
            // amount_of_raid_infos: u32
            w.write_all(&(self.raid_infos.len() as u32).to_le_bytes()).await?;

            // raid_infos: RaidInfo[amount_of_raid_infos]
            for i in self.raid_infos.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_RAID_INSTANCE_INFO {
    fn size(&self) -> usize {
        0
        + 4 // amount_of_raid_infos: u32
        + self.raid_infos.iter().fold(0, |acc, x| acc + RaidInfo::size()) // raid_infos: RaidInfo[amount_of_raid_infos]
    }
}

impl MaximumPossibleSized for SMSG_RAID_INSTANCE_INFO {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
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

