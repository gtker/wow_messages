use std::io::{Read, Write};

use crate::tbc::{
    ChatType, Language,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm#L30):
/// ```text
/// cmsg CMSG_MESSAGECHAT = 0x0095 {
///     (u32)ChatType chat_type;
///     (u32)Language language;
///     if (chat_type == WHISPER) {
///         CString target_player;
///     }
///     else if (chat_type == CHANNEL) {
///         CString channel;
///     }
///     CString message;
/// }
/// ```
pub struct CMSG_MESSAGECHAT {
    pub chat_type: CMSG_MESSAGECHAT_ChatType,
    pub language: Language,
    pub message: String,
}

impl crate::private::Sealed for CMSG_MESSAGECHAT {}
impl crate::Message for CMSG_MESSAGECHAT {
    const OPCODE: u32 = 0x0095;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&u32::from(self.chat_type.as_int()).to_le_bytes())?;

        // language: Language
        w.write_all(&u32::from(self.language.as_int()).to_le_bytes())?;

        match &self.chat_type {
            CMSG_MESSAGECHAT_ChatType::Whisper {
                target_player,
            } => {
                // target_player: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(target_player.as_bytes().iter().rev().next(), Some(&0_u8), "String `target_player` must not be null-terminated.");
                w.write_all(target_player.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::Channel {
                channel,
            } => {
                // channel: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(channel.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel` must not be null-terminated.");
                w.write_all(channel.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            _ => {}
        }

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=520).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0095, size: body_size });
        }

        // chat_type: ChatType
        let chat_type: ChatType = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // language: Language
        let language: Language = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        let chat_type_if = match chat_type {
            ChatType::System => CMSG_MESSAGECHAT_ChatType::System,
            ChatType::Say => CMSG_MESSAGECHAT_ChatType::Say,
            ChatType::Party => CMSG_MESSAGECHAT_ChatType::Party,
            ChatType::Raid => CMSG_MESSAGECHAT_ChatType::Raid,
            ChatType::Guild => CMSG_MESSAGECHAT_ChatType::Guild,
            ChatType::Officer => CMSG_MESSAGECHAT_ChatType::Officer,
            ChatType::Yell => CMSG_MESSAGECHAT_ChatType::Yell,
            ChatType::Whisper => {
                // target_player: CString
                let target_player = {
                    let target_player = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(target_player)?
                };

                CMSG_MESSAGECHAT_ChatType::Whisper {
                    target_player,
                }
            }
            ChatType::WhisperInform => CMSG_MESSAGECHAT_ChatType::WhisperInform,
            ChatType::Reply => CMSG_MESSAGECHAT_ChatType::Reply,
            ChatType::Emote => CMSG_MESSAGECHAT_ChatType::Emote,
            ChatType::TextEmote => CMSG_MESSAGECHAT_ChatType::TextEmote,
            ChatType::MonsterSay => CMSG_MESSAGECHAT_ChatType::MonsterSay,
            ChatType::MonsterParty => CMSG_MESSAGECHAT_ChatType::MonsterParty,
            ChatType::MonsterYell => CMSG_MESSAGECHAT_ChatType::MonsterYell,
            ChatType::MonsterWhisper => CMSG_MESSAGECHAT_ChatType::MonsterWhisper,
            ChatType::MonsterEmote => CMSG_MESSAGECHAT_ChatType::MonsterEmote,
            ChatType::Channel => {
                // channel: CString
                let channel = {
                    let channel = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(channel)?
                };

                CMSG_MESSAGECHAT_ChatType::Channel {
                    channel,
                }
            }
            ChatType::ChannelJoin => CMSG_MESSAGECHAT_ChatType::ChannelJoin,
            ChatType::ChannelLeave => CMSG_MESSAGECHAT_ChatType::ChannelLeave,
            ChatType::ChannelList => CMSG_MESSAGECHAT_ChatType::ChannelList,
            ChatType::ChannelNotice => CMSG_MESSAGECHAT_ChatType::ChannelNotice,
            ChatType::ChannelNoticeUser => CMSG_MESSAGECHAT_ChatType::ChannelNoticeUser,
            ChatType::Afk => CMSG_MESSAGECHAT_ChatType::Afk,
            ChatType::Dnd => CMSG_MESSAGECHAT_ChatType::Dnd,
            ChatType::Ignored => CMSG_MESSAGECHAT_ChatType::Ignored,
            ChatType::Skill => CMSG_MESSAGECHAT_ChatType::Skill,
            ChatType::Loot => CMSG_MESSAGECHAT_ChatType::Loot,
            ChatType::Money => CMSG_MESSAGECHAT_ChatType::Money,
            ChatType::Opening => CMSG_MESSAGECHAT_ChatType::Opening,
            ChatType::Tradeskills => CMSG_MESSAGECHAT_ChatType::Tradeskills,
            ChatType::PetInfo => CMSG_MESSAGECHAT_ChatType::PetInfo,
            ChatType::CombatMiscInfo => CMSG_MESSAGECHAT_ChatType::CombatMiscInfo,
            ChatType::CombatXpGain => CMSG_MESSAGECHAT_ChatType::CombatXpGain,
            ChatType::CombatHonorGain => CMSG_MESSAGECHAT_ChatType::CombatHonorGain,
            ChatType::CombatFactionChange => CMSG_MESSAGECHAT_ChatType::CombatFactionChange,
            ChatType::BgSystemNeutral => CMSG_MESSAGECHAT_ChatType::BgSystemNeutral,
            ChatType::BgSystemAlliance => CMSG_MESSAGECHAT_ChatType::BgSystemAlliance,
            ChatType::BgSystemHorde => CMSG_MESSAGECHAT_ChatType::BgSystemHorde,
            ChatType::RaidLeader => CMSG_MESSAGECHAT_ChatType::RaidLeader,
            ChatType::RaidWarning => CMSG_MESSAGECHAT_ChatType::RaidWarning,
            ChatType::RaidBossWhisper => CMSG_MESSAGECHAT_ChatType::RaidBossWhisper,
            ChatType::RaidBossEmote => CMSG_MESSAGECHAT_ChatType::RaidBossEmote,
            ChatType::Filtered => CMSG_MESSAGECHAT_ChatType::Filtered,
            ChatType::Battleground => CMSG_MESSAGECHAT_ChatType::Battleground,
            ChatType::BattlegroundLeader => CMSG_MESSAGECHAT_ChatType::BattlegroundLeader,
            ChatType::Restricted => CMSG_MESSAGECHAT_ChatType::Restricted,
        };

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        Ok(Self {
            chat_type: chat_type_if,
            language,
            message,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_MESSAGECHAT {}

impl CMSG_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: CMSG_MESSAGECHAT_ChatType
        + 4 // language: Language
        + self.message.len() + 1 // message: CString
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMSG_MESSAGECHAT_ChatType {
    System,
    Say,
    Party,
    Raid,
    Guild,
    Officer,
    Yell,
    Whisper {
        target_player: String,
    },
    WhisperInform,
    Reply,
    Emote,
    TextEmote,
    MonsterSay,
    MonsterParty,
    MonsterYell,
    MonsterWhisper,
    MonsterEmote,
    Channel {
        channel: String,
    },
    ChannelJoin,
    ChannelLeave,
    ChannelList,
    ChannelNotice,
    ChannelNoticeUser,
    Afk,
    Dnd,
    Ignored,
    Skill,
    Loot,
    Money,
    Opening,
    Tradeskills,
    PetInfo,
    CombatMiscInfo,
    CombatXpGain,
    CombatHonorGain,
    CombatFactionChange,
    BgSystemNeutral,
    BgSystemAlliance,
    BgSystemHorde,
    RaidLeader,
    RaidWarning,
    RaidBossWhisper,
    RaidBossEmote,
    Filtered,
    Battleground,
    BattlegroundLeader,
    Restricted,
}

impl Default for CMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::System
    }
}

impl CMSG_MESSAGECHAT_ChatType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::System => 0,
            Self::Say => 1,
            Self::Party => 2,
            Self::Raid => 3,
            Self::Guild => 4,
            Self::Officer => 5,
            Self::Yell => 6,
            Self::Whisper { .. } => 7,
            Self::WhisperInform => 8,
            Self::Reply => 9,
            Self::Emote => 10,
            Self::TextEmote => 11,
            Self::MonsterSay => 12,
            Self::MonsterParty => 13,
            Self::MonsterYell => 14,
            Self::MonsterWhisper => 15,
            Self::MonsterEmote => 16,
            Self::Channel { .. } => 17,
            Self::ChannelJoin => 18,
            Self::ChannelLeave => 19,
            Self::ChannelList => 20,
            Self::ChannelNotice => 21,
            Self::ChannelNoticeUser => 22,
            Self::Afk => 23,
            Self::Dnd => 24,
            Self::Ignored => 25,
            Self::Skill => 26,
            Self::Loot => 27,
            Self::Money => 28,
            Self::Opening => 29,
            Self::Tradeskills => 30,
            Self::PetInfo => 31,
            Self::CombatMiscInfo => 32,
            Self::CombatXpGain => 33,
            Self::CombatHonorGain => 34,
            Self::CombatFactionChange => 35,
            Self::BgSystemNeutral => 36,
            Self::BgSystemAlliance => 37,
            Self::BgSystemHorde => 38,
            Self::RaidLeader => 39,
            Self::RaidWarning => 40,
            Self::RaidBossWhisper => 41,
            Self::RaidBossEmote => 42,
            Self::Filtered => 43,
            Self::Battleground => 44,
            Self::BattlegroundLeader => 45,
            Self::Restricted => 46,
        }
    }

}

impl CMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Whisper {
                target_player,
            } => {
                4
                + target_player.len() + 1 // target_player: CString
            }
            Self::Channel {
                channel,
            } => {
                4
                + channel.len() + 1 // channel: CString
            }
            _ => 4,
        }
    }
}

