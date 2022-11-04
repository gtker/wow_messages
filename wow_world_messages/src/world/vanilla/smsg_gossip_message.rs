use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::GossipItem;
use crate::world::vanilla::QuestItem;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm#L14):
/// ```text
/// smsg SMSG_GOSSIP_MESSAGE = 0x017D {
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
    pub gossips: Vec<GossipItem>,
    pub quests: Vec<QuestItem>,
}

impl crate::Message for SMSG_GOSSIP_MESSAGE {
    const OPCODE: u32 = 0x017d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // title_text_id: u32
        w.write_all(&self.title_text_id.to_le_bytes())?;

        // amount_of_gossip_items: u32
        w.write_all(&(self.gossips.len() as u32).to_le_bytes())?;

        // gossips: GossipItem[amount_of_gossip_items]
        for i in self.gossips.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_quests: u32
        w.write_all(&(self.quests.len() as u32).to_le_bytes())?;

        // quests: QuestItem[amount_of_quests]
        for i in self.quests.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(20..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x017D, size: body_size as u32 });
        }

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
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GOSSIP_MESSAGE {}

impl SMSG_GOSSIP_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // title_text_id: u32
        + 4 // amount_of_gossip_items: u32
        + self.gossips.iter().fold(0, |acc, x| acc + x.size()) // gossips: GossipItem[amount_of_gossip_items]
        + 4 // amount_of_quests: u32
        + self.quests.iter().fold(0, |acc, x| acc + x.size()) // quests: QuestItem[amount_of_quests]
    }
}

