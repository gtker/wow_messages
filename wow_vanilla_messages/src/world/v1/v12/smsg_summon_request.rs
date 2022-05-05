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
pub struct SMSG_SUMMON_REQUEST {
    pub summoner_guid: Guid,
    pub zone_id: u32,
    pub auto_decline_time_in_msecs: u32,
}

impl ServerMessageWrite for SMSG_SUMMON_REQUEST {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_SUMMON_REQUEST {
    const OPCODE: u16 = 0x02ab;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // summoner_guid: Guid
        let summoner_guid = Guid::read(r)?;

        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        // auto_decline_time_in_msecs: u32
        let auto_decline_time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            summoner_guid,
            zone_id,
            auto_decline_time_in_msecs,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // summoner_guid: Guid
        self.summoner_guid.write(w)?;

        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        // auto_decline_time_in_msecs: u32
        w.write_all(&self.auto_decline_time_in_msecs.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // summoner_guid: Guid
        let summoner_guid = Guid::tokio_read(r).await?;

        // zone_id: u32
        let zone_id = crate::util::tokio_read_u32_le(r).await?;

        // auto_decline_time_in_msecs: u32
        let auto_decline_time_in_msecs = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            summoner_guid,
            zone_id,
            auto_decline_time_in_msecs,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // summoner_guid: Guid
        self.summoner_guid.tokio_write(w).await?;

        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes()).await?;

        // auto_decline_time_in_msecs: u32
        w.write_all(&self.auto_decline_time_in_msecs.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // summoner_guid: Guid
        let summoner_guid = Guid::astd_read(r).await?;

        // zone_id: u32
        let zone_id = crate::util::astd_read_u32_le(r).await?;

        // auto_decline_time_in_msecs: u32
        let auto_decline_time_in_msecs = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            summoner_guid,
            zone_id,
            auto_decline_time_in_msecs,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // summoner_guid: Guid
        self.summoner_guid.astd_write(w).await?;

        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes()).await?;

        // auto_decline_time_in_msecs: u32
        w.write_all(&self.auto_decline_time_in_msecs.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_SUMMON_REQUEST {}

impl MaximumPossibleSized for SMSG_SUMMON_REQUEST {
    fn maximum_possible_size() -> usize {
        8 // summoner_guid: Guid
        + 4 // zone_id: u32
        + 4 // auto_decline_time_in_msecs: u32
    }
}

