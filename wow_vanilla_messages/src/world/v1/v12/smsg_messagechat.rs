use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{ChatType, ChatTypeError};
use crate::world::v1::v12::{Language, LanguageError};
use crate::world::v1::v12::{PlayerChatTag, PlayerChatTagError};
use crate::{WorldServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_MESSAGECHAT {
    pub chat_type: SMSG_MESSAGECHATChatType,
    pub language: Language,
    pub message_length: u32,
    pub message: String,
    pub tag: PlayerChatTag,
}

impl WorldServerMessageWrite for SMSG_MESSAGECHAT {
    const OPCODE: u16 = 0x96;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for SMSG_MESSAGECHAT {
    type Error = SMSG_MESSAGECHATError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // chat_type: ChatType
        let chat_type = ChatType::read(r)?;

        // language: Language
        let language = Language::read(r)?;

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
                    sender_name_length,
                    sender_name,
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
                    sender_name_length,
                    sender_name,
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
                    name_length,
                    monster_name,
                    monster_guid,
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
                    player_rank,
                    player_guid,
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
                    name_length,
                    monster_name,
                    monster_guid,
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
                    name_length,
                    monster_name,
                    monster_guid,
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
        let tag = PlayerChatTag::read(r)?;

        Ok(Self {
            chat_type: chat_type_if,
            language,
            message_length,
            message,
            tag,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // chat_type: ChatType
        self.chat_type.write(w)?;

        // language: Language
        self.language.write(w)?;

        match &self.chat_type {
            SMSG_MESSAGECHATChatType::SAY {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                sender_guid1.write(w)?;

                // sender_guid2: Guid
                sender_guid2.write(w)?;

            }
            SMSG_MESSAGECHATChatType::PARTY {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                sender_guid1.write(w)?;

                // sender_guid2: Guid
                sender_guid2.write(w)?;

            }
            SMSG_MESSAGECHATChatType::RAID {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::GUILD {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::OFFICER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::YELL {
                sender_guid1,
                sender_guid2,
            } => {
                // sender_guid1: Guid
                sender_guid1.write(w)?;

                // sender_guid2: Guid
                sender_guid2.write(w)?;

            }
            SMSG_MESSAGECHATChatType::WHISPER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::WHISPER_INFORM {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::EMOTE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::TEXT_EMOTE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::SYSTEM {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::MONSTER_SAY {
                sender_guid3,
                sender_name_length,
                sender_name,
                target_guid,
            } => {
                // sender_guid3: Guid
                sender_guid3.write(w)?;

                // sender_name_length: u32
                w.write_all(&sender_name_length.to_le_bytes())?;

                // sender_name: CString
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target_guid: Guid
                target_guid.write(w)?;

            }
            SMSG_MESSAGECHATChatType::MONSTER_YELL {
                sender_guid3,
                sender_name_length,
                sender_name,
                target_guid,
            } => {
                // sender_guid3: Guid
                sender_guid3.write(w)?;

                // sender_name_length: u32
                w.write_all(&sender_name_length.to_le_bytes())?;

                // sender_name: CString
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target_guid: Guid
                target_guid.write(w)?;

            }
            SMSG_MESSAGECHATChatType::MONSTER_EMOTE {
                name_length,
                monster_name,
                monster_guid,
            } => {
                // name_length: u32
                w.write_all(&name_length.to_le_bytes())?;

                // monster_name: CString
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                monster_guid.write(w)?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL {
                channel_name,
                player_rank,
                player_guid,
            } => {
                // channel_name: CString
                w.write_all(channel_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // player_rank: u32
                w.write_all(&player_rank.to_le_bytes())?;

                // player_guid: Guid
                player_guid.write(w)?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL_JOIN {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL_LEAVE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL_LIST {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL_NOTICE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::CHANNEL_NOTICE_USER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::AFK {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::DND {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::IGNORED {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::SKILL {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::LOOT {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::MONSTER_WHISPER {
                name_length,
                monster_name,
                monster_guid,
            } => {
                // name_length: u32
                w.write_all(&name_length.to_le_bytes())?;

                // monster_name: CString
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                monster_guid.write(w)?;

            }
            SMSG_MESSAGECHATChatType::BG_SYSTEM_NEUTRAL {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::BG_SYSTEM_ALLIANCE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::BG_SYSTEM_HORDE {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::RAID_LEADER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::RAID_WARNING {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::RAID_BOSS_WHISPER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::RAID_BOSS_EMOTE {
                name_length,
                monster_name,
                monster_guid,
            } => {
                // name_length: u32
                w.write_all(&name_length.to_le_bytes())?;

                // monster_name: CString
                w.write_all(monster_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // monster_guid: Guid
                monster_guid.write(w)?;

            }
            SMSG_MESSAGECHATChatType::BATTLEGROUND {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
            SMSG_MESSAGECHATChatType::BATTLEGROUND_LEADER {
                sender_guid4,
            } => {
                // sender_guid4: Guid
                sender_guid4.write(w)?;

            }
        }

        // message_length: u32
        w.write_all(&self.message_length.to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // tag: PlayerChatTag
        self.tag.write(w)?;

        Ok(())
    }
}

impl VariableSized for SMSG_MESSAGECHAT {
    fn size(&self) -> usize {
        self.chat_type.size() // chat_type: ChatType and subfields
        + Language::size() // language: Language
        + 4 // message_length: u32
        + self.message.len() + 1 // message: CString and Null Terminator
        + PlayerChatTag::size() // tag: PlayerChatTag
    }
}

impl MaximumPossibleSized for SMSG_MESSAGECHAT {
    fn maximum_possible_size() -> usize {
        ChatType::maximum_possible_size() // chat_type: ChatType
        + Language::maximum_possible_size() // language: Language
        + 4 // message_length: u32
        + 256 // message: CString
        + PlayerChatTag::maximum_possible_size() // tag: PlayerChatTag
    }
}

#[derive(Debug)]
pub enum SMSG_MESSAGECHATError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    ChatType(ChatTypeError),
    Language(LanguageError),
    PlayerChatTag(PlayerChatTagError),
}

impl std::error::Error for SMSG_MESSAGECHATError {}
impl std::fmt::Display for SMSG_MESSAGECHATError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::ChatType(i) => i.fmt(f),
            Self::Language(i) => i.fmt(f),
            Self::PlayerChatTag(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MESSAGECHATError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_MESSAGECHATError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ChatTypeError> for SMSG_MESSAGECHATError {
    fn from(e: ChatTypeError) -> Self {
        Self::ChatType(e)
    }
}

impl From<LanguageError> for SMSG_MESSAGECHATError {
    fn from(e: LanguageError) -> Self {
        Self::Language(e)
    }
}

impl From<PlayerChatTagError> for SMSG_MESSAGECHATError {
    fn from(e: PlayerChatTagError) -> Self {
        Self::PlayerChatTag(e)
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
        sender_name_length: u32,
        sender_name: String,
        target_guid: Guid,
    },
    MONSTER_YELL {
        sender_guid3: Guid,
        sender_name_length: u32,
        sender_name: String,
        target_guid: Guid,
    },
    MONSTER_EMOTE {
        name_length: u32,
        monster_name: String,
        monster_guid: Guid,
    },
    CHANNEL {
        channel_name: String,
        player_rank: u32,
        player_guid: Guid,
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
        name_length: u32,
        monster_name: String,
        monster_guid: Guid,
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
        name_length: u32,
        monster_name: String,
        monster_guid: Guid,
    },
    BATTLEGROUND {
        sender_guid4: Guid,
    },
    BATTLEGROUND_LEADER {
        sender_guid4: Guid,
    },
}

impl From<&ChatType> for SMSG_MESSAGECHATChatType {
    fn from(e: &ChatType) -> Self {
        match &e {
            ChatType::SAY => Self::SAY {
                sender_guid1: Default::default(),
                sender_guid2: Default::default(),
            },
            ChatType::PARTY => Self::PARTY {
                sender_guid1: Default::default(),
                sender_guid2: Default::default(),
            },
            ChatType::RAID => Self::RAID {
                sender_guid4: Default::default(),
            },
            ChatType::GUILD => Self::GUILD {
                sender_guid4: Default::default(),
            },
            ChatType::OFFICER => Self::OFFICER {
                sender_guid4: Default::default(),
            },
            ChatType::YELL => Self::YELL {
                sender_guid1: Default::default(),
                sender_guid2: Default::default(),
            },
            ChatType::WHISPER => Self::WHISPER {
                sender_guid4: Default::default(),
            },
            ChatType::WHISPER_INFORM => Self::WHISPER_INFORM {
                sender_guid4: Default::default(),
            },
            ChatType::EMOTE => Self::EMOTE {
                sender_guid4: Default::default(),
            },
            ChatType::TEXT_EMOTE => Self::TEXT_EMOTE {
                sender_guid4: Default::default(),
            },
            ChatType::SYSTEM => Self::SYSTEM {
                sender_guid4: Default::default(),
            },
            ChatType::MONSTER_SAY => Self::MONSTER_SAY {
                sender_guid3: Default::default(),
                sender_name_length: Default::default(),
                sender_name: Default::default(),
                target_guid: Default::default(),
            },
            ChatType::MONSTER_YELL => Self::MONSTER_YELL {
                sender_guid3: Default::default(),
                sender_name_length: Default::default(),
                sender_name: Default::default(),
                target_guid: Default::default(),
            },
            ChatType::MONSTER_EMOTE => Self::MONSTER_EMOTE {
                name_length: Default::default(),
                monster_name: Default::default(),
                monster_guid: Default::default(),
            },
            ChatType::CHANNEL => Self::CHANNEL {
                channel_name: Default::default(),
                player_rank: Default::default(),
                player_guid: Default::default(),
            },
            ChatType::CHANNEL_JOIN => Self::CHANNEL_JOIN {
                sender_guid4: Default::default(),
            },
            ChatType::CHANNEL_LEAVE => Self::CHANNEL_LEAVE {
                sender_guid4: Default::default(),
            },
            ChatType::CHANNEL_LIST => Self::CHANNEL_LIST {
                sender_guid4: Default::default(),
            },
            ChatType::CHANNEL_NOTICE => Self::CHANNEL_NOTICE {
                sender_guid4: Default::default(),
            },
            ChatType::CHANNEL_NOTICE_USER => Self::CHANNEL_NOTICE_USER {
                sender_guid4: Default::default(),
            },
            ChatType::AFK => Self::AFK {
                sender_guid4: Default::default(),
            },
            ChatType::DND => Self::DND {
                sender_guid4: Default::default(),
            },
            ChatType::IGNORED => Self::IGNORED {
                sender_guid4: Default::default(),
            },
            ChatType::SKILL => Self::SKILL {
                sender_guid4: Default::default(),
            },
            ChatType::LOOT => Self::LOOT {
                sender_guid4: Default::default(),
            },
            ChatType::MONSTER_WHISPER => Self::MONSTER_WHISPER {
                name_length: Default::default(),
                monster_name: Default::default(),
                monster_guid: Default::default(),
            },
            ChatType::BG_SYSTEM_NEUTRAL => Self::BG_SYSTEM_NEUTRAL {
                sender_guid4: Default::default(),
            },
            ChatType::BG_SYSTEM_ALLIANCE => Self::BG_SYSTEM_ALLIANCE {
                sender_guid4: Default::default(),
            },
            ChatType::BG_SYSTEM_HORDE => Self::BG_SYSTEM_HORDE {
                sender_guid4: Default::default(),
            },
            ChatType::RAID_LEADER => Self::RAID_LEADER {
                sender_guid4: Default::default(),
            },
            ChatType::RAID_WARNING => Self::RAID_WARNING {
                sender_guid4: Default::default(),
            },
            ChatType::RAID_BOSS_WHISPER => Self::RAID_BOSS_WHISPER {
                sender_guid4: Default::default(),
            },
            ChatType::RAID_BOSS_EMOTE => Self::RAID_BOSS_EMOTE {
                name_length: Default::default(),
                monster_name: Default::default(),
                monster_guid: Default::default(),
            },
            ChatType::BATTLEGROUND => Self::BATTLEGROUND {
                sender_guid4: Default::default(),
            },
            ChatType::BATTLEGROUND_LEADER => Self::BATTLEGROUND_LEADER {
                sender_guid4: Default::default(),
            },
        }
    }
}

impl From<&SMSG_MESSAGECHATChatType> for ChatType {
    fn from(v: &SMSG_MESSAGECHATChatType) -> Self {
        match &v {
            SMSG_MESSAGECHATChatType::SAY { .. } => Self::SAY,
            SMSG_MESSAGECHATChatType::PARTY { .. } => Self::PARTY,
            SMSG_MESSAGECHATChatType::RAID { .. } => Self::RAID,
            SMSG_MESSAGECHATChatType::GUILD { .. } => Self::GUILD,
            SMSG_MESSAGECHATChatType::OFFICER { .. } => Self::OFFICER,
            SMSG_MESSAGECHATChatType::YELL { .. } => Self::YELL,
            SMSG_MESSAGECHATChatType::WHISPER { .. } => Self::WHISPER,
            SMSG_MESSAGECHATChatType::WHISPER_INFORM { .. } => Self::WHISPER_INFORM,
            SMSG_MESSAGECHATChatType::EMOTE { .. } => Self::EMOTE,
            SMSG_MESSAGECHATChatType::TEXT_EMOTE { .. } => Self::TEXT_EMOTE,
            SMSG_MESSAGECHATChatType::SYSTEM { .. } => Self::SYSTEM,
            SMSG_MESSAGECHATChatType::MONSTER_SAY { .. } => Self::MONSTER_SAY,
            SMSG_MESSAGECHATChatType::MONSTER_YELL { .. } => Self::MONSTER_YELL,
            SMSG_MESSAGECHATChatType::MONSTER_EMOTE { .. } => Self::MONSTER_EMOTE,
            SMSG_MESSAGECHATChatType::CHANNEL { .. } => Self::CHANNEL,
            SMSG_MESSAGECHATChatType::CHANNEL_JOIN { .. } => Self::CHANNEL_JOIN,
            SMSG_MESSAGECHATChatType::CHANNEL_LEAVE { .. } => Self::CHANNEL_LEAVE,
            SMSG_MESSAGECHATChatType::CHANNEL_LIST { .. } => Self::CHANNEL_LIST,
            SMSG_MESSAGECHATChatType::CHANNEL_NOTICE { .. } => Self::CHANNEL_NOTICE,
            SMSG_MESSAGECHATChatType::CHANNEL_NOTICE_USER { .. } => Self::CHANNEL_NOTICE_USER,
            SMSG_MESSAGECHATChatType::AFK { .. } => Self::AFK,
            SMSG_MESSAGECHATChatType::DND { .. } => Self::DND,
            SMSG_MESSAGECHATChatType::IGNORED { .. } => Self::IGNORED,
            SMSG_MESSAGECHATChatType::SKILL { .. } => Self::SKILL,
            SMSG_MESSAGECHATChatType::LOOT { .. } => Self::LOOT,
            SMSG_MESSAGECHATChatType::MONSTER_WHISPER { .. } => Self::MONSTER_WHISPER,
            SMSG_MESSAGECHATChatType::BG_SYSTEM_NEUTRAL { .. } => Self::BG_SYSTEM_NEUTRAL,
            SMSG_MESSAGECHATChatType::BG_SYSTEM_ALLIANCE { .. } => Self::BG_SYSTEM_ALLIANCE,
            SMSG_MESSAGECHATChatType::BG_SYSTEM_HORDE { .. } => Self::BG_SYSTEM_HORDE,
            SMSG_MESSAGECHATChatType::RAID_LEADER { .. } => Self::RAID_LEADER,
            SMSG_MESSAGECHATChatType::RAID_WARNING { .. } => Self::RAID_WARNING,
            SMSG_MESSAGECHATChatType::RAID_BOSS_WHISPER { .. } => Self::RAID_BOSS_WHISPER,
            SMSG_MESSAGECHATChatType::RAID_BOSS_EMOTE { .. } => Self::RAID_BOSS_EMOTE,
            SMSG_MESSAGECHATChatType::BATTLEGROUND { .. } => Self::BATTLEGROUND,
            SMSG_MESSAGECHATChatType::BATTLEGROUND_LEADER { .. } => Self::BATTLEGROUND_LEADER,
        }
    }
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
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.tokio_write_u16_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.astd_write_u16_le(w).await
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.tokio_write_u16_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.astd_write_u16_be(w).await
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.astd_write_u32_le(w).await
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.astd_write_u32_be(w).await
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.astd_write_u64_le(w).await
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ChatType = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for SMSG_MESSAGECHATChatType {
    fn size(&self) -> usize {
        match self {
            Self::SAY  {
                sender_guid1,
                sender_guid2,
            } => {
                1
                + 8 // sender_guid1: Guid
                + 8 // sender_guid2: Guid
            }
            Self::PARTY  {
                sender_guid1,
                sender_guid2,
            } => {
                1
                + 8 // sender_guid1: Guid
                + 8 // sender_guid2: Guid
            }
            Self::RAID  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::GUILD  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::OFFICER  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::YELL  {
                sender_guid1,
                sender_guid2,
            } => {
                1
                + 8 // sender_guid1: Guid
                + 8 // sender_guid2: Guid
            }
            Self::WHISPER  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::WHISPER_INFORM  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::EMOTE  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::TEXT_EMOTE  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::SYSTEM  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::MONSTER_SAY  {
                sender_guid3,
                sender_name_length,
                sender_name,
                target_guid,
            } => {
                1
                + 8 // sender_guid3: Guid
                + 4 // sender_name_length: u32
                + sender_name.len() + 1 // sender_name: CString and Null Terminator
                + 8 // target_guid: Guid
            }
            Self::MONSTER_YELL  {
                sender_guid3,
                sender_name_length,
                sender_name,
                target_guid,
            } => {
                1
                + 8 // sender_guid3: Guid
                + 4 // sender_name_length: u32
                + sender_name.len() + 1 // sender_name: CString and Null Terminator
                + 8 // target_guid: Guid
            }
            Self::MONSTER_EMOTE  {
                name_length,
                monster_name,
                monster_guid,
            } => {
                1
                + 4 // name_length: u32
                + monster_name.len() + 1 // monster_name: CString and Null Terminator
                + 8 // monster_guid: Guid
            }
            Self::CHANNEL  {
                channel_name,
                player_rank,
                player_guid,
            } => {
                1
                + channel_name.len() + 1 // channel_name: CString and Null Terminator
                + 4 // player_rank: u32
                + 8 // player_guid: Guid
            }
            Self::CHANNEL_JOIN  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::CHANNEL_LEAVE  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::CHANNEL_LIST  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::CHANNEL_NOTICE  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::CHANNEL_NOTICE_USER  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::AFK  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::DND  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::IGNORED  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::SKILL  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::LOOT  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::MONSTER_WHISPER  {
                name_length,
                monster_name,
                monster_guid,
            } => {
                1
                + 4 // name_length: u32
                + monster_name.len() + 1 // monster_name: CString and Null Terminator
                + 8 // monster_guid: Guid
            }
            Self::BG_SYSTEM_NEUTRAL  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::BG_SYSTEM_ALLIANCE  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::BG_SYSTEM_HORDE  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RAID_LEADER  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RAID_WARNING  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RAID_BOSS_WHISPER  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::RAID_BOSS_EMOTE  {
                name_length,
                monster_name,
                monster_guid,
            } => {
                1
                + 4 // name_length: u32
                + monster_name.len() + 1 // monster_name: CString and Null Terminator
                + 8 // monster_guid: Guid
            }
            Self::BATTLEGROUND  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
            Self::BATTLEGROUND_LEADER  {
                sender_guid4,
            } => {
                1
                + 8 // sender_guid4: Guid
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_MESSAGECHATChatType {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

