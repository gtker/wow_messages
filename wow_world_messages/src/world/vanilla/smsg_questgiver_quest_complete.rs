use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::QuestItemReward;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_complete.wowm#L3):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_COMPLETE = 0x0191 {
///     u32 quest_id;
///     u32 unknown;
///     u32 experience_reward;
///     u32 money_reward;
///     u32 amount_of_item_rewards;
///     QuestItemReward[amount_of_item_rewards] item_rewards;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_COMPLETE {
    pub quest_id: u32,
    /// cmangos/vmangos/mangoszero: set to 0x03
    ///
    pub unknown: u32,
    pub experience_reward: u32,
    pub money_reward: u32,
    pub item_rewards: Vec<QuestItemReward>,
}

impl crate::Message for SMSG_QUESTGIVER_QUEST_COMPLETE {
    const OPCODE: u32 = 0x0191;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(r)?;

        // money_reward: u32
        let money_reward = crate::util::read_u32_le(r)?;

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(r)?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
        for i in 0..amount_of_item_rewards {
            let o = QuestItemReward::read(r)?;
            item_rewards.push(o);
        }

        Ok(Self {
            quest_id,
            unknown,
            experience_reward,
            money_reward,
            item_rewards,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_QUESTGIVER_QUEST_COMPLETE {}

impl SMSG_QUESTGIVER_QUEST_COMPLETE {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // unknown: u32
        + 4 // experience_reward: u32
        + 4 // money_reward: u32
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 8 // item_rewards: QuestItemReward[amount_of_item_rewards]
    }
}

