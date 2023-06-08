use std::io::{Read, Write};

use crate::tbc::{
    ChatType, Language,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm#L30):
/// ```text
/// cmsg CMSG_MESSAGECHAT = 0x0095 {
///     (u32)ChatType chat_type;
///     (u32)Language language;
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

impl crate::private::Sealed for CMSG_MESSAGECHAT {}
impl crate::Message for CMSG_MESSAGECHAT {
    const OPCODE: u32 = 0x0095;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_MESSAGECHAT {{").unwrap();
        // Members
        writeln!(s, "    chat_type = {};", crate::tbc::ChatType::try_from(self.chat_type.as_int()as u8).unwrap().as_test_case_value()).unwrap();
        writeln!(s, "    language = {};", self.language.as_test_case_value()).unwrap();
        match &self.chat_type {
            crate::tbc::CMSG_MESSAGECHAT_ChatType::Whisper {
                target_player,
            } => {
                writeln!(s, "    target_player = \"{}\";", target_player).unwrap();
            }
            crate::tbc::CMSG_MESSAGECHAT_ChatType::Channel {
                channel,
            } => {
                writeln!(s, "    channel = \"{}\";", channel).unwrap();
            }
            _ => {}
        }

        writeln!(s, "    message = \"{}\";", self.message).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 149_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "chat_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "language", "    ");
        match &self.chat_type {
            crate::tbc::CMSG_MESSAGECHAT_ChatType::Whisper {
                target_player,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, target_player.len() + 1, "target_player", "    ");
            }
            crate::tbc::CMSG_MESSAGECHAT_ChatType::Channel {
                channel,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, channel.len() + 1, "channel", "    ");
            }
            _ => {}
        }

        crate::util::write_bytes(&mut s, &mut bytes, self.message.len() + 1, "message", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int().to_le_bytes()))?;

        // language: Language
        w.write_all(&u32::from(self.language.as_int()).to_le_bytes())?;

        match &self.chat_type {
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
            _ => {}
        }

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=520).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0095, size: body_size });
        }

        // chat_type: ChatType
        let chat_type = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // language: Language
        let language = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        let chat_type_if = match chat_type {
            ChatType::System => CMSG_MESSAGECHAT_ChatType::System,
            ChatType::Say => CMSG_MESSAGECHAT_ChatType::Say,
            ChatType::Party => CMSG_MESSAGECHAT_ChatType::Party,
            ChatType::Raid => CMSG_MESSAGECHAT_ChatType::Raid,
            ChatType::Guild => CMSG_MESSAGECHAT_ChatType::Guild,
            ChatType::Officer => CMSG_MESSAGECHAT_ChatType::Officer,
            ChatType::Yell => CMSG_MESSAGECHAT_ChatType::Yell,
            ChatType::Whisper => {
                // target_player: CString
                let target_player = {
                    let target_player = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(target_player)?
                };

                CMSG_MESSAGECHAT_ChatType::Whisper {
                    target_player,
                }
            }
            ChatType::WhisperInform => CMSG_MESSAGECHAT_ChatType::WhisperInform,
            ChatType::Reply => CMSG_MESSAGECHAT_ChatType::Reply,
            ChatType::Emote => CMSG_MESSAGECHAT_ChatType::Emote,
            ChatType::TextEmote => CMSG_MESSAGECHAT_ChatType::TextEmote,
            ChatType::MonsterSay => CMSG_MESSAGECHAT_ChatType::MonsterSay,
            ChatType::MonsterParty => CMSG_MESSAGECHAT_ChatType::MonsterParty,
            ChatType::MonsterYell => CMSG_MESSAGECHAT_ChatType::MonsterYell,
            ChatType::MonsterWhisper => CMSG_MESSAGECHAT_ChatType::MonsterWhisper,
            ChatType::MonsterEmote => CMSG_MESSAGECHAT_ChatType::MonsterEmote,
            ChatType::Channel => {
                // channel: CString
                let channel = {
                    let channel = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(channel)?
                };

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
            ChatType::Money => CMSG_MESSAGECHAT_ChatType::Money,
            ChatType::Opening => CMSG_MESSAGECHAT_ChatType::Opening,
            ChatType::Tradeskills => CMSG_MESSAGECHAT_ChatType::Tradeskills,
            ChatType::PetInfo => CMSG_MESSAGECHAT_ChatType::PetInfo,
            ChatType::CombatMiscInfo => CMSG_MESSAGECHAT_ChatType::CombatMiscInfo,
            ChatType::CombatXpGain => CMSG_MESSAGECHAT_ChatType::CombatXpGain,
            ChatType::CombatHonorGain => CMSG_MESSAGECHAT_ChatType::CombatHonorGain,
            ChatType::CombatFactionChange => CMSG_MESSAGECHAT_ChatType::CombatFactionChange,
            ChatType::BgSystemNeutral => CMSG_MESSAGECHAT_ChatType::BgSystemNeutral,
            ChatType::BgSystemAlliance => CMSG_MESSAGECHAT_ChatType::BgSystemAlliance,
            ChatType::BgSystemHorde => CMSG_MESSAGECHAT_ChatType::BgSystemHorde,
            ChatType::RaidLeader => CMSG_MESSAGECHAT_ChatType::RaidLeader,
            ChatType::RaidWarning => CMSG_MESSAGECHAT_ChatType::RaidWarning,
            ChatType::RaidBossWhisper => CMSG_MESSAGECHAT_ChatType::RaidBossWhisper,
            ChatType::RaidBossEmote => CMSG_MESSAGECHAT_ChatType::RaidBossEmote,
            ChatType::Filtered => CMSG_MESSAGECHAT_ChatType::Filtered,
            ChatType::Battleground => CMSG_MESSAGECHAT_ChatType::Battleground,
            ChatType::BattlegroundLeader => CMSG_MESSAGECHAT_ChatType::BattlegroundLeader,
            ChatType::Restricted => CMSG_MESSAGECHAT_ChatType::Restricted,
        };

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        Ok(Self {
            chat_type: chat_type_if,
            language,
            message,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_MESSAGECHAT {}

impl CMSG_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: CMSG_MESSAGECHAT_ChatType
        + 4 // language: Language
        + self.message.len() + 1 // message: CString
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMSG_MESSAGECHAT_ChatType {
    System,
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
    Reply,
    Emote,
    TextEmote,
    MonsterSay,
    MonsterParty,
    MonsterYell,
    MonsterWhisper,
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
    Money,
    Opening,
    Tradeskills,
    PetInfo,
    CombatMiscInfo,
    CombatXpGain,
    CombatHonorGain,
    CombatFactionChange,
    BgSystemNeutral,
    BgSystemAlliance,
    BgSystemHorde,
    RaidLeader,
    RaidWarning,
    RaidBossWhisper,
    RaidBossEmote,
    Filtered,
    Battleground,
    BattlegroundLeader,
    Restricted,
}

impl Default for CMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::System
    }
}

impl CMSG_MESSAGECHAT_ChatType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::System => 0,
            Self::Say => 1,
            Self::Party => 2,
            Self::Raid => 3,
            Self::Guild => 4,
            Self::Officer => 5,
            Self::Yell => 6,
            Self::Whisper { .. } => 7,
            Self::WhisperInform => 8,
            Self::Reply => 9,
            Self::Emote => 10,
            Self::TextEmote => 11,
            Self::MonsterSay => 12,
            Self::MonsterParty => 13,
            Self::MonsterYell => 14,
            Self::MonsterWhisper => 15,
            Self::MonsterEmote => 16,
            Self::Channel { .. } => 17,
            Self::ChannelJoin => 18,
            Self::ChannelLeave => 19,
            Self::ChannelList => 20,
            Self::ChannelNotice => 21,
            Self::ChannelNoticeUser => 22,
            Self::Afk => 23,
            Self::Dnd => 24,
            Self::Ignored => 25,
            Self::Skill => 26,
            Self::Loot => 27,
            Self::Money => 28,
            Self::Opening => 29,
            Self::Tradeskills => 30,
            Self::PetInfo => 31,
            Self::CombatMiscInfo => 32,
            Self::CombatXpGain => 33,
            Self::CombatHonorGain => 34,
            Self::CombatFactionChange => 35,
            Self::BgSystemNeutral => 36,
            Self::BgSystemAlliance => 37,
            Self::BgSystemHorde => 38,
            Self::RaidLeader => 39,
            Self::RaidWarning => 40,
            Self::RaidBossWhisper => 41,
            Self::RaidBossEmote => 42,
            Self::Filtered => 43,
            Self::Battleground => 44,
            Self::BattlegroundLeader => 45,
            Self::Restricted => 46,
        }
    }

}

