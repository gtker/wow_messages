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

impl ClientMessage for CMSG_MESSAGECHAT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int() as u32).to_le_bytes())?;

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

        match &self.chat_type {
            CMSG_MESSAGECHAT_ChatType::SAY => {}
            CMSG_MESSAGECHAT_ChatType::PARTY => {}
            CMSG_MESSAGECHAT_ChatType::RAID => {}
            CMSG_MESSAGECHAT_ChatType::GUILD => {}
            CMSG_MESSAGECHAT_ChatType::OFFICER => {}
            CMSG_MESSAGECHAT_ChatType::YELL => {}
            CMSG_MESSAGECHAT_ChatType::WHISPER {
                target_player,
            } => {
                // target_player: CString
                w.write_all(target_player.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::WHISPER_INFORM => {}
            CMSG_MESSAGECHAT_ChatType::EMOTE => {}
            CMSG_MESSAGECHAT_ChatType::TEXT_EMOTE => {}
            CMSG_MESSAGECHAT_ChatType::SYSTEM => {}
            CMSG_MESSAGECHAT_ChatType::MONSTER_SAY => {}
            CMSG_MESSAGECHAT_ChatType::MONSTER_YELL => {}
            CMSG_MESSAGECHAT_ChatType::MONSTER_EMOTE => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL {
                channel,
            } => {
                // channel: CString
                w.write_all(channel.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            CMSG_MESSAGECHAT_ChatType::CHANNEL_JOIN => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL_LEAVE => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL_LIST => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE => {}
            CMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE_USER => {}
            CMSG_MESSAGECHAT_ChatType::AFK => {}
            CMSG_MESSAGECHAT_ChatType::DND => {}
            CMSG_MESSAGECHAT_ChatType::IGNORED => {}
            CMSG_MESSAGECHAT_ChatType::SKILL => {}
            CMSG_MESSAGECHAT_ChatType::LOOT => {}
            CMSG_MESSAGECHAT_ChatType::MONSTER_WHISPER => {}
            CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_NEUTRAL => {}
            CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_ALLIANCE => {}
            CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_HORDE => {}
            CMSG_MESSAGECHAT_ChatType::RAID_LEADER => {}
            CMSG_MESSAGECHAT_ChatType::RAID_WARNING => {}
            CMSG_MESSAGECHAT_ChatType::RAID_BOSS_WHISPER => {}
            CMSG_MESSAGECHAT_ChatType::RAID_BOSS_EMOTE => {}
            CMSG_MESSAGECHAT_ChatType::BATTLEGROUND => {}
            CMSG_MESSAGECHAT_ChatType::BATTLEGROUND_LEADER => {}
        }

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

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
            ChatType::SAY => CMSG_MESSAGECHAT_ChatType::SAY,
            ChatType::PARTY => CMSG_MESSAGECHAT_ChatType::PARTY,
            ChatType::RAID => CMSG_MESSAGECHAT_ChatType::RAID,
            ChatType::GUILD => CMSG_MESSAGECHAT_ChatType::GUILD,
            ChatType::OFFICER => CMSG_MESSAGECHAT_ChatType::OFFICER,
            ChatType::YELL => CMSG_MESSAGECHAT_ChatType::YELL,
            ChatType::WHISPER => {
                // target_player: CString
                let target_player = crate::util::read_c_string_to_vec(r)?;
                let target_player = String::from_utf8(target_player)?;

                CMSG_MESSAGECHAT_ChatType::WHISPER {
                    target_player,
                }
            }
            ChatType::WHISPER_INFORM => CMSG_MESSAGECHAT_ChatType::WHISPER_INFORM,
            ChatType::EMOTE => CMSG_MESSAGECHAT_ChatType::EMOTE,
            ChatType::TEXT_EMOTE => CMSG_MESSAGECHAT_ChatType::TEXT_EMOTE,
            ChatType::SYSTEM => CMSG_MESSAGECHAT_ChatType::SYSTEM,
            ChatType::MONSTER_SAY => CMSG_MESSAGECHAT_ChatType::MONSTER_SAY,
            ChatType::MONSTER_YELL => CMSG_MESSAGECHAT_ChatType::MONSTER_YELL,
            ChatType::MONSTER_EMOTE => CMSG_MESSAGECHAT_ChatType::MONSTER_EMOTE,
            ChatType::CHANNEL => {
                // channel: CString
                let channel = crate::util::read_c_string_to_vec(r)?;
                let channel = String::from_utf8(channel)?;

                CMSG_MESSAGECHAT_ChatType::CHANNEL {
                    channel,
                }
            }
            ChatType::CHANNEL_JOIN => CMSG_MESSAGECHAT_ChatType::CHANNEL_JOIN,
            ChatType::CHANNEL_LEAVE => CMSG_MESSAGECHAT_ChatType::CHANNEL_LEAVE,
            ChatType::CHANNEL_LIST => CMSG_MESSAGECHAT_ChatType::CHANNEL_LIST,
            ChatType::CHANNEL_NOTICE => CMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE,
            ChatType::CHANNEL_NOTICE_USER => CMSG_MESSAGECHAT_ChatType::CHANNEL_NOTICE_USER,
            ChatType::AFK => CMSG_MESSAGECHAT_ChatType::AFK,
            ChatType::DND => CMSG_MESSAGECHAT_ChatType::DND,
            ChatType::IGNORED => CMSG_MESSAGECHAT_ChatType::IGNORED,
            ChatType::SKILL => CMSG_MESSAGECHAT_ChatType::SKILL,
            ChatType::LOOT => CMSG_MESSAGECHAT_ChatType::LOOT,
            ChatType::MONSTER_WHISPER => CMSG_MESSAGECHAT_ChatType::MONSTER_WHISPER,
            ChatType::BG_SYSTEM_NEUTRAL => CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_NEUTRAL,
            ChatType::BG_SYSTEM_ALLIANCE => CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_ALLIANCE,
            ChatType::BG_SYSTEM_HORDE => CMSG_MESSAGECHAT_ChatType::BG_SYSTEM_HORDE,
            ChatType::RAID_LEADER => CMSG_MESSAGECHAT_ChatType::RAID_LEADER,
            ChatType::RAID_WARNING => CMSG_MESSAGECHAT_ChatType::RAID_WARNING,
            ChatType::RAID_BOSS_WHISPER => CMSG_MESSAGECHAT_ChatType::RAID_BOSS_WHISPER,
            ChatType::RAID_BOSS_EMOTE => CMSG_MESSAGECHAT_ChatType::RAID_BOSS_EMOTE,
            ChatType::BATTLEGROUND => CMSG_MESSAGECHAT_ChatType::BATTLEGROUND,
            ChatType::BATTLEGROUND_LEADER => CMSG_MESSAGECHAT_ChatType::BATTLEGROUND_LEADER,
        };

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            chat_type: chat_type_if,
            language,
            message,
        })
    }

}

