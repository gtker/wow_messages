use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    ChatType, Language, NamedGuid, PlayerChatTag,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_messagechat.wowm:86`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_messagechat.wowm#L86):
/// ```text
/// smsg SMSG_MESSAGECHAT = 0x0096 {
///     ChatType chat_type;
///     (u32)Language language;
///     if (chat_type == MONSTER_SAY
///         || chat_type == MONSTER_PARTY
///         || chat_type == MONSTER_YELL
///         || chat_type == MONSTER_WHISPER
///         || chat_type == RAID_BOSS_WHISPER
///         || chat_type == RAID_BOSS_EMOTE
///         || chat_type == MONSTER_EMOTE) {
///         SizedCString sender;
///         NamedGuid target1;
///     }
///     else if (chat_type == BG_SYSTEM_NEUTRAL
///         || chat_type == BG_SYSTEM_ALLIANCE
///         || chat_type == BG_SYSTEM_HORDE) {
///         NamedGuid target2;
///     }
///     else if (chat_type == CHANNEL) {
///         CString channel_name;
///         Guid target4;
///     }
///     else {
///         Guid target5;
///     }
///     SizedCString message;
///     PlayerChatTag tag;
/// }
/// ```
pub struct SMSG_MESSAGECHAT {
    pub chat_type: SMSG_MESSAGECHAT_ChatType,
    pub language: Language,
    pub message: String,
    pub tag: PlayerChatTag,
}

