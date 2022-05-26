use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::GossipItem;
use crate::world::v1::v12::QuestItem;
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
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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

        Ok(())
    }
    const OPCODE: u16 = 0x017d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

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

