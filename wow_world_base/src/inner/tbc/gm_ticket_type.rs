/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm#L31):
/// ```text
/// enum GmTicketType : u8 {
///     NOT_SET = 0;
///     STUCK = 1;
///     BEHAVIOR_HARASSMENT = 2;
///     GUILD = 3;
///     ITEM = 4;
///     ENVIRONMENTAL = 5;
///     NON_QUEST_CREEP = 6;
///     QUEST_QUEST_NPC = 7;
///     TECHNICAL = 8;
///     ACCOUNT_BILLING = 9;
///     CHARACTER = 10;
///     ARENA_HONOR_ITEM_ISSUES = 11;
///     ARENA_HONOR_POINTS_ISSUES = 12;
///     BOTTING_CHEATING_HACKING = 13;
///     BUG_REPORT = 14;
///     COMPROMISED_ACCOUNT_ISSUE = 15;
///     GAME_SUGGESTIONS = 16;
///     GAMEPLAY_QUESTION = 17;
///     GUILD_BANK_ISSUE = 18;
///     GUILD_MASTER_ISSUE = 19;
///     HARASSMENT_SCAM_REPORT = 20;
///     INAPPROPRIATE_NAME_GUILD_ARENA_CHARACTER_PET = 21;
///     KNOWN_ISSUE_FIX = 22;
///     LATENCY_LAG_REPORT = 23;
///     LOOTING_ISSUE_MISTAKE = 24;
///     MAIL_ISSUE = 25;
///     NON_IN_GAME_RELATED_INQUIRY = 26;
///     PARENTAL_CONTROLS_CAIS = 27;
///     PCNC = 28;
///     PCT = 29;
///     RESTORATION_STATUS_FOLLOW_UP = 30;
///     SERVER_INSTANCE_ISSUES = 31;
///     SPAM = 32;
///     SUICIDE_CASE = 33;
///     SUSPENSION_QUESTIONS = 34;
///     TECHNICAL_SOUND_GRAPHICS_ISSUE = 35;
///     UI_ISSUE = 36;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GmTicketType {
    NotSet,
    Stuck,
    BehaviorHarassment,
    Guild,
    Item,
    Environmental,
    NonQuestCreep,
    QuestQuestNpc,
    Technical,
    AccountBilling,
    Character,
    ArenaHonorItemIssues,
    ArenaHonorPointsIssues,
    BottingCheatingHacking,
    BugReport,
    CompromisedAccountIssue,
    GameSuggestions,
    GameplayQuestion,
    GuildBankIssue,
    GuildMasterIssue,
    HarassmentScamReport,
    InappropriateNameGuildArenaCharacterPet,
    KnownIssueFix,
    LatencyLagReport,
    LootingIssueMistake,
    MailIssue,
    NonInGameRelatedInquiry,
    ParentalControlsCais,
    Pcnc,
    Pct,
    RestorationStatusFollowUp,
    ServerInstanceIssues,
    Spam,
    SuicideCase,
    SuspensionQuestions,
    TechnicalSoundGraphicsIssue,
    UiIssue,
}

