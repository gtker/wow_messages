use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ChatType, ChatTypeError};
use crate::world::v1::v12::{Language, LanguageError};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_MESSAGECHAT {
    pub chat_type: CMSG_MESSAGECHATChatType,
    pub language: Language,
}

impl CMSG_MESSAGECHAT {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int() as u32).to_le_bytes())?;

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

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

        Ok(w)
    }
}

impl ClientMessage for CMSG_MESSAGECHAT {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int() as u32).to_le_bytes())?;

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

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

        Ok(w)
    }
    const OPCODE: u16 = 0x0095;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_MESSAGECHATError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // chat_type: ChatType
        let chat_type: ChatType = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // language: Language
        let language: Language = crate::util::read_u32_le(r)?.try_into()?;

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

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // chat_type: ChatType
            let chat_type: ChatType = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            // language: Language
            let language: Language = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            let chat_type_if = match chat_type {
                ChatType::SAY => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::SAY {
                        message,
                    }
                }
                ChatType::PARTY => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::PARTY {
                        message,
                    }
                }
                ChatType::RAID => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::RAID {
                        message,
                    }
                }
                ChatType::GUILD => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::GUILD {
                        message,
                    }
                }
                ChatType::OFFICER => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::OFFICER {
                        message,
                    }
                }
                ChatType::YELL => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::YELL {
                        message,
                    }
                }
                ChatType::WHISPER => {
                    // target_player: CString
                    let target_player = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let target_player = String::from_utf8(target_player)?;

                    // whisper_message: CString
                    let whisper_message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let whisper_message = String::from_utf8(whisper_message)?;

                    CMSG_MESSAGECHATChatType::WHISPER {
                        target_player,
                        whisper_message,
                    }
                }
                ChatType::WHISPER_INFORM => CMSG_MESSAGECHATChatType::WHISPER_INFORM,
                ChatType::EMOTE => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
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
                    let channel = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let channel = String::from_utf8(channel)?;

                    // channel_message: CString
                    let channel_message = crate::util::tokio_read_c_string_to_vec(r).await?;
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
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::AFK {
                        message,
                    }
                }
                ChatType::DND => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
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
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::RAID_LEADER {
                        message,
                    }
                }
                ChatType::RAID_WARNING => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::RAID_WARNING {
                        message,
                    }
                }
                ChatType::RAID_BOSS_WHISPER => CMSG_MESSAGECHATChatType::RAID_BOSS_WHISPER,
                ChatType::RAID_BOSS_EMOTE => CMSG_MESSAGECHATChatType::RAID_BOSS_EMOTE,
                ChatType::BATTLEGROUND => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::BATTLEGROUND {
                        message,
                    }
                }
                ChatType::BATTLEGROUND_LEADER => {
                    // message: CString
                    let message = crate::util::tokio_read_c_string_to_vec(r).await?;
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
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // chat_type: ChatType
            let chat_type: ChatType = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            // language: Language
            let language: Language = crate::util::astd_read_u32_le(r).await?.try_into()?;

            let chat_type_if = match chat_type {
                ChatType::SAY => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::SAY {
                        message,
                    }
                }
                ChatType::PARTY => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::PARTY {
                        message,
                    }
                }
                ChatType::RAID => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::RAID {
                        message,
                    }
                }
                ChatType::GUILD => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::GUILD {
                        message,
                    }
                }
                ChatType::OFFICER => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::OFFICER {
                        message,
                    }
                }
                ChatType::YELL => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::YELL {
                        message,
                    }
                }
                ChatType::WHISPER => {
                    // target_player: CString
                    let target_player = crate::util::astd_read_c_string_to_vec(r).await?;
                    let target_player = String::from_utf8(target_player)?;

                    // whisper_message: CString
                    let whisper_message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let whisper_message = String::from_utf8(whisper_message)?;

                    CMSG_MESSAGECHATChatType::WHISPER {
                        target_player,
                        whisper_message,
                    }
                }
                ChatType::WHISPER_INFORM => CMSG_MESSAGECHATChatType::WHISPER_INFORM,
                ChatType::EMOTE => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
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
                    let channel = crate::util::astd_read_c_string_to_vec(r).await?;
                    let channel = String::from_utf8(channel)?;

                    // channel_message: CString
                    let channel_message = crate::util::astd_read_c_string_to_vec(r).await?;
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
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::AFK {
                        message,
                    }
                }
                ChatType::DND => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
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
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::RAID_LEADER {
                        message,
                    }
                }
                ChatType::RAID_WARNING => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::RAID_WARNING {
                        message,
                    }
                }
                ChatType::RAID_BOSS_WHISPER => CMSG_MESSAGECHATChatType::RAID_BOSS_WHISPER,
                ChatType::RAID_BOSS_EMOTE => CMSG_MESSAGECHATChatType::RAID_BOSS_EMOTE,
                ChatType::BATTLEGROUND => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
                    let message = String::from_utf8(message)?;

                    CMSG_MESSAGECHATChatType::BATTLEGROUND {
                        message,
                    }
                }
                ChatType::BATTLEGROUND_LEADER => {
                    // message: CString
                    let message = crate::util::astd_read_c_string_to_vec(r).await?;
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
        })
    }

}

