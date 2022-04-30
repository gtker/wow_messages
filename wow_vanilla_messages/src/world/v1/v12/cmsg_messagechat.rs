use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ChatType, ChatTypeError};
use crate::world::v1::v12::{Language, LanguageError};
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_MESSAGECHAT {
    pub chat_type: CMSG_MESSAGECHATChatType,
    pub language: Language,
}

impl ClientMessageWrite for CMSG_MESSAGECHAT {
    const OPCODE: u32 = 0x95;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as ClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as ClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for CMSG_MESSAGECHAT {
    type Error = CMSG_MESSAGECHATError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // chat_type: ChatType
        let chat_type = ChatType::read_u32_le(r)?;

        // language: Language
        let language = Language::read(r)?;

        let chat_type_if = match chat_type {
            ChatType::SAY => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::SAY {
                    message,
                }
            }
            ChatType::PARTY => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::PARTY {
                    message,
                }
            }
            ChatType::RAID => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::RAID {
                    message,
                }
            }
            ChatType::GUILD => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::GUILD {
                    message,
                }
            }
            ChatType::OFFICER => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::OFFICER {
                    message,
                }
            }
            ChatType::YELL => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::YELL {
                    message,
                }
            }
            ChatType::WHISPER => {
                // target_player: CString
                let target_player = crate::util::read_c_string_to_vec(r)?;
                let target_player = String::from_utf8(target_player)?;

                // whisper_message: CString
                let whisper_message = crate::util::read_c_string_to_vec(r)?;
                let whisper_message = String::from_utf8(whisper_message)?;

                CMSG_MESSAGECHATChatType::WHISPER {
                    target_player,
                    whisper_message,
                }
            }
            ChatType::WHISPER_INFORM => CMSG_MESSAGECHATChatType::WHISPER_INFORM,
            ChatType::EMOTE => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::EMOTE {
                    message,
                }
            }
            ChatType::TEXT_EMOTE => CMSG_MESSAGECHATChatType::TEXT_EMOTE,
            ChatType::SYSTEM => CMSG_MESSAGECHATChatType::SYSTEM,
            ChatType::MONSTER_SAY => CMSG_MESSAGECHATChatType::MONSTER_SAY,
            ChatType::MONSTER_YELL => CMSG_MESSAGECHATChatType::MONSTER_YELL,
            ChatType::MONSTER_EMOTE => CMSG_MESSAGECHATChatType::MONSTER_EMOTE,
            ChatType::CHANNEL => {
                // channel: CString
                let channel = crate::util::read_c_string_to_vec(r)?;
                let channel = String::from_utf8(channel)?;

                // channel_message: CString
                let channel_message = crate::util::read_c_string_to_vec(r)?;
                let channel_message = String::from_utf8(channel_message)?;

                CMSG_MESSAGECHATChatType::CHANNEL {
                    channel,
                    channel_message,
                }
            }
            ChatType::CHANNEL_JOIN => CMSG_MESSAGECHATChatType::CHANNEL_JOIN,
            ChatType::CHANNEL_LEAVE => CMSG_MESSAGECHATChatType::CHANNEL_LEAVE,
            ChatType::CHANNEL_LIST => CMSG_MESSAGECHATChatType::CHANNEL_LIST,
            ChatType::CHANNEL_NOTICE => CMSG_MESSAGECHATChatType::CHANNEL_NOTICE,
            ChatType::CHANNEL_NOTICE_USER => CMSG_MESSAGECHATChatType::CHANNEL_NOTICE_USER,
            ChatType::AFK => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::AFK {
                    message,
                }
            }
            ChatType::DND => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::DND {
                    message,
                }
            }
            ChatType::IGNORED => CMSG_MESSAGECHATChatType::IGNORED,
            ChatType::SKILL => CMSG_MESSAGECHATChatType::SKILL,
            ChatType::LOOT => CMSG_MESSAGECHATChatType::LOOT,
            ChatType::MONSTER_WHISPER => CMSG_MESSAGECHATChatType::MONSTER_WHISPER,
            ChatType::BG_SYSTEM_NEUTRAL => CMSG_MESSAGECHATChatType::BG_SYSTEM_NEUTRAL,
            ChatType::BG_SYSTEM_ALLIANCE => CMSG_MESSAGECHATChatType::BG_SYSTEM_ALLIANCE,
            ChatType::BG_SYSTEM_HORDE => CMSG_MESSAGECHATChatType::BG_SYSTEM_HORDE,
            ChatType::RAID_LEADER => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::RAID_LEADER {
                    message,
                }
            }
            ChatType::RAID_WARNING => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::RAID_WARNING {
                    message,
                }
            }
            ChatType::RAID_BOSS_WHISPER => CMSG_MESSAGECHATChatType::RAID_BOSS_WHISPER,
            ChatType::RAID_BOSS_EMOTE => CMSG_MESSAGECHATChatType::RAID_BOSS_EMOTE,
            ChatType::BATTLEGROUND => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::BATTLEGROUND {
                    message,
                }
            }
            ChatType::BATTLEGROUND_LEADER => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHATChatType::BATTLEGROUND_LEADER {
                    message,
                }
            }
        };

        Ok(Self {
            chat_type: chat_type_if,
            language,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // chat_type: ChatType
        self.chat_type.write_u32_le(w)?;

        // language: Language
        self.language.write(w)?;

        match &self.chat_type {
            CMSG_MESSAGECHATChatType::SAY {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::PARTY {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::RAID {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::GUILD {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::OFFICER {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::YELL {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::WHISPER {
                target_player,
                whisper_message,
            } => {
                // target_player: CString
                w.write_all(target_player.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // whisper_message: CString
                w.write_all(whisper_message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::WHISPER_INFORM => {}
            CMSG_MESSAGECHATChatType::EMOTE {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::TEXT_EMOTE => {}
            CMSG_MESSAGECHATChatType::SYSTEM => {}
            CMSG_MESSAGECHATChatType::MONSTER_SAY => {}
            CMSG_MESSAGECHATChatType::MONSTER_YELL => {}
            CMSG_MESSAGECHATChatType::MONSTER_EMOTE => {}
            CMSG_MESSAGECHATChatType::CHANNEL {
                channel,
                channel_message,
            } => {
                // channel: CString
                w.write_all(channel.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // channel_message: CString
                w.write_all(channel_message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::CHANNEL_JOIN => {}
            CMSG_MESSAGECHATChatType::CHANNEL_LEAVE => {}
            CMSG_MESSAGECHATChatType::CHANNEL_LIST => {}
            CMSG_MESSAGECHATChatType::CHANNEL_NOTICE => {}
            CMSG_MESSAGECHATChatType::CHANNEL_NOTICE_USER => {}
            CMSG_MESSAGECHATChatType::AFK {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::DND {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::IGNORED => {}
            CMSG_MESSAGECHATChatType::SKILL => {}
            CMSG_MESSAGECHATChatType::LOOT => {}
            CMSG_MESSAGECHATChatType::MONSTER_WHISPER => {}
            CMSG_MESSAGECHATChatType::BG_SYSTEM_NEUTRAL => {}
            CMSG_MESSAGECHATChatType::BG_SYSTEM_ALLIANCE => {}
            CMSG_MESSAGECHATChatType::BG_SYSTEM_HORDE => {}
            CMSG_MESSAGECHATChatType::RAID_LEADER {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::RAID_WARNING {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::RAID_BOSS_WHISPER => {}
            CMSG_MESSAGECHATChatType::RAID_BOSS_EMOTE => {}
            CMSG_MESSAGECHATChatType::BATTLEGROUND {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHATChatType::BATTLEGROUND_LEADER {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
        }

        Ok(())
    }
}

impl VariableSized for CMSG_MESSAGECHAT {
    fn size(&self) -> usize {
        4 // chat_type: ChatType upcasted to u32
        + Language::size() // language: Language
    }
}

impl MaximumPossibleSized for CMSG_MESSAGECHAT {
    fn maximum_possible_size() -> usize {
        ChatType::maximum_possible_size() // chat_type: ChatType
        + Language::maximum_possible_size() // language: Language
    }
}

#[derive(Debug)]
pub enum CMSG_MESSAGECHATError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    ChatType(ChatTypeError),
    Language(LanguageError),
}

impl std::error::Error for CMSG_MESSAGECHATError {}
impl std::fmt::Display for CMSG_MESSAGECHATError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::ChatType(i) => i.fmt(f),
            Self::Language(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_MESSAGECHATError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_MESSAGECHATError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ChatTypeError> for CMSG_MESSAGECHATError {
    fn from(e: ChatTypeError) -> Self {
        Self::ChatType(e)
    }
}

impl From<LanguageError> for CMSG_MESSAGECHATError {
    fn from(e: LanguageError) -> Self {
        Self::Language(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMSG_MESSAGECHATChatType {
    SAY {
        message: String,
    },
    PARTY {
        message: String,
    },
    RAID {
        message: String,
    },
    GUILD {
        message: String,
    },
    OFFICER {
        message: String,
    },
    YELL {
        message: String,
    },
    WHISPER {
        target_player: String,
        whisper_message: String,
    },
    WHISPER_INFORM,
    EMOTE {
        message: String,
    },
    TEXT_EMOTE,
    SYSTEM,
    MONSTER_SAY,
    MONSTER_YELL,
    MONSTER_EMOTE,
    CHANNEL {
        channel: String,
        channel_message: String,
    },
    CHANNEL_JOIN,
    CHANNEL_LEAVE,
    CHANNEL_LIST,
    CHANNEL_NOTICE,
    CHANNEL_NOTICE_USER,
    AFK {
        message: String,
    },
    DND {
        message: String,
    },
    IGNORED,
    SKILL,
    LOOT,
    MONSTER_WHISPER,
    BG_SYSTEM_NEUTRAL,
    BG_SYSTEM_ALLIANCE,
    BG_SYSTEM_HORDE,
    RAID_LEADER {
        message: String,
    },
    RAID_WARNING {
        message: String,
    },
    RAID_BOSS_WHISPER,
    RAID_BOSS_EMOTE,
    BATTLEGROUND {
        message: String,
    },
    BATTLEGROUND_LEADER {
        message: String,
    },
}

impl From<&ChatType> for CMSG_MESSAGECHATChatType {
    fn from(e: &ChatType) -> Self {
        match &e {
            ChatType::SAY => Self::SAY {
                message: Default::default(),
            },
            ChatType::PARTY => Self::PARTY {
                message: Default::default(),
            },
            ChatType::RAID => Self::RAID {
                message: Default::default(),
            },
            ChatType::GUILD => Self::GUILD {
                message: Default::default(),
            },
            ChatType::OFFICER => Self::OFFICER {
                message: Default::default(),
            },
            ChatType::YELL => Self::YELL {
                message: Default::default(),
            },
            ChatType::WHISPER => Self::WHISPER {
                target_player: Default::default(),
                whisper_message: Default::default(),
            },
            ChatType::WHISPER_INFORM => Self::WHISPER_INFORM,
            ChatType::EMOTE => Self::EMOTE {
                message: Default::default(),
            },
            ChatType::TEXT_EMOTE => Self::TEXT_EMOTE,
            ChatType::SYSTEM => Self::SYSTEM,
            ChatType::MONSTER_SAY => Self::MONSTER_SAY,
            ChatType::MONSTER_YELL => Self::MONSTER_YELL,
            ChatType::MONSTER_EMOTE => Self::MONSTER_EMOTE,
            ChatType::CHANNEL => Self::CHANNEL {
                channel: Default::default(),
                channel_message: Default::default(),
            },
            ChatType::CHANNEL_JOIN => Self::CHANNEL_JOIN,
            ChatType::CHANNEL_LEAVE => Self::CHANNEL_LEAVE,
            ChatType::CHANNEL_LIST => Self::CHANNEL_LIST,
            ChatType::CHANNEL_NOTICE => Self::CHANNEL_NOTICE,
            ChatType::CHANNEL_NOTICE_USER => Self::CHANNEL_NOTICE_USER,
            ChatType::AFK => Self::AFK {
                message: Default::default(),
            },
            ChatType::DND => Self::DND {
                message: Default::default(),
            },
            ChatType::IGNORED => Self::IGNORED,
            ChatType::SKILL => Self::SKILL,
            ChatType::LOOT => Self::LOOT,
            ChatType::MONSTER_WHISPER => Self::MONSTER_WHISPER,
            ChatType::BG_SYSTEM_NEUTRAL => Self::BG_SYSTEM_NEUTRAL,
            ChatType::BG_SYSTEM_ALLIANCE => Self::BG_SYSTEM_ALLIANCE,
            ChatType::BG_SYSTEM_HORDE => Self::BG_SYSTEM_HORDE,
            ChatType::RAID_LEADER => Self::RAID_LEADER {
                message: Default::default(),
            },
            ChatType::RAID_WARNING => Self::RAID_WARNING {
                message: Default::default(),
            },
            ChatType::RAID_BOSS_WHISPER => Self::RAID_BOSS_WHISPER,
            ChatType::RAID_BOSS_EMOTE => Self::RAID_BOSS_EMOTE,
            ChatType::BATTLEGROUND => Self::BATTLEGROUND {
                message: Default::default(),
            },
            ChatType::BATTLEGROUND_LEADER => Self::BATTLEGROUND_LEADER {
                message: Default::default(),
            },
        }
    }
}

impl From<&CMSG_MESSAGECHATChatType> for ChatType {
    fn from(v: &CMSG_MESSAGECHATChatType) -> Self {
        match &v {
            CMSG_MESSAGECHATChatType::SAY { .. } => Self::SAY,
            CMSG_MESSAGECHATChatType::PARTY { .. } => Self::PARTY,
            CMSG_MESSAGECHATChatType::RAID { .. } => Self::RAID,
            CMSG_MESSAGECHATChatType::GUILD { .. } => Self::GUILD,
            CMSG_MESSAGECHATChatType::OFFICER { .. } => Self::OFFICER,
            CMSG_MESSAGECHATChatType::YELL { .. } => Self::YELL,
            CMSG_MESSAGECHATChatType::WHISPER { .. } => Self::WHISPER,
            CMSG_MESSAGECHATChatType::WHISPER_INFORM => Self::WHISPER_INFORM,
            CMSG_MESSAGECHATChatType::EMOTE { .. } => Self::EMOTE,
            CMSG_MESSAGECHATChatType::TEXT_EMOTE => Self::TEXT_EMOTE,
            CMSG_MESSAGECHATChatType::SYSTEM => Self::SYSTEM,
            CMSG_MESSAGECHATChatType::MONSTER_SAY => Self::MONSTER_SAY,
            CMSG_MESSAGECHATChatType::MONSTER_YELL => Self::MONSTER_YELL,
            CMSG_MESSAGECHATChatType::MONSTER_EMOTE => Self::MONSTER_EMOTE,
            CMSG_MESSAGECHATChatType::CHANNEL { .. } => Self::CHANNEL,
            CMSG_MESSAGECHATChatType::CHANNEL_JOIN => Self::CHANNEL_JOIN,
            CMSG_MESSAGECHATChatType::CHANNEL_LEAVE => Self::CHANNEL_LEAVE,
            CMSG_MESSAGECHATChatType::CHANNEL_LIST => Self::CHANNEL_LIST,
            CMSG_MESSAGECHATChatType::CHANNEL_NOTICE => Self::CHANNEL_NOTICE,
            CMSG_MESSAGECHATChatType::CHANNEL_NOTICE_USER => Self::CHANNEL_NOTICE_USER,
            CMSG_MESSAGECHATChatType::AFK { .. } => Self::AFK,
            CMSG_MESSAGECHATChatType::DND { .. } => Self::DND,
            CMSG_MESSAGECHATChatType::IGNORED => Self::IGNORED,
            CMSG_MESSAGECHATChatType::SKILL => Self::SKILL,
            CMSG_MESSAGECHATChatType::LOOT => Self::LOOT,
            CMSG_MESSAGECHATChatType::MONSTER_WHISPER => Self::MONSTER_WHISPER,
            CMSG_MESSAGECHATChatType::BG_SYSTEM_NEUTRAL => Self::BG_SYSTEM_NEUTRAL,
            CMSG_MESSAGECHATChatType::BG_SYSTEM_ALLIANCE => Self::BG_SYSTEM_ALLIANCE,
            CMSG_MESSAGECHATChatType::BG_SYSTEM_HORDE => Self::BG_SYSTEM_HORDE,
            CMSG_MESSAGECHATChatType::RAID_LEADER { .. } => Self::RAID_LEADER,
            CMSG_MESSAGECHATChatType::RAID_WARNING { .. } => Self::RAID_WARNING,
            CMSG_MESSAGECHATChatType::RAID_BOSS_WHISPER => Self::RAID_BOSS_WHISPER,
            CMSG_MESSAGECHATChatType::RAID_BOSS_EMOTE => Self::RAID_BOSS_EMOTE,
            CMSG_MESSAGECHATChatType::BATTLEGROUND { .. } => Self::BATTLEGROUND,
            CMSG_MESSAGECHATChatType::BATTLEGROUND_LEADER { .. } => Self::BATTLEGROUND_LEADER,
        }
    }
}

impl Default for CMSG_MESSAGECHATChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SAY {
            message: Default::default(),
        }
    }
}

impl CMSG_MESSAGECHATChatType {
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

impl VariableSized for CMSG_MESSAGECHATChatType {
    fn size(&self) -> usize {
        match self {
            Self::SAY  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::PARTY  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::RAID  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::GUILD  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::OFFICER  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::YELL  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::WHISPER  {
                target_player,
                whisper_message,
            } => {
                1
                + target_player.len() + 1 // target_player: CString and Null Terminator
                + whisper_message.len() + 1 // whisper_message: CString and Null Terminator
            }
            Self::WHISPER_INFORM =>  {
                1
            }
            Self::EMOTE  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::TEXT_EMOTE =>  {
                1
            }
            Self::SYSTEM =>  {
                1
            }
            Self::MONSTER_SAY =>  {
                1
            }
            Self::MONSTER_YELL =>  {
                1
            }
            Self::MONSTER_EMOTE =>  {
                1
            }
            Self::CHANNEL  {
                channel,
                channel_message,
            } => {
                1
                + channel.len() + 1 // channel: CString and Null Terminator
                + channel_message.len() + 1 // channel_message: CString and Null Terminator
            }
            Self::CHANNEL_JOIN =>  {
                1
            }
            Self::CHANNEL_LEAVE =>  {
                1
            }
            Self::CHANNEL_LIST =>  {
                1
            }
            Self::CHANNEL_NOTICE =>  {
                1
            }
            Self::CHANNEL_NOTICE_USER =>  {
                1
            }
            Self::AFK  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::DND  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::IGNORED =>  {
                1
            }
            Self::SKILL =>  {
                1
            }
            Self::LOOT =>  {
                1
            }
            Self::MONSTER_WHISPER =>  {
                1
            }
            Self::BG_SYSTEM_NEUTRAL =>  {
                1
            }
            Self::BG_SYSTEM_ALLIANCE =>  {
                1
            }
            Self::BG_SYSTEM_HORDE =>  {
                1
            }
            Self::RAID_LEADER  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::RAID_WARNING  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::RAID_BOSS_WHISPER =>  {
                1
            }
            Self::RAID_BOSS_EMOTE =>  {
                1
            }
            Self::BATTLEGROUND  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
            Self::BATTLEGROUND_LEADER  {
                message,
            } => {
                1
                + message.len() + 1 // message: CString and Null Terminator
            }
        }
    }
}

impl MaximumPossibleSized for CMSG_MESSAGECHATChatType {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

