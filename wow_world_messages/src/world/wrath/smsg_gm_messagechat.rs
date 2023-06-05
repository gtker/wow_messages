use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    ChatType, Language, NamedGuid, PlayerChatTag,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_gm_messagechat.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_gm_messagechat.wowm#L40):
/// ```text
/// smsg SMSG_GM_MESSAGECHAT = 0x03B3 {
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
///         SizedCString sender_name;
///         Guid target6;
///     }
///     SizedCString message;
///     PlayerChatTag chat_tag;
///     if (chat_type == ACHIEVEMENT
///         || chat_type == GUILD_ACHIEVEMENT) {
///         u32 achievement_id;
///     }
/// }
/// ```
pub struct SMSG_GM_MESSAGECHAT {
    pub chat_type: SMSG_GM_MESSAGECHAT_ChatType,
    pub language: Language,
    pub sender: Guid,
    /// azerothcore sets to 0.
    ///
    pub flags: u32,
    pub message: String,
    pub chat_tag: PlayerChatTag,
}

impl crate::private::Sealed for SMSG_GM_MESSAGECHAT {}
impl crate::Message for SMSG_GM_MESSAGECHAT {
    const OPCODE: u32 = 0x03b3;

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
            SMSG_GM_MESSAGECHAT_ChatType::System {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Say {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Party {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Raid {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Guild {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Officer {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Yell {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Whisper {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::WhisperForeign {
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
            SMSG_GM_MESSAGECHAT_ChatType::WhisperInform {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Emote {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::TextEmote {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterSay {
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
            SMSG_GM_MESSAGECHAT_ChatType::MonsterParty {
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
            SMSG_GM_MESSAGECHAT_ChatType::MonsterYell {
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
            SMSG_GM_MESSAGECHAT_ChatType::MonsterWhisper {
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
            SMSG_GM_MESSAGECHAT_ChatType::MonsterEmote {
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
            SMSG_GM_MESSAGECHAT_ChatType::Channel {
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
            SMSG_GM_MESSAGECHAT_ChatType::ChannelJoin {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelLeave {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelList {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelNotice {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelNoticeUser {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Afk {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Dnd {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Ignored {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Skill {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Loot {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Money {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Opening {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Tradeskills {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::PetInfo {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatMiscInfo {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatXpGain {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatHonorGain {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatFactionChange {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
                // target3: NamedGuid
                target3.write_into_vec(&mut w)?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
                // target3: NamedGuid
                target3.write_into_vec(&mut w)?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
                // target3: NamedGuid
                target3.write_into_vec(&mut w)?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidLeader {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidWarning {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidBossEmote {
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
            SMSG_GM_MESSAGECHAT_ChatType::RaidBossWhisper {
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
            SMSG_GM_MESSAGECHAT_ChatType::Filtered {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Battleground {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::BattlegroundLeader {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Restricted {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Battlenet {
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
            SMSG_GM_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ArenaPoints {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::PartyLeader {
                sender_name,
                target6,
            } => {
                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
        }

        // message: SizedCString
        w.write_all(&((self.message.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // chat_tag: PlayerChatTag
        w.write_all(&(self.chat_tag.as_int().to_le_bytes()))?;

        match &self.chat_type {
            SMSG_GM_MESSAGECHAT_ChatType::System {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Say {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Party {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Raid {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Guild {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Officer {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Yell {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Whisper {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::WhisperForeign {
                sender2,
                target2,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::WhisperInform {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Emote {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::TextEmote {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                target1,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterParty {
                sender1,
                target1,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                target1,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterWhisper {
                sender1,
                target1,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterEmote {
                sender1,
                target1,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target5,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelJoin {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelLeave {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelList {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelNotice {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelNoticeUser {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Afk {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Dnd {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Ignored {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Skill {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Loot {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Money {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Opening {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Tradeskills {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::PetInfo {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatMiscInfo {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatXpGain {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatHonorGain {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatFactionChange {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidLeader {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidWarning {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidBossEmote {
                sender1,
                target1,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender1,
                target1,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Filtered {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Battleground {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::BattlegroundLeader {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Restricted {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Battlenet {
                sender1,
                target1,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                // achievement_id: u32
                w.write_all(&achievement_id.to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                // achievement_id: u32
                w.write_all(&achievement_id.to_le_bytes())?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ArenaPoints {
                sender_name,
                target6,
            } => {
            }
            SMSG_GM_MESSAGECHAT_ChatType::PartyLeader {
                sender_name,
                target6,
            } => {
            }
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(31..=24038).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03B3, size: body_size });
        }

        let mut chat_type_if_sender1 = Default::default();
        let mut chat_type_if_target1 = Default::default();
        let mut chat_type_if_sender2 = Default::default();
        let mut chat_type_if_target2 = Default::default();
        let mut chat_type_if_target3 = Default::default();
        let mut chat_type_if_target4 = Default::default();
        let mut chat_type_if_channel_name = Default::default();
        let mut chat_type_if_target5 = Default::default();
        let mut chat_type_if_sender_name = Default::default();
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
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Say => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Party => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Raid => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Guild => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Officer => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Yell => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Whisper => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

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
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Emote => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::TextEmote => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

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
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ChannelLeave => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ChannelList => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ChannelNotice => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::ChannelNoticeUser => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Afk => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Dnd => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Ignored => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Skill => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Loot => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Money => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Opening => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Tradeskills => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::PetInfo => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::CombatMiscInfo => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::CombatXpGain => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::CombatHonorGain => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::CombatFactionChange => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

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
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::RaidWarning => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

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
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Battleground => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::BattlegroundLeader => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::Restricted => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

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
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target6: Guid
                chat_type_if_target6 = crate::util::read_guid(&mut r)?;

            }
            ChatType::PartyLeader => {
                // sender_name: SizedCString
                chat_type_if_sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

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

        // chat_tag: PlayerChatTag
        let chat_tag = crate::util::read_u8_le(&mut r)?.try_into()?;

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
                SMSG_GM_MESSAGECHAT_ChatType::System {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Say => {
                SMSG_GM_MESSAGECHAT_ChatType::Say {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Party => {
                SMSG_GM_MESSAGECHAT_ChatType::Party {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Raid => {
                SMSG_GM_MESSAGECHAT_ChatType::Raid {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Guild => {
                SMSG_GM_MESSAGECHAT_ChatType::Guild {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Officer => {
                SMSG_GM_MESSAGECHAT_ChatType::Officer {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Yell => {
                SMSG_GM_MESSAGECHAT_ChatType::Yell {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Whisper => {
                SMSG_GM_MESSAGECHAT_ChatType::Whisper {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::WhisperForeign => {
                SMSG_GM_MESSAGECHAT_ChatType::WhisperForeign {
                    sender2: chat_type_if_sender2,
                    target2: chat_type_if_target2,
                }
            }
            ChatType::WhisperInform => {
                SMSG_GM_MESSAGECHAT_ChatType::WhisperInform {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Emote => {
                SMSG_GM_MESSAGECHAT_ChatType::Emote {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::TextEmote => {
                SMSG_GM_MESSAGECHAT_ChatType::TextEmote {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::MonsterSay => {
                SMSG_GM_MESSAGECHAT_ChatType::MonsterSay {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::MonsterParty => {
                SMSG_GM_MESSAGECHAT_ChatType::MonsterParty {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::MonsterYell => {
                SMSG_GM_MESSAGECHAT_ChatType::MonsterYell {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::MonsterWhisper => {
                SMSG_GM_MESSAGECHAT_ChatType::MonsterWhisper {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::MonsterEmote => {
                SMSG_GM_MESSAGECHAT_ChatType::MonsterEmote {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::Channel => {
                SMSG_GM_MESSAGECHAT_ChatType::Channel {
                    channel_name: chat_type_if_channel_name,
                    target5: chat_type_if_target5,
                }
            }
            ChatType::ChannelJoin => {
                SMSG_GM_MESSAGECHAT_ChatType::ChannelJoin {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::ChannelLeave => {
                SMSG_GM_MESSAGECHAT_ChatType::ChannelLeave {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::ChannelList => {
                SMSG_GM_MESSAGECHAT_ChatType::ChannelList {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::ChannelNotice => {
                SMSG_GM_MESSAGECHAT_ChatType::ChannelNotice {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::ChannelNoticeUser => {
                SMSG_GM_MESSAGECHAT_ChatType::ChannelNoticeUser {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Afk => {
                SMSG_GM_MESSAGECHAT_ChatType::Afk {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Dnd => {
                SMSG_GM_MESSAGECHAT_ChatType::Dnd {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Ignored => {
                SMSG_GM_MESSAGECHAT_ChatType::Ignored {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Skill => {
                SMSG_GM_MESSAGECHAT_ChatType::Skill {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Loot => {
                SMSG_GM_MESSAGECHAT_ChatType::Loot {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Money => {
                SMSG_GM_MESSAGECHAT_ChatType::Money {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Opening => {
                SMSG_GM_MESSAGECHAT_ChatType::Opening {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Tradeskills => {
                SMSG_GM_MESSAGECHAT_ChatType::Tradeskills {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::PetInfo => {
                SMSG_GM_MESSAGECHAT_ChatType::PetInfo {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::CombatMiscInfo => {
                SMSG_GM_MESSAGECHAT_ChatType::CombatMiscInfo {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::CombatXpGain => {
                SMSG_GM_MESSAGECHAT_ChatType::CombatXpGain {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::CombatHonorGain => {
                SMSG_GM_MESSAGECHAT_ChatType::CombatHonorGain {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::CombatFactionChange => {
                SMSG_GM_MESSAGECHAT_ChatType::CombatFactionChange {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::BgSystemNeutral => {
                SMSG_GM_MESSAGECHAT_ChatType::BgSystemNeutral {
                    target3: chat_type_if_target3,
                }
            }
            ChatType::BgSystemAlliance => {
                SMSG_GM_MESSAGECHAT_ChatType::BgSystemAlliance {
                    target3: chat_type_if_target3,
                }
            }
            ChatType::BgSystemHorde => {
                SMSG_GM_MESSAGECHAT_ChatType::BgSystemHorde {
                    target3: chat_type_if_target3,
                }
            }
            ChatType::RaidLeader => {
                SMSG_GM_MESSAGECHAT_ChatType::RaidLeader {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::RaidWarning => {
                SMSG_GM_MESSAGECHAT_ChatType::RaidWarning {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::RaidBossEmote => {
                SMSG_GM_MESSAGECHAT_ChatType::RaidBossEmote {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::RaidBossWhisper => {
                SMSG_GM_MESSAGECHAT_ChatType::RaidBossWhisper {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::Filtered => {
                SMSG_GM_MESSAGECHAT_ChatType::Filtered {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Battleground => {
                SMSG_GM_MESSAGECHAT_ChatType::Battleground {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::BattlegroundLeader => {
                SMSG_GM_MESSAGECHAT_ChatType::BattlegroundLeader {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Restricted => {
                SMSG_GM_MESSAGECHAT_ChatType::Restricted {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::Battlenet => {
                SMSG_GM_MESSAGECHAT_ChatType::Battlenet {
                    sender1: chat_type_if_sender1,
                    target1: chat_type_if_target1,
                }
            }
            ChatType::Achievement => {
                SMSG_GM_MESSAGECHAT_ChatType::Achievement {
                    achievement_id: chat_type_if_achievement_id,
                    target4: chat_type_if_target4,
                }
            }
            ChatType::GuildAchievement => {
                SMSG_GM_MESSAGECHAT_ChatType::GuildAchievement {
                    achievement_id: chat_type_if_achievement_id,
                    target4: chat_type_if_target4,
                }
            }
            ChatType::ArenaPoints => {
                SMSG_GM_MESSAGECHAT_ChatType::ArenaPoints {
                    sender_name: chat_type_if_sender_name,
                    target6: chat_type_if_target6,
                }
            }
            ChatType::PartyLeader => {
                SMSG_GM_MESSAGECHAT_ChatType::PartyLeader {
                    sender_name: chat_type_if_sender_name,
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
            chat_tag,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GM_MESSAGECHAT {}

impl SMSG_GM_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: SMSG_GM_MESSAGECHAT_ChatType
        + 4 // language: Language
        + 8 // sender: Guid
        + 4 // flags: u32
        + self.message.len() + 5 // message: SizedCString
        + 1 // chat_tag: PlayerChatTag
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_GM_MESSAGECHAT_ChatType {
    System {
        sender_name: String,
        target6: Guid,
    },
    Say {
        sender_name: String,
        target6: Guid,
    },
    Party {
        sender_name: String,
        target6: Guid,
    },
    Raid {
        sender_name: String,
        target6: Guid,
    },
    Guild {
        sender_name: String,
        target6: Guid,
    },
    Officer {
        sender_name: String,
        target6: Guid,
    },
    Yell {
        sender_name: String,
        target6: Guid,
    },
    Whisper {
        sender_name: String,
        target6: Guid,
    },
    WhisperForeign {
        sender2: String,
        target2: Guid,
    },
    WhisperInform {
        sender_name: String,
        target6: Guid,
    },
    Emote {
        sender_name: String,
        target6: Guid,
    },
    TextEmote {
        sender_name: String,
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
        sender_name: String,
        target6: Guid,
    },
    ChannelLeave {
        sender_name: String,
        target6: Guid,
    },
    ChannelList {
        sender_name: String,
        target6: Guid,
    },
    ChannelNotice {
        sender_name: String,
        target6: Guid,
    },
    ChannelNoticeUser {
        sender_name: String,
        target6: Guid,
    },
    Afk {
        sender_name: String,
        target6: Guid,
    },
    Dnd {
        sender_name: String,
        target6: Guid,
    },
    Ignored {
        sender_name: String,
        target6: Guid,
    },
    Skill {
        sender_name: String,
        target6: Guid,
    },
    Loot {
        sender_name: String,
        target6: Guid,
    },
    Money {
        sender_name: String,
        target6: Guid,
    },
    Opening {
        sender_name: String,
        target6: Guid,
    },
    Tradeskills {
        sender_name: String,
        target6: Guid,
    },
    PetInfo {
        sender_name: String,
        target6: Guid,
    },
    CombatMiscInfo {
        sender_name: String,
        target6: Guid,
    },
    CombatXpGain {
        sender_name: String,
        target6: Guid,
    },
    CombatHonorGain {
        sender_name: String,
        target6: Guid,
    },
    CombatFactionChange {
        sender_name: String,
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
        sender_name: String,
        target6: Guid,
    },
    RaidWarning {
        sender_name: String,
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
        sender_name: String,
        target6: Guid,
    },
    Battleground {
        sender_name: String,
        target6: Guid,
    },
    BattlegroundLeader {
        sender_name: String,
        target6: Guid,
    },
    Restricted {
        sender_name: String,
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
        sender_name: String,
        target6: Guid,
    },
    PartyLeader {
        sender_name: String,
        target6: Guid,
    },
}

impl Default for SMSG_GM_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::System {
            sender_name: Default::default(),
            target6: Default::default(),
        }
    }
}

impl SMSG_GM_MESSAGECHAT_ChatType {
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

impl std::fmt::Display for SMSG_GM_MESSAGECHAT_ChatType {
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

impl SMSG_GM_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::System {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Say {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Party {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Raid {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Guild {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Officer {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Yell {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Whisper {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
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
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Emote {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::TextEmote {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
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
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::ChannelLeave {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::ChannelList {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::ChannelNotice {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::ChannelNoticeUser {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Afk {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Dnd {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Ignored {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Skill {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Loot {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Money {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Opening {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Tradeskills {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::PetInfo {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::CombatMiscInfo {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::CombatXpGain {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::CombatHonorGain {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::CombatFactionChange {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
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
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::RaidWarning {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
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
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Battleground {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::BattlegroundLeader {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::Restricted {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
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
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
            Self::PartyLeader {
                sender_name,
                ..
            } => {
                1
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target6: Guid
            }
        }
    }
}

