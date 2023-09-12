use std::io::{Read, Write};

use crate::shared::gold_vanilla_tbc_wrath::Gold;
use crate::tbc::QuestItemReward;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_complete.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_complete.wowm#L16):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_COMPLETE = 0x0191 {
///     u32 quest_id;
///     u32 unknown;
///     u32 experience_reward;
///     Gold money_reward;
///     u32 honor_reward;
///     u32 amount_of_item_rewards;
///     QuestItemReward[amount_of_item_rewards] item_rewards;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_COMPLETE {
    pub quest_id: u32,
    /// cmangos/vmangos/mangoszero: set to 0x03
    pub unknown: u32,
    pub experience_reward: u32,
    pub money_reward: Gold,
    pub honor_reward: u32,
    pub item_rewards: Vec<QuestItemReward>,
}

impl crate::private::Sealed for SMSG_QUESTGIVER_QUEST_COMPLETE {}
impl SMSG_QUESTGIVER_QUEST_COMPLETE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(24..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(&mut r)?;

        // money_reward: Gold
        let money_reward = Gold::new(crate::util::read_u32_le(&mut r)?);

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(&mut r)?;

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(&mut r)?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        let item_rewards = {
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for _ in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemReward::read(&mut r)?);
            }
            item_rewards
        };

        Ok(Self {
            quest_id,
            unknown,
            experience_reward,
            money_reward,
            honor_reward,
            item_rewards,
        })
    }

}

impl crate::Message for SMSG_QUESTGIVER_QUEST_COMPLETE {
    const OPCODE: u32 = 0x0191;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_QUESTGIVER_QUEST_COMPLETE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTGIVER_QUEST_COMPLETE {{").unwrap();
        // Members
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();
        writeln!(s, "    experience_reward = {};", self.experience_reward).unwrap();
        writeln!(s, "    money_reward = {};", self.money_reward.as_int()).unwrap();
        writeln!(s, "    honor_reward = {};", self.honor_reward).unwrap();
        writeln!(s, "    amount_of_item_rewards = {};", self.item_rewards.len()).unwrap();
        write!(s, "    item_rewards = [").unwrap();
        for v in self.item_rewards.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        item_count = {};", v.item_count).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 401_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "experience_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_item_rewards", "    ");
        if !self.item_rewards.is_empty() {
            writeln!(s, "    /* item_rewards: QuestItemReward[amount_of_item_rewards] start */").unwrap();
            for (i, v) in self.item_rewards.iter().enumerate() {
                writeln!(s, "    /* item_rewards: QuestItemReward[amount_of_item_rewards] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
                writeln!(s, "    /* item_rewards: QuestItemReward[amount_of_item_rewards] {i} end */").unwrap();
            }
            writeln!(s, "    /* item_rewards: QuestItemReward[amount_of_item_rewards] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // money_reward: Gold
        w.write_all((self.money_reward.as_int()).to_le_bytes().as_slice())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(401, "SMSG_QUESTGIVER_QUEST_COMPLETE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTGIVER_QUEST_COMPLETE {}

impl SMSG_QUESTGIVER_QUEST_COMPLETE {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // unknown: u32
        + 4 // experience_reward: u32
        + 4 // money_reward: Gold
        + 4 // honor_reward: u32
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 8 // item_rewards: QuestItemReward[amount_of_item_rewards]
    }
}

