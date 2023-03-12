use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    ChatType,
    Language,
    PlayerChatTag,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_messagechat.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_messagechat.wowm#L10):
/// ```text
/// smsg SMSG_MESSAGECHAT = 0x0096 {
///     ChatType chat_type;
///     Language language;
///     if (chat_type == MONSTER_WHISPER
///         || chat_type == RAID_BOSS_EMOTE
///         || chat_type == MONSTER_EMOTE) {
///         SizedCString monster_name;
///         Guid monster;
///     }
///     else if (chat_type == SAY
///         || chat_type == PARTY
///         || chat_type == YELL) {
///         Guid speech_bubble_credit;
///         Guid chat_credit;
///     }
///     else if (chat_type == MONSTER_SAY
///         || chat_type == MONSTER_YELL) {
///         Guid sender1;
///         SizedCString sender_name;
///         Guid target;
///     }
///     else if (chat_type == CHANNEL) {
///         CString channel_name;
///         u32 player_rank;
///         Guid player;
///     }
///     else {
///         Guid sender2;
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
            SMSG_MESSAGECHAT_ChatType::Say {
                chat_credit,
                speech_bubble_credit,
            } => {
                // speech_bubble_credit: Guid
                w.write_all(&speech_bubble_credit.guid().to_le_bytes())?;

                // chat_credit: Guid
                w.write_all(&chat_credit.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Party {
                chat_credit,
                speech_bubble_credit,
            } => {
                // speech_bubble_credit: Guid
                w.write_all(&speech_bubble_credit.guid().to_le_bytes())?;

                // chat_credit: Guid
                w.write_all(&chat_credit.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Raid {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Guild {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Officer {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Yell {
                chat_credit,
                speech_bubble_credit,
            } => {
                // speech_bubble_credit: Guid
                w.write_all(&speech_bubble_credit.guid().to_le_bytes())?;

                // chat_credit: Guid
                w.write_all(&chat_credit.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Whisper {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WhisperInform {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Emote {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::TextEmote {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::System {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                sender_name,
                target,
            } => {
                // sender1: Guid
                w.write_all(&sender1.guid().to_le_bytes())?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                sender_name,
                target,
            } => {
                // sender1: Guid
                w.write_all(&sender1.guid().to_le_bytes())?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                monster,
                monster_name,
            } => {
                // monster_name: SizedCString
                w.write_all(&((monster_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster: Guid
                w.write_all(&monster.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                player,
                player_rank,
            } => {
                // channel_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
                w.write_all(channel_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // player_rank: u32
                w.write_all(&player_rank.to_le_bytes())?;

                // player: Guid
                w.write_all(&player.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelList {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Afk {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Dnd {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Ignored {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Skill {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Loot {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                monster,
                monster_name,
            } => {
                // monster_name: SizedCString
                w.write_all(&((monster_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster: Guid
                w.write_all(&monster.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidLeader {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidWarning {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                monster,
                monster_name,
            } => {
                // monster_name: SizedCString
                w.write_all(&((monster_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster: Guid
                w.write_all(&monster.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Battleground {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                sender2,
            } => {
                // sender2: Guid
                w.write_all(&sender2.guid().to_le_bytes())?;

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
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(19..=16030).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0096, size: body_size as u32 });
        }

        // chat_type: ChatType
        let chat_type: ChatType = crate::util::read_u8_le(&mut r)?.try_into()?;

        // language: Language
        let language: Language = crate::util::read_u32_le(&mut r)?.try_into()?;

        let chat_type_if = match chat_type {
            ChatType::Say => {
                // speech_bubble_credit: Guid
                let speech_bubble_credit = Guid::read(&mut r)?;

                // chat_credit: Guid
                let chat_credit = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Say {
                    chat_credit,
                    speech_bubble_credit,
                }
            }
            ChatType::Party => {
                // speech_bubble_credit: Guid
                let speech_bubble_credit = Guid::read(&mut r)?;

                // chat_credit: Guid
                let chat_credit = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Party {
                    chat_credit,
                    speech_bubble_credit,
                }
            }
            ChatType::Raid => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Raid {
                    sender2,
                }
            }
            ChatType::Guild => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Guild {
                    sender2,
                }
            }
            ChatType::Officer => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Officer {
                    sender2,
                }
            }
            ChatType::Yell => {
                // speech_bubble_credit: Guid
                let speech_bubble_credit = Guid::read(&mut r)?;

                // chat_credit: Guid
                let chat_credit = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Yell {
                    chat_credit,
                    speech_bubble_credit,
                }
            }
            ChatType::Whisper => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Whisper {
                    sender2,
                }
            }
            ChatType::WhisperInform => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::WhisperInform {
                    sender2,
                }
            }
            ChatType::Emote => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Emote {
                    sender2,
                }
            }
            ChatType::TextEmote => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::TextEmote {
                    sender2,
                }
            }
            ChatType::System => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::System {
                    sender2,
                }
            }
            ChatType::MonsterSay => {
                // sender1: Guid
                let sender1 = Guid::read(&mut r)?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target: Guid
                let target = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterSay {
                    sender1,
                    sender_name,
                    target,
                }
            }
            ChatType::MonsterYell => {
                // sender1: Guid
                let sender1 = Guid::read(&mut r)?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                // target: Guid
                let target = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterYell {
                    sender1,
                    sender_name,
                    target,
                }
            }
            ChatType::MonsterEmote => {
                // monster_name: SizedCString
                let monster_name = {
                    let monster_name = crate::util::read_u32_le(&mut r)?;
                    let monster_name = crate::util::read_sized_c_string_to_vec(&mut r, monster_name)?;
                    String::from_utf8(monster_name)?
                };

                // monster: Guid
                let monster = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                    monster,
                    monster_name,
                }
            }
            ChatType::Channel => {
                // channel_name: CString
                let channel_name = {
                    let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(channel_name)?
                };

                // player_rank: u32
                let player_rank = crate::util::read_u32_le(&mut r)?;

                // player: Guid
                let player = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Channel {
                    channel_name,
                    player,
                    player_rank,
                }
            }
            ChatType::ChannelJoin => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                    sender2,
                }
            }
            ChatType::ChannelLeave => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                    sender2,
                }
            }
            ChatType::ChannelList => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelList {
                    sender2,
                }
            }
            ChatType::ChannelNotice => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                    sender2,
                }
            }
            ChatType::ChannelNoticeUser => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                    sender2,
                }
            }
            ChatType::Afk => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Afk {
                    sender2,
                }
            }
            ChatType::Dnd => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Dnd {
                    sender2,
                }
            }
            ChatType::Ignored => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Ignored {
                    sender2,
                }
            }
            ChatType::Skill => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Skill {
                    sender2,
                }
            }
            ChatType::Loot => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Loot {
                    sender2,
                }
            }
            ChatType::MonsterWhisper => {
                // monster_name: SizedCString
                let monster_name = {
                    let monster_name = crate::util::read_u32_le(&mut r)?;
                    let monster_name = crate::util::read_sized_c_string_to_vec(&mut r, monster_name)?;
                    String::from_utf8(monster_name)?
                };

                // monster: Guid
                let monster = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                    monster,
                    monster_name,
                }
            }
            ChatType::BgSystemNeutral => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                    sender2,
                }
            }
            ChatType::BgSystemAlliance => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                    sender2,
                }
            }
            ChatType::BgSystemHorde => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                    sender2,
                }
            }
            ChatType::RaidLeader => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::RaidLeader {
                    sender2,
                }
            }
            ChatType::RaidWarning => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::RaidWarning {
                    sender2,
                }
            }
            ChatType::RaidBossWhisper => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                    sender2,
                }
            }
            ChatType::RaidBossEmote => {
                // monster_name: SizedCString
                let monster_name = {
                    let monster_name = crate::util::read_u32_le(&mut r)?;
                    let monster_name = crate::util::read_sized_c_string_to_vec(&mut r, monster_name)?;
                    String::from_utf8(monster_name)?
                };

                // monster: Guid
                let monster = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                    monster,
                    monster_name,
                }
            }
            ChatType::Battleground => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::Battleground {
                    sender2,
                }
            }
            ChatType::BattlegroundLeader => {
                // sender2: Guid
                let sender2 = Guid::read(&mut r)?;

                SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                    sender2,
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
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MESSAGECHAT {}

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
    Say {
        chat_credit: Guid,
        speech_bubble_credit: Guid,
    },
    Party {
        chat_credit: Guid,
        speech_bubble_credit: Guid,
    },
    Raid {
        sender2: Guid,
    },
    Guild {
        sender2: Guid,
    },
    Officer {
        sender2: Guid,
    },
    Yell {
        chat_credit: Guid,
        speech_bubble_credit: Guid,
    },
    Whisper {
        sender2: Guid,
    },
    WhisperInform {
        sender2: Guid,
    },
    Emote {
        sender2: Guid,
    },
    TextEmote {
        sender2: Guid,
    },
    System {
        sender2: Guid,
    },
    MonsterSay {
        sender1: Guid,
        sender_name: String,
        target: Guid,
    },
    MonsterYell {
        sender1: Guid,
        sender_name: String,
        target: Guid,
    },
    MonsterEmote {
        monster: Guid,
        monster_name: String,
    },
    Channel {
        channel_name: String,
        player: Guid,
        player_rank: u32,
    },
    ChannelJoin {
        sender2: Guid,
    },
    ChannelLeave {
        sender2: Guid,
    },
    ChannelList {
        sender2: Guid,
    },
    ChannelNotice {
        sender2: Guid,
    },
    ChannelNoticeUser {
        sender2: Guid,
    },
    Afk {
        sender2: Guid,
    },
    Dnd {
        sender2: Guid,
    },
    Ignored {
        sender2: Guid,
    },
    Skill {
        sender2: Guid,
    },
    Loot {
        sender2: Guid,
    },
    MonsterWhisper {
        monster: Guid,
        monster_name: String,
    },
    BgSystemNeutral {
        sender2: Guid,
    },
    BgSystemAlliance {
        sender2: Guid,
    },
    BgSystemHorde {
        sender2: Guid,
    },
    RaidLeader {
        sender2: Guid,
    },
    RaidWarning {
        sender2: Guid,
    },
    RaidBossWhisper {
        sender2: Guid,
    },
    RaidBossEmote {
        monster: Guid,
        monster_name: String,
    },
    Battleground {
        sender2: Guid,
    },
    BattlegroundLeader {
        sender2: Guid,
    },
}

