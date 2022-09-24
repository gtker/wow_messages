use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::ChatType;
use crate::world::vanilla::Language;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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

impl crate::Message for CMSG_MESSAGECHAT {
    const OPCODE: u32 = 0x0095;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int() as u32).to_le_bytes())?;

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

        match &self.chat_type {
            CMSG_MESSAGECHAT_ChatType::Say => {}
            CMSG_MESSAGECHAT_ChatType::Party => {}
            CMSG_MESSAGECHAT_ChatType::Raid => {}
            CMSG_MESSAGECHAT_ChatType::Guild => {}
            CMSG_MESSAGECHAT_ChatType::Officer => {}
            CMSG_MESSAGECHAT_ChatType::Yell => {}
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
            CMSG_MESSAGECHAT_ChatType::WhisperInform => {}
            CMSG_MESSAGECHAT_ChatType::Emote => {}
            CMSG_MESSAGECHAT_ChatType::TextEmote => {}
            CMSG_MESSAGECHAT_ChatType::System => {}
            CMSG_MESSAGECHAT_ChatType::MonsterSay => {}
            CMSG_MESSAGECHAT_ChatType::MonsterYell => {}
            CMSG_MESSAGECHAT_ChatType::MonsterEmote => {}
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
            CMSG_MESSAGECHAT_ChatType::ChannelJoin => {}
            CMSG_MESSAGECHAT_ChatType::ChannelLeave => {}
            CMSG_MESSAGECHAT_ChatType::ChannelList => {}
            CMSG_MESSAGECHAT_ChatType::ChannelNotice => {}
            CMSG_MESSAGECHAT_ChatType::ChannelNoticeUser => {}
            CMSG_MESSAGECHAT_ChatType::Afk => {}
            CMSG_MESSAGECHAT_ChatType::Dnd => {}
            CMSG_MESSAGECHAT_ChatType::Ignored => {}
            CMSG_MESSAGECHAT_ChatType::Skill => {}
            CMSG_MESSAGECHAT_ChatType::Loot => {}
            CMSG_MESSAGECHAT_ChatType::MonsterWhisper => {}
            CMSG_MESSAGECHAT_ChatType::BgSystemNeutral => {}
            CMSG_MESSAGECHAT_ChatType::BgSystemAlliance => {}
            CMSG_MESSAGECHAT_ChatType::BgSystemHorde => {}
            CMSG_MESSAGECHAT_ChatType::RaidLeader => {}
            CMSG_MESSAGECHAT_ChatType::RaidWarning => {}
            CMSG_MESSAGECHAT_ChatType::RaidBossWhisper => {}
            CMSG_MESSAGECHAT_ChatType::RaidBossEmote => {}
            CMSG_MESSAGECHAT_ChatType::Battleground => {}
            CMSG_MESSAGECHAT_ChatType::BattlegroundLeader => {}
        }

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // chat_type: ChatType
        let chat_type: ChatType = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // language: Language
        let language: Language = crate::util::read_u32_le(r)?.try_into()?;

        let chat_type_if = match chat_type {
            ChatType::Say => CMSG_MESSAGECHAT_ChatType::Say,
            ChatType::Party => CMSG_MESSAGECHAT_ChatType::Party,
            ChatType::Raid => CMSG_MESSAGECHAT_ChatType::Raid,
            ChatType::Guild => CMSG_MESSAGECHAT_ChatType::Guild,
            ChatType::Officer => CMSG_MESSAGECHAT_ChatType::Officer,
            ChatType::Yell => CMSG_MESSAGECHAT_ChatType::Yell,
            ChatType::Whisper => {
                // target_player: CString
                let target_player = crate::util::read_c_string_to_vec(r)?;
                let target_player = String::from_utf8(target_player)?;

                CMSG_MESSAGECHAT_ChatType::Whisper {
                    target_player,
                }
            }
            ChatType::WhisperInform => CMSG_MESSAGECHAT_ChatType::WhisperInform,
            ChatType::Emote => CMSG_MESSAGECHAT_ChatType::Emote,
            ChatType::TextEmote => CMSG_MESSAGECHAT_ChatType::TextEmote,
            ChatType::System => CMSG_MESSAGECHAT_ChatType::System,
            ChatType::MonsterSay => CMSG_MESSAGECHAT_ChatType::MonsterSay,
            ChatType::MonsterYell => CMSG_MESSAGECHAT_ChatType::MonsterYell,
            ChatType::MonsterEmote => CMSG_MESSAGECHAT_ChatType::MonsterEmote,
            ChatType::Channel => {
                // channel: CString
                let channel = crate::util::read_c_string_to_vec(r)?;
                let channel = String::from_utf8(channel)?;

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
            ChatType::MonsterWhisper => CMSG_MESSAGECHAT_ChatType::MonsterWhisper,
            ChatType::BgSystemNeutral => CMSG_MESSAGECHAT_ChatType::BgSystemNeutral,
            ChatType::BgSystemAlliance => CMSG_MESSAGECHAT_ChatType::BgSystemAlliance,
            ChatType::BgSystemHorde => CMSG_MESSAGECHAT_ChatType::BgSystemHorde,
            ChatType::RaidLeader => CMSG_MESSAGECHAT_ChatType::RaidLeader,
            ChatType::RaidWarning => CMSG_MESSAGECHAT_ChatType::RaidWarning,
            ChatType::RaidBossWhisper => CMSG_MESSAGECHAT_ChatType::RaidBossWhisper,
            ChatType::RaidBossEmote => CMSG_MESSAGECHAT_ChatType::RaidBossEmote,
            ChatType::Battleground => CMSG_MESSAGECHAT_ChatType::Battleground,
            ChatType::BattlegroundLeader => CMSG_MESSAGECHAT_ChatType::BattlegroundLeader,
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
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_MESSAGECHAT {}

impl CMSG_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: CMSG_MESSAGECHAT_ChatType
        + 4 // language: Language
        + self.message.len() + 1 // message: CString
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CMSG_MESSAGECHAT_ChatType {
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
    Emote,
    TextEmote,
    System,
    MonsterSay,
    MonsterYell,
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
    MonsterWhisper,
    BgSystemNeutral,
    BgSystemAlliance,
    BgSystemHorde,
    RaidLeader,
    RaidWarning,
    RaidBossWhisper,
    RaidBossEmote,
    Battleground,
    BattlegroundLeader,
}

impl Default for CMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Say
    }
}

