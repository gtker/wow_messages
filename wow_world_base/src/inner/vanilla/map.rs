/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/map.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/map.wowm#L1):
/// ```text
/// enum Map : u32 {
///     EASTERN_KINGDOMS = 0;
///     KALIMDOR = 1;
///     TESTING = 13;
///     SCOTT_TEST = 25;
///     CASH_TEST = 29;
///     ALTERAC_VALLEY = 30;
///     SHADOWFANG_KEEP = 33;
///     STORMWIND_STOCKADE = 34;
///     STORMWIND_PRISON = 35;
///     DEADMINES = 36;
///     AZSHARA_CRATER = 37;
///     COLLINS_TEST = 42;
///     WAILING_CAVERNS = 43;
///     MONASTERY_UNUSED = 44;
///     RAZORFEN_KRAUL = 47;
///     BLACKFATHOM_DEEPS = 48;
///     ULDAMAN = 70;
///     GNOMEREGAN = 90;
///     SUNKEN_TEMPLE = 109;
///     RAZORFEN_DOWNS = 129;
///     EMERALD_DREAM = 169;
///     SCARLET_MONASTERY = 189;
///     ZUL_FARRAK = 209;
///     BLACKROCK_SPIRE = 229;
///     BLACKROCK_DEPTHS = 230;
///     ONYXIAS_LAIR = 249;
///     OPENING_OF_THE_DARK_PORTAL = 269;
///     SCHOLOMANCE = 289;
///     ZUL_GURUB = 309;
///     STRATHOLME = 329;
///     MARAUDON = 349;
///     DEEPRUN_TRAM = 369;
///     RAGEFIRE_CHASM = 389;
///     MOLTEN_CORE = 409;
///     DIRE_MAUL = 429;
///     ALLIANCE_PVP_BARRACKS = 449;
///     HORDE_PVP_BARRACKS = 450;
///     DEVELOPMENT_LAND = 451;
///     BLACKWING_LAIR = 469;
///     WARSONG_GULCH = 489;
///     RUINS_OF_AHN_QIRAJ = 509;
///     ARATHI_BASIN = 529;
///     AHN_QIRAJ_TEMPLE = 531;
///     NAXXRAMAS = 533;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Map {
    EasternKingdoms,
    Kalimdor,
    Testing,
    ScottTest,
    CashTest,
    AlteracValley,
    ShadowfangKeep,
    StormwindStockade,
    StormwindPrison,
    Deadmines,
    AzsharaCrater,
    CollinsTest,
    WailingCaverns,
    MonasteryUnused,
    RazorfenKraul,
    BlackfathomDeeps,
    Uldaman,
    Gnomeregan,
    SunkenTemple,
    RazorfenDowns,
    EmeraldDream,
    ScarletMonastery,
    ZulFarrak,
    BlackrockSpire,
    BlackrockDepths,
    OnyxiasLair,
    /// Also called 'Caverns of Time' in Vanilla. Named this way for consistency across versions.
    OpeningOfTheDarkPortal,
    Scholomance,
    ZulGurub,
    Stratholme,
    Maraudon,
    DeeprunTram,
    RagefireChasm,
    MoltenCore,
    DireMaul,
    AlliancePvpBarracks,
    HordePvpBarracks,
    DevelopmentLand,
    BlackwingLair,
    WarsongGulch,
    RuinsOfAhnQiraj,
    ArathiBasin,
    AhnQirajTemple,
    Naxxramas,
}

