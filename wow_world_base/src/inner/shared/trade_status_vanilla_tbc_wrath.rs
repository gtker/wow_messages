/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status.wowm:254`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status.wowm#L254):
/// ```text
/// enum TradeStatus : u32 {
///     BUSY = 0;
///     BEGIN_TRADE = 1;
///     OPEN_WINDOW = 2;
///     TRADE_CANCELED = 3;
///     TRADE_ACCEPT = 4;
///     BUSY_2 = 5;
///     NO_TARGET = 6;
///     BACK_TO_TRADE = 7;
///     TRADE_COMPLETE = 8;
///     TRADE_REJECTED = 9;
///     TARGET_TO_FAR = 10;
///     WRONG_FACTION = 11;
///     CLOSE_WINDOW = 12;
///     UNKNOWN_13 = 13;
///     IGNORE_YOU = 14;
///     YOU_STUNNED = 15;
///     TARGET_STUNNED = 16;
///     YOU_DEAD = 17;
///     TARGET_DEAD = 18;
///     YOU_LOGOUT = 19;
///     TARGET_LOGOUT = 20;
///     TRIAL_ACCOUNT = 21;
///     ONLY_CONJURED = 22;
///     NOT_ON_TAPLIST = 23;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TradeStatus {
    Busy,
    BeginTrade,
    OpenWindow,
    TradeCanceled,
    TradeAccept,
    Busy2,
    NoTarget,
    BackToTrade,
    TradeComplete,
    TradeRejected,
    TargetToFar,
    WrongFaction,
    CloseWindow,
    Unknown13,
    IgnoreYou,
    YouStunned,
    TargetStunned,
    YouDead,
    TargetDead,
    YouLogout,
    TargetLogout,
    TrialAccount,
    OnlyConjured,
    NotOnTaplist,
}

impl TradeStatus {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Busy => 0x0,
            Self::BeginTrade => 0x1,
            Self::OpenWindow => 0x2,
            Self::TradeCanceled => 0x3,
            Self::TradeAccept => 0x4,
            Self::Busy2 => 0x5,
            Self::NoTarget => 0x6,
            Self::BackToTrade => 0x7,
            Self::TradeComplete => 0x8,
            Self::TradeRejected => 0x9,
            Self::TargetToFar => 0xa,
            Self::WrongFaction => 0xb,
            Self::CloseWindow => 0xc,
            Self::Unknown13 => 0xd,
            Self::IgnoreYou => 0xe,
            Self::YouStunned => 0xf,
            Self::TargetStunned => 0x10,
            Self::YouDead => 0x11,
            Self::TargetDead => 0x12,
            Self::YouLogout => 0x13,
            Self::TargetLogout => 0x14,
            Self::TrialAccount => 0x15,
            Self::OnlyConjured => 0x16,
            Self::NotOnTaplist => 0x17,
        }
    }

    pub const fn variants() -> [Self; 24] {
        [
            Self::Busy,
            Self::BeginTrade,
            Self::OpenWindow,
            Self::TradeCanceled,
            Self::TradeAccept,
            Self::Busy2,
            Self::NoTarget,
            Self::BackToTrade,
            Self::TradeComplete,
            Self::TradeRejected,
            Self::TargetToFar,
            Self::WrongFaction,
            Self::CloseWindow,
            Self::Unknown13,
            Self::IgnoreYou,
            Self::YouStunned,
            Self::TargetStunned,
            Self::YouDead,
            Self::TargetDead,
            Self::YouLogout,
            Self::TargetLogout,
            Self::TrialAccount,
            Self::OnlyConjured,
            Self::NotOnTaplist,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Busy),
            1 => Ok(Self::BeginTrade),
            2 => Ok(Self::OpenWindow),
            3 => Ok(Self::TradeCanceled),
            4 => Ok(Self::TradeAccept),
            5 => Ok(Self::Busy2),
            6 => Ok(Self::NoTarget),
            7 => Ok(Self::BackToTrade),
            8 => Ok(Self::TradeComplete),
            9 => Ok(Self::TradeRejected),
            10 => Ok(Self::TargetToFar),
            11 => Ok(Self::WrongFaction),
            12 => Ok(Self::CloseWindow),
            13 => Ok(Self::Unknown13),
            14 => Ok(Self::IgnoreYou),
            15 => Ok(Self::YouStunned),
            16 => Ok(Self::TargetStunned),
            17 => Ok(Self::YouDead),
            18 => Ok(Self::TargetDead),
            19 => Ok(Self::YouLogout),
            20 => Ok(Self::TargetLogout),
            21 => Ok(Self::TrialAccount),
            22 => Ok(Self::OnlyConjured),
            23 => Ok(Self::NotOnTaplist),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl TradeStatus {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Busy => "BUSY",
            Self::BeginTrade => "BEGIN_TRADE",
            Self::OpenWindow => "OPEN_WINDOW",
            Self::TradeCanceled => "TRADE_CANCELED",
            Self::TradeAccept => "TRADE_ACCEPT",
            Self::Busy2 => "BUSY_2",
            Self::NoTarget => "NO_TARGET",
            Self::BackToTrade => "BACK_TO_TRADE",
            Self::TradeComplete => "TRADE_COMPLETE",
            Self::TradeRejected => "TRADE_REJECTED",
            Self::TargetToFar => "TARGET_TO_FAR",
            Self::WrongFaction => "WRONG_FACTION",
            Self::CloseWindow => "CLOSE_WINDOW",
            Self::Unknown13 => "UNKNOWN_13",
            Self::IgnoreYou => "IGNORE_YOU",
            Self::YouStunned => "YOU_STUNNED",
            Self::TargetStunned => "TARGET_STUNNED",
            Self::YouDead => "YOU_DEAD",
            Self::TargetDead => "TARGET_DEAD",
            Self::YouLogout => "YOU_LOGOUT",
            Self::TargetLogout => "TARGET_LOGOUT",
            Self::TrialAccount => "TRIAL_ACCOUNT",
            Self::OnlyConjured => "ONLY_CONJURED",
            Self::NotOnTaplist => "NOT_ON_TAPLIST",
        }
    }

}

