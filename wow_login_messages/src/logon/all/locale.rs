/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/challenge_client_commons.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/challenge_client_commons.wowm#L15):
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
    EnGb,
    EnUs,
    EsMx,
    PtBr,
    FrFr,
    DeDe,
    EsEs,
    PtPt,
    ItIt,
    RuRu,
    KoKr,
    ZhTw,
    EnTw,
    EnCn,
    Other(u32),
}

impl Locale {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::EnGb => 0x656e4742,
            Self::EnUs => 0x656e5553,
            Self::EsMx => 0x65734d58,
            Self::PtBr => 0x70744252,
            Self::FrFr => 0x66724652,
            Self::DeDe => 0x64654445,
            Self::EsEs => 0x65734553,
            Self::PtPt => 0x70745054,
            Self::ItIt => 0x69744954,
            Self::RuRu => 0x72755255,
            Self::KoKr => 0x6b6f4b52,
            Self::ZhTw => 0x7a685457,
            Self::EnTw => 0x656e5457,
            Self::EnCn => 0x656e434e,
            Self::Other(v) => *v,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl Locale {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::EnGb => "EN_GB",
            Self::EnUs => "EN_US",
            Self::EsMx => "ES_MX",
            Self::PtBr => "PT_BR",
            Self::FrFr => "FR_FR",
            Self::DeDe => "DE_DE",
            Self::EsEs => "ES_ES",
            Self::PtPt => "PT_PT",
            Self::ItIt => "IT_IT",
            Self::RuRu => "RU_RU",
            Self::KoKr => "KO_KR",
            Self::ZhTw => "ZH_TW",
            Self::EnTw => "EN_TW",
            Self::EnCn => "EN_CN",
            Self::Other(_) => "OTHER",
        }
    }

}

impl Default for Locale {
    fn default() -> Self {
        Self::EnGb
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnGb => f.write_str("EnGb"),
            Self::EnUs => f.write_str("EnUs"),
            Self::EsMx => f.write_str("EsMx"),
            Self::PtBr => f.write_str("PtBr"),
            Self::FrFr => f.write_str("FrFr"),
            Self::DeDe => f.write_str("DeDe"),
            Self::EsEs => f.write_str("EsEs"),
            Self::PtPt => f.write_str("PtPt"),
            Self::ItIt => f.write_str("ItIt"),
            Self::RuRu => f.write_str("RuRu"),
            Self::KoKr => f.write_str("KoKr"),
            Self::ZhTw => f.write_str("ZhTw"),
            Self::EnTw => f.write_str("EnTw"),
            Self::EnCn => f.write_str("EnCn"),
            Self::Other(v) => f.write_fmt(format_args!("Other({v})")),
        }
    }
}

impl From<u32> for Locale {
    fn from(value: u32) -> Self {
        match value {
            1701726018 => Self::EnGb,
            1701729619 => Self::EnUs,
            1702055256 => Self::EsMx,
            1886667346 => Self::PtBr,
            1718765138 => Self::FrFr,
            1684358213 => Self::DeDe,
            1702053203 => Self::EsEs,
            1886670932 => Self::PtPt,
            1769228628 => Self::ItIt,
            1920291413 => Self::RuRu,
            1802455890 => Self::KoKr,
            2053657687 => Self::ZhTw,
            1701729367 => Self::EnTw,
            1701725006 => Self::EnCn,
            v => Self::Other(v)
        }
    }
}