impl GmTicketType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NotSet => 0x0,
            Self::Stuck => 0x1,
            Self::BehaviorHarassment => 0x2,
            Self::Guild => 0x3,
            Self::Item => 0x4,
            Self::Environmental => 0x5,
            Self::NonQuestCreep => 0x6,
            Self::QuestQuestNpc => 0x7,
            Self::Technical => 0x8,
            Self::AccountBilling => 0x9,
            Self::Character => 0xa,
            Self::ArenaHonorItemIssues => 0xb,
            Self::ArenaHonorPointsIssues => 0xc,
            Self::BottingCheatingHacking => 0xd,
            Self::BugReport => 0xe,
            Self::CompromisedAccountIssue => 0xf,
            Self::GameSuggestions => 0x10,
            Self::GameplayQuestion => 0x11,
            Self::GuildBankIssue => 0x12,
            Self::GuildMasterIssue => 0x13,
            Self::HarassmentScamReport => 0x14,
            Self::InappropriateNameGuildArenaCharacterPet => 0x15,
            Self::KnownIssueFix => 0x16,
            Self::LatencyLagReport => 0x17,
            Self::LootingIssueMistake => 0x18,
            Self::MailIssue => 0x19,
            Self::NonInGameRelatedInquiry => 0x1a,
            Self::ParentalControlsCais => 0x1b,
            Self::Pcnc => 0x1c,
            Self::Pct => 0x1d,
            Self::RestorationStatusFollowUp => 0x1e,
            Self::ServerInstanceIssues => 0x1f,
            Self::Spam => 0x20,
            Self::SuicideCase => 0x21,
            Self::SuspensionQuestions => 0x22,
            Self::TechnicalSoundGraphicsIssue => 0x23,
            Self::UiIssue => 0x24,
        }
    }

    pub const fn variants() -> [Self; 37] {
        [
            Self::NotSet,
            Self::Stuck,
            Self::BehaviorHarassment,
            Self::Guild,
            Self::Item,
            Self::Environmental,
            Self::NonQuestCreep,
            Self::QuestQuestNpc,
            Self::Technical,
            Self::AccountBilling,
            Self::Character,
            Self::ArenaHonorItemIssues,
            Self::ArenaHonorPointsIssues,
            Self::BottingCheatingHacking,
            Self::BugReport,
            Self::CompromisedAccountIssue,
            Self::GameSuggestions,
            Self::GameplayQuestion,
            Self::GuildBankIssue,
            Self::GuildMasterIssue,
            Self::HarassmentScamReport,
            Self::InappropriateNameGuildArenaCharacterPet,
            Self::KnownIssueFix,
            Self::LatencyLagReport,
            Self::LootingIssueMistake,
            Self::MailIssue,
            Self::NonInGameRelatedInquiry,
            Self::ParentalControlsCais,
            Self::Pcnc,
            Self::Pct,
            Self::RestorationStatusFollowUp,
            Self::ServerInstanceIssues,
            Self::Spam,
            Self::SuicideCase,
            Self::SuspensionQuestions,
            Self::TechnicalSoundGraphicsIssue,
            Self::UiIssue,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::NotSet),
            1 => Ok(Self::Stuck),
            2 => Ok(Self::BehaviorHarassment),
            3 => Ok(Self::Guild),
            4 => Ok(Self::Item),
            5 => Ok(Self::Environmental),
            6 => Ok(Self::NonQuestCreep),
            7 => Ok(Self::QuestQuestNpc),
            8 => Ok(Self::Technical),
            9 => Ok(Self::AccountBilling),
            10 => Ok(Self::Character),
            11 => Ok(Self::ArenaHonorItemIssues),
            12 => Ok(Self::ArenaHonorPointsIssues),
            13 => Ok(Self::BottingCheatingHacking),
            14 => Ok(Self::BugReport),
            15 => Ok(Self::CompromisedAccountIssue),
            16 => Ok(Self::GameSuggestions),
            17 => Ok(Self::GameplayQuestion),
            18 => Ok(Self::GuildBankIssue),
            19 => Ok(Self::GuildMasterIssue),
            20 => Ok(Self::HarassmentScamReport),
            21 => Ok(Self::InappropriateNameGuildArenaCharacterPet),
            22 => Ok(Self::KnownIssueFix),
            23 => Ok(Self::LatencyLagReport),
            24 => Ok(Self::LootingIssueMistake),
            25 => Ok(Self::MailIssue),
            26 => Ok(Self::NonInGameRelatedInquiry),
            27 => Ok(Self::ParentalControlsCais),
            28 => Ok(Self::Pcnc),
            29 => Ok(Self::Pct),
            30 => Ok(Self::RestorationStatusFollowUp),
            31 => Ok(Self::ServerInstanceIssues),
            32 => Ok(Self::Spam),
            33 => Ok(Self::SuicideCase),
            34 => Ok(Self::SuspensionQuestions),
            35 => Ok(Self::TechnicalSoundGraphicsIssue),
            36 => Ok(Self::UiIssue),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl GmTicketType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NotSet => "NOT_SET",
            Self::Stuck => "STUCK",
            Self::BehaviorHarassment => "BEHAVIOR_HARASSMENT",
            Self::Guild => "GUILD",
            Self::Item => "ITEM",
            Self::Environmental => "ENVIRONMENTAL",
            Self::NonQuestCreep => "NON_QUEST_CREEP",
            Self::QuestQuestNpc => "QUEST_QUEST_NPC",
            Self::Technical => "TECHNICAL",
            Self::AccountBilling => "ACCOUNT_BILLING",
            Self::Character => "CHARACTER",
            Self::ArenaHonorItemIssues => "ARENA_HONOR_ITEM_ISSUES",
            Self::ArenaHonorPointsIssues => "ARENA_HONOR_POINTS_ISSUES",
            Self::BottingCheatingHacking => "BOTTING_CHEATING_HACKING",
            Self::BugReport => "BUG_REPORT",
            Self::CompromisedAccountIssue => "COMPROMISED_ACCOUNT_ISSUE",
            Self::GameSuggestions => "GAME_SUGGESTIONS",
            Self::GameplayQuestion => "GAMEPLAY_QUESTION",
            Self::GuildBankIssue => "GUILD_BANK_ISSUE",
            Self::GuildMasterIssue => "GUILD_MASTER_ISSUE",
            Self::HarassmentScamReport => "HARASSMENT_SCAM_REPORT",
            Self::InappropriateNameGuildArenaCharacterPet => "INAPPROPRIATE_NAME_GUILD_ARENA_CHARACTER_PET",
            Self::KnownIssueFix => "KNOWN_ISSUE_FIX",
            Self::LatencyLagReport => "LATENCY_LAG_REPORT",
            Self::LootingIssueMistake => "LOOTING_ISSUE_MISTAKE",
            Self::MailIssue => "MAIL_ISSUE",
            Self::NonInGameRelatedInquiry => "NON_IN_GAME_RELATED_INQUIRY",
            Self::ParentalControlsCais => "PARENTAL_CONTROLS_CAIS",
            Self::Pcnc => "PCNC",
            Self::Pct => "PCT",
            Self::RestorationStatusFollowUp => "RESTORATION_STATUS_FOLLOW_UP",
            Self::ServerInstanceIssues => "SERVER_INSTANCE_ISSUES",
            Self::Spam => "SPAM",
            Self::SuicideCase => "SUICIDE_CASE",
            Self::SuspensionQuestions => "SUSPENSION_QUESTIONS",
            Self::TechnicalSoundGraphicsIssue => "TECHNICAL_SOUND_GRAPHICS_ISSUE",
            Self::UiIssue => "UI_ISSUE",
        }
    }

}