impl CMSG_MESSAGECHAT_ChatType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Say => 0,
            Self::Party => 1,
            Self::Raid => 2,
            Self::Guild => 3,
            Self::Officer => 4,
            Self::Yell => 5,
            Self::Whisper { .. } => 6,
            Self::WhisperInform => 7,
            Self::Emote => 8,
            Self::TextEmote => 9,
            Self::System => 10,
            Self::MonsterSay => 11,
            Self::MonsterYell => 12,
            Self::MonsterEmote => 13,
            Self::Channel { .. } => 14,
            Self::ChannelJoin => 15,
            Self::ChannelLeave => 16,
            Self::ChannelList => 17,
            Self::ChannelNotice => 18,
            Self::ChannelNoticeUser => 19,
            Self::Afk => 20,
            Self::Dnd => 21,
            Self::Ignored => 22,
            Self::Skill => 23,
            Self::Loot => 24,
            Self::MonsterWhisper => 26,
            Self::BgSystemNeutral => 82,
            Self::BgSystemAlliance => 83,
            Self::BgSystemHorde => 84,
            Self::RaidLeader => 87,
            Self::RaidWarning => 88,
            Self::RaidBossWhisper => 89,
            Self::RaidBossEmote => 90,
            Self::Battleground => 92,
            Self::BattlegroundLeader => 93,
        }
    }

}

impl CMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Say => {
                4
            }
            Self::Party => {
                4
            }
            Self::Raid => {
                4
            }
            Self::Guild => {
                4
            }
            Self::Officer => {
                4
            }
            Self::Yell => {
                4
            }
            Self::Whisper {
                target_player,
            } => {
                4
                + target_player.len() + 1 // target_player: CString
            }
            Self::WhisperInform => {
                4
            }
            Self::Emote => {
                4
            }
            Self::TextEmote => {
                4
            }
            Self::System => {
                4
            }
            Self::MonsterSay => {
                4
            }
            Self::MonsterYell => {
                4
            }
            Self::MonsterEmote => {
                4
            }
            Self::Channel {
                channel,
            } => {
                4
                + channel.len() + 1 // channel: CString
            }
            Self::ChannelJoin => {
                4
            }
            Self::ChannelLeave => {
                4
            }
            Self::ChannelList => {
                4
            }
            Self::ChannelNotice => {
                4
            }
            Self::ChannelNoticeUser => {
                4
            }
            Self::Afk => {
                4
            }
            Self::Dnd => {
                4
            }
            Self::Ignored => {
                4
            }
            Self::Skill => {
                4
            }
            Self::Loot => {
                4
            }
            Self::MonsterWhisper => {
                4
            }
            Self::BgSystemNeutral => {
                4
            }
            Self::BgSystemAlliance => {
                4
            }
            Self::BgSystemHorde => {
                4
            }
            Self::RaidLeader => {
                4
            }
            Self::RaidWarning => {
                4
            }
            Self::RaidBossWhisper => {
                4
            }
            Self::RaidBossEmote => {
                4
            }
            Self::Battleground => {
                4
            }
            Self::BattlegroundLeader => {
                4
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::CMSG_MESSAGECHAT;
    use crate::world::vanilla::ChatType;
    use crate::world::vanilla::Language;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ClientOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 37] = [ 0x00, 0x23, 0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x07, 0x00, 0x00, 0x00, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
         0x20, 0x61, 0x20, 0x73, 0x61, 0x79, 0x20, 0x6D, 0x65, 0x73, 0x73, 0x61,
         0x67, 0x65, 0x2E, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_MESSAGECHAT0() {
        let expected = CMSG_MESSAGECHAT {
            chat_type: CMSG_MESSAGECHAT_ChatType::Say,
            language: Language::Common,
            message: String::from("This is a say message."),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_MESSAGECHAT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_MESSAGECHAT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.chat_type, expected.chat_type);
        assert_eq!(t.language, expected.language);
        assert_eq!(t.message, expected.message);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_MESSAGECHAT0() {
        let expected = CMSG_MESSAGECHAT {
            chat_type: CMSG_MESSAGECHAT_ChatType::Say,
            language: Language::Common,
            message: String::from("This is a say message."),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_MESSAGECHAT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_MESSAGECHAT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.chat_type, expected.chat_type);
        assert_eq!(t.language, expected.language);
        assert_eq!(t.message, expected.message);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_MESSAGECHAT0() {
        let expected = CMSG_MESSAGECHAT {
            chat_type: CMSG_MESSAGECHAT_ChatType::Say,
            language: Language::Common,
            message: String::from("This is a say message."),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_MESSAGECHAT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_MESSAGECHAT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.chat_type, expected.chat_type);
        assert_eq!(t.language, expected.language);
        assert_eq!(t.message, expected.message);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

