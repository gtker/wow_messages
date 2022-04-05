use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::GossipItem;
use crate::world::v1::v12::{QuestItem, QuestItemError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:4808`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L4808):
/// ```text
/// smsg SMSG_GOSSIP_MESSAGE = 0x17D {
///     Guid guid;
///     u32 title_text_id;
///     u32 amount_of_gossip_items;
///     GossipItem[amount_of_gossip_items] gossips;
///     u32 amount_of_quests;
///     QuestItem[amount_of_quests] quests;
/// }
/// ```
pub struct SMSG_GOSSIP_MESSAGE {
    pub guid: Guid,
    pub title_text_id: u32,
    pub amount_of_gossip_items: u32,
    pub gossips: Vec<GossipItem>,
    pub amount_of_quests: u32,
    pub quests: Vec<QuestItem>,
}

impl WorldServerMessageWrite for SMSG_GOSSIP_MESSAGE {
    const OPCODE: u16 = 0x17d;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_GOSSIP_MESSAGE {
    type Error = SMSG_GOSSIP_MESSAGEError;

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
            amount_of_gossip_items,
            gossips,
            amount_of_quests,
            quests,
        })
    }

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
}

impl VariableSized for SMSG_GOSSIP_MESSAGE {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // title_text_id: u32
        + 4 // amount_of_gossip_items: u32
        + self.gossips.iter().fold(0, |acc, x| acc + GossipItem::size()) // gossips: GossipItem[amount_of_gossip_items]
        + 4 // amount_of_quests: u32
        + self.quests.iter().fold(0, |acc, x| acc + x.size()) // quests: QuestItem[amount_of_quests]
    }
}

impl MaximumPossibleSized for SMSG_GOSSIP_MESSAGE {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // title_text_id: u32
        + 4 // amount_of_gossip_items: u32
        + 4294967295 * GossipItem::maximum_possible_size() // gossips: GossipItem[amount_of_gossip_items]
        + 4 // amount_of_quests: u32
        + 4294967295 * QuestItem::maximum_possible_size() // quests: QuestItem[amount_of_quests]
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

