use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::ChatType;
use crate::world::v1::v12::Language;
use crate::world::v1::v12::PlayerChatTag;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_MESSAGECHAT {
    pub chat_type: SMSG_MESSAGECHATChatType,
    pub language: Language,
    pub message_length: u32,
    pub message: String,
    pub tag: PlayerChatTag,
}

impl ServerMessage for SMSG_MESSAGECHAT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int() as u8).to_le_bytes())?;

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

        match &self.chat_type {
            SMSG_MESSAGECHATChatType::SAY {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                w.write_all(&sender_guid1.guid().to_le_bytes())?;

                // sender_guid2: Guid
                w.write_all(&sender_guid2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::PARTY {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                w.write_all(&sender_guid1.guid().to_le_bytes())?;

                // sender_guid2: Guid
                w.write_all(&sender_guid2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::RAID {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::GUILD {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::OFFICER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::YELL {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                w.write_all(&sender_guid1.guid().to_le_bytes())?;

                // sender_guid2: Guid
                w.write_all(&sender_guid2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::WHISPER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::WHISPER_INFORM {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::EMOTE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::TEXT_EMOTE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::SYSTEM {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::MONSTER_SAY {
                sender_guid3,
                sender_name,
                sender_name_length,
                target_guid,
            } => {
                // sender_guid3: Guid
                w.write_all(&sender_guid3.guid().to_le_bytes())?;

                // sender_name_length: u32
                w.write_all(&sender_name_length.to_le_bytes())?;

                // sender_name: CString
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target_guid: Guid
                w.write_all(&target_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::MONSTER_YELL {
                sender_guid3,
                sender_name,
                sender_name_length,
                target_guid,
            } => {
                // sender_guid3: Guid
                w.write_all(&sender_guid3.guid().to_le_bytes())?;

                // sender_name_length: u32
                w.write_all(&sender_name_length.to_le_bytes())?;

                // sender_name: CString
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target_guid: Guid
                w.write_all(&target_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::MONSTER_EMOTE {
                monster_guid,
                monster_name,
                name_length,
            } => {
                // name_length: u32
                w.write_all(&name_length.to_le_bytes())?;

                // monster_name: CString
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                w.write_all(&monster_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL {
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
            SMSG_MESSAGECHATChatType::CHANNEL_JOIN {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL_LEAVE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL_LIST {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL_NOTICE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL_NOTICE_USER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::AFK {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::DND {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::IGNORED {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::SKILL {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::LOOT {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::MONSTER_WHISPER {
                monster_guid,
                monster_name,
                name_length,
            } => {
                // name_length: u32
                w.write_all(&name_length.to_le_bytes())?;

                // monster_name: CString
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                w.write_all(&monster_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::BG_SYSTEM_NEUTRAL {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::BG_SYSTEM_ALLIANCE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::BG_SYSTEM_HORDE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::RAID_LEADER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::RAID_WARNING {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::RAID_BOSS_WHISPER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::RAID_BOSS_EMOTE {
                monster_guid,
                monster_name,
                name_length,
            } => {
                // name_length: u32
                w.write_all(&name_length.to_le_bytes())?;

                // monster_name: CString
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                w.write_all(&monster_guid.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::BATTLEGROUND {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHATChatType::BATTLEGROUND_LEADER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                w.write_all(&sender_guid4.guid().to_le_bytes())?;

            }
        }

        // message_length: u32
        w.write_all(&self.message_length.to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // tag: PlayerChatTag
        w.write_all(&(self.tag.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0096;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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

                SMSG_MESSAGECHATChatType::SAY {
                    sender_guid1,
                    sender_guid2,
                }
            }
            ChatType::PARTY => {
                // sender_guid1: Guid
                let sender_guid1 = Guid::read(r)?;

                // sender_guid2: Guid
                let sender_guid2 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::PARTY {
                    sender_guid1,
                    sender_guid2,
                }
            }
            ChatType::RAID => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::RAID {
                    sender_guid4,
                }
            }
            ChatType::GUILD => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::GUILD {
                    sender_guid4,
                }
            }
            ChatType::OFFICER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::OFFICER {
                    sender_guid4,
                }
            }
            ChatType::YELL => {
                // sender_guid1: Guid
                let sender_guid1 = Guid::read(r)?;

                // sender_guid2: Guid
                let sender_guid2 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::YELL {
                    sender_guid1,
                    sender_guid2,
                }
            }
            ChatType::WHISPER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::WHISPER {
                    sender_guid4,
                }
            }
            ChatType::WHISPER_INFORM => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::WHISPER_INFORM {
                    sender_guid4,
                }
            }
            ChatType::EMOTE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::EMOTE {
                    sender_guid4,
                }
            }
            ChatType::TEXT_EMOTE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::TEXT_EMOTE {
                    sender_guid4,
                }
            }
            ChatType::SYSTEM => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::SYSTEM {
                    sender_guid4,
                }
            }
            ChatType::MONSTER_SAY => {
                // sender_guid3: Guid
                let sender_guid3 = Guid::read(r)?;

                // sender_name_length: u32
                let sender_name_length = crate::util::read_u32_le(r)?;

                // sender_name: CString
                let sender_name = crate::util::read_c_string_to_vec(r)?;
                let sender_name = String::from_utf8(sender_name)?;

                // target_guid: Guid
                let target_guid = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::MONSTER_SAY {
                    sender_guid3,
                    sender_name,
                    sender_name_length,
                    target_guid,
                }
            }
            ChatType::MONSTER_YELL => {
                // sender_guid3: Guid
                let sender_guid3 = Guid::read(r)?;

                // sender_name_length: u32
                let sender_name_length = crate::util::read_u32_le(r)?;

                // sender_name: CString
                let sender_name = crate::util::read_c_string_to_vec(r)?;
                let sender_name = String::from_utf8(sender_name)?;

                // target_guid: Guid
                let target_guid = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::MONSTER_YELL {
                    sender_guid3,
                    sender_name,
                    sender_name_length,
                    target_guid,
                }
            }
            ChatType::MONSTER_EMOTE => {
                // name_length: u32
                let name_length = crate::util::read_u32_le(r)?;

                // monster_name: CString
                let monster_name = crate::util::read_c_string_to_vec(r)?;
                let monster_name = String::from_utf8(monster_name)?;

                // monster_guid: Guid
                let monster_guid = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::MONSTER_EMOTE {
                    monster_guid,
                    monster_name,
                    name_length,
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

                SMSG_MESSAGECHATChatType::CHANNEL {
                    channel_name,
                    player_guid,
                    player_rank,
                }
            }
            ChatType::CHANNEL_JOIN => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::CHANNEL_JOIN {
                    sender_guid4,
                }
            }
            ChatType::CHANNEL_LEAVE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::CHANNEL_LEAVE {
                    sender_guid4,
                }
            }
            ChatType::CHANNEL_LIST => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::CHANNEL_LIST {
                    sender_guid4,
                }
            }
            ChatType::CHANNEL_NOTICE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::CHANNEL_NOTICE {
                    sender_guid4,
                }
            }
            ChatType::CHANNEL_NOTICE_USER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::CHANNEL_NOTICE_USER {
                    sender_guid4,
                }
            }
            ChatType::AFK => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::AFK {
                    sender_guid4,
                }
            }
            ChatType::DND => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::DND {
                    sender_guid4,
                }
            }
            ChatType::IGNORED => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::IGNORED {
                    sender_guid4,
                }
            }
            ChatType::SKILL => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::SKILL {
                    sender_guid4,
                }
            }
            ChatType::LOOT => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::LOOT {
                    sender_guid4,
                }
            }
            ChatType::MONSTER_WHISPER => {
                // name_length: u32
                let name_length = crate::util::read_u32_le(r)?;

                // monster_name: CString
                let monster_name = crate::util::read_c_string_to_vec(r)?;
                let monster_name = String::from_utf8(monster_name)?;

                // monster_guid: Guid
                let monster_guid = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::MONSTER_WHISPER {
                    monster_guid,
                    monster_name,
                    name_length,
                }
            }
            ChatType::BG_SYSTEM_NEUTRAL => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::BG_SYSTEM_NEUTRAL {
                    sender_guid4,
                }
            }
            ChatType::BG_SYSTEM_ALLIANCE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::BG_SYSTEM_ALLIANCE {
                    sender_guid4,
                }
            }
            ChatType::BG_SYSTEM_HORDE => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::BG_SYSTEM_HORDE {
                    sender_guid4,
                }
            }
            ChatType::RAID_LEADER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::RAID_LEADER {
                    sender_guid4,
                }
            }
            ChatType::RAID_WARNING => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::RAID_WARNING {
                    sender_guid4,
                }
            }
            ChatType::RAID_BOSS_WHISPER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::RAID_BOSS_WHISPER {
                    sender_guid4,
                }
            }
            ChatType::RAID_BOSS_EMOTE => {
                // name_length: u32
                let name_length = crate::util::read_u32_le(r)?;

                // monster_name: CString
                let monster_name = crate::util::read_c_string_to_vec(r)?;
                let monster_name = String::from_utf8(monster_name)?;

                // monster_guid: Guid
                let monster_guid = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::RAID_BOSS_EMOTE {
                    monster_guid,
                    monster_name,
                    name_length,
                }
            }
            ChatType::BATTLEGROUND => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::BATTLEGROUND {
                    sender_guid4,
                }
            }
            ChatType::BATTLEGROUND_LEADER => {
                // sender_guid4: Guid
                let sender_guid4 = Guid::read(r)?;

                SMSG_MESSAGECHATChatType::BATTLEGROUND_LEADER {
                    sender_guid4,
                }
            }
        };

        // message_length: u32
        let message_length = crate::util::read_u32_le(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        // tag: PlayerChatTag
        let tag: PlayerChatTag = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            chat_type: chat_type_if,
            language,
            message_length,
            message,
            tag,
        })
    }

}

impl SMSG_MESSAGECHAT {
    pub fn size(&self) -> usize {
        0
        + self.chat_type.size() // chat_type: SMSG_MESSAGECHATChatType
        + 4 // language: Language
        + 4 // message_length: u32
        + self.message.len() + 1 // message: CString
        + 1 // tag: PlayerChatTag
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_MESSAGECHATChatType {
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
        sender_name_length: u32,
        target_guid: Guid,
    },
    MONSTER_YELL {
        sender_guid3: Guid,
        sender_name: String,
        sender_name_length: u32,
        target_guid: Guid,
    },
    MONSTER_EMOTE {
        monster_guid: Guid,
        monster_name: String,
        name_length: u32,
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
        name_length: u32,
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
        name_length: u32,
    },
    BATTLEGROUND {
        sender_guid4: Guid,
    },
    BATTLEGROUND_LEADER {
        sender_guid4: Guid,
    },
}

impl Default for SMSG_MESSAGECHATChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SAY {
            sender_guid1: Default::default(),
            sender_guid2: Default::default(),
        }
    }
}

impl SMSG_MESSAGECHATChatType {
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

impl SMSG_MESSAGECHATChatType {
    pub fn size(&self) -> usize {
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
                sender_name_length,
                target_guid,
            } => {
                1
                + 8 // sender_guid3: Guid
                + sender_name.len() + 1 // sender_name: CString
                + 4 // sender_name_length: u32
                + 8 // target_guid: Guid
            }
            Self::MONSTER_YELL {
                sender_guid3,
                sender_name,
                sender_name_length,
                target_guid,
            } => {
                1
                + 8 // sender_guid3: Guid
                + sender_name.len() + 1 // sender_name: CString
                + 4 // sender_name_length: u32
                + 8 // target_guid: Guid
            }
            Self::MONSTER_EMOTE {
                monster_guid,
                monster_name,
                name_length,
            } => {
                1
                + 8 // monster_guid: Guid
                + monster_name.len() + 1 // monster_name: CString
                + 4 // name_length: u32
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
                name_length,
            } => {
                1
                + 8 // monster_guid: Guid
                + monster_name.len() + 1 // monster_name: CString
                + 4 // name_length: u32
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
                name_length,
            } => {
                1
                + 8 // monster_guid: Guid
                + monster_name.len() + 1 // monster_name: CString
                + 4 // name_length: u32
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

