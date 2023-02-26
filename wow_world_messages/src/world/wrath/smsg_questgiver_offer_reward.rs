use crate::Guid;
use crate::wrath::Gold;
use crate::wrath::NpcTextUpdateEmote;
use crate::wrath::QuestItemRequirement;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_offer_reward.wowm:54`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_offer_reward.wowm#L54):
/// ```text
/// smsg SMSG_QUESTGIVER_OFFER_REWARD = 0x018D {
///     Guid npc;
///     u32 quest_id;
///     CString title;
///     CString offer_reward_text;
///     Bool32 auto_finish;
///     u32 flags1;
///     u32 suggested_players;
///     u32 amount_of_emotes;
///     NpcTextUpdateEmote[amount_of_emotes] emotes;
///     u32 amount_of_choice_item_rewards;
///     QuestItemRequirement[amount_of_choice_item_rewards] choice_item_rewards;
///     u32 amount_of_item_rewards;
///     QuestItemRequirement[amount_of_item_rewards] item_rewards;
///     Gold money_reward;
///     u32 experience_reward;
///     u32 honor_reward;
///     f32 honor_reward_multiplier;
///     u32 unknown1;
///     u32 reward_spell;
///     u32 reward_spell_cast;
///     u32 title_reward;
///     u32 reward_talents;
///     u32 reward_arena_points;
///     u32 reward_reputation_mask;
///     u32[5] reward_factions;
///     u32[5] reward_reputations;
///     u32[5] reward_reputations_override;
/// }
/// ```
pub struct SMSG_QUESTGIVER_OFFER_REWARD {
    pub npc: Guid,
    pub quest_id: u32,
    pub title: String,
    pub offer_reward_text: String,
    pub auto_finish: bool,
    pub flags1: u32,
    pub suggested_players: u32,
    pub emotes: Vec<NpcTextUpdateEmote>,
    pub choice_item_rewards: Vec<QuestItemRequirement>,
    pub item_rewards: Vec<QuestItemRequirement>,
    pub money_reward: Gold,
    pub experience_reward: u32,
    pub honor_reward: u32,
    pub honor_reward_multiplier: f32,
    /// mangostwo: unused by client?
    /// mangostwo sets to 0x08.
    ///
    pub unknown1: u32,
    pub reward_spell: u32,
    /// mangoszero and cmangos disagree about which field is _cast, although they both agree that the _cast field should not be in zero (vanilla). They still both include both fields in the code though.
    ///
    pub reward_spell_cast: u32,
    pub title_reward: u32,
    pub reward_talents: u32,
    pub reward_arena_points: u32,
    pub reward_reputation_mask: u32,
    pub reward_factions: [u32; 5],
    /// mangostwo: columnid in QuestFactionReward.dbc (if negative, from second row)
    ///
    pub reward_reputations: [u32; 5],
    /// mangostwo: reward reputation override. No diplomacy bonus is expected given, reward also does not display in chat window
    ///
    pub reward_reputations_override: [u32; 5],
}

impl crate::Message for SMSG_QUESTGIVER_OFFER_REWARD {
    const OPCODE: u32 = 0x018d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // offer_reward_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.offer_reward_text.as_bytes().iter().rev().next(), Some(&0_u8), "String `offer_reward_text` must not be null-terminated.");
        w.write_all(self.offer_reward_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // auto_finish: Bool32
        w.write_all(u32::from(self.auto_finish).to_le_bytes().as_slice())?;

        // flags1: u32
        w.write_all(&self.flags1.to_le_bytes())?;

        // suggested_players: u32
        w.write_all(&self.suggested_players.to_le_bytes())?;

        // amount_of_emotes: u32
        w.write_all(&(self.emotes.len() as u32).to_le_bytes())?;

        // emotes: NpcTextUpdateEmote[amount_of_emotes]
        for i in self.emotes.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemRequirement[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemRequirement[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(w)?;
        }

        // money_reward: Gold
        w.write_all(u32::from(self.money_reward.as_int()).to_le_bytes().as_slice())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // honor_reward_multiplier: f32
        w.write_all(&self.honor_reward_multiplier.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // reward_spell_cast: u32
        w.write_all(&self.reward_spell_cast.to_le_bytes())?;

        // title_reward: u32
        w.write_all(&self.title_reward.to_le_bytes())?;

        // reward_talents: u32
        w.write_all(&self.reward_talents.to_le_bytes())?;

        // reward_arena_points: u32
        w.write_all(&self.reward_arena_points.to_le_bytes())?;

        // reward_reputation_mask: u32
        w.write_all(&self.reward_reputation_mask.to_le_bytes())?;

        // reward_factions: u32[5]
        for i in self.reward_factions.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // reward_reputations: u32[5]
        for i in self.reward_reputations.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // reward_reputations_override: u32[5]
        for i in self.reward_reputations_override.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(142..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x018D, size: body_size as u32 });
        }

        // npc: Guid
        let npc = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(title)?
        };