impl crate::private::Sealed for SMSG_MESSAGECHAT {}
impl crate::Message for SMSG_MESSAGECHAT {
    const OPCODE: u32 = 0x0096;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&u8::from(self.chat_type.as_int()).to_le_bytes())?;

        // language: Language
        w.write_all(&u32::from(self.language.as_int()).to_le_bytes())?;

        match &self.chat_type {
            SMSG_MESSAGECHAT_ChatType::System {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Say {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Party {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Raid {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Guild {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Officer {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Yell {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Whisper {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WhisperInform {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Reply {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Emote {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::TextEmote {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterParty {
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target4,
            } => {
                // channel_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
                w.write_all(channel_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelList {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Afk {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Dnd {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Ignored {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Skill {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Loot {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Money {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Opening {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Tradeskills {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::PetInfo {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                target2,
            } => {
                // target2: NamedGuid
                target2.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                target2,
            } => {
                // target2: NamedGuid
                target2.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                target2,
            } => {
                // target2: NamedGuid
                target2.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidLeader {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidWarning {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

            }
            SMSG_MESSAGECHAT_ChatType::Filtered {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Battleground {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Restricted {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
        }

        // message: SizedCString
        w.write_all(&((self.message.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // tag: PlayerChatTag
        w.write_all(&u8::from(self.tag.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(19..=24022).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0096, size: body_size as u32 });
        }

        // chat_type: ChatType
        let chat_type: ChatType = crate::util::read_u8_le(&mut r)?.try_into()?;

        // language: Language
        let language: Language = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        let chat_type_if = match chat_type {
            ChatType::System => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::System {
                    target5,
                }
            }
            ChatType::Say => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Say {
                    target5,
                }
            }
            ChatType::Party => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Party {
                    target5,
                }
            }
            ChatType::Raid => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Raid {
                    target5,
                }
            }
            ChatType::Guild => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Guild {
                    target5,
                }
            }
            ChatType::Officer => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Officer {
                    target5,
                }
            }
            ChatType::Yell => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Yell {
                    target5,
                }
            }
            ChatType::Whisper => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Whisper {
                    target5,
                }
            }
            ChatType::WhisperInform => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::WhisperInform {
                    target5,
                }
            }
            ChatType::Reply => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Reply {
                    target5,
                }
            }
            ChatType::Emote => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Emote {
                    target5,
                }
            }
            ChatType::TextEmote => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::TextEmote {
                    target5,
                }
            }
            ChatType::MonsterSay => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterSay {
                    sender,
                    target1,
                }
            }
            ChatType::MonsterParty => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterParty {
                    sender,
                    target1,
                }
            }
            ChatType::MonsterYell => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterYell {
                    sender,
                    target1,
                }
            }
            ChatType::MonsterWhisper => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                    sender,
                    target1,
                }
            }
            ChatType::MonsterEmote => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                    sender,
                    target1,
                }
            }
            ChatType::Channel => {
                // channel_name: CString
                let channel_name = {
                    let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(channel_name)?
                };

                // target4: Guid
                let target4 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Channel {
                    channel_name,
                    target4,
                }
            }
            ChatType::ChannelJoin => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                    target5,
                }
            }
            ChatType::ChannelLeave => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                    target5,
                }
            }
            ChatType::ChannelList => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelList {
                    target5,
                }
            }
            ChatType::ChannelNotice => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                    target5,
                }
            }
            ChatType::ChannelNoticeUser => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                    target5,
                }
            }
            ChatType::Afk => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Afk {
                    target5,
                }
            }
            ChatType::Dnd => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Dnd {
                    target5,
                }
            }
            ChatType::Ignored => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Ignored {
                    target5,
                }
            }
            ChatType::Skill => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Skill {
                    target5,
                }
            }
            ChatType::Loot => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Loot {
                    target5,
                }
            }
            ChatType::Money => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Money {
                    target5,
                }
            }
            ChatType::Opening => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Opening {
                    target5,
                }
            }
            ChatType::Tradeskills => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Tradeskills {
                    target5,
                }
            }
            ChatType::PetInfo => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::PetInfo {
                    target5,
                }
            }
            ChatType::CombatMiscInfo => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                    target5,
                }
            }
            ChatType::CombatXpGain => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                    target5,
                }
            }
            ChatType::CombatHonorGain => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                    target5,
                }
            }
            ChatType::CombatFactionChange => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                    target5,
                }
            }
            ChatType::BgSystemNeutral => {
                // target2: NamedGuid
                let target2 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                    target2,
                }
            }
            ChatType::BgSystemAlliance => {
                // target2: NamedGuid
                let target2 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                    target2,
                }
            }
            ChatType::BgSystemHorde => {
                // target2: NamedGuid
                let target2 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                    target2,
                }
            }
            ChatType::RaidLeader => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::RaidLeader {
                    target5,
                }
            }
            ChatType::RaidWarning => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::RaidWarning {
                    target5,
                }
            }
            ChatType::RaidBossWhisper => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                    sender,
                    target1,
                }
            }
            ChatType::RaidBossEmote => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                    sender,
                    target1,
                }
            }
            ChatType::Filtered => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Filtered {
                    target5,
                }
            }
            ChatType::Battleground => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Battleground {
                    target5,
                }
            }
            ChatType::BattlegroundLeader => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                    target5,
                }
            }
            ChatType::Restricted => {
                // target5: Guid
                let target5 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Restricted {
                    target5,
                }
            }
        };

        // message: SizedCString
        let message = {
            let message = crate::util::read_u32_le(&mut r)?;
            let message = crate::util::read_sized_c_string_to_vec(&mut r, message)?;
            String::from_utf8(message)?
        };

        // tag: PlayerChatTag
        let tag: PlayerChatTag = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            chat_type: chat_type_if,
            language,
            message,
            tag,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MESSAGECHAT {}

