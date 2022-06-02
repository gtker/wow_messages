use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm#L15):
/// ```text
/// enum Locale : u32 {
///     EN_GB = "enGB";
///     EN_US = "enUS";
///     ES_MX = "esMX";
///     PT_BR = "ptBR";
///     FR_FR = "frFR";
///     DE_DE = "deDE";
///     ES_ES = "esES";
///     PT_PT = "ptPT";
///     IT_IT = "itIT";
///     RU_RU = "ruRU";
///     KO_KR = "koKR";
///     ZH_TW = "zhTW";
///     EN_TW = "enTW";
///     EN_CN = "enCN";
///     OTHER = self.value
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Locale {
    EN_GB,
    EN_US,
    ES_MX,
    PT_BR,
    FR_FR,
    DE_DE,
    ES_ES,
    PT_PT,
    IT_IT,
    RU_RU,
    KO_KR,
    ZH_TW,
    EN_TW,
    EN_CN,
    OTHER(u32),
}

impl Locale {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::EN_GB => 0x656e4742,
            Self::EN_US => 0x656e5553,
            Self::ES_MX => 0x65734d58,
            Self::PT_BR => 0x70744252,
            Self::FR_FR => 0x66724652,
            Self::DE_DE => 0x64654445,
            Self::ES_ES => 0x65734553,
            Self::PT_PT => 0x70745054,
            Self::IT_IT => 0x69744954,
            Self::RU_RU => 0x72755255,
            Self::KO_KR => 0x6b6f4b52,
            Self::ZH_TW => 0x7a685457,
            Self::EN_TW => 0x656e5457,
            Self::EN_CN => 0x656e434e,
            Self::OTHER(v) => *v,
        }
    }

}

impl Default for Locale {
    fn default() -> Self {
        Self::EN_GB
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EN_GB => f.write_str("EN_GB"),
            Self::EN_US => f.write_str("EN_US"),
            Self::ES_MX => f.write_str("ES_MX"),
            Self::PT_BR => f.write_str("PT_BR"),
            Self::FR_FR => f.write_str("FR_FR"),
            Self::DE_DE => f.write_str("DE_DE"),
            Self::ES_ES => f.write_str("ES_ES"),
            Self::PT_PT => f.write_str("PT_PT"),
            Self::IT_IT => f.write_str("IT_IT"),
            Self::RU_RU => f.write_str("RU_RU"),
            Self::KO_KR => f.write_str("KO_KR"),
            Self::ZH_TW => f.write_str("ZH_TW"),
            Self::EN_TW => f.write_str("EN_TW"),
            Self::EN_CN => f.write_str("EN_CN"),
            Self::OTHER(v) => f.write_fmt(format_args!("OTHER({})", v)),
        }
    }
}

impl From<u32> for Locale {
    fn from(value: u32) -> Self {
        match value {
            1701726018 => Self::EN_GB,
            1701729619 => Self::EN_US,
            1702055256 => Self::ES_MX,
            1886667346 => Self::PT_BR,
            1718765138 => Self::FR_FR,
            1684358213 => Self::DE_DE,
            1702053203 => Self::ES_ES,
            1886670932 => Self::PT_PT,
            1769228628 => Self::IT_IT,
            1920291413 => Self::RU_RU,
            1802455890 => Self::KO_KR,
            2053657687 => Self::ZH_TW,
            1701729367 => Self::EN_TW,
            1701725006 => Self::EN_CN,
            v => Self::OTHER(v)
        }
    }
}

