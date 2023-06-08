/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L26):
/// ```text
/// enum ChatType : u8 {
///     SAY = 0x00;
///     PARTY = 0x01;
///     RAID = 0x02;
///     GUILD = 0x03;
///     OFFICER = 0x04;
///     YELL = 0x05;
///     WHISPER = 0x06;
///     WHISPER_INFORM = 0x07;
///     EMOTE = 0x08;
///     TEXT_EMOTE = 0x09;
///     SYSTEM = 0x0A;
///     MONSTER_SAY = 0x0B;
///     MONSTER_YELL = 0x0C;
///     MONSTER_EMOTE = 0x0D;
///     CHANNEL = 0x0E;
///     CHANNEL_JOIN = 0x0F;
///     CHANNEL_LEAVE = 0x10;
///     CHANNEL_LIST = 0x11;
///     CHANNEL_NOTICE = 0x12;
///     CHANNEL_NOTICE_USER = 0x13;
///     AFK = 0x14;
///     DND = 0x15;
///     IGNORED = 0x16;
///     SKILL = 0x17;
///     LOOT = 0x18;
///     MONSTER_WHISPER = 0x1A;
///     BG_SYSTEM_NEUTRAL = 0x52;
///     BG_SYSTEM_ALLIANCE = 0x53;
///     BG_SYSTEM_HORDE = 0x54;
///     RAID_LEADER = 0x57;
///     RAID_WARNING = 0x58;
///     RAID_BOSS_WHISPER = 0x59;
///     RAID_BOSS_EMOTE = 0x5A;
///     BATTLEGROUND = 0x5C;
///     BATTLEGROUND_LEADER = 0x5D;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ChatType {
    Say,
    Party,
    Raid,
    Guild,
    Officer,
    Yell,
    Whisper,
    WhisperInform,
    Emote,
    TextEmote,
    System,
    MonsterSay,
    MonsterYell,
    MonsterEmote,
    Channel,
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

impl ChatType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Say => 0x0,
            Self::Party => 0x1,
            Self::Raid => 0x2,
            Self::Guild => 0x3,
            Self::Officer => 0x4,
            Self::Yell => 0x5,
            Self::Whisper => 0x6,
            Self::WhisperInform => 0x7,
            Self::Emote => 0x8,
            Self::TextEmote => 0x9,
            Self::System => 0xa,
            Self::MonsterSay => 0xb,
            Self::MonsterYell => 0xc,
            Self::MonsterEmote => 0xd,
            Self::Channel => 0xe,
            Self::ChannelJoin => 0xf,
            Self::ChannelLeave => 0x10,
            Self::ChannelList => 0x11,
            Self::ChannelNotice => 0x12,
            Self::ChannelNoticeUser => 0x13,
            Self::Afk => 0x14,
            Self::Dnd => 0x15,
            Self::Ignored => 0x16,
            Self::Skill => 0x17,
            Self::Loot => 0x18,
            Self::MonsterWhisper => 0x1a,
            Self::BgSystemNeutral => 0x52,
            Self::BgSystemAlliance => 0x53,
            Self::BgSystemHorde => 0x54,
            Self::RaidLeader => 0x57,
            Self::RaidWarning => 0x58,
            Self::RaidBossWhisper => 0x59,
            Self::RaidBossEmote => 0x5a,
            Self::Battleground => 0x5c,
            Self::BattlegroundLeader => 0x5d,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl ChatType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Say => "SAY",
            Self::Party => "PARTY",
            Self::Raid => "RAID",
            Self::Guild => "GUILD",
            Self::Officer => "OFFICER",
            Self::Yell => "YELL",
            Self::Whisper => "WHISPER",
            Self::WhisperInform => "WHISPER_INFORM",
            Self::Emote => "EMOTE",
            Self::TextEmote => "TEXT_EMOTE",
            Self::System => "SYSTEM",
            Self::MonsterSay => "MONSTER_SAY",
            Self::MonsterYell => "MONSTER_YELL",
            Self::MonsterEmote => "MONSTER_EMOTE",
            Self::Channel => "CHANNEL",
            Self::ChannelJoin => "CHANNEL_JOIN",
            Self::ChannelLeave => "CHANNEL_LEAVE",
            Self::ChannelList => "CHANNEL_LIST",
            Self::ChannelNotice => "CHANNEL_NOTICE",
            Self::ChannelNoticeUser => "CHANNEL_NOTICE_USER",
            Self::Afk => "AFK",
            Self::Dnd => "DND",
            Self::Ignored => "IGNORED",
            Self::Skill => "SKILL",
            Self::Loot => "LOOT",
            Self::MonsterWhisper => "MONSTER_WHISPER",
            Self::BgSystemNeutral => "BG_SYSTEM_NEUTRAL",
            Self::BgSystemAlliance => "BG_SYSTEM_ALLIANCE",
            Self::BgSystemHorde => "BG_SYSTEM_HORDE",
            Self::RaidLeader => "RAID_LEADER",
            Self::RaidWarning => "RAID_WARNING",
            Self::RaidBossWhisper => "RAID_BOSS_WHISPER",
            Self::RaidBossEmote => "RAID_BOSS_EMOTE",
            Self::Battleground => "BATTLEGROUND",
            Self::BattlegroundLeader => "BATTLEGROUND_LEADER",
        }
    }

}

