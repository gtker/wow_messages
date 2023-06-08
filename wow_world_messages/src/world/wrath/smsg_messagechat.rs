use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    ChatType, Language, NamedGuid, PlayerChatTag,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_messagechat.wowm:118`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_messagechat.wowm#L118):
/// ```text
/// smsg SMSG_MESSAGECHAT = 0x0096 {
///     ChatType chat_type;
///     (u32)Language language;
///     Guid sender;
///     u32 flags;
///     if (chat_type == MONSTER_SAY
///         || chat_type == MONSTER_PARTY
///         || chat_type == MONSTER_YELL
///         || chat_type == MONSTER_WHISPER
///         || chat_type == RAID_BOSS_WHISPER
///         || chat_type == RAID_BOSS_EMOTE
///         || chat_type == MONSTER_EMOTE
///         || chat_type == BATTLENET) {
///         SizedCString sender1;
///         NamedGuid target1;
///     }
///     else if (chat_type == WHISPER_FOREIGN) {
///         SizedCString sender2;
///         Guid target2;
///     }
///     else if (chat_type == BG_SYSTEM_NEUTRAL
///         || chat_type == BG_SYSTEM_ALLIANCE
///         || chat_type == BG_SYSTEM_HORDE) {
///         NamedGuid target3;
///     }
///     else if (chat_type == ACHIEVEMENT
///         || chat_type == GUILD_ACHIEVEMENT) {
///         Guid target4;
///     }
///     else if (chat_type == CHANNEL) {
///         CString channel_name;
///         Guid target5;
///     }
///     else {
///         Guid target6;
///     }
///     SizedCString message;
///     PlayerChatTag tag;
///     if (chat_type == ACHIEVEMENT
///         || chat_type == GUILD_ACHIEVEMENT) {
///         u32 achievement_id;
///     }
/// }
/// ```
pub struct SMSG_MESSAGECHAT {
    pub chat_type: SMSG_MESSAGECHAT_ChatType,
    pub language: Language,
    pub sender: Guid,
    /// azerothcore sets to 0.
    pub flags: u32,
    pub message: String,
    pub tag: PlayerChatTag,
}