impl Map {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::EasternKingdoms => 0x0,
            Self::Kalimdor => 0x1,
            Self::Testing => 0xd,
            Self::ScottTest => 0x19,
            Self::CashTest => 0x1d,
            Self::AlteracValley => 0x1e,
            Self::ShadowfangKeep => 0x21,
            Self::StormwindStockade => 0x22,
            Self::StormwindPrison => 0x23,
            Self::Deadmines => 0x24,
            Self::AzsharaCrater => 0x25,
            Self::CollinsTest => 0x2a,
            Self::WailingCaverns => 0x2b,
            Self::MonasteryUnused => 0x2c,
            Self::RazorfenKraul => 0x2f,
            Self::BlackfathomDeeps => 0x30,
            Self::Uldaman => 0x46,
            Self::Gnomeregan => 0x5a,
            Self::SunkenTemple => 0x6d,
            Self::RazorfenDowns => 0x81,
            Self::EmeraldDream => 0xa9,
            Self::ScarletMonastery => 0xbd,
            Self::ZulFarrak => 0xd1,
            Self::BlackrockSpire => 0xe5,
            Self::BlackrockDepths => 0xe6,
            Self::OnyxiasLair => 0xf9,
            Self::OpeningOfTheDarkPortal => 0x10d,
            Self::Scholomance => 0x121,
            Self::ZulGurub => 0x135,
            Self::Stratholme => 0x149,
            Self::Maraudon => 0x15d,
            Self::DeeprunTram => 0x171,
            Self::RagefireChasm => 0x185,
            Self::MoltenCore => 0x199,
            Self::DireMaul => 0x1ad,
            Self::AlliancePvpBarracks => 0x1c1,
            Self::HordePvpBarracks => 0x1c2,
            Self::DevelopmentLand => 0x1c3,
            Self::BlackwingLair => 0x1d5,
            Self::WarsongGulch => 0x1e9,
            Self::RuinsOfAhnQiraj => 0x1fd,
            Self::ArathiBasin => 0x211,
            Self::AhnQirajTemple => 0x213,
            Self::Naxxramas => 0x215,
        }
    }

    pub const fn variants() -> [Self; 44] {
        [
            Self::EasternKingdoms,
            Self::Kalimdor,
            Self::Testing,
            Self::ScottTest,
            Self::CashTest,
            Self::AlteracValley,
            Self::ShadowfangKeep,
            Self::StormwindStockade,
            Self::StormwindPrison,
            Self::Deadmines,
            Self::AzsharaCrater,
            Self::CollinsTest,
            Self::WailingCaverns,
            Self::MonasteryUnused,
            Self::RazorfenKraul,
            Self::BlackfathomDeeps,
            Self::Uldaman,
            Self::Gnomeregan,
            Self::SunkenTemple,
            Self::RazorfenDowns,
            Self::EmeraldDream,
            Self::ScarletMonastery,
            Self::ZulFarrak,
            Self::BlackrockSpire,
            Self::BlackrockDepths,
            Self::OnyxiasLair,
            Self::OpeningOfTheDarkPortal,
            Self::Scholomance,
            Self::ZulGurub,
            Self::Stratholme,
            Self::Maraudon,
            Self::DeeprunTram,
            Self::RagefireChasm,
            Self::MoltenCore,
            Self::DireMaul,
            Self::AlliancePvpBarracks,
            Self::HordePvpBarracks,
            Self::DevelopmentLand,
            Self::BlackwingLair,
            Self::WarsongGulch,
            Self::RuinsOfAhnQiraj,
            Self::ArathiBasin,
            Self::AhnQirajTemple,
            Self::Naxxramas,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl Map {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::EasternKingdoms => "EASTERN_KINGDOMS",
            Self::Kalimdor => "KALIMDOR",
            Self::Testing => "TESTING",
            Self::ScottTest => "SCOTT_TEST",
            Self::CashTest => "CASH_TEST",
            Self::AlteracValley => "ALTERAC_VALLEY",
            Self::ShadowfangKeep => "SHADOWFANG_KEEP",
            Self::StormwindStockade => "STORMWIND_STOCKADE",
            Self::StormwindPrison => "STORMWIND_PRISON",
            Self::Deadmines => "DEADMINES",
            Self::AzsharaCrater => "AZSHARA_CRATER",
            Self::CollinsTest => "COLLINS_TEST",
            Self::WailingCaverns => "WAILING_CAVERNS",
            Self::MonasteryUnused => "MONASTERY_UNUSED",
            Self::RazorfenKraul => "RAZORFEN_KRAUL",
            Self::BlackfathomDeeps => "BLACKFATHOM_DEEPS",
            Self::Uldaman => "ULDAMAN",
            Self::Gnomeregan => "GNOMEREGAN",
            Self::SunkenTemple => "SUNKEN_TEMPLE",
            Self::RazorfenDowns => "RAZORFEN_DOWNS",
            Self::EmeraldDream => "EMERALD_DREAM",
            Self::ScarletMonastery => "SCARLET_MONASTERY",
            Self::ZulFarrak => "ZUL_FARRAK",
            Self::BlackrockSpire => "BLACKROCK_SPIRE",
            Self::BlackrockDepths => "BLACKROCK_DEPTHS",
            Self::OnyxiasLair => "ONYXIAS_LAIR",
            Self::OpeningOfTheDarkPortal => "OPENING_OF_THE_DARK_PORTAL",
            Self::Scholomance => "SCHOLOMANCE",
            Self::ZulGurub => "ZUL_GURUB",
            Self::Stratholme => "STRATHOLME",
            Self::Maraudon => "MARAUDON",
            Self::DeeprunTram => "DEEPRUN_TRAM",
            Self::RagefireChasm => "RAGEFIRE_CHASM",
            Self::MoltenCore => "MOLTEN_CORE",
            Self::DireMaul => "DIRE_MAUL",
            Self::AlliancePvpBarracks => "ALLIANCE_PVP_BARRACKS",
            Self::HordePvpBarracks => "HORDE_PVP_BARRACKS",
            Self::DevelopmentLand => "DEVELOPMENT_LAND",
            Self::BlackwingLair => "BLACKWING_LAIR",
            Self::WarsongGulch => "WARSONG_GULCH",
            Self::RuinsOfAhnQiraj => "RUINS_OF_AHN_QIRAJ",
            Self::ArathiBasin => "ARATHI_BASIN",
            Self::AhnQirajTemple => "AHN_QIRAJ_TEMPLE",
            Self::Naxxramas => "NAXXRAMAS",
        }
    }

}

