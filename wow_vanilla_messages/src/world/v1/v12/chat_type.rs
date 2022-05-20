use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum ChatType {
    SAY,
    PARTY,
    RAID,
    GUILD,
    OFFICER,
    YELL,
    WHISPER,
    WHISPER_INFORM,
    EMOTE,
    TEXT_EMOTE,
    SYSTEM,
    MONSTER_SAY,
    MONSTER_YELL,
    MONSTER_EMOTE,
    CHANNEL,
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

impl ChatType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SAY => 0x0,
            Self::PARTY => 0x1,
            Self::RAID => 0x2,
            Self::GUILD => 0x3,
            Self::OFFICER => 0x4,
            Self::YELL => 0x5,
            Self::WHISPER => 0x6,
            Self::WHISPER_INFORM => 0x7,
            Self::EMOTE => 0x8,
            Self::TEXT_EMOTE => 0x9,
            Self::SYSTEM => 0xa,
            Self::MONSTER_SAY => 0xb,
            Self::MONSTER_YELL => 0xc,
            Self::MONSTER_EMOTE => 0xd,
            Self::CHANNEL => 0xe,
            Self::CHANNEL_JOIN => 0xf,
            Self::CHANNEL_LEAVE => 0x10,
            Self::CHANNEL_LIST => 0x11,
            Self::CHANNEL_NOTICE => 0x12,
            Self::CHANNEL_NOTICE_USER => 0x13,
            Self::AFK => 0x14,
            Self::DND => 0x15,
            Self::IGNORED => 0x16,
            Self::SKILL => 0x17,
            Self::LOOT => 0x18,
            Self::MONSTER_WHISPER => 0x1a,
            Self::BG_SYSTEM_NEUTRAL => 0x52,
            Self::BG_SYSTEM_ALLIANCE => 0x53,
            Self::BG_SYSTEM_HORDE => 0x54,
            Self::RAID_LEADER => 0x57,
            Self::RAID_WARNING => 0x58,
            Self::RAID_BOSS_WHISPER => 0x59,
            Self::RAID_BOSS_EMOTE => 0x5a,
            Self::BATTLEGROUND => 0x5c,
            Self::BATTLEGROUND_LEADER => 0x5d,
        }
    }

}

impl Default for ChatType {
    fn default() -> Self {
        Self::SAY
    }
}

impl std::fmt::Display for ChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SAY => f.write_str("SAY"),
            Self::PARTY => f.write_str("PARTY"),
            Self::RAID => f.write_str("RAID"),
            Self::GUILD => f.write_str("GUILD"),
            Self::OFFICER => f.write_str("OFFICER"),
            Self::YELL => f.write_str("YELL"),
            Self::WHISPER => f.write_str("WHISPER"),
            Self::WHISPER_INFORM => f.write_str("WHISPER_INFORM"),
            Self::EMOTE => f.write_str("EMOTE"),
            Self::TEXT_EMOTE => f.write_str("TEXT_EMOTE"),
            Self::SYSTEM => f.write_str("SYSTEM"),
            Self::MONSTER_SAY => f.write_str("MONSTER_SAY"),
            Self::MONSTER_YELL => f.write_str("MONSTER_YELL"),
            Self::MONSTER_EMOTE => f.write_str("MONSTER_EMOTE"),
            Self::CHANNEL => f.write_str("CHANNEL"),
            Self::CHANNEL_JOIN => f.write_str("CHANNEL_JOIN"),
            Self::CHANNEL_LEAVE => f.write_str("CHANNEL_LEAVE"),
            Self::CHANNEL_LIST => f.write_str("CHANNEL_LIST"),
            Self::CHANNEL_NOTICE => f.write_str("CHANNEL_NOTICE"),
            Self::CHANNEL_NOTICE_USER => f.write_str("CHANNEL_NOTICE_USER"),
            Self::AFK => f.write_str("AFK"),
            Self::DND => f.write_str("DND"),
            Self::IGNORED => f.write_str("IGNORED"),
            Self::SKILL => f.write_str("SKILL"),
            Self::LOOT => f.write_str("LOOT"),
            Self::MONSTER_WHISPER => f.write_str("MONSTER_WHISPER"),
            Self::BG_SYSTEM_NEUTRAL => f.write_str("BG_SYSTEM_NEUTRAL"),
            Self::BG_SYSTEM_ALLIANCE => f.write_str("BG_SYSTEM_ALLIANCE"),
            Self::BG_SYSTEM_HORDE => f.write_str("BG_SYSTEM_HORDE"),
            Self::RAID_LEADER => f.write_str("RAID_LEADER"),
            Self::RAID_WARNING => f.write_str("RAID_WARNING"),
            Self::RAID_BOSS_WHISPER => f.write_str("RAID_BOSS_WHISPER"),
            Self::RAID_BOSS_EMOTE => f.write_str("RAID_BOSS_EMOTE"),
            Self::BATTLEGROUND => f.write_str("BATTLEGROUND"),
            Self::BATTLEGROUND_LEADER => f.write_str("BATTLEGROUND_LEADER"),
        }
    }
}

impl TryFrom<u8> for ChatType {
    type Error = ChatTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SAY),
            1 => Ok(Self::PARTY),
            2 => Ok(Self::RAID),
            3 => Ok(Self::GUILD),
            4 => Ok(Self::OFFICER),
            5 => Ok(Self::YELL),
            6 => Ok(Self::WHISPER),
            7 => Ok(Self::WHISPER_INFORM),
            8 => Ok(Self::EMOTE),
            9 => Ok(Self::TEXT_EMOTE),
            10 => Ok(Self::SYSTEM),
            11 => Ok(Self::MONSTER_SAY),
            12 => Ok(Self::MONSTER_YELL),
            13 => Ok(Self::MONSTER_EMOTE),
            14 => Ok(Self::CHANNEL),
            15 => Ok(Self::CHANNEL_JOIN),
            16 => Ok(Self::CHANNEL_LEAVE),
            17 => Ok(Self::CHANNEL_LIST),
            18 => Ok(Self::CHANNEL_NOTICE),
            19 => Ok(Self::CHANNEL_NOTICE_USER),
            20 => Ok(Self::AFK),
            21 => Ok(Self::DND),
            22 => Ok(Self::IGNORED),
            23 => Ok(Self::SKILL),
            24 => Ok(Self::LOOT),
            26 => Ok(Self::MONSTER_WHISPER),
            82 => Ok(Self::BG_SYSTEM_NEUTRAL),
            83 => Ok(Self::BG_SYSTEM_ALLIANCE),
            84 => Ok(Self::BG_SYSTEM_HORDE),
            87 => Ok(Self::RAID_LEADER),
            88 => Ok(Self::RAID_WARNING),
            89 => Ok(Self::RAID_BOSS_WHISPER),
            90 => Ok(Self::RAID_BOSS_EMOTE),
            92 => Ok(Self::BATTLEGROUND),
            93 => Ok(Self::BATTLEGROUND_LEADER),
            _ => Err(ChatTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct ChatTypeError {
    pub value: u8,
}

impl ChatTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for ChatTypeError {}
impl std::fmt::Display for ChatTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ChatType': '{}'", self.value))
    }
}

