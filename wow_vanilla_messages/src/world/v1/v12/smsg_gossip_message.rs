use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::GossipItem;
use crate::world::v1::v12::{QuestItem, QuestItemError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GOSSIP_MESSAGE {
    pub guid: Guid,
    pub title_text_id: u32,
    pub gossips: Vec<GossipItem>,
    pub quests: Vec<QuestItem>,
}

impl SMSG_GOSSIP_MESSAGE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // title_text_id: u32
        w.write_all(&self.title_text_id.to_le_bytes())?;

        // amount_of_gossip_items: u32
        w.write_all(&(self.gossips.len() as u32).to_le_bytes())?;

        // gossips: GossipItem[amount_of_gossip_items]
        for i in self.gossips.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        // amount_of_quests: u32
        w.write_all(&(self.quests.len() as u32).to_le_bytes())?;

        // quests: QuestItem[amount_of_quests]
        for i in self.quests.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_GOSSIP_MESSAGE {
    const OPCODE: u16 = 0x017d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_GOSSIP_MESSAGEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // title_text_id: u32
        let title_text_id = crate::util::read_u32_le(r)?;

        // amount_of_gossip_items: u32
        let amount_of_gossip_items = crate::util::read_u32_le(r)?;

        // gossips: GossipItem[amount_of_gossip_items]
        let mut gossips = Vec::with_capacity(amount_of_gossip_items as usize);
        for i in 0..amount_of_gossip_items {
            gossips.push(GossipItem::read(r)?);
        }

        // amount_of_quests: u32
        let amount_of_quests = crate::util::read_u32_le(r)?;

        // quests: QuestItem[amount_of_quests]
        let mut quests = Vec::with_capacity(amount_of_quests as usize);
        for i in 0..amount_of_quests {
            quests.push(QuestItem::read(r)?);
        }

        Ok(Self {
            guid,
            title_text_id,
            gossips,
            quests,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // title_text_id: u32
            let title_text_id = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_gossip_items: u32
            let amount_of_gossip_items = crate::util::tokio_read_u32_le(r).await?;

            // gossips: GossipItem[amount_of_gossip_items]
            let mut gossips = Vec::with_capacity(amount_of_gossip_items as usize);
            for i in 0..amount_of_gossip_items {
                gossips.push(GossipItem::tokio_read(r).await?);
            }

            // amount_of_quests: u32
            let amount_of_quests = crate::util::tokio_read_u32_le(r).await?;

            // quests: QuestItem[amount_of_quests]
            let mut quests = Vec::with_capacity(amount_of_quests as usize);
            for i in 0..amount_of_quests {
                quests.push(QuestItem::tokio_read(r).await?);
            }

            Ok(Self {
                guid,
                title_text_id,
                gossips,
                quests,
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
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // title_text_id: u32
            let title_text_id = crate::util::astd_read_u32_le(r).await?;

            // amount_of_gossip_items: u32
            let amount_of_gossip_items = crate::util::astd_read_u32_le(r).await?;

            // gossips: GossipItem[amount_of_gossip_items]
            let mut gossips = Vec::with_capacity(amount_of_gossip_items as usize);
            for i in 0..amount_of_gossip_items {
                gossips.push(GossipItem::astd_read(r).await?);
            }

            // amount_of_quests: u32
            let amount_of_quests = crate::util::astd_read_u32_le(r).await?;

            // quests: QuestItem[amount_of_quests]
            let mut quests = Vec::with_capacity(amount_of_quests as usize);
            for i in 0..amount_of_quests {
                quests.push(QuestItem::astd_read(r).await?);
            }

            Ok(Self {
                guid,
                title_text_id,
                gossips,
                quests,
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
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_GOSSIP_MESSAGE {
    pub fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // title_text_id: u32
        + 4 // amount_of_gossip_items: u32
        + self.gossips.len() * 6 // gossips: GossipItem[amount_of_gossip_items]
        + 4 // amount_of_quests: u32
        + self.quests.iter().fold(0, |acc, x| acc + x.size()) // quests: QuestItem[amount_of_quests]
    }
}

#[derive(Debug)]
pub enum SMSG_GOSSIP_MESSAGEError {
    Io(std::io::Error),
    QuestItem(QuestItemError),
}

impl std::error::Error for SMSG_GOSSIP_MESSAGEError {}
impl std::fmt::Display for SMSG_GOSSIP_MESSAGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::QuestItem(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GOSSIP_MESSAGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<QuestItemError> for SMSG_GOSSIP_MESSAGEError {
    fn from(e: QuestItemError) -> Self {
        Self::QuestItem(e)
    }
}

