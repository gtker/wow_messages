use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    QuestDetailsEmote, QuestItemReward,
};
use std::time::Duration;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm#L27):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_DETAILS = 0x0188 {
///     Guid guid;
///     u32 quest_id;
///     CString title;
///     CString details;
///     CString objectives;
///     Bool32 auto_finish;
///     u32 suggested_players;
///     u32 amount_of_choice_item_rewards;
///     QuestItemReward[amount_of_choice_item_rewards] choice_item_rewards;
///     u32 amount_of_item_rewards;
///     QuestItemReward[amount_of_item_rewards] item_rewards;
///     Gold money_reward;
///     u32 honor_reward;
///     u32 reward_spell;
///     u32 casted_spell;
///     u32 title_reward;
///     u32 amount_of_emotes;
///     QuestDetailsEmote[amount_of_emotes] emotes;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_DETAILS {
    pub guid: Guid,
    pub quest_id: u32,
    pub title: String,
    pub details: String,
    pub objectives: String,
    pub auto_finish: bool,
    pub suggested_players: u32,
    pub choice_item_rewards: Vec<QuestItemReward>,
    pub item_rewards: Vec<QuestItemReward>,
    pub money_reward: Gold,
    pub honor_reward: u32,
    /// mangosone: reward spell, this spell will display (icon) (casted if RewSpellCast==0)
    pub reward_spell: u32,
    pub casted_spell: u32,
    /// mangosone: CharTitle, new 2.4.0, player gets this title (bit index from CharTitles)
    pub title_reward: u32,
    pub emotes: Vec<QuestDetailsEmote>,
}

impl crate::private::Sealed for SMSG_QUESTGIVER_QUEST_DETAILS {}
impl SMSG_QUESTGIVER_QUEST_DETAILS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(55..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0188, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // details: CString
        let details = {
            let details = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(details)?
        };

        // objectives: CString
        let objectives = {
            let objectives = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(objectives)?
        };

        // auto_finish: Bool32
        let auto_finish = crate::util::read_u32_le(&mut r)? != 0;

        // suggested_players: u32
        let suggested_players = crate::util::read_u32_le(&mut r)?;

        // amount_of_choice_item_rewards: u32
        let amount_of_choice_item_rewards = crate::util::read_u32_le(&mut r)?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        let choice_item_rewards = {
            let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
            for _ in 0..amount_of_choice_item_rewards {
                choice_item_rewards.push(QuestItemReward::read(&mut r)?);
            }
            choice_item_rewards
        };

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

        // money_reward: Gold
        let money_reward = Gold::new(crate::util::read_u32_le(&mut r)?);

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(&mut r)?;

        // reward_spell: u32
        let reward_spell = crate::util::read_u32_le(&mut r)?;

        // casted_spell: u32
        let casted_spell = crate::util::read_u32_le(&mut r)?;

        // title_reward: u32
        let title_reward = crate::util::read_u32_le(&mut r)?;

        // amount_of_emotes: u32
        let amount_of_emotes = crate::util::read_u32_le(&mut r)?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        let emotes = {
            let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
            for _ in 0..amount_of_emotes {
                emotes.push(QuestDetailsEmote::read(&mut r)?);
            }
            emotes
        };

        Ok(Self {
            guid,
            quest_id,
            title,
            details,
            objectives,
            auto_finish,
            suggested_players,
            choice_item_rewards,
            item_rewards,
            money_reward,
            honor_reward,
            reward_spell,
            casted_spell,
            title_reward,
            emotes,
        })
    }

}