impl Default for ChatType {
    fn default() -> Self {
        Self::Say
    }
}

impl std::fmt::Display for ChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Say => f.write_str("Say"),
            Self::Party => f.write_str("Party"),
            Self::Raid => f.write_str("Raid"),
            Self::Guild => f.write_str("Guild"),
            Self::Officer => f.write_str("Officer"),
            Self::Yell => f.write_str("Yell"),
            Self::Whisper => f.write_str("Whisper"),
            Self::WhisperInform => f.write_str("WhisperInform"),
            Self::Emote => f.write_str("Emote"),
            Self::TextEmote => f.write_str("TextEmote"),
            Self::System => f.write_str("System"),
            Self::MonsterSay => f.write_str("MonsterSay"),
            Self::MonsterYell => f.write_str("MonsterYell"),
            Self::MonsterEmote => f.write_str("MonsterEmote"),
            Self::Channel => f.write_str("Channel"),
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
            Self::MonsterWhisper => f.write_str("MonsterWhisper"),
            Self::BgSystemNeutral => f.write_str("BgSystemNeutral"),
            Self::BgSystemAlliance => f.write_str("BgSystemAlliance"),
            Self::BgSystemHorde => f.write_str("BgSystemHorde"),
            Self::RaidLeader => f.write_str("RaidLeader"),
            Self::RaidWarning => f.write_str("RaidWarning"),
            Self::RaidBossWhisper => f.write_str("RaidBossWhisper"),
            Self::RaidBossEmote => f.write_str("RaidBossEmote"),
            Self::Battleground => f.write_str("Battleground"),
            Self::BattlegroundLeader => f.write_str("BattlegroundLeader"),
        }
    }
}

impl TryFrom<u8> for ChatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Say),
            1 => Ok(Self::Party),
            2 => Ok(Self::Raid),
            3 => Ok(Self::Guild),
            4 => Ok(Self::Officer),
            5 => Ok(Self::Yell),
            6 => Ok(Self::Whisper),
            7 => Ok(Self::WhisperInform),
            8 => Ok(Self::Emote),
            9 => Ok(Self::TextEmote),
            10 => Ok(Self::System),
            11 => Ok(Self::MonsterSay),
            12 => Ok(Self::MonsterYell),
            13 => Ok(Self::MonsterEmote),
            14 => Ok(Self::Channel),
            15 => Ok(Self::ChannelJoin),
            16 => Ok(Self::ChannelLeave),
            17 => Ok(Self::ChannelList),
            18 => Ok(Self::ChannelNotice),
            19 => Ok(Self::ChannelNoticeUser),
            20 => Ok(Self::Afk),
            21 => Ok(Self::Dnd),
            22 => Ok(Self::Ignored),
            23 => Ok(Self::Skill),
            24 => Ok(Self::Loot),
            26 => Ok(Self::MonsterWhisper),
            82 => Ok(Self::BgSystemNeutral),
            83 => Ok(Self::BgSystemAlliance),
            84 => Ok(Self::BgSystemHorde),
            87 => Ok(Self::RaidLeader),
            88 => Ok(Self::RaidWarning),
            89 => Ok(Self::RaidBossWhisper),
            90 => Ok(Self::RaidBossEmote),
            92 => Ok(Self::Battleground),
            93 => Ok(Self::BattlegroundLeader),
            v => Err(crate::errors::EnumError::new("ChatType", v as u64),)
        }
    }
}

