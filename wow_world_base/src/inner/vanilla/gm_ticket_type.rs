/// vmangos: From GMTicketCategory.dbc
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm#L14):
/// ```text
/// enum GmTicketType : u8 {
///     STUCK = 1;
///     BEHAVIOR_HARASSMENT = 2;
///     GUILD = 3;
///     ITEM = 4;
///     ENVIRONMENTAL = 5;
///     NONQUEST_CREEP = 6;
///     QUEST_QUESTNPC = 7;
///     TECHNICAL = 8;
///     ACCOUNT_BILLING = 9;
///     CHARACTER = 10;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GmTicketType {
    Stuck,
    BehaviorHarassment,
    Guild,
    Item,
    Environmental,
    NonquestCreep,
    QuestQuestnpc,
    Technical,
    AccountBilling,
    Character,
}

impl GmTicketType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Stuck => 0x1,
            Self::BehaviorHarassment => 0x2,
            Self::Guild => 0x3,
            Self::Item => 0x4,
            Self::Environmental => 0x5,
            Self::NonquestCreep => 0x6,
            Self::QuestQuestnpc => 0x7,
            Self::Technical => 0x8,
            Self::AccountBilling => 0x9,
            Self::Character => 0xa,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl GmTicketType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Stuck => "STUCK",
            Self::BehaviorHarassment => "BEHAVIOR_HARASSMENT",
            Self::Guild => "GUILD",
            Self::Item => "ITEM",
            Self::Environmental => "ENVIRONMENTAL",
            Self::NonquestCreep => "NONQUEST_CREEP",
            Self::QuestQuestnpc => "QUEST_QUESTNPC",
            Self::Technical => "TECHNICAL",
            Self::AccountBilling => "ACCOUNT_BILLING",
            Self::Character => "CHARACTER",
        }
    }

}

impl Default for GmTicketType {
    fn default() -> Self {
        Self::Stuck
    }
}

impl std::fmt::Display for GmTicketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stuck => f.write_str("Stuck"),
            Self::BehaviorHarassment => f.write_str("BehaviorHarassment"),
            Self::Guild => f.write_str("Guild"),
            Self::Item => f.write_str("Item"),
            Self::Environmental => f.write_str("Environmental"),
            Self::NonquestCreep => f.write_str("NonquestCreep"),
            Self::QuestQuestnpc => f.write_str("QuestQuestnpc"),
            Self::Technical => f.write_str("Technical"),
            Self::AccountBilling => f.write_str("AccountBilling"),
            Self::Character => f.write_str("Character"),
        }
    }
}

impl TryFrom<u8> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Stuck),
            2 => Ok(Self::BehaviorHarassment),
            3 => Ok(Self::Guild),
            4 => Ok(Self::Item),
            5 => Ok(Self::Environmental),
            6 => Ok(Self::NonquestCreep),
            7 => Ok(Self::QuestQuestnpc),
            8 => Ok(Self::Technical),
            9 => Ok(Self::AccountBilling),
            10 => Ok(Self::Character),
            v => Err(crate::errors::EnumError::new("GmTicketType", v.into()),)
        }
    }
}