impl std::fmt::Display for CMSG_MESSAGECHAT_ChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::System => f.write_str("System"),
            Self::Say => f.write_str("Say"),
            Self::Party => f.write_str("Party"),
            Self::Raid => f.write_str("Raid"),
            Self::Guild => f.write_str("Guild"),
            Self::Officer => f.write_str("Officer"),
            Self::Yell => f.write_str("Yell"),
            Self::Whisper{ .. } => f.write_str("Whisper"),
            Self::WhisperInform => f.write_str("WhisperInform"),
            Self::Reply => f.write_str("Reply"),
            Self::Emote => f.write_str("Emote"),
            Self::TextEmote => f.write_str("TextEmote"),
            Self::MonsterSay => f.write_str("MonsterSay"),
            Self::MonsterParty => f.write_str("MonsterParty"),
            Self::MonsterYell => f.write_str("MonsterYell"),
            Self::MonsterWhisper => f.write_str("MonsterWhisper"),
            Self::MonsterEmote => f.write_str("MonsterEmote"),
            Self::Channel{ .. } => f.write_str("Channel"),
            Self::ChannelJoin => f.write_str("ChannelJoin"),
            Self::ChannelLeave => f.write_str("ChannelLeave"),
            Self::ChannelList => f.write_str("ChannelList"),
            Self::ChannelNotice => f.write_str("ChannelNotice"),
            Self::ChannelNoticeUser => f.write_str("ChannelNoticeUser"),
            Self::Afk => f.write_str("Afk"),
            Self::Dnd => f.write_str("Dnd"),
            Self::Ignored => f.write_str("Ignored"),
            Self::Skill => f.write_str("Skill"),
            Self::Loot => f.write_str("Loot"),
            Self::Money => f.write_str("Money"),
            Self::Opening => f.write_str("Opening"),
            Self::Tradeskills => f.write_str("Tradeskills"),
            Self::PetInfo => f.write_str("PetInfo"),
            Self::CombatMiscInfo => f.write_str("CombatMiscInfo"),
            Self::CombatXpGain => f.write_str("CombatXpGain"),
            Self::CombatHonorGain => f.write_str("CombatHonorGain"),
            Self::CombatFactionChange => f.write_str("CombatFactionChange"),
            Self::BgSystemNeutral => f.write_str("BgSystemNeutral"),
            Self::BgSystemAlliance => f.write_str("BgSystemAlliance"),
            Self::BgSystemHorde => f.write_str("BgSystemHorde"),
            Self::RaidLeader => f.write_str("RaidLeader"),
            Self::RaidWarning => f.write_str("RaidWarning"),
            Self::RaidBossWhisper => f.write_str("RaidBossWhisper"),
            Self::RaidBossEmote => f.write_str("RaidBossEmote"),
            Self::Filtered => f.write_str("Filtered"),
            Self::Battleground => f.write_str("Battleground"),
            Self::BattlegroundLeader => f.write_str("BattlegroundLeader"),
            Self::Restricted => f.write_str("Restricted"),
        }
    }
}

impl CMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Whisper {
                target_player,
            } => {
                4
                + target_player.len() + 1 // target_player: CString
            }
            Self::Channel {
                channel,
            } => {
                4
                + channel.len() + 1 // channel: CString
            }
            _ => 4,
        }
    }
}