impl Default for SMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Say {
            chat_credit: Default::default(),
            speech_bubble_credit: Default::default(),
        }
    }
}

impl SMSG_MESSAGECHAT_ChatType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Say { .. } => 0,
            Self::Party { .. } => 1,
            Self::Raid { .. } => 2,
            Self::Guild { .. } => 3,
            Self::Officer { .. } => 4,
            Self::Yell { .. } => 5,
            Self::Whisper { .. } => 6,
            Self::WhisperInform { .. } => 7,
            Self::Emote { .. } => 8,
            Self::TextEmote { .. } => 9,
            Self::System { .. } => 10,
            Self::MonsterSay { .. } => 11,
            Self::MonsterYell { .. } => 12,
            Self::MonsterEmote { .. } => 13,
            Self::Channel { .. } => 14,
            Self::ChannelJoin { .. } => 15,
            Self::ChannelLeave { .. } => 16,
            Self::ChannelList { .. } => 17,
            Self::ChannelNotice { .. } => 18,
            Self::ChannelNoticeUser { .. } => 19,
            Self::Afk { .. } => 20,
            Self::Dnd { .. } => 21,
            Self::Ignored { .. } => 22,
            Self::Skill { .. } => 23,
            Self::Loot { .. } => 24,
            Self::MonsterWhisper { .. } => 26,
            Self::BgSystemNeutral { .. } => 82,
            Self::BgSystemAlliance { .. } => 83,
            Self::BgSystemHorde { .. } => 84,
            Self::RaidLeader { .. } => 87,
            Self::RaidWarning { .. } => 88,
            Self::RaidBossWhisper { .. } => 89,
            Self::RaidBossEmote { .. } => 90,
            Self::Battleground { .. } => 92,
            Self::BattlegroundLeader { .. } => 93,
        }
    }

}

