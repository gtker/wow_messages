/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common_3_3_5.wowm:181`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common_3_3_5.wowm#L181):
/// ```text
/// enum ChatType : u8 {
///     SYSTEM = 0x00;
///     SAY = 0x01;
///     PARTY = 0x02;
///     RAID = 0x03;
///     GUILD = 0x04;
///     OFFICER = 0x05;
///     YELL = 0x06;
///     WHISPER = 0x07;
///     WHISPER_FOREIGN = 0x08;
///     WHISPER_INFORM = 0x09;
///     EMOTE = 0x0A;
///     TEXT_EMOTE = 0x0B;
///     MONSTER_SAY = 0x0C;
///     MONSTER_PARTY = 0x0D;
///     MONSTER_YELL = 0x0E;
///     MONSTER_WHISPER = 0x0F;
///     MONSTER_EMOTE = 0x10;
///     CHANNEL = 0x11;
///     CHANNEL_JOIN = 0x12;
///     CHANNEL_LEAVE = 0x13;
///     CHANNEL_LIST = 0x14;
///     CHANNEL_NOTICE = 0x15;
///     CHANNEL_NOTICE_USER = 0x16;
///     AFK = 0x17;
///     DND = 0x18;
///     IGNORED = 0x19;
///     SKILL = 0x1A;
///     LOOT = 0x1B;
///     MONEY = 0x1C;
///     OPENING = 0x1D;
///     TRADESKILLS = 0x1E;
///     PET_INFO = 0x1F;
///     COMBAT_MISC_INFO = 0x20;
///     COMBAT_XP_GAIN = 0x21;
///     COMBAT_HONOR_GAIN = 0x22;
///     COMBAT_FACTION_CHANGE = 0x23;
///     BG_SYSTEM_NEUTRAL = 0x24;
///     BG_SYSTEM_ALLIANCE = 0x25;
///     BG_SYSTEM_HORDE = 0x26;
///     RAID_LEADER = 0x27;
///     RAID_WARNING = 0x28;
///     RAID_BOSS_EMOTE = 0x29;
///     RAID_BOSS_WHISPER = 0x2A;
///     FILTERED = 0x2B;
///     BATTLEGROUND = 0x2C;
///     BATTLEGROUND_LEADER = 0x2D;
///     RESTRICTED = 0x2E;
///     BATTLENET = 0x2F;
///     ACHIEVEMENT = 0x30;
///     GUILD_ACHIEVEMENT = 0x31;
///     ARENA_POINTS = 0x32;
///     PARTY_LEADER = 0x33;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ChatType {
    System,
    Say,
    Party,
    Raid,
    Guild,
    Officer,
    Yell,
    Whisper,
    WhisperForeign,
    WhisperInform,
    Emote,
    TextEmote,
    MonsterSay,
    MonsterParty,
    MonsterYell,
    MonsterWhisper,
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
    RaidBossEmote,
    RaidBossWhisper,
    Filtered,
    Battleground,
    BattlegroundLeader,
    Restricted,
    Battlenet,
    Achievement,
    GuildAchievement,
    ArenaPoints,
    PartyLeader,
}

impl ChatType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::System => 0x0,
            Self::Say => 0x1,
            Self::Party => 0x2,
            Self::Raid => 0x3,
            Self::Guild => 0x4,
            Self::Officer => 0x5,
            Self::Yell => 0x6,
            Self::Whisper => 0x7,
            Self::WhisperForeign => 0x8,
            Self::WhisperInform => 0x9,
            Self::Emote => 0xa,
            Self::TextEmote => 0xb,
            Self::MonsterSay => 0xc,
            Self::MonsterParty => 0xd,
            Self::MonsterYell => 0xe,
            Self::MonsterWhisper => 0xf,
            Self::MonsterEmote => 0x10,
            Self::Channel => 0x11,
            Self::ChannelJoin => 0x12,
            Self::ChannelLeave => 0x13,
            Self::ChannelList => 0x14,
            Self::ChannelNotice => 0x15,
            Self::ChannelNoticeUser => 0x16,
            Self::Afk => 0x17,
            Self::Dnd => 0x18,
            Self::Ignored => 0x19,
            Self::Skill => 0x1a,
            Self::Loot => 0x1b,
            Self::Money => 0x1c,
            Self::Opening => 0x1d,
            Self::Tradeskills => 0x1e,
            Self::PetInfo => 0x1f,
            Self::CombatMiscInfo => 0x20,
            Self::CombatXpGain => 0x21,
            Self::CombatHonorGain => 0x22,
            Self::CombatFactionChange => 0x23,
            Self::BgSystemNeutral => 0x24,
            Self::BgSystemAlliance => 0x25,
            Self::BgSystemHorde => 0x26,
            Self::RaidLeader => 0x27,
            Self::RaidWarning => 0x28,
            Self::RaidBossEmote => 0x29,
            Self::RaidBossWhisper => 0x2a,
            Self::Filtered => 0x2b,
            Self::Battleground => 0x2c,
            Self::BattlegroundLeader => 0x2d,
            Self::Restricted => 0x2e,
            Self::Battlenet => 0x2f,
            Self::Achievement => 0x30,
            Self::GuildAchievement => 0x31,
            Self::ArenaPoints => 0x32,
            Self::PartyLeader => 0x33,
        }
    }

}

