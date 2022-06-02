use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::NpcTextUpdateEmote;
use crate::world::version_1_12::QuestItemReward;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_offer_reward.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_offer_reward.wowm#L3):
/// ```text
/// smsg SMSG_QUESTGIVER_OFFER_REWARD = 0x018D {
///     Guid npc;
///     u32 quest_id;
///     CString title;
///     CString offer_reward_text;
///     u32 enable_next;
///     u32 amount_of_emotes;
///     NpcTextUpdateEmote[amount_of_emotes] emotes;
///     u32 amount_of_choice_item_rewards;
///     QuestItemReward[amount_of_choice_item_rewards] choice_item_rewards;
///     u32 amount_of_item_rewards;
///     QuestItemReward[amount_of_item_rewards] item_rewards;
///     u32 money_reward;
///     u32 reward_spell;
///     u32 reward_spell_cast;
/// }
/// ```
pub struct SMSG_QUESTGIVER_OFFER_REWARD {
    pub npc: Guid,
    pub quest_id: u32,
    pub title: String,
    pub offer_reward_text: String,
    /// vmangos/mangoszero/cmangos: Auto Finish
    ///
    pub enable_next: u32,
    pub emotes: Vec<NpcTextUpdateEmote>,
    pub choice_item_rewards: Vec<QuestItemReward>,
    pub item_rewards: Vec<QuestItemReward>,
    pub money_reward: u32,
    pub reward_spell: u32,
    /// mangoszero and cmangos disagree about which field is _cast, although they both agree that the _cast field should not be in zero (vanilla). They still both include both fields in the code though.
    ///
    pub reward_spell_cast: u32,
}

impl ServerMessage for SMSG_QUESTGIVER_OFFER_REWARD {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // offer_reward_text: CString
        w.write_all(self.offer_reward_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // enable_next: u32
        w.write_all(&self.enable_next.to_le_bytes())?;

        // amount_of_emotes: u32
        w.write_all(&(self.emotes.len() as u32).to_le_bytes())?;

        // emotes: NpcTextUpdateEmote[amount_of_emotes]
        for i in self.emotes.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(w)?;
        }

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // reward_spell_cast: u32
        w.write_all(&self.reward_spell_cast.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x018d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // offer_reward_text: CString
        let offer_reward_text = crate::util::read_c_string_to_vec(r)?;
        let offer_reward_text = String::from_utf8(offer_reward_text)?;

        // enable_next: u32
        let enable_next = crate::util::read_u32_le(r)?;

        // amount_of_emotes: u32
        let amount_of_emotes = crate::util::read_u32_le(r)?;

        // emotes: NpcTextUpdateEmote[amount_of_emotes]
        let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
        for i in 0..amount_of_emotes {
            emotes.push(NpcTextUpdateEmote::read(r)?);
        }

        // amount_of_choice_item_rewards: u32
        let amount_of_choice_item_rewards = crate::util::read_u32_le(r)?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
        for i in 0..amount_of_choice_item_rewards {
            choice_item_rewards.push(QuestItemReward::read(r)?);
        }

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(r)?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
        for i in 0..amount_of_item_rewards {
            item_rewards.push(QuestItemReward::read(r)?);
        }

        // money_reward: u32
        let money_reward = crate::util::read_u32_le(r)?;

        // reward_spell: u32
        let reward_spell = crate::util::read_u32_le(r)?;

        // reward_spell_cast: u32
        let reward_spell_cast = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc,
            quest_id,
            title,
            offer_reward_text,
            enable_next,
            emotes,
            choice_item_rewards,
            item_rewards,
            money_reward,
            reward_spell,
            reward_spell_cast,
        })
    }

}

impl SMSG_QUESTGIVER_OFFER_REWARD {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.offer_reward_text.len() + 1 // offer_reward_text: CString
        + 4 // enable_next: u32
        + 4 // amount_of_emotes: u32
        + self.emotes.len() * 8 // emotes: NpcTextUpdateEmote[amount_of_emotes]
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.len() * 8 // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 8 // item_rewards: QuestItemReward[amount_of_item_rewards]
        + 4 // money_reward: u32
        + 4 // reward_spell: u32
        + 4 // reward_spell_cast: u32
    }
}