impl SMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Say {
                chat_credit,
                speech_bubble_credit,
            } => {
                1
                + 8 // chat_credit: Guid
                + 8 // speech_bubble_credit: Guid
            }
            Self::Party {
                chat_credit,
                speech_bubble_credit,
            } => {
                1
                + 8 // chat_credit: Guid
                + 8 // speech_bubble_credit: Guid
            }
            Self::Raid {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::Guild {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::Officer {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::Yell {
                chat_credit,
                speech_bubble_credit,
            } => {
                1
                + 8 // chat_credit: Guid
                + 8 // speech_bubble_credit: Guid
            }
            Self::Whisper {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::WhisperInform {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::Emote {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::TextEmote {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::System {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::MonsterSay {
                sender1,
                sender_name,
                target,
            } => {
                1
                + 8 // sender1: Guid
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target: Guid
            }
            Self::MonsterYell {
                sender1,
                sender_name,
                target,
            } => {
                1
                + 8 // sender1: Guid
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target: Guid
            }
            Self::MonsterEmote {
                monster,
                monster_name,
            } => {
                1
                + 8 // monster: Guid
                + monster_name.len() + 5 // monster_name: SizedCString
            }
            Self::Channel {
                channel_name,
                player,
                player_rank,
            } => {
                1
                + channel_name.len() + 1 // channel_name: CString
                + 8 // player: Guid
                + 4 // player_rank: u32
            }
            Self::ChannelJoin {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::ChannelLeave {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::ChannelList {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::ChannelNotice {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::ChannelNoticeUser {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::Afk {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::Dnd {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::Ignored {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::Skill {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::Loot {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::MonsterWhisper {
                monster,
                monster_name,
            } => {
                1
                + 8 // monster: Guid
                + monster_name.len() + 5 // monster_name: SizedCString
            }
            Self::BgSystemNeutral {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::BgSystemAlliance {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::BgSystemHorde {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::RaidLeader {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::RaidWarning {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::RaidBossWhisper {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::RaidBossEmote {
                monster,
                monster_name,
            } => {
                1
                + 8 // monster: Guid
                + monster_name.len() + 5 // monster_name: SizedCString
            }
            Self::Battleground {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
            Self::BattlegroundLeader {
                sender2,
            } => {
                1
                + 8 // sender2: Guid
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_MESSAGECHAT;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 53] = [ 0x00, 0x33, 0x96, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x17, 0x00, 0x00, 0x00, 0x54, 0x68, 0x69, 0x73,
         0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x73, 0x61, 0x79, 0x20, 0x6D, 0x65,
         0x73, 0x73, 0x61, 0x67, 0x65, 0x2E, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/chat/smsg_messagechat.wowm` line 53.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_MESSAGECHAT0() {
        let expected = SMSG_MESSAGECHAT {
            chat_type: SMSG_MESSAGECHAT_ChatType::Say {
                chat_credit: Guid::new(0x5),
                speech_bubble_credit: Guid::new(0x5),
            },
            language: Language::Universal,
            message: String::from("This is a say message."),
            tag: PlayerChatTag::None,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MESSAGECHAT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MESSAGECHAT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.chat_type, expected.chat_type);
        assert_eq!(t.language, expected.language);
        assert_eq!(t.message, expected.message);
        assert_eq!(t.tag, expected.tag);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/smsg_messagechat.wowm` line 53.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_MESSAGECHAT0() {
        let expected = SMSG_MESSAGECHAT {
            chat_type: SMSG_MESSAGECHAT_ChatType::Say {
                chat_credit: Guid::new(0x5),
                speech_bubble_credit: Guid::new(0x5),
            },
            language: Language::Universal,
            message: String::from("This is a say message."),
            tag: PlayerChatTag::None,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MESSAGECHAT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MESSAGECHAT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.chat_type, expected.chat_type);
        assert_eq!(t.language, expected.language);
        assert_eq!(t.message, expected.message);
        assert_eq!(t.tag, expected.tag);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/smsg_messagechat.wowm` line 53.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_MESSAGECHAT0() {
        let expected = SMSG_MESSAGECHAT {
            chat_type: SMSG_MESSAGECHAT_ChatType::Say {
                chat_credit: Guid::new(0x5),
                speech_bubble_credit: Guid::new(0x5),
            },
            language: Language::Universal,
            message: String::from("This is a say message."),
            tag: PlayerChatTag::None,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MESSAGECHAT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MESSAGECHAT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.chat_type, expected.chat_type);
        assert_eq!(t.language, expected.language);
        assert_eq!(t.message, expected.message);
        assert_eq!(t.tag, expected.tag);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