        // offer_reward_text: CString
        let offer_reward_text = {
            let offer_reward_text = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(offer_reward_text)?
        };

        // auto_finish: Bool32
        let auto_finish = crate::util::read_u32_le(r)? != 0;

        // flags1: u32
        let flags1 = crate::util::read_u32_le(r)?;

        // suggested_players: u32
        let suggested_players = crate::util::read_u32_le(r)?;

        // amount_of_emotes: u32
        let amount_of_emotes = crate::util::read_u32_le(r)?;

        // emotes: NpcTextUpdateEmote[amount_of_emotes]
        let emotes = {
            let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
            for i in 0..amount_of_emotes {
                emotes.push(NpcTextUpdateEmote::read(r)?);
            }
            emotes
        };

        // amount_of_choice_item_rewards: u32
        let amount_of_choice_item_rewards = crate::util::read_u32_le(r)?;

        // choice_item_rewards: QuestItemRequirement[amount_of_choice_item_rewards]
        let choice_item_rewards = {
            let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
            for i in 0..amount_of_choice_item_rewards {
                choice_item_rewards.push(QuestItemRequirement::read(r)?);
            }
            choice_item_rewards
        };

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(r)?;

        // item_rewards: QuestItemRequirement[amount_of_item_rewards]
        let item_rewards = {
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for i in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemRequirement::read(r)?);
            }
            item_rewards
        };

        // money_reward: Gold
        let money_reward = Gold::new(crate::util::read_u32_le(r)?);

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(r)?;

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(r)?;

        // honor_reward_multiplier: f32
        let honor_reward_multiplier = crate::util::read_f32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // reward_spell: u32
        let reward_spell = crate::util::read_u32_le(r)?;

        // reward_spell_cast: u32
        let reward_spell_cast = crate::util::read_u32_le(r)?;

        // title_reward: u32
        let title_reward = crate::util::read_u32_le(r)?;

        // reward_talents: u32
        let reward_talents = crate::util::read_u32_le(r)?;

        // reward_arena_points: u32
        let reward_arena_points = crate::util::read_u32_le(r)?;

        // reward_reputation_mask: u32
        let reward_reputation_mask = crate::util::read_u32_le(r)?;

        // reward_factions: u32[5]
        let reward_factions = {
            let mut reward_factions = [u32::default(); 5];
            for i in reward_factions.iter_mut() {
                *i = crate::util::read_u32_le(r)?;
            }
            reward_factions
        };

        // reward_reputations: u32[5]
        let reward_reputations = {
            let mut reward_reputations = [u32::default(); 5];
            for i in reward_reputations.iter_mut() {
                *i = crate::util::read_u32_le(r)?;
            }
            reward_reputations
        };

        // reward_reputations_override: u32[5]
        let reward_reputations_override = {
            let mut reward_reputations_override = [u32::default(); 5];
            for i in reward_reputations_override.iter_mut() {
                *i = crate::util::read_u32_le(r)?;
            }
            reward_reputations_override
        };

        Ok(Self {
            npc,
            quest_id,
            title,
            offer_reward_text,
            auto_finish,
            flags1,
            suggested_players,
            emotes,
            choice_item_rewards,
            item_rewards,
            money_reward,
            experience_reward,
            honor_reward,
            honor_reward_multiplier,
            unknown1,
            reward_spell,
            reward_spell_cast,
            title_reward,
            reward_talents,
            reward_arena_points,
            reward_reputation_mask,
            reward_factions,
            reward_reputations,
            reward_reputations_override,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTGIVER_OFFER_REWARD {}

impl SMSG_QUESTGIVER_OFFER_REWARD {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.offer_reward_text.len() + 1 // offer_reward_text: CString
        + 4 // auto_finish: Bool32
        + 4 // flags1: u32
        + 4 // suggested_players: u32
        + 4 // amount_of_emotes: u32
        + self.emotes.len() * 8 // emotes: NpcTextUpdateEmote[amount_of_emotes]
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.len() * 12 // choice_item_rewards: QuestItemRequirement[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 12 // item_rewards: QuestItemRequirement[amount_of_item_rewards]
        + 8 // money_reward: Gold
        + 4 // experience_reward: u32
        + 4 // honor_reward: u32
        + 4 // honor_reward_multiplier: f32
        + 4 // unknown1: u32
        + 4 // reward_spell: u32
        + 4 // reward_spell_cast: u32
        + 4 // title_reward: u32
        + 4 // reward_talents: u32
        + 4 // reward_arena_points: u32
        + 4 // reward_reputation_mask: u32
        + 5 * core::mem::size_of::<u32>() // reward_factions: u32[5]
        + 5 * core::mem::size_of::<u32>() // reward_reputations: u32[5]
        + 5 * core::mem::size_of::<u32>() // reward_reputations_override: u32[5]
    }
}