impl CMSG_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: CMSG_MESSAGECHAT_ChatType
        + 4 // language: Language
        + self.message.len() + 1 // message: CString
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMSG_MESSAGECHAT_ChatType {
    SAY,
    PARTY,
    RAID,
    GUILD,
    OFFICER,
    YELL,
    WHISPER {
        target_player: String,
    },
    WHISPER_INFORM,
    EMOTE,
    TEXT_EMOTE,
    SYSTEM,
    MONSTER_SAY,
    MONSTER_YELL,
    MONSTER_EMOTE,
    CHANNEL {
        channel: String,
    },
    CHANNEL_JOIN,
    CHANNEL_LEAVE,
    CHANNEL_LIST,
    CHANNEL_NOTICE,
    CHANNEL_NOTICE_USER,
    AFK,
    DND,
    IGNORED,
    SKILL,
    LOOT,
    MONSTER_WHISPER,
    BG_SYSTEM_NEUTRAL,
    BG_SYSTEM_ALLIANCE,
    BG_SYSTEM_HORDE,
    RAID_LEADER,
    RAID_WARNING,
    RAID_BOSS_WHISPER,
    RAID_BOSS_EMOTE,
    BATTLEGROUND,
    BATTLEGROUND_LEADER,
}

impl Default for CMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SAY
    }
}

impl CMSG_MESSAGECHAT_ChatType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SAY => 0,
            Self::PARTY => 1,
            Self::RAID => 2,
            Self::GUILD => 3,
            Self::OFFICER => 4,
            Self::YELL => 5,
            Self::WHISPER { .. } => 6,
            Self::WHISPER_INFORM => 7,
            Self::EMOTE => 8,
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
            Self::AFK => 20,
            Self::DND => 21,
            Self::IGNORED => 22,
            Self::SKILL => 23,
            Self::LOOT => 24,
            Self::MONSTER_WHISPER => 26,
            Self::BG_SYSTEM_NEUTRAL => 82,
            Self::BG_SYSTEM_ALLIANCE => 83,
            Self::BG_SYSTEM_HORDE => 84,
            Self::RAID_LEADER => 87,
            Self::RAID_WARNING => 88,
            Self::RAID_BOSS_WHISPER => 89,
            Self::RAID_BOSS_EMOTE => 90,
            Self::BATTLEGROUND => 92,
            Self::BATTLEGROUND_LEADER => 93,
        }
    }

}

impl CMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::SAY => {
                4
            }
            Self::PARTY => {
                4
            }
            Self::RAID => {
                4
            }
            Self::GUILD => {
                4
            }
            Self::OFFICER => {
                4
            }
            Self::YELL => {
                4
            }
            Self::WHISPER {
                target_player,
            } => {
                4
                + target_player.len() + 1 // target_player: CString
            }
            Self::WHISPER_INFORM => {
                4
            }
            Self::EMOTE => {
                4
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
            } => {
                4
                + channel.len() + 1 // channel: CString
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
            Self::AFK => {
                4
            }
            Self::DND => {
                4
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
            Self::RAID_LEADER => {
                4
            }
            Self::RAID_WARNING => {
                4
            }
            Self::RAID_BOSS_WHISPER => {
                4
            }
            Self::RAID_BOSS_EMOTE => {
                4
            }
            Self::BATTLEGROUND => {
                4
            }
            Self::BATTLEGROUND_LEADER => {
                4
            }
        }
    }
}