const NAME: &str = "Map";

impl Default for Map {
    fn default() -> Self {
        Self::EasternKingdoms
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EasternKingdoms => f.write_str("Eastern Kingdoms"),
            Self::Kalimdor => f.write_str("Kalimdor"),
            Self::Testing => f.write_str("Testing"),
            Self::ScottTest => f.write_str("Scott Test"),
            Self::CashTest => f.write_str("CashTest"),
            Self::AlteracValley => f.write_str("Alterac Valley"),
            Self::ShadowfangKeep => f.write_str("Shadowfang Keep"),
            Self::StormwindStockade => f.write_str("Stormwind Stockade"),
            Self::StormwindPrison => f.write_str("Stormwind Prison"),
            Self::Deadmines => f.write_str("Deadmines"),
            Self::AzsharaCrater => f.write_str("Azshara Crater"),
            Self::CollinsTest => f.write_str("Collin's Test"),
            Self::WailingCaverns => f.write_str("Wailing Caverns"),
            Self::MonasteryUnused => f.write_str("<Unused> Monastery"),
            Self::RazorfenKraul => f.write_str("Razorfen Kraul"),
            Self::BlackfathomDeeps => f.write_str("Blackfathom Deeps"),
            Self::Uldaman => f.write_str("Uldaman"),
            Self::Gnomeregan => f.write_str("Gnomeregan"),
            Self::SunkenTemple => f.write_str("Sunken Temple"),
            Self::RazorfenDowns => f.write_str("Razorfen Downs"),
            Self::EmeraldDream => f.write_str("Emerald Dream"),
            Self::ScarletMonastery => f.write_str("Scarlet Monastery"),
            Self::ZulFarrak => f.write_str("Zul'Farrak"),
            Self::BlackrockSpire => f.write_str("Blackrock Spire"),
            Self::BlackrockDepths => f.write_str("Blackrock Depths"),
            Self::OnyxiasLair => f.write_str("Onyxia's Lair"),
            Self::OpeningOfTheDarkPortal => f.write_str("Opening of the Dark Portal"),
            Self::Scholomance => f.write_str("Scholomance"),
            Self::ZulGurub => f.write_str("Zul'Gurub"),
            Self::Stratholme => f.write_str("Stratholme"),
            Self::Maraudon => f.write_str("Maraudon"),
            Self::DeeprunTram => f.write_str("Deeprun Tram"),
            Self::RagefireChasm => f.write_str("Ragefire Chasm"),
            Self::MoltenCore => f.write_str("Molten Core"),
            Self::DireMaul => f.write_str("Dire Maul"),
            Self::AlliancePvpBarracks => f.write_str("Alliance PVP Barracks"),
            Self::HordePvpBarracks => f.write_str("Horde PVP Barracks"),
            Self::DevelopmentLand => f.write_str("Development Land"),
            Self::BlackwingLair => f.write_str("Blackwing Lair"),
            Self::WarsongGulch => f.write_str("Warsong Gulch"),
            Self::RuinsOfAhnQiraj => f.write_str("Ruins of Ahn'Qiraj"),
            Self::ArathiBasin => f.write_str("Arathi Basin"),
            Self::AhnQirajTemple => f.write_str("Ahn'Qiraj Temple"),
            Self::Naxxramas => f.write_str("Naxxramas"),
        }
    }
}

