use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::ChatType;
use crate::world::version_1_12::Language;
use crate::world::version_1_12::PlayerChatTag;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_messagechat.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_messagechat.wowm#L3):
/// ```text
/// smsg SMSG_MESSAGECHAT = 0x0096 {
///     ChatType chat_type;
///     Language language;
///     if (chat_type == MONSTER_WHISPER
///         || chat_type == RAID_BOSS_EMOTE
///         || chat_type == MONSTER_EMOTE) {
///         SizedCString monster_name;
///         Guid monster_guid;
///     }
///     else if (chat_type == SAY
///         || chat_type == PARTY
///         || chat_type == YELL) {
///         Guid sender_guid1;
///         Guid sender_guid2;
///     }
///     else if (chat_type == MONSTER_SAY
///         || chat_type == MONSTER_YELL) {
///         Guid sender_guid3;
///         SizedCString sender_name;
///         Guid target_guid;
///     }
///     else if (chat_type == CHANNEL) {
///         CString channel_name;
///         u32 player_rank;
///         Guid player_guid;
///     }
///     else {
///         Guid sender_guid4;
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

impl ServerMessage for SMSG_MESSAGECHAT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int() as u8).to_le_bytes())?;

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

        match &self.chat_type {
            SMSG_MESSAGECHAT_ChatType::Say {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                w.write_all(&sender_guid1.guid().to_le_bytes())?;

                // sender_guid2: Guid
                w.write_all(&sender_guid2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Party {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                w.write_all(&sender_guid1.guid().to_le_bytes())?;

                // sender_guid2: Guid
                w.write_all(&sender_guid2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Raid {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Guild {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Officer {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Yell {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                w.write_all(&sender_guid1.guid().to_le_bytes())?;

                // sender_guid2: Guid
                w.write_all(&sender_guid2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Whisper {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WhisperInform {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Emote {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::TextEmote {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::System {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender_guid3,
                sender_name,
                target_guid,
            } => {
                // sender_guid3: Guid
                w.write_all(&sender_guid3.guid().to_le_bytes())?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target_guid: Guid
                w.write_all(&target_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender_guid3,
                sender_name,
                target_guid,
            } => {
                // sender_guid3: Guid
                w.write_all(&sender_guid3.guid().to_le_bytes())?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target_guid: Guid
                w.write_all(&target_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                monster_guid,
                monster_name,
            } => {
                // monster_name: SizedCString
                w.write_all(&((monster_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                w.write_all(&monster_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                player_guid,
                player_rank,
            } => {
                // channel_name: CString
                w.write_all(channel_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // player_rank: u32
                w.write_all(&player_rank.to_le_bytes())?;

                // player_guid: Guid
                w.write_all(&player_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelList {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Afk {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Dnd {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Ignored {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Skill {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Loot {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                monster_guid,
                monster_name,
            } => {
                // monster_name: SizedCString
                w.write_all(&((monster_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                w.write_all(&monster_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidLeader {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidWarning {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                monster_guid,
                monster_name,
            } => {
                // monster_name: SizedCString
                w.write_all(&((monster_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                w.write_all(&monster_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Battleground {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
        }

        // message: SizedCString
        w.write_all(&((self.message.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // tag: PlayerChatTag
        w.write_all(&(self.tag.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0096;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // chat_type: ChatType
        let chat_type: ChatType = crate::util::read_u8_le(r)?.try_into()?;

        // language: Language
        let language: Language = crate::util::read_u32_le(r)?.try_into()?;

        let chat_type_if = match chat_type {
            ChatType::Say => {
                // sender_guid1: Guid
                let sender_guid1 = Guid::read(r)?;

                // sender_guid2: Guid
                let sender_guid2 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Say {
                    sender_guid1,
                    sender_guid2,
                }
            }
            ChatType::Party => {
                // sender_guid1: Guid
                let sender_guid1 = Guid::read(r)?;

                // sender_guid2: Guid
                let sender_guid2 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Party {
                    sender_guid1,
                    sender_guid2,
                }
            }
            ChatType::Raid => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Raid {
                    sender_guid4,
                }
            }
            ChatType::Guild => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Guild {
                    sender_guid4,
                }
            }
            ChatType::Officer => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Officer {
                    sender_guid4,
                }
            }
            ChatType::Yell => {
                // sender_guid1: Guid
                let sender_guid1 = Guid::read(r)?;

                // sender_guid2: Guid
                let sender_guid2 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Yell {
                    sender_guid1,
                    sender_guid2,
                }
            }
            ChatType::Whisper => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Whisper {
                    sender_guid4,
                }
            }
            ChatType::WhisperInform => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::WhisperInform {
                    sender_guid4,
                }
            }
            ChatType::Emote => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Emote {
                    sender_guid4,
                }
            }
            ChatType::TextEmote => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::TextEmote {
                    sender_guid4,
                }
            }
            ChatType::System => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::System {
                    sender_guid4,
                }
            }
            ChatType::MonsterSay => {
                // sender_guid3: Guid
                let sender_guid3 = Guid::read(r)?;

                // sender_name: SizedCString
                let sender_name = crate::util::read_u32_le(r)?;
                let sender_name = crate::util::read_sized_c_string_to_vec(r, sender_name)?;
                let sender_name = String::from_utf8(sender_name)?;;
                // target_guid: Guid
                let target_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterSay {
                    sender_guid3,
                    sender_name,
                    target_guid,
                }
            }
            ChatType::MonsterYell => {
                // sender_guid3: Guid
                let sender_guid3 = Guid::read(r)?;

                // sender_name: SizedCString
                let sender_name = crate::util::read_u32_le(r)?;
                let sender_name = crate::util::read_sized_c_string_to_vec(r, sender_name)?;
                let sender_name = String::from_utf8(sender_name)?;;
                // target_guid: Guid
                let target_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterYell {
                    sender_guid3,
                    sender_name,
                    target_guid,
                }
            }
            ChatType::MonsterEmote => {
                // monster_name: SizedCString
                let monster_name = crate::util::read_u32_le(r)?;
                let monster_name = crate::util::read_sized_c_string_to_vec(r, monster_name)?;
                let monster_name = String::from_utf8(monster_name)?;;
                // monster_guid: Guid
                let monster_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                    monster_guid,
                    monster_name,
                }
            }
            ChatType::Channel => {
                // channel_name: CString
                let channel_name = crate::util::read_c_string_to_vec(r)?;
                let channel_name = String::from_utf8(channel_name)?;

                // player_rank: u32
                let player_rank = crate::util::read_u32_le(r)?;

                // player_guid: Guid
                let player_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Channel {
                    channel_name,
                    player_guid,
                    player_rank,
                }
            }
            ChatType::ChannelJoin => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                    sender_guid4,
                }
            }
            ChatType::ChannelLeave => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                    sender_guid4,
                }
            }
            ChatType::ChannelList => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelList {
                    sender_guid4,
                }
            }
            ChatType::ChannelNotice => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                    sender_guid4,
                }
            }
            ChatType::ChannelNoticeUser => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                    sender_guid4,
                }
            }
            ChatType::Afk => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Afk {
                    sender_guid4,
                }
            }
            ChatType::Dnd => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Dnd {
                    sender_guid4,
                }
            }
            ChatType::Ignored => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Ignored {
                    sender_guid4,
                }
            }
            ChatType::Skill => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Skill {
                    sender_guid4,
                }
            }
            ChatType::Loot => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Loot {
                    sender_guid4,
                }
            }
            ChatType::MonsterWhisper => {
                // monster_name: SizedCString
                let monster_name = crate::util::read_u32_le(r)?;
                let monster_name = crate::util::read_sized_c_string_to_vec(r, monster_name)?;
                let monster_name = String::from_utf8(monster_name)?;;
                // monster_guid: Guid
                let monster_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                    monster_guid,
                    monster_name,
                }
            }
            ChatType::BgSystemNeutral => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                    sender_guid4,
                }
            }
            ChatType::BgSystemAlliance => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                    sender_guid4,
                }
            }
            ChatType::BgSystemHorde => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                    sender_guid4,
                }
            }
            ChatType::RaidLeader => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidLeader {
                    sender_guid4,
                }
            }
            ChatType::RaidWarning => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidWarning {
                    sender_guid4,
                }
            }
            ChatType::RaidBossWhisper => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                    sender_guid4,
                }
            }
            ChatType::RaidBossEmote => {
                // monster_name: SizedCString
                let monster_name = crate::util::read_u32_le(r)?;
                let monster_name = crate::util::read_sized_c_string_to_vec(r, monster_name)?;
                let monster_name = String::from_utf8(monster_name)?;;
                // monster_guid: Guid
                let monster_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                    monster_guid,
                    monster_name,
                }
            }
            ChatType::Battleground => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Battleground {
                    sender_guid4,
                }
            }
            ChatType::BattlegroundLeader => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                    sender_guid4,
                }
            }
        };

        // message: SizedCString
        let message = crate::util::read_u32_le(r)?;
        let message = crate::util::read_sized_c_string_to_vec(r, message)?;
        let message = String::from_utf8(message)?;;
        // tag: PlayerChatTag
        let tag: PlayerChatTag = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            chat_type: chat_type_if,
            language,
            message,
            tag,
        })
    }

}

impl SMSG_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: SMSG_MESSAGECHAT_ChatType
        + 4 // language: Language
        + self.message.len() + 5 // message: SizedCString
        + 1 // tag: PlayerChatTag
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_MESSAGECHAT_ChatType {
    Say {
        sender_guid1: Guid,
        sender_guid2: Guid,
    },
    Party {
        sender_guid1: Guid,
        sender_guid2: Guid,
    },
    Raid {
        sender_guid4: Guid,
    },
    Guild {
        sender_guid4: Guid,
    },
    Officer {
        sender_guid4: Guid,
    },
    Yell {
        sender_guid1: Guid,
        sender_guid2: Guid,
    },
    Whisper {
        sender_guid4: Guid,
    },
    WhisperInform {
        sender_guid4: Guid,
    },
    Emote {
        sender_guid4: Guid,
    },
    TextEmote {
        sender_guid4: Guid,
    },
    System {
        sender_guid4: Guid,
    },
    MonsterSay {
        sender_guid3: Guid,
        sender_name: String,
        target_guid: Guid,
    },
    MonsterYell {
        sender_guid3: Guid,
        sender_name: String,
        target_guid: Guid,
    },
    MonsterEmote {
        monster_guid: Guid,
        monster_name: String,
    },
    Channel {
        channel_name: String,
        player_guid: Guid,
        player_rank: u32,
    },
    ChannelJoin {
        sender_guid4: Guid,
    },
    ChannelLeave {
        sender_guid4: Guid,
    },
    ChannelList {
        sender_guid4: Guid,
    },
    ChannelNotice {
        sender_guid4: Guid,
    },
    ChannelNoticeUser {
        sender_guid4: Guid,
    },
    Afk {
        sender_guid4: Guid,
    },
    Dnd {
        sender_guid4: Guid,
    },
    Ignored {
        sender_guid4: Guid,
    },
    Skill {
        sender_guid4: Guid,
    },
    Loot {
        sender_guid4: Guid,
    },
    MonsterWhisper {
        monster_guid: Guid,
        monster_name: String,
    },
    BgSystemNeutral {
        sender_guid4: Guid,
    },
    BgSystemAlliance {
        sender_guid4: Guid,
    },
    BgSystemHorde {
        sender_guid4: Guid,
    },
    RaidLeader {
        sender_guid4: Guid,
    },
    RaidWarning {
        sender_guid4: Guid,
    },
    RaidBossWhisper {
        sender_guid4: Guid,
    },
    RaidBossEmote {
        monster_guid: Guid,
        monster_name: String,
    },
    Battleground {
        sender_guid4: Guid,
    },
    BattlegroundLeader {
        sender_guid4: Guid,
    },
}

impl Default for SMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Say {
            sender_guid1: Default::default(),
            sender_guid2: Default::default(),
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
                sender_guid1,
                sender_guid2,
            } => {
                1
                + 8 // sender_guid1: Guid
                + 8 // sender_guid2: Guid
            }
            Self::Party {
                sender_guid1,
                sender_guid2,
            } => {
                1
                + 8 // sender_guid1: Guid
                + 8 // sender_guid2: Guid
            }
            Self::Raid {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::Guild {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::Officer {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::Yell {
                sender_guid1,
                sender_guid2,
            } => {
                1
                + 8 // sender_guid1: Guid
                + 8 // sender_guid2: Guid
            }
            Self::Whisper {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::WhisperInform {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::Emote {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::TextEmote {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::System {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::MonsterSay {
                sender_guid3,
                sender_name,
                target_guid,
            } => {
                1
                + 8 // sender_guid3: Guid
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target_guid: Guid
            }
            Self::MonsterYell {
                sender_guid3,
                sender_name,
                target_guid,
            } => {
                1
                + 8 // sender_guid3: Guid
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target_guid: Guid
            }
            Self::MonsterEmote {
                monster_guid,
                monster_name,
            } => {
                1
                + 8 // monster_guid: Guid
                + monster_name.len() + 5 // monster_name: SizedCString
            }
            Self::Channel {
                channel_name,
                player_guid,
                player_rank,
            } => {
                1
                + channel_name.len() + 1 // channel_name: CString
                + 8 // player_guid: Guid
                + 4 // player_rank: u32
            }
            Self::ChannelJoin {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::ChannelLeave {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::ChannelList {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::ChannelNotice {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::ChannelNoticeUser {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::Afk {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::Dnd {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::Ignored {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::Skill {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::Loot {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::MonsterWhisper {
                monster_guid,
                monster_name,
            } => {
                1
                + 8 // monster_guid: Guid
                + monster_name.len() + 5 // monster_name: SizedCString
            }
            Self::BgSystemNeutral {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::BgSystemAlliance {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::BgSystemHorde {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RaidLeader {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RaidWarning {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RaidBossWhisper {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RaidBossEmote {
                monster_guid,
                monster_name,
            } => {
                1
                + 8 // monster_guid: Guid
                + monster_name.len() + 5 // monster_name: SizedCString
            }
            Self::Battleground {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::BattlegroundLeader {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
        }
    }
}