#[cfg(feature = "print-testcase")]
impl SMSG_MESSAGECHAT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MESSAGECHAT {{").unwrap();
        // Members
        writeln!(s, "    chat_type = {};", crate::wrath::ChatType::try_from(self.chat_type.as_int()).unwrap().as_test_case_value()).unwrap();
        writeln!(s, "    language = {};", self.language.as_test_case_value()).unwrap();
        writeln!(s, "    sender = {};", self.sender.guid()).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        match &self.chat_type {
            crate::wrath::SMSG_MESSAGECHAT_ChatType::System {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Say {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Party {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Raid {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Guild {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Officer {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Yell {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Whisper {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                sender2,
                target2,
            } => {
                writeln!(s, "    sender2 = \"{}\";", sender2).unwrap();
                writeln!(s, "    target2 = {};", target2.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::WhisperInform {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Emote {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::TextEmote {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                target1,
            } => {
                writeln!(s, "    sender1 = \"{}\";", sender1).unwrap();
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterParty {
                sender1,
                target1,
            } => {
                writeln!(s, "    sender1 = \"{}\";", sender1).unwrap();
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                target1,
            } => {
                writeln!(s, "    sender1 = \"{}\";", sender1).unwrap();
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                sender1,
                target1,
            } => {
                writeln!(s, "    sender1 = \"{}\";", sender1).unwrap();
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                sender1,
                target1,
            } => {
                writeln!(s, "    sender1 = \"{}\";", sender1).unwrap();
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target5,
            } => {
                writeln!(s, "    channel_name = \"{}\";", channel_name).unwrap();
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelList {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Afk {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Dnd {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Ignored {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Skill {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Loot {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Money {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Opening {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Tradeskills {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::PetInfo {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidLeader {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidWarning {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                sender1,
                target1,
            } => {
                writeln!(s, "    sender1 = \"{}\";", sender1).unwrap();
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender1,
                target1,
            } => {
                writeln!(s, "    sender1 = \"{}\";", sender1).unwrap();
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Filtered {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Battleground {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Restricted {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Battlenet {
                sender1,
                target1,
            } => {
                writeln!(s, "    sender1 = \"{}\";", sender1).unwrap();
                return None;
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                writeln!(s, "    target4 = {};", target4.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                writeln!(s, "    target4 = {};", target4.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::PartyLeader {
                target6,
            } => {
                writeln!(s, "    target6 = {};", target6.guid()).unwrap();
            }
        }

        writeln!(s, "    message = \"{}\";", self.message).unwrap();
        writeln!(s, "    tag = {};", self.tag.as_test_case_value()).unwrap();
        match &self.chat_type {
            crate::wrath::SMSG_MESSAGECHAT_ChatType::System {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Say {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Party {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Raid {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Guild {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Officer {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Yell {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Whisper {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                sender2,
                target2,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::WhisperInform {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Emote {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::TextEmote {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterParty {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target5,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelList {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Afk {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Dnd {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Ignored {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Skill {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Loot {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Money {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Opening {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Tradeskills {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::PetInfo {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidLeader {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidWarning {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Filtered {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Battleground {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Restricted {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Battlenet {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                writeln!(s, "    achievement_id = {};", achievement_id).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                writeln!(s, "    achievement_id = {};", achievement_id).unwrap();
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::PartyLeader {
                target6,
            } => {
            }
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 150_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "language", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "sender", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
        match &self.chat_type {
            crate::wrath::SMSG_MESSAGECHAT_ChatType::System {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Say {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Party {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Raid {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Guild {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Officer {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Yell {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Whisper {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                sender2,
                target2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender2.len() + 5, "sender2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target2", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::WhisperInform {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Emote {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::TextEmote {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender1.len() + 5, "sender1", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterParty {
                sender1,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender1.len() + 5, "sender1", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender1.len() + 5, "sender1", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                sender1,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender1.len() + 5, "sender1", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                sender1,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender1.len() + 5, "sender1", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, channel_name.len() + 1, "channel_name", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelList {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Afk {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Dnd {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Ignored {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Skill {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Loot {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Money {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Opening {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Tradeskills {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::PetInfo {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
                panic!("unsupported type NamedGuid for variable 'target3'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
                panic!("unsupported type NamedGuid for variable 'target3'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
                panic!("unsupported type NamedGuid for variable 'target3'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidLeader {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidWarning {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                sender1,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender1.len() + 5, "sender1", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender1,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender1.len() + 5, "sender1", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Filtered {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Battleground {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Restricted {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Battlenet {
                sender1,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender1.len() + 5, "sender1", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target4", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target4", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::PartyLeader {
                target6,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "    ");
            }
        }

        crate::util::write_bytes(&mut s, &mut bytes, self.message.len() + 5, "message", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "tag", "    ");
        match &self.chat_type {
            crate::wrath::SMSG_MESSAGECHAT_ChatType::System {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Say {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Party {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Raid {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Guild {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Officer {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Yell {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Whisper {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                sender2,
                target2,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::WhisperInform {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Emote {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::TextEmote {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterParty {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target5,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelList {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Afk {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Dnd {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Ignored {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Skill {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Loot {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Money {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Opening {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Tradeskills {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::PetInfo {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidLeader {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidWarning {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Filtered {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Battleground {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Restricted {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Battlenet {
                sender1,
                target1,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "achievement_id", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "achievement_id", "    ");
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                target6,
            } => {
            }
            crate::wrath::SMSG_MESSAGECHAT_ChatType::PartyLeader {
                target6,
            } => {
            }
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_MESSAGECHAT {}
impl crate::Message for SMSG_MESSAGECHAT {
    const OPCODE: u32 = 0x0096;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_MESSAGECHAT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int().to_le_bytes()))?;

        // language: Language
        w.write_all(&u32::from(self.language.as_int()).to_le_bytes())?;

        // sender: Guid
        w.write_all(&self.sender.guid().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        match &self.chat_type {
            SMSG_MESSAGECHAT_ChatType::System {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Say {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Party {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Raid {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Guild {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Officer {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Yell {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Whisper {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                sender2,
                target2,
            } => {
                // sender2: SizedCString
                w.write_all(&((sender2.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender2.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target2: Guid
                w.write_all(&target2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WhisperInform {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Emote {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::TextEmote {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterParty {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target5,
            } => {
                // channel_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
                w.write_all(channel_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelList {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Afk {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Dnd {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Ignored {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Skill {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Loot {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Money {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Opening {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Tradeskills {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::PetInfo {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
                // target3: NamedGuid
                target3.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
                // target3: NamedGuid
                target3.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
                // target3: NamedGuid
                target3.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidLeader {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidWarning {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::Filtered {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Battleground {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Restricted {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Battlenet {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::PartyLeader {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
        }

        // message: SizedCString
        w.write_all(&((self.message.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // tag: PlayerChatTag
        w.write_all(&(self.tag.as_int().to_le_bytes()))?;

        match &self.chat_type {
            SMSG_MESSAGECHAT_ChatType::System {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Say {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Party {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Raid {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Guild {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Officer {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Yell {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Whisper {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                sender2,
                target2,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::WhisperInform {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Emote {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::TextEmote {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                target1,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::MonsterParty {
                sender1,
                target1,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                target1,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                sender1,
                target1,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                sender1,
                target1,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target5,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::ChannelList {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Afk {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Dnd {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Ignored {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Skill {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Loot {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Money {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Opening {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Tradeskills {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::PetInfo {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::RaidLeader {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::RaidWarning {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                sender1,
                target1,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender1,
                target1,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Filtered {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Battleground {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Restricted {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Battlenet {
                sender1,
                target1,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                // achievement_id: u32
                w.write_all(&achievement_id.to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                // achievement_id: u32
                w.write_all(&achievement_id.to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                target6,
            } => {
            }
            SMSG_MESSAGECHAT_ChatType::PartyLeader {
                target6,
            } => {
            }
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(31..=24038).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0096, size: body_size });
        }

        let mut chat_type_if_sender1 = Default::default();
        let mut chat_type_if_target1 = Default::default();
        let mut chat_type_if_sender2 = Default::default();
        let mut chat_type_if_target2 = Default::default();
        let mut chat_type_if_target3 = Default::default();
        let mut chat_type_if_target4 = Default::default();
        let mut chat_type_if_channel_name = Default::default();
        let mut chat_type_if_target5 = Default::default();
        let mut chat_type_if_target6 = Default::default();
        let mut chat_type_if_achievement_id = Default::default();

        // chat_type: ChatType
        let chat_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // language: Language
        let language = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // sender: Guid
        let sender = crate::util::read_guid(&mut r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        match chat_type {
            ChatType::System => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Say => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Party => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Raid => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Guild => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Officer => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Yell => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Whisper => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::WhisperForeign => {
                // sender2: SizedCString
                chat_type_if_sender2 = {
                    let sender2 = crate::util::read_u32_le(&mut r)?;
                    let sender2 = crate::util::read_sized_c_string_to_vec(&mut r, sender2)?;
                    String::from_utf8(sender2)?
                };

                // target2: Guid
                chat_type_if_target2 = crate::util::read_guid(&mut r)?;

            }
            ChatType::WhisperInform => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Emote => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::TextEmote => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::MonsterSay => {
                // sender1: SizedCString
                chat_type_if_sender1 = {
                    let sender1 = crate::util::read_u32_le(&mut r)?;
                    let sender1 = crate::util::read_sized_c_string_to_vec(&mut r, sender1)?;
                    String::from_utf8(sender1)?
                };

                // target1: NamedGuid
                chat_type_if_target1 = NamedGuid::read(&mut r)?;

            }
            ChatType::MonsterParty => {
                // sender1: SizedCString
                chat_type_if_sender1 = {
                    let sender1 = crate::util::read_u32_le(&mut r)?;
                    let sender1 = crate::util::read_sized_c_string_to_vec(&mut r, sender1)?;
                    String::from_utf8(sender1)?
                };

                // target1: NamedGuid
                chat_type_if_target1 = NamedGuid::read(&mut r)?;

            }
            ChatType::MonsterYell => {
                // sender1: SizedCString
                chat_type_if_sender1 = {
                    let sender1 = crate::util::read_u32_le(&mut r)?;
                    let sender1 = crate::util::read_sized_c_string_to_vec(&mut r, sender1)?;
                    String::from_utf8(sender1)?
                };

                // target1: NamedGuid
                chat_type_if_target1 = NamedGuid::read(&mut r)?;

            }
            ChatType::MonsterWhisper => {
                // sender1: SizedCString
                chat_type_if_sender1 = {
                    let sender1 = crate::util::read_u32_le(&mut r)?;
                    let sender1 = crate::util::read_sized_c_string_to_vec(&mut r, sender1)?;
                    String::from_utf8(sender1)?
                };

                // target1: NamedGuid
                chat_type_if_target1 = NamedGuid::read(&mut r)?;

            }
            ChatType::MonsterEmote => {
                // sender1: SizedCString
                chat_type_if_sender1 = {
                    let sender1 = crate::util::read_u32_le(&mut r)?;
                    let sender1 = crate::util::read_sized_c_string_to_vec(&mut r, sender1)?;
                    String::from_utf8(sender1)?
                };

                // target1: NamedGuid
                chat_type_if_target1 = NamedGuid::read(&mut r)?;

            }
            ChatType::Channel => {
                // channel_name: CString
                chat_type_if_channel_name = {
                    let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(channel_name)?
                };

                // target5: Guid
                chat_type_if_target5 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ChannelJoin => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ChannelLeave => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ChannelList => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ChannelNotice => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ChannelNoticeUser => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Afk => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Dnd => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Ignored => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Skill => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Loot => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Money => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Opening => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Tradeskills => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::PetInfo => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::CombatMiscInfo => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::CombatXpGain => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::CombatHonorGain => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::CombatFactionChange => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::BgSystemNeutral => {
                // target3: NamedGuid
                chat_type_if_target3 = NamedGuid::read(&mut r)?;

            }
            ChatType::BgSystemAlliance => {
                // target3: NamedGuid
                chat_type_if_target3 = NamedGuid::read(&mut r)?;

            }
            ChatType::BgSystemHorde => {
                // target3: NamedGuid
                chat_type_if_target3 = NamedGuid::read(&mut r)?;

            }
            ChatType::RaidLeader => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::RaidWarning => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::RaidBossEmote => {
                // sender1: SizedCString
                chat_type_if_sender1 = {
                    let sender1 = crate::util::read_u32_le(&mut r)?;
                    let sender1 = crate::util::read_sized_c_string_to_vec(&mut r, sender1)?;
                    String::from_utf8(sender1)?
                };

                // target1: NamedGuid
                chat_type_if_target1 = NamedGuid::read(&mut r)?;

            }
            ChatType::RaidBossWhisper => {
                // sender1: SizedCString
                chat_type_if_sender1 = {
                    let sender1 = crate::util::read_u32_le(&mut r)?;
                    let sender1 = crate::util::read_sized_c_string_to_vec(&mut r, sender1)?;
                    String::from_utf8(sender1)?
                };

                // target1: NamedGuid
                chat_type_if_target1 = NamedGuid::read(&mut r)?;

            }
            ChatType::Filtered => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Battleground => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::BattlegroundLeader => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Restricted => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Battlenet => {
                // sender1: SizedCString
                chat_type_if_sender1 = {
                    let sender1 = crate::util::read_u32_le(&mut r)?;
                    let sender1 = crate::util::read_sized_c_string_to_vec(&mut r, sender1)?;
                    String::from_utf8(sender1)?
                };

                // target1: NamedGuid
                chat_type_if_target1 = NamedGuid::read(&mut r)?;

            }
            ChatType::Achievement => {
                // target4: Guid
                chat_type_if_target4 = crate::util::read_guid(&mut r)?;

            }
            ChatType::GuildAchievement => {
                // target4: Guid
                chat_type_if_target4 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ArenaPoints => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::PartyLeader => {
                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
        };

        // message: SizedCString
        let message = {
            let message = crate::util::read_u32_le(&mut r)?;
            let message = crate::util::read_sized_c_string_to_vec(&mut r, message)?;
            String::from_utf8(message)?
        };

        // tag: PlayerChatTag
        let tag = crate::util::read_u8_le(&mut r)?.try_into()?;

        match chat_type {
            ChatType::System => {
            }
            ChatType::Say => {
            }
            ChatType::Party => {
            }
            ChatType::Raid => {
            }
            ChatType::Guild => {
            }
            ChatType::Officer => {
            }
            ChatType::Yell => {
            }
            ChatType::Whisper => {
            }
            ChatType::WhisperForeign => {
            }
            ChatType::WhisperInform => {
            }
            ChatType::Emote => {
            }
            ChatType::TextEmote => {
            }
            ChatType::MonsterSay => {
            }
            ChatType::MonsterParty => {
            }
            ChatType::MonsterYell => {
            }
            ChatType::MonsterWhisper => {
            }
            ChatType::MonsterEmote => {
            }
            ChatType::Channel => {
            }
            ChatType::ChannelJoin => {
            }
            ChatType::ChannelLeave => {
            }
            ChatType::ChannelList => {
            }
            ChatType::ChannelNotice => {
            }
            ChatType::ChannelNoticeUser => {
            }
            ChatType::Afk => {
            }
            ChatType::Dnd => {
            }
            ChatType::Ignored => {
            }
            ChatType::Skill => {
            }
            ChatType::Loot => {
            }
            ChatType::Money => {
            }
            ChatType::Opening => {
            }
            ChatType::Tradeskills => {
            }
            ChatType::PetInfo => {
            }
            ChatType::CombatMiscInfo => {
            }
            ChatType::CombatXpGain => {
            }
            ChatType::CombatHonorGain => {
            }
            ChatType::CombatFactionChange => {
            }
            ChatType::BgSystemNeutral => {
            }
            ChatType::BgSystemAlliance => {
            }
            ChatType::BgSystemHorde => {
            }
            ChatType::RaidLeader => {
            }
            ChatType::RaidWarning => {
            }
            ChatType::RaidBossEmote => {
            }
            ChatType::RaidBossWhisper => {
            }
            ChatType::Filtered => {
            }
            ChatType::Battleground => {
            }
            ChatType::BattlegroundLeader => {
            }
            ChatType::Restricted => {
            }
            ChatType::Battlenet => {
            }
            ChatType::Achievement => {
                // achievement_id: u32
                chat_type_if_achievement_id = crate::util::read_u32_le(&mut r)?;

            }
            ChatType::GuildAchievement => {
                // achievement_id: u32
                chat_type_if_achievement_id = crate::util::read_u32_le(&mut r)?;

            }
            ChatType::ArenaPoints => {
            }
            ChatType::PartyLeader => {
            }
        };

        let chat_type_if = match chat_type {
            ChatType::System => {
                SMSG_MESSAGECHAT_ChatType::System {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Say => {
                SMSG_MESSAGECHAT_ChatType::Say {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Party => {
                SMSG_MESSAGECHAT_ChatType::Party {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Raid => {
                SMSG_MESSAGECHAT_ChatType::Raid {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Guild => {
                SMSG_MESSAGECHAT_ChatType::Guild {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Officer => {
                SMSG_MESSAGECHAT_ChatType::Officer {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Yell => {
                SMSG_MESSAGECHAT_ChatType::Yell {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Whisper => {
                SMSG_MESSAGECHAT_ChatType::Whisper {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::WhisperForeign => {
                SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                    sender2: chat_type_if_sender2,
                    target2: chat_type_if_target2,
                }
            }
            ChatType::WhisperInform => {
                SMSG_MESSAGECHAT_ChatType::WhisperInform {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Emote => {
                SMSG_MESSAGECHAT_ChatType::Emote {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::TextEmote => {
                SMSG_MESSAGECHAT_ChatType::TextEmote {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::MonsterSay => {
                SMSG_MESSAGECHAT_ChatType::MonsterSay {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::MonsterParty => {
                SMSG_MESSAGECHAT_ChatType::MonsterParty {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::MonsterYell => {
                SMSG_MESSAGECHAT_ChatType::MonsterYell {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::MonsterWhisper => {
                SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::MonsterEmote => {
                SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::Channel => {
                SMSG_MESSAGECHAT_ChatType::Channel {
                    channel_name: chat_type_if_channel_name,
                    target5: chat_type_if_target5,
                }
            }
            ChatType::ChannelJoin => {
                SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::ChannelLeave => {
                SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::ChannelList => {
                SMSG_MESSAGECHAT_ChatType::ChannelList {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::ChannelNotice => {
                SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::ChannelNoticeUser => {
                SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Afk => {
                SMSG_MESSAGECHAT_ChatType::Afk {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Dnd => {
                SMSG_MESSAGECHAT_ChatType::Dnd {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Ignored => {
                SMSG_MESSAGECHAT_ChatType::Ignored {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Skill => {
                SMSG_MESSAGECHAT_ChatType::Skill {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Loot => {
                SMSG_MESSAGECHAT_ChatType::Loot {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Money => {
                SMSG_MESSAGECHAT_ChatType::Money {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Opening => {
                SMSG_MESSAGECHAT_ChatType::Opening {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Tradeskills => {
                SMSG_MESSAGECHAT_ChatType::Tradeskills {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::PetInfo => {
                SMSG_MESSAGECHAT_ChatType::PetInfo {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::CombatMiscInfo => {
                SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::CombatXpGain => {
                SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::CombatHonorGain => {
                SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::CombatFactionChange => {
                SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::BgSystemNeutral => {
                SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                    target3: chat_type_if_target3,
                }
            }
            ChatType::BgSystemAlliance => {
                SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                    target3: chat_type_if_target3,
                }
            }
            ChatType::BgSystemHorde => {
                SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                    target3: chat_type_if_target3,
                }
            }
            ChatType::RaidLeader => {
                SMSG_MESSAGECHAT_ChatType::RaidLeader {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::RaidWarning => {
                SMSG_MESSAGECHAT_ChatType::RaidWarning {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::RaidBossEmote => {
                SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::RaidBossWhisper => {
                SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::Filtered => {
                SMSG_MESSAGECHAT_ChatType::Filtered {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Battleground => {
                SMSG_MESSAGECHAT_ChatType::Battleground {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::BattlegroundLeader => {
                SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Restricted => {
                SMSG_MESSAGECHAT_ChatType::Restricted {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Battlenet => {
                SMSG_MESSAGECHAT_ChatType::Battlenet {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::Achievement => {
                SMSG_MESSAGECHAT_ChatType::Achievement {
                    achievement_id: chat_type_if_achievement_id,
                    target4: chat_type_if_target4,
                }
            }
            ChatType::GuildAchievement => {
                SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                    achievement_id: chat_type_if_achievement_id,
                    target4: chat_type_if_target4,
                }
            }
            ChatType::ArenaPoints => {
                SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                    target6: chat_type_if_target6,
                }
            }
            ChatType::PartyLeader => {
                SMSG_MESSAGECHAT_ChatType::PartyLeader {
                    target6: chat_type_if_target6,
                }
            }
        };

        Ok(Self {
            chat_type: chat_type_if,
            language,
            sender,
            flags,
            message,
            tag,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MESSAGECHAT {}

impl SMSG_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: SMSG_MESSAGECHAT_ChatType
        + 4 // language: Language
        + 8 // sender: Guid
        + 4 // flags: u32
        + self.message.len() + 5 // message: SizedCString
        + 1 // tag: PlayerChatTag
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_MESSAGECHAT_ChatType {
    System {
        target6: Guid,
    },
    Say {
        target6: Guid,
    },
    Party {
        target6: Guid,
    },
    Raid {
        target6: Guid,
    },
    Guild {
        target6: Guid,
    },
    Officer {
        target6: Guid,
    },
    Yell {
        target6: Guid,
    },
    Whisper {
        target6: Guid,
    },
    WhisperForeign {
        sender2: String,
        target2: Guid,
    },
    WhisperInform {
        target6: Guid,
    },
    Emote {
        target6: Guid,
    },
    TextEmote {
        target6: Guid,
    },
    MonsterSay {
        sender1: String,
        target1: NamedGuid,
    },
    MonsterParty {
        sender1: String,
        target1: NamedGuid,
    },
    MonsterYell {
        sender1: String,
        target1: NamedGuid,
    },
    MonsterWhisper {
        sender1: String,
        target1: NamedGuid,
    },
    MonsterEmote {
        sender1: String,
        target1: NamedGuid,
    },
    Channel {
        channel_name: String,
        target5: Guid,
    },
    ChannelJoin {
        target6: Guid,
    },
    ChannelLeave {
        target6: Guid,
    },
    ChannelList {
        target6: Guid,
    },
    ChannelNotice {
        target6: Guid,
    },
    ChannelNoticeUser {
        target6: Guid,
    },
    Afk {
        target6: Guid,
    },
    Dnd {
        target6: Guid,
    },
    Ignored {
        target6: Guid,
    },
    Skill {
        target6: Guid,
    },
    Loot {
        target6: Guid,
    },
    Money {
        target6: Guid,
    },
    Opening {
        target6: Guid,
    },
    Tradeskills {
        target6: Guid,
    },
    PetInfo {
        target6: Guid,
    },
    CombatMiscInfo {
        target6: Guid,
    },
    CombatXpGain {
        target6: Guid,
    },
    CombatHonorGain {
        target6: Guid,
    },
    CombatFactionChange {
        target6: Guid,
    },
    BgSystemNeutral {
        target3: NamedGuid,
    },
    BgSystemAlliance {
        target3: NamedGuid,
    },
    BgSystemHorde {
        target3: NamedGuid,
    },
    RaidLeader {
        target6: Guid,
    },
    RaidWarning {
        target6: Guid,
    },
    RaidBossEmote {
        sender1: String,
        target1: NamedGuid,
    },
    RaidBossWhisper {
        sender1: String,
        target1: NamedGuid,
    },
    Filtered {
        target6: Guid,
    },
    Battleground {
        target6: Guid,
    },
    BattlegroundLeader {
        target6: Guid,
    },
    Restricted {
        target6: Guid,
    },
    Battlenet {
        sender1: String,
        target1: NamedGuid,
    },
    Achievement {
        achievement_id: u32,
        target4: Guid,
    },
    GuildAchievement {
        achievement_id: u32,
        target4: Guid,
    },
    ArenaPoints {
        target6: Guid,
    },
    PartyLeader {
        target6: Guid,
    },
}

impl Default for SMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::System {
            target6: Default::default(),
        }
    }
}

impl SMSG_MESSAGECHAT_ChatType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::System { .. } => 0,
            Self::Say { .. } => 1,
            Self::Party { .. } => 2,
            Self::Raid { .. } => 3,
            Self::Guild { .. } => 4,
            Self::Officer { .. } => 5,
            Self::Yell { .. } => 6,
            Self::Whisper { .. } => 7,
            Self::WhisperForeign { .. } => 8,
            Self::WhisperInform { .. } => 9,
            Self::Emote { .. } => 10,
            Self::TextEmote { .. } => 11,
            Self::MonsterSay { .. } => 12,
            Self::MonsterParty { .. } => 13,
            Self::MonsterYell { .. } => 14,
            Self::MonsterWhisper { .. } => 15,
            Self::MonsterEmote { .. } => 16,
            Self::Channel { .. } => 17,
            Self::ChannelJoin { .. } => 18,
            Self::ChannelLeave { .. } => 19,
            Self::ChannelList { .. } => 20,
            Self::ChannelNotice { .. } => 21,
            Self::ChannelNoticeUser { .. } => 22,
            Self::Afk { .. } => 23,
            Self::Dnd { .. } => 24,
            Self::Ignored { .. } => 25,
            Self::Skill { .. } => 26,
            Self::Loot { .. } => 27,
            Self::Money { .. } => 28,
            Self::Opening { .. } => 29,
            Self::Tradeskills { .. } => 30,
            Self::PetInfo { .. } => 31,
            Self::CombatMiscInfo { .. } => 32,
            Self::CombatXpGain { .. } => 33,
            Self::CombatHonorGain { .. } => 34,
            Self::CombatFactionChange { .. } => 35,
            Self::BgSystemNeutral { .. } => 36,
            Self::BgSystemAlliance { .. } => 37,
            Self::BgSystemHorde { .. } => 38,
            Self::RaidLeader { .. } => 39,
            Self::RaidWarning { .. } => 40,
            Self::RaidBossEmote { .. } => 41,
            Self::RaidBossWhisper { .. } => 42,
            Self::Filtered { .. } => 43,
            Self::Battleground { .. } => 44,
            Self::BattlegroundLeader { .. } => 45,
            Self::Restricted { .. } => 46,
            Self::Battlenet { .. } => 47,
            Self::Achievement { .. } => 48,
            Self::GuildAchievement { .. } => 49,
            Self::ArenaPoints { .. } => 50,
            Self::PartyLeader { .. } => 51,
        }
    }

}

impl std::fmt::Display for SMSG_MESSAGECHAT_ChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::System{ .. } => f.write_str("System"),
            Self::Say{ .. } => f.write_str("Say"),
            Self::Party{ .. } => f.write_str("Party"),
            Self::Raid{ .. } => f.write_str("Raid"),
            Self::Guild{ .. } => f.write_str("Guild"),
            Self::Officer{ .. } => f.write_str("Officer"),
            Self::Yell{ .. } => f.write_str("Yell"),
            Self::Whisper{ .. } => f.write_str("Whisper"),
            Self::WhisperForeign{ .. } => f.write_str("WhisperForeign"),
            Self::WhisperInform{ .. } => f.write_str("WhisperInform"),
            Self::Emote{ .. } => f.write_str("Emote"),
            Self::TextEmote{ .. } => f.write_str("TextEmote"),
            Self::MonsterSay{ .. } => f.write_str("MonsterSay"),
            Self::MonsterParty{ .. } => f.write_str("MonsterParty"),
            Self::MonsterYell{ .. } => f.write_str("MonsterYell"),
            Self::MonsterWhisper{ .. } => f.write_str("MonsterWhisper"),
            Self::MonsterEmote{ .. } => f.write_str("MonsterEmote"),
            Self::Channel{ .. } => f.write_str("Channel"),
            Self::ChannelJoin{ .. } => f.write_str("ChannelJoin"),
            Self::ChannelLeave{ .. } => f.write_str("ChannelLeave"),
            Self::ChannelList{ .. } => f.write_str("ChannelList"),
            Self::ChannelNotice{ .. } => f.write_str("ChannelNotice"),
            Self::ChannelNoticeUser{ .. } => f.write_str("ChannelNoticeUser"),
            Self::Afk{ .. } => f.write_str("Afk"),
            Self::Dnd{ .. } => f.write_str("Dnd"),
            Self::Ignored{ .. } => f.write_str("Ignored"),
            Self::Skill{ .. } => f.write_str("Skill"),
            Self::Loot{ .. } => f.write_str("Loot"),
            Self::Money{ .. } => f.write_str("Money"),
            Self::Opening{ .. } => f.write_str("Opening"),
            Self::Tradeskills{ .. } => f.write_str("Tradeskills"),
            Self::PetInfo{ .. } => f.write_str("PetInfo"),
            Self::CombatMiscInfo{ .. } => f.write_str("CombatMiscInfo"),
            Self::CombatXpGain{ .. } => f.write_str("CombatXpGain"),
            Self::CombatHonorGain{ .. } => f.write_str("CombatHonorGain"),
            Self::CombatFactionChange{ .. } => f.write_str("CombatFactionChange"),
            Self::BgSystemNeutral{ .. } => f.write_str("BgSystemNeutral"),
            Self::BgSystemAlliance{ .. } => f.write_str("BgSystemAlliance"),
            Self::BgSystemHorde{ .. } => f.write_str("BgSystemHorde"),
            Self::RaidLeader{ .. } => f.write_str("RaidLeader"),
            Self::RaidWarning{ .. } => f.write_str("RaidWarning"),
            Self::RaidBossEmote{ .. } => f.write_str("RaidBossEmote"),
            Self::RaidBossWhisper{ .. } => f.write_str("RaidBossWhisper"),
            Self::Filtered{ .. } => f.write_str("Filtered"),
            Self::Battleground{ .. } => f.write_str("Battleground"),
            Self::BattlegroundLeader{ .. } => f.write_str("BattlegroundLeader"),
            Self::Restricted{ .. } => f.write_str("Restricted"),
            Self::Battlenet{ .. } => f.write_str("Battlenet"),
            Self::Achievement{ .. } => f.write_str("Achievement"),
            Self::GuildAchievement{ .. } => f.write_str("GuildAchievement"),
            Self::ArenaPoints{ .. } => f.write_str("ArenaPoints"),
            Self::PartyLeader{ .. } => f.write_str("PartyLeader"),
        }
    }
}

impl SMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::System {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Say {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Party {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Raid {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Guild {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Officer {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Yell {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Whisper {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::WhisperForeign {
                sender2,
                ..
            } => {
                1
                + sender2.len() + 5 // sender2: SizedCString
                + 8 // target2: Guid
            }
            Self::WhisperInform {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Emote {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::TextEmote {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::MonsterSay {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterParty {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterYell {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterWhisper {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterEmote {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::Channel {
                channel_name,
                ..
            } => {
                1
                + channel_name.len() + 1 // channel_name: CString
                + 8 // target5: Guid
            }
            Self::ChannelJoin {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::ChannelLeave {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::ChannelList {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::ChannelNotice {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::ChannelNoticeUser {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Afk {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Dnd {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Ignored {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Skill {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Loot {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Money {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Opening {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Tradeskills {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::PetInfo {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::CombatMiscInfo {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::CombatXpGain {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::CombatHonorGain {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::CombatFactionChange {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::BgSystemNeutral {
                target3,
            } => {
                1
                + target3.size() // target3: NamedGuid
            }
            Self::BgSystemAlliance {
                target3,
            } => {
                1
                + target3.size() // target3: NamedGuid
            }
            Self::BgSystemHorde {
                target3,
            } => {
                1
                + target3.size() // target3: NamedGuid
            }
            Self::RaidLeader {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::RaidWarning {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::RaidBossEmote {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::RaidBossWhisper {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::Filtered {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Battleground {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::BattlegroundLeader {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Restricted {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Battlenet {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::Achievement {
                ..
            } => {
                1
                + 4 // achievement_id: u32
                + 8 // target4: Guid
            }
            Self::GuildAchievement {
                ..
            } => {
                1
                + 4 // achievement_id: u32
                + 8 // target4: Guid
            }
            Self::ArenaPoints {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
            Self::PartyLeader {
                ..
            } => {
                1
                + 8 // target6: Guid
            }
        }
    }
}

