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
            SMSG_MESSAGECHAT_ChatType::SAY {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                w.write_all(&sender_guid1.guid().to_le_bytes())?;

                // sender_guid2: Guid
                w.write_all(&sender_guid2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::PARTY {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                w.write_all(&sender_guid1.guid().to_le_bytes())?;

                // sender_guid2: Guid
                w.write_all(&sender_guid2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RAID {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::GUILD {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::OFFICER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::YELL {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                w.write_all(&sender_guid1.guid().to_le_bytes())?;

                // sender_guid2: Guid
                w.write_all(&sender_guid2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WHISPER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WHISPER_INFORM {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::EMOTE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::TEXT_EMOTE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::SYSTEM {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MONSTER_SAY {
                sender_guid3,
                sender_name,
                target_guid,
            } => {
                // sender_guid3: Guid
                w.write_all(&sender_guid3.guid().to_le_bytes())?;

                // sender_name: SizedCString
                w.write_all(&(sender_name.len() as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target_guid: Guid
                w.write_all(&target_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MONSTER_YELL {
                sender_guid3,
                sender_name,
                target_guid,
            } => {
                // sender_guid3: Guid
                w.write_all(&sender_guid3.guid().to_le_bytes())?;

                // sender_name: SizedCString
                w.write_all(&(sender_name.len() as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target_guid: Guid
                w.write_all(&target_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MONSTER_EMOTE {
                monster_guid,
                monster_name,
            } => {
                // monster_name: SizedCString
                w.write_all(&(monster_name.len() as u32).to_le_bytes())?;
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                w.write_all(&monster_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CHANNEL {
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
            SMSG_MESSAGECHAT_ChatType::CHANNEL_JOIN {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CHANNEL_LEAVE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CHANNEL_LIST {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE_USER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::AFK {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::DND {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::IGNORED {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::SKILL {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::LOOT {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MONSTER_WHISPER {
                monster_guid,
                monster_name,
            } => {
                // monster_name: SizedCString
                w.write_all(&(monster_name.len() as u32).to_le_bytes())?;
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                w.write_all(&monster_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BG_SYSTEM_NEUTRAL {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BG_SYSTEM_ALLIANCE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BG_SYSTEM_HORDE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RAID_LEADER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RAID_WARNING {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RAID_BOSS_WHISPER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RAID_BOSS_EMOTE {
                monster_guid,
                monster_name,
            } => {
                // monster_name: SizedCString
                w.write_all(&(monster_name.len() as u32).to_le_bytes())?;
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                w.write_all(&monster_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BATTLEGROUND {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BATTLEGROUND_LEADER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
        }

        // message: SizedCString
        w.write_all(&(self.message.len() as u32).to_le_bytes())?;
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
            ChatType::SAY => {
                // sender_guid1: Guid
                let sender_guid1 = Guid::read(r)?;

                // sender_guid2: Guid
                let sender_guid2 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::SAY {
                    sender_guid1,
                    sender_guid2,
                }
            }
            ChatType::PARTY => {
                // sender_guid1: Guid
                let sender_guid1 = Guid::read(r)?;

                // sender_guid2: Guid
                let sender_guid2 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::PARTY {
                    sender_guid1,
                    sender_guid2,
                }
            }
            ChatType::RAID => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RAID {
                    sender_guid4,
                }
            }
            ChatType::GUILD => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::GUILD {
                    sender_guid4,
                }
            }
            ChatType::OFFICER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::OFFICER {
                    sender_guid4,
                }
            }
            ChatType::YELL => {
                // sender_guid1: Guid
                let sender_guid1 = Guid::read(r)?;

                // sender_guid2: Guid
                let sender_guid2 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::YELL {
                    sender_guid1,
                    sender_guid2,
                }
            }
            ChatType::WHISPER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::WHISPER {
                    sender_guid4,
                }
            }
            ChatType::WHISPER_INFORM => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::WHISPER_INFORM {
                    sender_guid4,
                }
            }
            ChatType::EMOTE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::EMOTE {
                    sender_guid4,
                }
            }
            ChatType::TEXT_EMOTE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::TEXT_EMOTE {
                    sender_guid4,
                }
            }
            ChatType::SYSTEM => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::SYSTEM {
                    sender_guid4,
                }
            }
            ChatType::MONSTER_SAY => {
                // sender_guid3: Guid
                let sender_guid3 = Guid::read(r)?;

                // sender_name: SizedCString
                let sender_name = crate::util::read_u32_le(r)?;
                let sender_name = crate::util::read_sized_c_string_to_vec(r, sender_name)?;
                let sender_name = String::from_utf8(sender_name)?;;
                // target_guid: Guid
                let target_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MONSTER_SAY {
                    sender_guid3,
                    sender_name,
                    target_guid,
                }
            }
            ChatType::MONSTER_YELL => {
                // sender_guid3: Guid
                let sender_guid3 = Guid::read(r)?;

                // sender_name: SizedCString
                let sender_name = crate::util::read_u32_le(r)?;
                let sender_name = crate::util::read_sized_c_string_to_vec(r, sender_name)?;
                let sender_name = String::from_utf8(sender_name)?;;
                // target_guid: Guid
                let target_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MONSTER_YELL {
                    sender_guid3,
                    sender_name,
                    target_guid,
                }
            }
            ChatType::MONSTER_EMOTE => {
                // monster_name: SizedCString
                let monster_name = crate::util::read_u32_le(r)?;
                let monster_name = crate::util::read_sized_c_string_to_vec(r, monster_name)?;
                let monster_name = String::from_utf8(monster_name)?;;
                // monster_guid: Guid
                let monster_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MONSTER_EMOTE {
                    monster_guid,
                    monster_name,
                }
            }
            ChatType::CHANNEL => {
                // channel_name: CString
                let channel_name = crate::util::read_c_string_to_vec(r)?;
                let channel_name = String::from_utf8(channel_name)?;

                // player_rank: u32
                let player_rank = crate::util::read_u32_le(r)?;

                // player_guid: Guid
                let player_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CHANNEL {
                    channel_name,
                    player_guid,
                    player_rank,
                }
            }
            ChatType::CHANNEL_JOIN => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CHANNEL_JOIN {
                    sender_guid4,
                }
            }
            ChatType::CHANNEL_LEAVE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CHANNEL_LEAVE {
                    sender_guid4,
                }
            }
            ChatType::CHANNEL_LIST => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CHANNEL_LIST {
                    sender_guid4,
                }
            }
            ChatType::CHANNEL_NOTICE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE {
                    sender_guid4,
                }
            }
            ChatType::CHANNEL_NOTICE_USER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE_USER {
                    sender_guid4,
                }
            }
            ChatType::AFK => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::AFK {
                    sender_guid4,
                }
            }
            ChatType::DND => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::DND {
                    sender_guid4,
                }
            }
            ChatType::IGNORED => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::IGNORED {
                    sender_guid4,
                }
            }
            ChatType::SKILL => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::SKILL {
                    sender_guid4,
                }
            }
            ChatType::LOOT => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::LOOT {
                    sender_guid4,
                }
            }
            ChatType::MONSTER_WHISPER => {
                // monster_name: SizedCString
                let monster_name = crate::util::read_u32_le(r)?;
                let monster_name = crate::util::read_sized_c_string_to_vec(r, monster_name)?;
                let monster_name = String::from_utf8(monster_name)?;;
                // monster_guid: Guid
                let monster_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MONSTER_WHISPER {
                    monster_guid,
                    monster_name,
                }
            }
            ChatType::BG_SYSTEM_NEUTRAL => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BG_SYSTEM_NEUTRAL {
                    sender_guid4,
                }
            }
            ChatType::BG_SYSTEM_ALLIANCE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BG_SYSTEM_ALLIANCE {
                    sender_guid4,
                }
            }
            ChatType::BG_SYSTEM_HORDE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BG_SYSTEM_HORDE {
                    sender_guid4,
                }
            }
            ChatType::RAID_LEADER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RAID_LEADER {
                    sender_guid4,
                }
            }
            ChatType::RAID_WARNING => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RAID_WARNING {
                    sender_guid4,
                }
            }
            ChatType::RAID_BOSS_WHISPER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RAID_BOSS_WHISPER {
                    sender_guid4,
                }
            }
            ChatType::RAID_BOSS_EMOTE => {
                // monster_name: SizedCString
                let monster_name = crate::util::read_u32_le(r)?;
                let monster_name = crate::util::read_sized_c_string_to_vec(r, monster_name)?;
                let monster_name = String::from_utf8(monster_name)?;;
                // monster_guid: Guid
                let monster_guid = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RAID_BOSS_EMOTE {
                    monster_guid,
                    monster_name,
                }
            }
            ChatType::BATTLEGROUND => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BATTLEGROUND {
                    sender_guid4,
                }
            }
            ChatType::BATTLEGROUND_LEADER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BATTLEGROUND_LEADER {
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
    SAY {
        sender_guid1: Guid,
        sender_guid2: Guid,
    },
    PARTY {
        sender_guid1: Guid,
        sender_guid2: Guid,
    },
    RAID {
        sender_guid4: Guid,
    },
    GUILD {
        sender_guid4: Guid,
    },
    OFFICER {
        sender_guid4: Guid,
    },
    YELL {
        sender_guid1: Guid,
        sender_guid2: Guid,
    },
    WHISPER {
        sender_guid4: Guid,
    },
    WHISPER_INFORM {
        sender_guid4: Guid,
    },
    EMOTE {
        sender_guid4: Guid,
    },
    TEXT_EMOTE {
        sender_guid4: Guid,
    },
    SYSTEM {
        sender_guid4: Guid,
    },
    MONSTER_SAY {
        sender_guid3: Guid,
        sender_name: String,
        target_guid: Guid,
    },
    MONSTER_YELL {
        sender_guid3: Guid,
        sender_name: String,
        target_guid: Guid,
    },
    MONSTER_EMOTE {
        monster_guid: Guid,
        monster_name: String,
    },
    CHANNEL {
        channel_name: String,
        player_guid: Guid,
        player_rank: u32,
    },
    CHANNEL_JOIN {
        sender_guid4: Guid,
    },
    CHANNEL_LEAVE {
        sender_guid4: Guid,
    },
    CHANNEL_LIST {
        sender_guid4: Guid,
    },
    CHANNEL_NOTICE {
        sender_guid4: Guid,
    },
    CHANNEL_NOTICE_USER {
        sender_guid4: Guid,
    },
    AFK {
        sender_guid4: Guid,
    },
    DND {
        sender_guid4: Guid,
    },
    IGNORED {
        sender_guid4: Guid,
    },
    SKILL {
        sender_guid4: Guid,
    },
    LOOT {
        sender_guid4: Guid,
    },
    MONSTER_WHISPER {
        monster_guid: Guid,
        monster_name: String,
    },
    BG_SYSTEM_NEUTRAL {
        sender_guid4: Guid,
    },
    BG_SYSTEM_ALLIANCE {
        sender_guid4: Guid,
    },
    BG_SYSTEM_HORDE {
        sender_guid4: Guid,
    },
    RAID_LEADER {
        sender_guid4: Guid,
    },
    RAID_WARNING {
        sender_guid4: Guid,
    },
    RAID_BOSS_WHISPER {
        sender_guid4: Guid,
    },
    RAID_BOSS_EMOTE {
        monster_guid: Guid,
        monster_name: String,
    },
    BATTLEGROUND {
        sender_guid4: Guid,
    },
    BATTLEGROUND_LEADER {
        sender_guid4: Guid,
    },
}

impl Default for SMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SAY {
            sender_guid1: Default::default(),
            sender_guid2: Default::default(),
        }
    }
}

impl SMSG_MESSAGECHAT_ChatType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SAY { .. } => 0,
            Self::PARTY { .. } => 1,
            Self::RAID { .. } => 2,
            Self::GUILD { .. } => 3,
            Self::OFFICER { .. } => 4,
            Self::YELL { .. } => 5,
            Self::WHISPER { .. } => 6,
            Self::WHISPER_INFORM { .. } => 7,
            Self::EMOTE { .. } => 8,
            Self::TEXT_EMOTE { .. } => 9,
            Self::SYSTEM { .. } => 10,
            Self::MONSTER_SAY { .. } => 11,
            Self::MONSTER_YELL { .. } => 12,
            Self::MONSTER_EMOTE { .. } => 13,
            Self::CHANNEL { .. } => 14,
            Self::CHANNEL_JOIN { .. } => 15,
            Self::CHANNEL_LEAVE { .. } => 16,
            Self::CHANNEL_LIST { .. } => 17,
            Self::CHANNEL_NOTICE { .. } => 18,
            Self::CHANNEL_NOTICE_USER { .. } => 19,
            Self::AFK { .. } => 20,
            Self::DND { .. } => 21,
            Self::IGNORED { .. } => 22,
            Self::SKILL { .. } => 23,
            Self::LOOT { .. } => 24,
            Self::MONSTER_WHISPER { .. } => 26,
            Self::BG_SYSTEM_NEUTRAL { .. } => 82,
            Self::BG_SYSTEM_ALLIANCE { .. } => 83,
            Self::BG_SYSTEM_HORDE { .. } => 84,
            Self::RAID_LEADER { .. } => 87,
            Self::RAID_WARNING { .. } => 88,
            Self::RAID_BOSS_WHISPER { .. } => 89,
            Self::RAID_BOSS_EMOTE { .. } => 90,
            Self::BATTLEGROUND { .. } => 92,
            Self::BATTLEGROUND_LEADER { .. } => 93,
        }
    }

}

impl SMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::SAY {
                sender_guid1,
                sender_guid2,
            } => {
                1
                + 8 // sender_guid1: Guid
                + 8 // sender_guid2: Guid
            }
            Self::PARTY {
                sender_guid1,
                sender_guid2,
            } => {
                1
                + 8 // sender_guid1: Guid
                + 8 // sender_guid2: Guid
            }
            Self::RAID {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::GUILD {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::OFFICER {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::YELL {
                sender_guid1,
                sender_guid2,
            } => {
                1
                + 8 // sender_guid1: Guid
                + 8 // sender_guid2: Guid
            }
            Self::WHISPER {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::WHISPER_INFORM {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::EMOTE {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::TEXT_EMOTE {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::SYSTEM {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::MONSTER_SAY {
                sender_guid3,
                sender_name,
                target_guid,
            } => {
                1
                + 8 // sender_guid3: Guid
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target_guid: Guid
            }
            Self::MONSTER_YELL {
                sender_guid3,
                sender_name,
                target_guid,
            } => {
                1
                + 8 // sender_guid3: Guid
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target_guid: Guid
            }
            Self::MONSTER_EMOTE {
                monster_guid,
                monster_name,
            } => {
                1
                + 8 // monster_guid: Guid
                + monster_name.len() + 5 // monster_name: SizedCString
            }
            Self::CHANNEL {
                channel_name,
                player_guid,
                player_rank,
            } => {
                1
                + channel_name.len() + 1 // channel_name: CString
                + 8 // player_guid: Guid
                + 4 // player_rank: u32
            }
            Self::CHANNEL_JOIN {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::CHANNEL_LEAVE {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::CHANNEL_LIST {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::CHANNEL_NOTICE {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::CHANNEL_NOTICE_USER {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::AFK {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::DND {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::IGNORED {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::SKILL {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::LOOT {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::MONSTER_WHISPER {
                monster_guid,
                monster_name,
            } => {
                1
                + 8 // monster_guid: Guid
                + monster_name.len() + 5 // monster_name: SizedCString
            }
            Self::BG_SYSTEM_NEUTRAL {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::BG_SYSTEM_ALLIANCE {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::BG_SYSTEM_HORDE {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RAID_LEADER {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RAID_WARNING {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RAID_BOSS_WHISPER {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RAID_BOSS_EMOTE {
                monster_guid,
                monster_name,
            } => {
                1
                + 8 // monster_guid: Guid
                + monster_name.len() + 5 // monster_name: SizedCString
            }
            Self::BATTLEGROUND {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::BATTLEGROUND_LEADER {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
        }
    }
}

