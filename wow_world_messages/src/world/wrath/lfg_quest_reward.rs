use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm#L1):
/// ```text
/// struct LfgQuestReward {
///     Item item;
///     u32 display_id;
///     u32 amount_of_rewards;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgQuestReward {
    pub item: u32,
    pub display_id: u32,
    pub amount_of_rewards: u32,
}

impl LfgQuestReward {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // display_id: u32
        w.write_all(&self.display_id.to_le_bytes())?;

        // amount_of_rewards: u32
        w.write_all(&self.amount_of_rewards.to_le_bytes())?;

        Ok(())
    }
}

impl LfgQuestReward {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // display_id: u32
        let display_id = crate::util::read_u32_le(&mut r)?;

        // amount_of_rewards: u32
        let amount_of_rewards = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item,
            display_id,
            amount_of_rewards,
        })
    }

}