impl TryFrom<u32> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EasternKingdoms),
            1 => Ok(Self::Kalimdor),
            13 => Ok(Self::Testing),
            25 => Ok(Self::ScottTest),
            29 => Ok(Self::CashTest),
            30 => Ok(Self::AlteracValley),
            33 => Ok(Self::ShadowfangKeep),
            34 => Ok(Self::StormwindStockade),
            35 => Ok(Self::StormwindPrison),
            36 => Ok(Self::Deadmines),
            37 => Ok(Self::AzsharaCrater),
            42 => Ok(Self::CollinsTest),
            43 => Ok(Self::WailingCaverns),
            44 => Ok(Self::MonasteryUnused),
            47 => Ok(Self::RazorfenKraul),
            48 => Ok(Self::BlackfathomDeeps),
            70 => Ok(Self::Uldaman),
            90 => Ok(Self::Gnomeregan),
            109 => Ok(Self::SunkenTemple),
            129 => Ok(Self::RazorfenDowns),
            169 => Ok(Self::EmeraldDream),
            189 => Ok(Self::ScarletMonastery),
            209 => Ok(Self::ZulFarrak),
            229 => Ok(Self::BlackrockSpire),
            230 => Ok(Self::BlackrockDepths),
            249 => Ok(Self::OnyxiasLair),
            269 => Ok(Self::OpeningOfTheDarkPortal),
            289 => Ok(Self::Scholomance),
            309 => Ok(Self::ZulGurub),
            329 => Ok(Self::Stratholme),
            349 => Ok(Self::Maraudon),
            369 => Ok(Self::DeeprunTram),
            389 => Ok(Self::RagefireChasm),
            409 => Ok(Self::MoltenCore),
            429 => Ok(Self::DireMaul),
            449 => Ok(Self::AlliancePvpBarracks),
            450 => Ok(Self::HordePvpBarracks),
            451 => Ok(Self::DevelopmentLand),
            469 => Ok(Self::BlackwingLair),
            489 => Ok(Self::WarsongGulch),
            509 => Ok(Self::RuinsOfAhnQiraj),
            529 => Ok(Self::ArathiBasin),
            531 => Ok(Self::AhnQirajTemple),
            533 => Ok(Self::Naxxramas),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

