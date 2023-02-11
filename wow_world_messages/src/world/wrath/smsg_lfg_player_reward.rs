use crate::wrath::QuestGiverReward;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_player_reward.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_player_reward.wowm#L1):
/// ```text
/// smsg SMSG_LFG_PLAYER_REWARD = 0x01FF {
///     u32 random_dungeon_entry;
///     u32 dungeon_finished_entry;
///     Bool done;
///     u32 unknown1;
///     u32 money_reward;
///     u32 experience_reward;
///     u32 unknown2;
///     u32 unknown3;
///     u8 amount_of_rewards;
///     QuestGiverReward[amount_of_rewards] rewards;
/// }
/// ```
pub struct SMSG_LFG_PLAYER_REWARD {
    pub random_dungeon_entry: u32,
    pub dungeon_finished_entry: u32,
    pub done: bool,
    /// emus set to 1.
    ///
    pub unknown1: u32,
    pub money_reward: u32,
    pub experience_reward: u32,
    /// emus set to 0.
    ///
    pub unknown2: u32,
    /// emus set to 0.
    ///
    pub unknown3: u32,
    pub rewards: Vec<QuestGiverReward>,
}

impl crate::Message for SMSG_LFG_PLAYER_REWARD {
    const OPCODE: u32 = 0x01ff;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // random_dungeon_entry: u32
        w.write_all(&self.random_dungeon_entry.to_le_bytes())?;

        // dungeon_finished_entry: u32
        w.write_all(&self.dungeon_finished_entry.to_le_bytes())?;

        // done: Bool
        w.write_all(u8::from(self.done).to_le_bytes().as_slice())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // amount_of_rewards: u8
        w.write_all(&(self.rewards.len() as u8).to_le_bytes())?;

        // rewards: QuestGiverReward[amount_of_rewards]
        for i in self.rewards.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(30..=3102).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FF, size: body_size as u32 });
        }

        // random_dungeon_entry: u32
        let random_dungeon_entry = crate::util::read_u32_le(r)?;

        // dungeon_finished_entry: u32
        let dungeon_finished_entry = crate::util::read_u32_le(r)?;

        // done: Bool
        let done = crate::util::read_u8_le(r)? != 0;
        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // money_reward: u32
        let money_reward = crate::util::read_u32_le(r)?;

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // unknown3: u32
        let unknown3 = crate::util::read_u32_le(r)?;

        // amount_of_rewards: u8
        let amount_of_rewards = crate::util::read_u8_le(r)?;

        // rewards: QuestGiverReward[amount_of_rewards]
        let mut rewards = Vec::with_capacity(amount_of_rewards as usize);
        for i in 0..amount_of_rewards {
            rewards.push(QuestGiverReward::read(r)?);
        }

        Ok(Self {
            random_dungeon_entry,
            dungeon_finished_entry,
            done,
            unknown1,
            money_reward,
            experience_reward,
            unknown2,
            unknown3,
            rewards,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_PLAYER_REWARD {}

impl SMSG_LFG_PLAYER_REWARD {
    pub(crate) fn size(&self) -> usize {
        4 // random_dungeon_entry: u32
        + 4 // dungeon_finished_entry: u32
        + 1 // done: Bool
        + 4 // unknown1: u32
        + 4 // money_reward: u32
        + 4 // experience_reward: u32
        + 4 // unknown2: u32
        + 4 // unknown3: u32
        + 1 // amount_of_rewards: u8
        + self.rewards.len() * 12 // rewards: QuestGiverReward[amount_of_rewards]
    }
}

