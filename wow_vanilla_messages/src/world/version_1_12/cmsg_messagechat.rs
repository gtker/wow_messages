use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::ChatType;
use crate::world::version_1_12::Language;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm#L10):
/// ```text
/// cmsg CMSG_MESSAGECHAT = 0x0095 {
///     ChatType chat_type;
///     Language language;
///     if (chat_type == SAY
///         || chat_type == EMOTE
///         || chat_type == YELL
///         || chat_type == PARTY
///         || chat_type == GUILD
///         || chat_type == OFFICER
///         || chat_type == RAID
///         || chat_type == RAID_LEADER
///         || chat_type == RAID_WARNING
///         || chat_type == BATTLEGROUND
///         || chat_type == BATTLEGROUND_LEADER
///         || chat_type == AFK
///         || chat_type == DND) {
///         CString message;
///     }
///     else if (chat_type == WHISPER) {
///         CString target_player;
///         CString whisper_message;
///     }
///     else if (chat_type == CHANNEL) {
///         CString channel;
///         CString channel_message;
///     }
/// }
/// ```
pub struct CMSG_MESSAGECHAT {
    pub chat_type: CMSG_MESSAGECHAT_ChatType,
    pub language: Language,
}

impl ClientMessage for CMSG_MESSAGECHAT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int() as u32).to_le_bytes())?;

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

        match &self.chat_type {
            CMSG_MESSAGECHAT_ChatType::SAY {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::PARTY {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::RAID {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::GUILD {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::OFFICER {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::YELL {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::WHISPER {
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
            CMSG_MESSAGECHAT_ChatType::WHISPER_INFORM => {}
            CMSG_MESSAGECHAT_ChatType::EMOTE {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::TEXT_EMOTE => {}
            CMSG_MESSAGECHAT_ChatType::SYSTEM => {}
            CMSG_MESSAGECHAT_ChatType::MONSTER_SAY => {}
            CMSG_MESSAGECHAT_ChatType::MONSTER_YELL => {}
            CMSG_MESSAGECHAT_ChatType::MONSTER_EMOTE => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL {
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
            CMSG_MESSAGECHAT_ChatType::CHANNEL_JOIN => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL_LEAVE => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL_LIST => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE_USER => {}
            CMSG_MESSAGECHAT_ChatType::AFK {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::DND {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::IGNORED => {}
            CMSG_MESSAGECHAT_ChatType::SKILL => {}
            CMSG_MESSAGECHAT_ChatType::LOOT => {}
            CMSG_MESSAGECHAT_ChatType::MONSTER_WHISPER => {}
            CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_NEUTRAL => {}
            CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_ALLIANCE => {}
            CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_HORDE => {}
            CMSG_MESSAGECHAT_ChatType::RAID_LEADER {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::RAID_WARNING {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::RAID_BOSS_WHISPER => {}
            CMSG_MESSAGECHAT_ChatType::RAID_BOSS_EMOTE => {}
            CMSG_MESSAGECHAT_ChatType::BATTLEGROUND {
                message,
            } => {
                // message: CString
                w.write_all(message.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::BATTLEGROUND_LEADER {
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
    const OPCODE: u16 = 0x0095;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // chat_type: ChatType
        let chat_type: ChatType = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // language: Language
        let language: Language = crate::util::read_u32_le(r)?.try_into()?;

        let chat_type_if = match chat_type {
            ChatType::SAY => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::SAY {
                    message,
                }
            }
            ChatType::PARTY => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::PARTY {
                    message,
                }
            }
            ChatType::RAID => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::RAID {
                    message,
                }
            }
            ChatType::GUILD => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::GUILD {
                    message,
                }
            }
            ChatType::OFFICER => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::OFFICER {
                    message,
                }
            }
            ChatType::YELL => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::YELL {
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

                CMSG_MESSAGECHAT_ChatType::WHISPER {
                    target_player,
                    whisper_message,
                }
            }
            ChatType::WHISPER_INFORM => CMSG_MESSAGECHAT_ChatType::WHISPER_INFORM,
            ChatType::EMOTE => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::EMOTE {
                    message,
                }
            }
            ChatType::TEXT_EMOTE => CMSG_MESSAGECHAT_ChatType::TEXT_EMOTE,
            ChatType::SYSTEM => CMSG_MESSAGECHAT_ChatType::SYSTEM,
            ChatType::MONSTER_SAY => CMSG_MESSAGECHAT_ChatType::MONSTER_SAY,
            ChatType::MONSTER_YELL => CMSG_MESSAGECHAT_ChatType::MONSTER_YELL,
            ChatType::MONSTER_EMOTE => CMSG_MESSAGECHAT_ChatType::MONSTER_EMOTE,
            ChatType::CHANNEL => {
                // channel: CString
                let channel = crate::util::read_c_string_to_vec(r)?;
                let channel = String::from_utf8(channel)?;

                // channel_message: CString
                let channel_message = crate::util::read_c_string_to_vec(r)?;
                let channel_message = String::from_utf8(channel_message)?;

                CMSG_MESSAGECHAT_ChatType::CHANNEL {
                    channel,
                    channel_message,
                }
            }
            ChatType::CHANNEL_JOIN => CMSG_MESSAGECHAT_ChatType::CHANNEL_JOIN,
            ChatType::CHANNEL_LEAVE => CMSG_MESSAGECHAT_ChatType::CHANNEL_LEAVE,
            ChatType::CHANNEL_LIST => CMSG_MESSAGECHAT_ChatType::CHANNEL_LIST,
            ChatType::CHANNEL_NOTICE => CMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE,
            ChatType::CHANNEL_NOTICE_USER => CMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE_USER,
            ChatType::AFK => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::AFK {
                    message,
                }
            }
            ChatType::DND => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::DND {
                    message,
                }
            }
            ChatType::IGNORED => CMSG_MESSAGECHAT_ChatType::IGNORED,
            ChatType::SKILL => CMSG_MESSAGECHAT_ChatType::SKILL,
            ChatType::LOOT => CMSG_MESSAGECHAT_ChatType::LOOT,
            ChatType::MONSTER_WHISPER => CMSG_MESSAGECHAT_ChatType::MONSTER_WHISPER,
            ChatType::BG_SYSTEM_NEUTRAL => CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_NEUTRAL,
            ChatType::BG_SYSTEM_ALLIANCE => CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_ALLIANCE,
            ChatType::BG_SYSTEM_HORDE => CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_HORDE,
            ChatType::RAID_LEADER => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::RAID_LEADER {
                    message,
                }
            }
            ChatType::RAID_WARNING => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::RAID_WARNING {
                    message,
                }
            }
            ChatType::RAID_BOSS_WHISPER => CMSG_MESSAGECHAT_ChatType::RAID_BOSS_WHISPER,
            ChatType::RAID_BOSS_EMOTE => CMSG_MESSAGECHAT_ChatType::RAID_BOSS_EMOTE,
            ChatType::BATTLEGROUND => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::BATTLEGROUND {
                    message,
                }
            }
            ChatType::BATTLEGROUND_LEADER => {
                // message: CString
                let message = crate::util::read_c_string_to_vec(r)?;
                let message = String::from_utf8(message)?;

                CMSG_MESSAGECHAT_ChatType::BATTLEGROUND_LEADER {
                    message,
                }
            }
        };

        Ok(Self {
            chat_type: chat_type_if,
            language,
        })
    }

}

impl CMSG_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: CMSG_MESSAGECHAT_ChatType
        + 4 // language: Language
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMSG_MESSAGECHAT_ChatType {
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

impl Default for CMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SAY {
            message: Default::default(),
        }
    }
}

impl CMSG_MESSAGECHAT_ChatType {
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

impl CMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
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

