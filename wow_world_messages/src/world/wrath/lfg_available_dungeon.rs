use crate::wrath::LfgQuestReward;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm#L9):
/// ```text
/// struct LfgAvailableDungeon {
///     u32 dungeon_entry;
///     Bool done;
///     u32 quest_reward;
///     u32 xp_reward;
///     u32 unknown1;
///     u32 unknown2;
///     u8 amount_of_rewards;
///     LfgQuestReward[amount_of_rewards] rewards;
/// }
/// ```
pub struct LfgAvailableDungeon {
    pub dungeon_entry: u32,
    pub done: bool,
    pub quest_reward: u32,
    pub xp_reward: u32,
    pub unknown1: u32,
    pub unknown2: u32,
    pub rewards: Vec<LfgQuestReward>,
}

impl LfgAvailableDungeon {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // dungeon_entry: u32
        w.write_all(&self.dungeon_entry.to_le_bytes())?;

        // done: Bool
        w.write_all(u8::from(self.done).to_le_bytes().as_slice())?;

        // quest_reward: u32
        w.write_all(&self.quest_reward.to_le_bytes())?;

        // xp_reward: u32
        w.write_all(&self.xp_reward.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // amount_of_rewards: u8
        w.write_all(&(self.rewards.len() as u8).to_le_bytes())?;

        // rewards: LfgQuestReward[amount_of_rewards]
        for i in self.rewards.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
}

impl LfgAvailableDungeon {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // dungeon_entry: u32
        let dungeon_entry = crate::util::read_u32_le(r)?;

        // done: Bool
        let done = crate::util::read_u8_le(r)? != 0;

        // quest_reward: u32
        let quest_reward = crate::util::read_u32_le(r)?;

        // xp_reward: u32
        let xp_reward = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // amount_of_rewards: u8
        let amount_of_rewards = crate::util::read_u8_le(r)?;

        // rewards: LfgQuestReward[amount_of_rewards]
        let rewards = {
            let mut rewards = Vec::with_capacity(amount_of_rewards as usize);
            for i in 0..amount_of_rewards {
                rewards.push(LfgQuestReward::read(r)?);
            }
            rewards
        };

        Ok(Self {
            dungeon_entry,
            done,
            quest_reward,
            xp_reward,
            unknown1,
            unknown2,
            rewards,
        })
    }

}

impl LfgAvailableDungeon {
    pub(crate) fn size(&self) -> usize {
        4 // dungeon_entry: u32
        + 1 // done: Bool
        + 4 // quest_reward: u32
        + 4 // xp_reward: u32
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 1 // amount_of_rewards: u8
        + self.rewards.len() * 12 // rewards: LfgQuestReward[amount_of_rewards]
    }
}