const NAME: &str = "GmTicketType";

impl Default for GmTicketType {
    fn default() -> Self {
        Self::NotSet
    }
}

impl std::fmt::Display for GmTicketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSet => f.write_str("NotSet"),
            Self::Stuck => f.write_str("Stuck"),
            Self::BehaviorHarassment => f.write_str("BehaviorHarassment"),
            Self::Guild => f.write_str("Guild"),
            Self::Item => f.write_str("Item"),
            Self::Environmental => f.write_str("Environmental"),
            Self::NonQuestCreep => f.write_str("NonQuestCreep"),
            Self::QuestQuestNpc => f.write_str("QuestQuestNpc"),
            Self::Technical => f.write_str("Technical"),
            Self::AccountBilling => f.write_str("AccountBilling"),
            Self::Character => f.write_str("Character"),
            Self::ArenaHonorItemIssues => f.write_str("ArenaHonorItemIssues"),
            Self::ArenaHonorPointsIssues => f.write_str("ArenaHonorPointsIssues"),
            Self::BottingCheatingHacking => f.write_str("BottingCheatingHacking"),
            Self::BugReport => f.write_str("BugReport"),
            Self::CompromisedAccountIssue => f.write_str("CompromisedAccountIssue"),
            Self::GameSuggestions => f.write_str("GameSuggestions"),
            Self::GameplayQuestion => f.write_str("GameplayQuestion"),
            Self::GuildBankIssue => f.write_str("GuildBankIssue"),
            Self::GuildMasterIssue => f.write_str("GuildMasterIssue"),
            Self::HarassmentScamReport => f.write_str("HarassmentScamReport"),
            Self::InappropriateNameGuildArenaCharacterPet => f.write_str("InappropriateNameGuildArenaCharacterPet"),
            Self::KnownIssueFix => f.write_str("KnownIssueFix"),
            Self::LatencyLagReport => f.write_str("LatencyLagReport"),
            Self::LootingIssueMistake => f.write_str("LootingIssueMistake"),
            Self::MailIssue => f.write_str("MailIssue"),
            Self::NonInGameRelatedInquiry => f.write_str("NonInGameRelatedInquiry"),
            Self::ParentalControlsCais => f.write_str("ParentalControlsCais"),
            Self::Pcnc => f.write_str("Pcnc"),
            Self::Pct => f.write_str("Pct"),
            Self::RestorationStatusFollowUp => f.write_str("RestorationStatusFollowUp"),
            Self::ServerInstanceIssues => f.write_str("ServerInstanceIssues"),
            Self::Spam => f.write_str("Spam"),
            Self::SuicideCase => f.write_str("SuicideCase"),
            Self::SuspensionQuestions => f.write_str("SuspensionQuestions"),
            Self::TechnicalSoundGraphicsIssue => f.write_str("TechnicalSoundGraphicsIssue"),
            Self::UiIssue => f.write_str("UiIssue"),
        }
    }
}

impl TryFrom<u8> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

