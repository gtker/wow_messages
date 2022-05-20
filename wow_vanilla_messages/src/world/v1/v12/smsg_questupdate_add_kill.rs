use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_QUESTUPDATE_ADD_KILL {
    pub quest_id: u32,
    pub create_id: u32,
    pub kill_count: u32,
    pub required_kill_count: u32,
    pub guid: Guid,
}

impl ServerMessageWrite for SMSG_QUESTUPDATE_ADD_KILL {}

impl MessageBody for SMSG_QUESTUPDATE_ADD_KILL {
    const OPCODE: u16 = 0x0199;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // create_id: u32
        let create_id = crate::util::read_u32_le(r)?;

        // kill_count: u32
        let kill_count = crate::util::read_u32_le(r)?;

        // required_kill_count: u32
        let required_kill_count = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            quest_id,
            create_id,
            kill_count,
            required_kill_count,
            guid,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // create_id: u32
        w.write_all(&self.create_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_kill_count: u32
        w.write_all(&self.required_kill_count.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "tokio")]
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
            // quest_id: u32
            let quest_id = crate::util::tokio_read_u32_le(r).await?;

            // create_id: u32
            let create_id = crate::util::tokio_read_u32_le(r).await?;

            // kill_count: u32
            let kill_count = crate::util::tokio_read_u32_le(r).await?;

            // required_kill_count: u32
            let required_kill_count = crate::util::tokio_read_u32_le(r).await?;

            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            Ok(Self {
                quest_id,
                create_id,
                kill_count,
                required_kill_count,
                guid,
            })
        })
    }

    #[cfg(feature = "tokio")]
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
            // quest_id: u32
            w.write_all(&self.quest_id.to_le_bytes()).await?;

            // create_id: u32
            w.write_all(&self.create_id.to_le_bytes()).await?;

            // kill_count: u32
            w.write_all(&self.kill_count.to_le_bytes()).await?;

            // required_kill_count: u32
            w.write_all(&self.required_kill_count.to_le_bytes()).await?;

            // guid: Guid
            w.write_all(&self.guid.guid().to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
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
            // quest_id: u32
            let quest_id = crate::util::astd_read_u32_le(r).await?;

            // create_id: u32
            let create_id = crate::util::astd_read_u32_le(r).await?;

            // kill_count: u32
            let kill_count = crate::util::astd_read_u32_le(r).await?;

            // required_kill_count: u32
            let required_kill_count = crate::util::astd_read_u32_le(r).await?;

            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            Ok(Self {
                quest_id,
                create_id,
                kill_count,
                required_kill_count,
                guid,
            })
        })
    }

    #[cfg(feature = "async-std")]
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
            // quest_id: u32
            w.write_all(&self.quest_id.to_le_bytes()).await?;

            // create_id: u32
            w.write_all(&self.create_id.to_le_bytes()).await?;

            // kill_count: u32
            w.write_all(&self.kill_count.to_le_bytes()).await?;

            // required_kill_count: u32
            w.write_all(&self.required_kill_count.to_le_bytes()).await?;

            // guid: Guid
            w.write_all(&self.guid.guid().to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl SMSG_QUESTUPDATE_ADD_KILL {
    pub(crate) fn size() -> usize {
        0
        + 4 // quest_id: u32
        + 4 // create_id: u32
        + 4 // kill_count: u32
        + 4 // required_kill_count: u32
        + 8 // guid: Guid
    }
}