const NAME: &str = "TradeStatus";

impl Default for TradeStatus {
    fn default() -> Self {
        Self::Busy
    }
}

impl std::fmt::Display for TradeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Busy => f.write_str("Busy"),
            Self::BeginTrade => f.write_str("BeginTrade"),
            Self::OpenWindow => f.write_str("OpenWindow"),
            Self::TradeCanceled => f.write_str("TradeCanceled"),
            Self::TradeAccept => f.write_str("TradeAccept"),
            Self::Busy2 => f.write_str("Busy2"),
            Self::NoTarget => f.write_str("NoTarget"),
            Self::BackToTrade => f.write_str("BackToTrade"),
            Self::TradeComplete => f.write_str("TradeComplete"),
            Self::TradeRejected => f.write_str("TradeRejected"),
            Self::TargetToFar => f.write_str("TargetToFar"),
            Self::WrongFaction => f.write_str("WrongFaction"),
            Self::CloseWindow => f.write_str("CloseWindow"),
            Self::Unknown13 => f.write_str("Unknown13"),
            Self::IgnoreYou => f.write_str("IgnoreYou"),
            Self::YouStunned => f.write_str("YouStunned"),
            Self::TargetStunned => f.write_str("TargetStunned"),
            Self::YouDead => f.write_str("YouDead"),
            Self::TargetDead => f.write_str("TargetDead"),
            Self::YouLogout => f.write_str("YouLogout"),
            Self::TargetLogout => f.write_str("TargetLogout"),
            Self::TrialAccount => f.write_str("TrialAccount"),
            Self::OnlyConjured => f.write_str("OnlyConjured"),
            Self::NotOnTaplist => f.write_str("NotOnTaplist"),
        }
    }
}

impl TryFrom<u32> for TradeStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for TradeStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for TradeStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for TradeStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for TradeStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for TradeStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for TradeStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for TradeStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for TradeStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

