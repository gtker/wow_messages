use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::GossipItem;
use crate::world::v1::v12::{QuestItem, QuestItemError};
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
pub struct SMSG_GOSSIP_MESSAGE {
    pub guid: Guid,
    pub title_text_id: u32,
    pub gossips: Vec<GossipItem>,
    pub quests: Vec<QuestItem>,
}

impl ServerMessageWrite for SMSG_GOSSIP_MESSAGE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_GOSSIP_MESSAGE {
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
        // guid: Guid
        self.guid.write(w)?;

        // title_text_id: u32
        w.write_all(&self.title_text_id.to_le_bytes())?;

        // amount_of_gossip_items: u32
        w.write_all(&(self.gossips.len() as u32).to_le_bytes())?;

        // gossips: GossipItem[amount_of_gossip_items]
        for i in self.gossips.iter() {
            i.write(w)?;
        }

        // amount_of_quests: u32
        w.write_all(&(self.quests.len() as u32).to_le_bytes())?;

        // quests: QuestItem[amount_of_quests]
        for i in self.quests.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // title_text_id: u32
        w.write_all(&self.title_text_id.to_le_bytes()).await?;

        // amount_of_gossip_items: u32
        w.write_all(&(self.gossips.len() as u32).to_le_bytes()).await?;

        // gossips: GossipItem[amount_of_gossip_items]
        for i in self.gossips.iter() {
            i.tokio_write(w).await?;
        }

        // amount_of_quests: u32
        w.write_all(&(self.quests.len() as u32).to_le_bytes()).await?;

        // quests: QuestItem[amount_of_quests]
        for i in self.quests.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // title_text_id: u32
        w.write_all(&self.title_text_id.to_le_bytes()).await?;

        // amount_of_gossip_items: u32
        w.write_all(&(self.gossips.len() as u32).to_le_bytes()).await?;

        // gossips: GossipItem[amount_of_gossip_items]
        for i in self.gossips.iter() {
            i.astd_write(w).await?;
        }

        // amount_of_quests: u32
        w.write_all(&(self.quests.len() as u32).to_le_bytes()).await?;

        // quests: QuestItem[amount_of_quests]
        for i in self.quests.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }

}

impl VariableSized for SMSG_GOSSIP_MESSAGE {
    fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // title_text_id: u32
        + 4 // amount_of_gossip_items: u32
        + self.gossips.iter().fold(0, |acc, x| acc + GossipItem::size()) // gossips: GossipItem[amount_of_gossip_items]
        + 4 // amount_of_quests: u32
        + self.quests.iter().fold(0, |acc, x| acc + x.size()) // quests: QuestItem[amount_of_quests]
    }
}

impl MaximumPossibleSized for SMSG_GOSSIP_MESSAGE {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
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