impl crate::Message for SMSG_QUESTGIVER_QUEST_DETAILS {
    const OPCODE: u32 = 0x0188;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTGIVER_QUEST_DETAILS {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    details = \"{}\";", self.details).unwrap();
        writeln!(s, "    objectives = \"{}\";", self.objectives).unwrap();
        writeln!(s, "    auto_finish = {};", if self.auto_finish { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    suggested_players = {};", self.suggested_players).unwrap();
        writeln!(s, "    amount_of_choice_item_rewards = {};", self.choice_item_rewards.len()).unwrap();
        write!(s, "    choice_item_rewards = [").unwrap();
        for v in self.choice_item_rewards.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        item_count = {};", v.item_count).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
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
        writeln!(s, "    money_reward = {};", self.money_reward.as_int()).unwrap();
        writeln!(s, "    honor_reward = {};", self.honor_reward).unwrap();
        writeln!(s, "    reward_spell = {};", self.reward_spell).unwrap();
        writeln!(s, "    casted_spell = {};", self.casted_spell).unwrap();
        writeln!(s, "    title_reward = {};", self.title_reward).unwrap();
        writeln!(s, "    amount_of_emotes = {};", self.emotes.len()).unwrap();
        write!(s, "    emotes = [").unwrap();
        for v in self.emotes.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        emote = {};", v.emote).unwrap();
            writeln!(s, "        emote_delay = {};", v.emote_delay.as_millis()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 392_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.details.len() + 1, "details", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.objectives.len() + 1, "objectives", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auto_finish", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "suggested_players", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_choice_item_rewards", "    ");
        if !self.choice_item_rewards.is_empty() {
            writeln!(s, "    /* choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards] start */").unwrap();
            for (i, v) in self.choice_item_rewards.iter().enumerate() {
                writeln!(s, "    /* choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
                writeln!(s, "    /* choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards] {i} end */").unwrap();
            }
            writeln!(s, "    /* choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards] end */").unwrap();
        }
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
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "casted_spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "title_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_emotes", "    ");
        if !self.emotes.is_empty() {
            writeln!(s, "    /* emotes: QuestDetailsEmote[amount_of_emotes] start */").unwrap();
            for (i, v) in self.emotes.iter().enumerate() {
                writeln!(s, "    /* emotes: QuestDetailsEmote[amount_of_emotes] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "emote", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "emote_delay", "        ");
                writeln!(s, "    /* emotes: QuestDetailsEmote[amount_of_emotes] {i} end */").unwrap();
            }
            writeln!(s, "    /* emotes: QuestDetailsEmote[amount_of_emotes] end */").unwrap();
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
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // details: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.details.as_bytes().iter().rev().next(), Some(&0_u8), "String `details` must not be null-terminated.");
        w.write_all(self.details.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objectives: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.objectives.as_bytes().iter().rev().next(), Some(&0_u8), "String `objectives` must not be null-terminated.");
        w.write_all(self.objectives.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // auto_finish: Bool32
        w.write_all(u32::from(self.auto_finish).to_le_bytes().as_slice())?;

        // suggested_players: u32
        w.write_all(&self.suggested_players.to_le_bytes())?;

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // money_reward: Gold
        w.write_all((self.money_reward.as_int()).to_le_bytes().as_slice())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // casted_spell: u32
        w.write_all(&self.casted_spell.to_le_bytes())?;

        // title_reward: u32
        w.write_all(&self.title_reward.to_le_bytes())?;

        // amount_of_emotes: u32
        w.write_all(&(self.emotes.len() as u32).to_le_bytes())?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        for i in self.emotes.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTGIVER_QUEST_DETAILS {}

impl SMSG_QUESTGIVER_QUEST_DETAILS {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.details.len() + 1 // details: CString
        + self.objectives.len() + 1 // objectives: CString
        + 4 // auto_finish: Bool32
        + 4 // suggested_players: u32
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.len() * 8 // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 8 // item_rewards: QuestItemReward[amount_of_item_rewards]
        + 4 // money_reward: Gold
        + 4 // honor_reward: u32
        + 4 // reward_spell: u32
        + 4 // casted_spell: u32
        + 4 // title_reward: u32
        + 4 // amount_of_emotes: u32
        + self.emotes.len() * 8 // emotes: QuestDetailsEmote[amount_of_emotes]
    }
}

