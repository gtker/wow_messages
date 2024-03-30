use std::io::{Read, Write};

use crate::Guid;
use crate::shared::gold_vanilla_tbc_wrath::Gold;
use crate::wrath::{
    NpcTextUpdateEmote, QuestItemRequirement,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_offer_reward.wowm:51`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_offer_reward.wowm#L51):
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
///     Spell reward_spell;
///     Spell reward_spell_cast;
///     u32 title_reward;
///     u32 reward_talents;
///     u32 reward_arena_points;
///     u32 reward_reputation_mask;
///     u32[5] reward_factions;
///     u32[5] reward_reputations;
///     u32[5] reward_reputations_override;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
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
    pub unknown1: u32,
    pub reward_spell: u32,
    /// mangoszero and cmangos disagree about which field is _cast, although they both agree that the _cast field should not be in zero (vanilla). They still both include both fields in the code though.
    pub reward_spell_cast: u32,
    pub title_reward: u32,
    pub reward_talents: u32,
    pub reward_arena_points: u32,
    pub reward_reputation_mask: u32,
    pub reward_factions: [u32; 5],
    /// mangostwo: columnid in QuestFactionReward.dbc (if negative, from second row)
    pub reward_reputations: [u32; 5],
    /// mangostwo: reward reputation override. No diplomacy bonus is expected given, reward also does not display in chat window
    pub reward_reputations_override: [u32; 5],
}

impl crate::private::Sealed for SMSG_QUESTGIVER_OFFER_REWARD {}
impl SMSG_QUESTGIVER_OFFER_REWARD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(142..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // offer_reward_text: CString
        let offer_reward_text = {
            let offer_reward_text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(offer_reward_text)?
        };

        // auto_finish: Bool32
        let auto_finish = crate::util::read_bool_u32(&mut r)?;

        // flags1: u32
        let flags1 = crate::util::read_u32_le(&mut r)?;

        // suggested_players: u32
        let suggested_players = crate::util::read_u32_le(&mut r)?;

        // amount_of_emotes: u32
        let amount_of_emotes = crate::util::read_u32_le(&mut r)?;

        // emotes: NpcTextUpdateEmote[amount_of_emotes]
        let emotes = {
            let mut emotes = Vec::with_capacity(amount_of_emotes as usize);

            let allocation_size = u64::from(amount_of_emotes) * 8;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_emotes {
                emotes.push(NpcTextUpdateEmote::read(&mut r)?);
            }
            emotes
        };

        // amount_of_choice_item_rewards: u32
        let amount_of_choice_item_rewards = crate::util::read_u32_le(&mut r)?;

        // choice_item_rewards: QuestItemRequirement[amount_of_choice_item_rewards]
        let choice_item_rewards = {
            let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);

            let allocation_size = u64::from(amount_of_choice_item_rewards) * 12;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_choice_item_rewards {
                choice_item_rewards.push(QuestItemRequirement::read(&mut r)?);
            }
            choice_item_rewards
        };

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(&mut r)?;

        // item_rewards: QuestItemRequirement[amount_of_item_rewards]
        let item_rewards = {
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);

            let allocation_size = u64::from(amount_of_item_rewards) * 12;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemRequirement::read(&mut r)?);
            }
            item_rewards
        };

        // money_reward: Gold
        let money_reward = Gold::new(crate::util::read_u32_le(&mut r)?);

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(&mut r)?;

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(&mut r)?;

        // honor_reward_multiplier: f32
        let honor_reward_multiplier = crate::util::read_f32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // reward_spell: Spell
        let reward_spell = crate::util::read_u32_le(&mut r)?;

        // reward_spell_cast: Spell
        let reward_spell_cast = crate::util::read_u32_le(&mut r)?;

        // title_reward: u32
        let title_reward = crate::util::read_u32_le(&mut r)?;

        // reward_talents: u32
        let reward_talents = crate::util::read_u32_le(&mut r)?;

        // reward_arena_points: u32
        let reward_arena_points = crate::util::read_u32_le(&mut r)?;

        // reward_reputation_mask: u32
        let reward_reputation_mask = crate::util::read_u32_le(&mut r)?;

        // reward_factions: u32[5]
        let reward_factions = {
            let mut reward_factions = [u32::default(); 5];
            for i in reward_factions.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            reward_factions
        };

        // reward_reputations: u32[5]
        let reward_reputations = {
            let mut reward_reputations = [u32::default(); 5];
            for i in reward_reputations.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            reward_reputations
        };

        // reward_reputations_override: u32[5]
        let reward_reputations_override = {
            let mut reward_reputations_override = [u32::default(); 5];
            for i in reward_reputations_override.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
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

impl crate::Message for SMSG_QUESTGIVER_OFFER_REWARD {
    const OPCODE: u32 = 0x018d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_QUESTGIVER_OFFER_REWARD"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTGIVER_OFFER_REWARD {{").unwrap();
        // Members
        writeln!(s, "    npc = {};", self.npc.guid()).unwrap();
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    offer_reward_text = \"{}\";", self.offer_reward_text).unwrap();
        writeln!(s, "    auto_finish = {};", if self.auto_finish { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    flags1 = {};", self.flags1).unwrap();
        writeln!(s, "    suggested_players = {};", self.suggested_players).unwrap();
        writeln!(s, "    amount_of_emotes = {};", self.emotes.len()).unwrap();
        writeln!(s, "    emotes = [").unwrap();
        for v in self.emotes.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            delay = {};", v.delay).unwrap();
            writeln!(s, "            emote = {};", v.emote).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    amount_of_choice_item_rewards = {};", self.choice_item_rewards.len()).unwrap();
        writeln!(s, "    choice_item_rewards = [").unwrap();
        for v in self.choice_item_rewards.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            item = {};", v.item).unwrap();
            writeln!(s, "            item_count = {};", v.item_count).unwrap();
            writeln!(s, "            item_display_id = {};", v.item_display_id).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    amount_of_item_rewards = {};", self.item_rewards.len()).unwrap();
        writeln!(s, "    item_rewards = [").unwrap();
        for v in self.item_rewards.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            item = {};", v.item).unwrap();
            writeln!(s, "            item_count = {};", v.item_count).unwrap();
            writeln!(s, "            item_display_id = {};", v.item_display_id).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    money_reward = {};", self.money_reward.as_int()).unwrap();
        writeln!(s, "    experience_reward = {};", self.experience_reward).unwrap();
        writeln!(s, "    honor_reward = {};", self.honor_reward).unwrap();
        writeln!(s, "    honor_reward_multiplier = {};", if self.honor_reward_multiplier.to_string().contains('.') { self.honor_reward_multiplier.to_string() } else { format!("{}.0", self.honor_reward_multiplier) }).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    reward_spell = {};", self.reward_spell).unwrap();
        writeln!(s, "    reward_spell_cast = {};", self.reward_spell_cast).unwrap();
        writeln!(s, "    title_reward = {};", self.title_reward).unwrap();
        writeln!(s, "    reward_talents = {};", self.reward_talents).unwrap();
        writeln!(s, "    reward_arena_points = {};", self.reward_arena_points).unwrap();
        writeln!(s, "    reward_reputation_mask = {};", self.reward_reputation_mask).unwrap();
        writeln!(s, "    reward_factions = [").unwrap();
        for v in self.reward_factions.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    reward_reputations = [").unwrap();
        for v in self.reward_reputations.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    reward_reputations_override = [").unwrap();
        for v in self.reward_reputations_override.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 397_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "npc", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.offer_reward_text.len() + 1, "offer_reward_text", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auto_finish", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "suggested_players", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_emotes", "    ");
        if !self.emotes.is_empty() {
            writeln!(s, "    /* emotes: NpcTextUpdateEmote[amount_of_emotes] start */").unwrap();
            for (i, v) in self.emotes.iter().enumerate() {
                writeln!(s, "    /* emotes: NpcTextUpdateEmote[amount_of_emotes] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "delay", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "emote", "        ");
                writeln!(s, "    /* emotes: NpcTextUpdateEmote[amount_of_emotes] {i} end */").unwrap();
            }
            writeln!(s, "    /* emotes: NpcTextUpdateEmote[amount_of_emotes] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_choice_item_rewards", "    ");
        if !self.choice_item_rewards.is_empty() {
            writeln!(s, "    /* choice_item_rewards: QuestItemRequirement[amount_of_choice_item_rewards] start */").unwrap();
            for (i, v) in self.choice_item_rewards.iter().enumerate() {
                writeln!(s, "    /* choice_item_rewards: QuestItemRequirement[amount_of_choice_item_rewards] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_display_id", "        ");
                writeln!(s, "    /* choice_item_rewards: QuestItemRequirement[amount_of_choice_item_rewards] {i} end */").unwrap();
            }
            writeln!(s, "    /* choice_item_rewards: QuestItemRequirement[amount_of_choice_item_rewards] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_item_rewards", "    ");
        if !self.item_rewards.is_empty() {
            writeln!(s, "    /* item_rewards: QuestItemRequirement[amount_of_item_rewards] start */").unwrap();
            for (i, v) in self.item_rewards.iter().enumerate() {
                writeln!(s, "    /* item_rewards: QuestItemRequirement[amount_of_item_rewards] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_display_id", "        ");
                writeln!(s, "    /* item_rewards: QuestItemRequirement[amount_of_item_rewards] {i} end */").unwrap();
            }
            writeln!(s, "    /* item_rewards: QuestItemRequirement[amount_of_item_rewards] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "experience_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_reward_multiplier", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_spell_cast", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "title_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_talents", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_arena_points", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_reputation_mask", "    ");
        writeln!(s, "    /* reward_factions: u32[5] start */").unwrap();
        for (i, v) in self.reward_factions.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reward_factions {i}"), "    ");
        }
        writeln!(s, "    /* reward_factions: u32[5] end */").unwrap();
        writeln!(s, "    /* reward_reputations: u32[5] start */").unwrap();
        for (i, v) in self.reward_reputations.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reward_reputations {i}"), "    ");
        }
        writeln!(s, "    /* reward_reputations: u32[5] end */").unwrap();
        writeln!(s, "    /* reward_reputations_override: u32[5] start */").unwrap();
        for (i, v) in self.reward_reputations_override.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reward_reputations_override {i}"), "    ");
        }
        writeln!(s, "    /* reward_reputations_override: u32[5] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().next_back(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // offer_reward_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.offer_reward_text.as_bytes().iter().next_back(), Some(&0_u8), "String `offer_reward_text` must not be null-terminated.");
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
            i.write_into_vec(&mut w)?;
        }

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemRequirement[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemRequirement[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // money_reward: Gold
        w.write_all((self.money_reward.as_int()).to_le_bytes().as_slice())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // honor_reward_multiplier: f32
        w.write_all(&self.honor_reward_multiplier.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // reward_spell: Spell
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // reward_spell_cast: Spell
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(397, "SMSG_QUESTGIVER_OFFER_REWARD", body_size, a))
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
        + 4 // money_reward: Gold
        + 4 // experience_reward: u32
        + 4 // honor_reward: u32
        + 4 // honor_reward_multiplier: f32
        + 4 // unknown1: u32
        + 4 // reward_spell: Spell
        + 4 // reward_spell_cast: Spell
        + 4 // title_reward: u32
        + 4 // reward_talents: u32
        + 4 // reward_arena_points: u32
        + 4 // reward_reputation_mask: u32
        + 20 // reward_factions: u32[5]
        + 20 // reward_reputations: u32[5]
        + 20 // reward_reputations_override: u32[5]
    }
}