impl Default for ChatType {
    fn default() -> Self {
        Self::System
    }
}

impl std::fmt::Display for ChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::System => f.write_str("System"),
            Self::Say => f.write_str("Say"),
            Self::Party => f.write_str("Party"),
            Self::Raid => f.write_str("Raid"),
            Self::Guild => f.write_str("Guild"),
            Self::Officer => f.write_str("Officer"),
            Self::Yell => f.write_str("Yell"),
            Self::Whisper => f.write_str("Whisper"),
            Self::WhisperForeign => f.write_str("WhisperForeign"),
            Self::WhisperInform => f.write_str("WhisperInform"),
            Self::Emote => f.write_str("Emote"),
            Self::TextEmote => f.write_str("TextEmote"),
            Self::MonsterSay => f.write_str("MonsterSay"),
            Self::MonsterParty => f.write_str("MonsterParty"),
            Self::MonsterYell => f.write_str("MonsterYell"),
            Self::MonsterWhisper => f.write_str("MonsterWhisper"),
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
            Self::RaidBossEmote => f.write_str("RaidBossEmote"),
            Self::RaidBossWhisper => f.write_str("RaidBossWhisper"),
            Self::Filtered => f.write_str("Filtered"),
            Self::Battleground => f.write_str("Battleground"),
            Self::BattlegroundLeader => f.write_str("BattlegroundLeader"),
            Self::Restricted => f.write_str("Restricted"),
            Self::Battlenet => f.write_str("Battlenet"),
            Self::Achievement => f.write_str("Achievement"),
            Self::GuildAchievement => f.write_str("GuildAchievement"),
            Self::ArenaPoints => f.write_str("ArenaPoints"),
            Self::PartyLeader => f.write_str("PartyLeader"),
        }
    }
}

impl TryFrom<u8> for ChatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::System),
            1 => Ok(Self::Say),
            2 => Ok(Self::Party),
            3 => Ok(Self::Raid),
            4 => Ok(Self::Guild),
            5 => Ok(Self::Officer),
            6 => Ok(Self::Yell),
            7 => Ok(Self::Whisper),
            8 => Ok(Self::WhisperForeign),
            9 => Ok(Self::WhisperInform),
            10 => Ok(Self::Emote),
            11 => Ok(Self::TextEmote),
            12 => Ok(Self::MonsterSay),
            13 => Ok(Self::MonsterParty),
            14 => Ok(Self::MonsterYell),
            15 => Ok(Self::MonsterWhisper),
            16 => Ok(Self::MonsterEmote),
            17 => Ok(Self::Channel),
            18 => Ok(Self::ChannelJoin),
            19 => Ok(Self::ChannelLeave),
            20 => Ok(Self::ChannelList),
            21 => Ok(Self::ChannelNotice),
            22 => Ok(Self::ChannelNoticeUser),
            23 => Ok(Self::Afk),
            24 => Ok(Self::Dnd),
            25 => Ok(Self::Ignored),
            26 => Ok(Self::Skill),
            27 => Ok(Self::Loot),
            28 => Ok(Self::Money),
            29 => Ok(Self::Opening),
            30 => Ok(Self::Tradeskills),
            31 => Ok(Self::PetInfo),
            32 => Ok(Self::CombatMiscInfo),
            33 => Ok(Self::CombatXpGain),
            34 => Ok(Self::CombatHonorGain),
            35 => Ok(Self::CombatFactionChange),
            36 => Ok(Self::BgSystemNeutral),
            37 => Ok(Self::BgSystemAlliance),
            38 => Ok(Self::BgSystemHorde),
            39 => Ok(Self::RaidLeader),
            40 => Ok(Self::RaidWarning),
            41 => Ok(Self::RaidBossEmote),
            42 => Ok(Self::RaidBossWhisper),
            43 => Ok(Self::Filtered),
            44 => Ok(Self::Battleground),
            45 => Ok(Self::BattlegroundLeader),
            46 => Ok(Self::Restricted),
            47 => Ok(Self::Battlenet),
            48 => Ok(Self::Achievement),
            49 => Ok(Self::GuildAchievement),
            50 => Ok(Self::ArenaPoints),
            51 => Ok(Self::PartyLeader),
            v => Err(crate::errors::EnumError::new("ChatType", v as u64),)
        }
    }
}