impl SMSG_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: SMSG_MESSAGECHAT_ChatType
        + 4 // language: Language
        + self.message.len() + 5 // message: SizedCString
        + 1 // tag: PlayerChatTag
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_MESSAGECHAT_ChatType {
    System {
        target5: Guid,
    },
    Say {
        target5: Guid,
    },
    Party {
        target5: Guid,
    },
    Raid {
        target5: Guid,
    },
    Guild {
        target5: Guid,
    },
    Officer {
        target5: Guid,
    },
    Yell {
        target5: Guid,
    },
    Whisper {
        target5: Guid,
    },
    WhisperInform {
        target5: Guid,
    },
    Reply {
        target5: Guid,
    },
    Emote {
        target5: Guid,
    },
    TextEmote {
        target5: Guid,
    },
    MonsterSay {
        sender: String,
        target1: NamedGuid,
    },
    MonsterParty {
        sender: String,
        target1: NamedGuid,
    },
    MonsterYell {
        sender: String,
        target1: NamedGuid,
    },
    MonsterWhisper {
        sender: String,
        target1: NamedGuid,
    },
    MonsterEmote {
        sender: String,
        target1: NamedGuid,
    },
    Channel {
        channel_name: String,
        target4: Guid,
    },
    ChannelJoin {
        target5: Guid,
    },
    ChannelLeave {
        target5: Guid,
    },
    ChannelList {
        target5: Guid,
    },
    ChannelNotice {
        target5: Guid,
    },
    ChannelNoticeUser {
        target5: Guid,
    },
    Afk {
        target5: Guid,
    },
    Dnd {
        target5: Guid,
    },
    Ignored {
        target5: Guid,
    },
    Skill {
        target5: Guid,
    },
    Loot {
        target5: Guid,
    },
    Money {
        target5: Guid,
    },
    Opening {
        target5: Guid,
    },
    Tradeskills {
        target5: Guid,
    },
    PetInfo {
        target5: Guid,
    },
    CombatMiscInfo {
        target5: Guid,
    },
    CombatXpGain {
        target5: Guid,
    },
    CombatHonorGain {
        target5: Guid,
    },
    CombatFactionChange {
        target5: Guid,
    },
    BgSystemNeutral {
        target2: NamedGuid,
    },
    BgSystemAlliance {
        target2: NamedGuid,
    },
    BgSystemHorde {
        target2: NamedGuid,
    },
    RaidLeader {
        target5: Guid,
    },
    RaidWarning {
        target5: Guid,
    },
    RaidBossWhisper {
        sender: String,
        target1: NamedGuid,
    },
    RaidBossEmote {
        sender: String,
        target1: NamedGuid,
    },
    Filtered {
        target5: Guid,
    },
    Battleground {
        target5: Guid,
    },
    BattlegroundLeader {
        target5: Guid,
    },
    Restricted {
        target5: Guid,
    },
}

impl Default for SMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::System {
            target5: Default::default(),
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
            Self::WhisperInform { .. } => 8,
            Self::Reply { .. } => 9,
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
            Self::RaidBossWhisper { .. } => 41,
            Self::RaidBossEmote { .. } => 42,
            Self::Filtered { .. } => 43,
            Self::Battleground { .. } => 44,
            Self::BattlegroundLeader { .. } => 45,
            Self::Restricted { .. } => 46,
        }
    }

}

impl SMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::System {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Say {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Party {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Raid {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Guild {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Officer {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Yell {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Whisper {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::WhisperInform {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Reply {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Emote {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::TextEmote {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::MonsterSay {
                sender,
                target1,
            } => {
                1
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterParty {
                sender,
                target1,
            } => {
                1
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterYell {
                sender,
                target1,
            } => {
                1
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterWhisper {
                sender,
                target1,
            } => {
                1
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterEmote {
                sender,
                target1,
            } => {
                1
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::Channel {
                channel_name,
                target4,
            } => {
                1
                + channel_name.len() + 1 // channel_name: CString
                + 8 // target4: Guid
            }
            Self::ChannelJoin {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::ChannelLeave {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::ChannelList {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::ChannelNotice {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::ChannelNoticeUser {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Afk {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Dnd {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Ignored {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Skill {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Loot {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Money {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Opening {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Tradeskills {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::PetInfo {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::CombatMiscInfo {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::CombatXpGain {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::CombatHonorGain {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::CombatFactionChange {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::BgSystemNeutral {
                target2,
            } => {
                1
                + target2.size() // target2: NamedGuid
            }
            Self::BgSystemAlliance {
                target2,
            } => {
                1
                + target2.size() // target2: NamedGuid
            }
            Self::BgSystemHorde {
                target2,
            } => {
                1
                + target2.size() // target2: NamedGuid
            }
            Self::RaidLeader {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::RaidWarning {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::RaidBossWhisper {
                sender,
                target1,
            } => {
                1
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::RaidBossEmote {
                sender,
                target1,
            } => {
                1
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::Filtered {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Battleground {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::BattlegroundLeader {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
            Self::Restricted {
                target5,
            } => {
                1
                + 8 // target5: Guid
            }
        }
    }
}