impl CMSG_MESSAGECHAT {
    pub fn size(&self) -> usize {
        0
        + self.chat_type.size() // chat_type: CMSG_MESSAGECHATChatType
        + 4 // language: Language
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

impl Default for CMSG_MESSAGECHATChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SAY {
            message: Default::default(),
        }
    }
}

impl CMSG_MESSAGECHATChatType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SAY { .. } => 0,
            Self::PARTY { .. } => 1,
            Self::RAID { .. } => 2,
            Self::GUILD { .. } => 3,
            Self::OFFICER { .. } => 4,
            Self::YELL { .. } => 5,
            Self::WHISPER { .. } => 6,
            Self::WHISPER_INFORM => 7,
            Self::EMOTE { .. } => 8,
            Self::TEXT_EMOTE => 9,
            Self::SYSTEM => 10,
            Self::MONSTER_SAY => 11,
            Self::MONSTER_YELL => 12,
            Self::MONSTER_EMOTE => 13,
            Self::CHANNEL { .. } => 14,
            Self::CHANNEL_JOIN => 15,
            Self::CHANNEL_LEAVE => 16,
            Self::CHANNEL_LIST => 17,
            Self::CHANNEL_NOTICE => 18,
            Self::CHANNEL_NOTICE_USER => 19,
            Self::AFK { .. } => 20,
            Self::DND { .. } => 21,
            Self::IGNORED => 22,
            Self::SKILL => 23,
            Self::LOOT => 24,
            Self::MONSTER_WHISPER => 26,
            Self::BG_SYSTEM_NEUTRAL => 82,
            Self::BG_SYSTEM_ALLIANCE => 83,
            Self::BG_SYSTEM_HORDE => 84,
            Self::RAID_LEADER { .. } => 87,
            Self::RAID_WARNING { .. } => 88,
            Self::RAID_BOSS_WHISPER => 89,
            Self::RAID_BOSS_EMOTE => 90,
            Self::BATTLEGROUND { .. } => 92,
            Self::BATTLEGROUND_LEADER { .. } => 93,
        }
    }

}

impl CMSG_MESSAGECHATChatType {
    pub fn size(&self) -> usize {
        match self {
            Self::SAY {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::PARTY {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::RAID {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::GUILD {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::OFFICER {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::YELL {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::WHISPER {
                target_player,
                whisper_message,
            } => {
                4
                + target_player.len() + 1 // target_player: CString
                + whisper_message.len() + 1 // whisper_message: CString
            }
            Self::WHISPER_INFORM => {
                4
            }
            Self::EMOTE {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::TEXT_EMOTE => {
                4
            }
            Self::SYSTEM => {
                4
            }
            Self::MONSTER_SAY => {
                4
            }
            Self::MONSTER_YELL => {
                4
            }
            Self::MONSTER_EMOTE => {
                4
            }
            Self::CHANNEL {
                channel,
                channel_message,
            } => {
                4
                + channel.len() + 1 // channel: CString
                + channel_message.len() + 1 // channel_message: CString
            }
            Self::CHANNEL_JOIN => {
                4
            }
            Self::CHANNEL_LEAVE => {
                4
            }
            Self::CHANNEL_LIST => {
                4
            }
            Self::CHANNEL_NOTICE => {
                4
            }
            Self::CHANNEL_NOTICE_USER => {
                4
            }
            Self::AFK {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::DND {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::IGNORED => {
                4
            }
            Self::SKILL => {
                4
            }
            Self::LOOT => {
                4
            }
            Self::MONSTER_WHISPER => {
                4
            }
            Self::BG_SYSTEM_NEUTRAL => {
                4
            }
            Self::BG_SYSTEM_ALLIANCE => {
                4
            }
            Self::BG_SYSTEM_HORDE => {
                4
            }
            Self::RAID_LEADER {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::RAID_WARNING {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::RAID_BOSS_WHISPER => {
                4
            }
            Self::RAID_BOSS_EMOTE => {
                4
            }
            Self::BATTLEGROUND {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
            Self::BATTLEGROUND_LEADER {
                message,
            } => {
                4
                + message.len() + 1 // message: CString
            }
        }
    }
}

