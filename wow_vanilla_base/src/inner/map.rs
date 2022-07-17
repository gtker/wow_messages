/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/map.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/map.wowm#L2):
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
///     MONASTERY = 44;
///     RAZORFEN_KRAUL = 47;
///     BLACKFATHOM_DEEPS = 48;
///     ULDAMAN = 70;
///     GNOMERAGON = 90;
///     SUNKEN_TEMPLE = 109;
///     RAZORFEN_DOWNS = 129;
///     EMERALD_DREAM = 169;
///     SCARLET_MONASTERY = 189;
///     ZUL_FARRAK = 209;
///     BLACKROCK_SPIRE = 229;
///     BLACKROCK_DEPTHS = 230;
///     ONYXIAS_LAIR = 249;
///     CAVERNS_OF_TIME = 269;
///     SCHOLOMANCE = 289;
///     ZUL_GURUB = 309;
///     STRATHOLME = 329;
///     MAURADON = 349;
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
pub enum Map {
    EASTERN_KINGDOMS,
    KALIMDOR,
    TESTING,
    SCOTT_TEST,
    CASH_TEST,
    ALTERAC_VALLEY,
    SHADOWFANG_KEEP,
    STORMWIND_STOCKADE,
    STORMWIND_PRISON,
    DEADMINES,
    AZSHARA_CRATER,
    COLLINS_TEST,
    WAILING_CAVERNS,
    MONASTERY,
    RAZORFEN_KRAUL,
    BLACKFATHOM_DEEPS,
    ULDAMAN,
    GNOMERAGON,
    SUNKEN_TEMPLE,
    RAZORFEN_DOWNS,
    EMERALD_DREAM,
    SCARLET_MONASTERY,
    ZUL_FARRAK,
    BLACKROCK_SPIRE,
    BLACKROCK_DEPTHS,
    ONYXIAS_LAIR,
    CAVERNS_OF_TIME,
    SCHOLOMANCE,
    ZUL_GURUB,
    STRATHOLME,
    MAURADON,
    DEEPRUN_TRAM,
    RAGEFIRE_CHASM,
    MOLTEN_CORE,
    DIRE_MAUL,
    ALLIANCE_PVP_BARRACKS,
    HORDE_PVP_BARRACKS,
    DEVELOPMENT_LAND,
    BLACKWING_LAIR,
    WARSONG_GULCH,
    RUINS_OF_AHN_QIRAJ,
    ARATHI_BASIN,
    AHN_QIRAJ_TEMPLE,
    NAXXRAMAS,
}

impl Default for Map {
    fn default() -> Self {
        Self::EASTERN_KINGDOMS
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EASTERN_KINGDOMS => f.write_str("Eastern Kingdoms"),
            Self::KALIMDOR => f.write_str("Kalimdor"),
            Self::TESTING => f.write_str("Testing"),
            Self::SCOTT_TEST => f.write_str("Scott Test"),
            Self::CASH_TEST => f.write_str("CashTest"),
            Self::ALTERAC_VALLEY => f.write_str("Alterac Valley"),
            Self::SHADOWFANG_KEEP => f.write_str("Shadowfang Keep"),
            Self::STORMWIND_STOCKADE => f.write_str("Stormwind Stockade"),
            Self::STORMWIND_PRISON => f.write_str("Stormwind Prison"),
            Self::DEADMINES => f.write_str("Deadmines"),
            Self::AZSHARA_CRATER => f.write_str("Azshara Crater"),
            Self::COLLINS_TEST => f.write_str("Collin's Test"),
            Self::WAILING_CAVERNS => f.write_str("Wailing Caverns"),
            Self::MONASTERY => f.write_str("Monastery"),
            Self::RAZORFEN_KRAUL => f.write_str("Razorfen Kraul"),
            Self::BLACKFATHOM_DEEPS => f.write_str("Blackfathom Deeps"),
            Self::ULDAMAN => f.write_str("Uldaman"),
            Self::GNOMERAGON => f.write_str("Gnomeragon"),
            Self::SUNKEN_TEMPLE => f.write_str("SunkenTemple"),
            Self::RAZORFEN_DOWNS => f.write_str("Razorfen Downs"),
            Self::EMERALD_DREAM => f.write_str("Emerald Dream"),
            Self::SCARLET_MONASTERY => f.write_str("Scarlet Monastery"),
            Self::ZUL_FARRAK => f.write_str("Zul'Farrak"),
            Self::BLACKROCK_SPIRE => f.write_str("Blackrock Spire"),
            Self::BLACKROCK_DEPTHS => f.write_str("Blackrock Depths"),
            Self::ONYXIAS_LAIR => f.write_str("Onyxia's Lair"),
            Self::CAVERNS_OF_TIME => f.write_str("Caverns of Time"),
            Self::SCHOLOMANCE => f.write_str("Scholomance"),
            Self::ZUL_GURUB => f.write_str("Zul'Gurub"),
            Self::STRATHOLME => f.write_str("Stratholme"),
            Self::MAURADON => f.write_str("Mauradon"),
            Self::DEEPRUN_TRAM => f.write_str("Deeprun Tram"),
            Self::RAGEFIRE_CHASM => f.write_str("Ragefire Chasm"),
            Self::MOLTEN_CORE => f.write_str("Molten Core"),
            Self::DIRE_MAUL => f.write_str("Dire Maul"),
            Self::ALLIANCE_PVP_BARRACKS => f.write_str("Alliance PVP Barracks"),
            Self::HORDE_PVP_BARRACKS => f.write_str("Horde PVP Barracks"),
            Self::DEVELOPMENT_LAND => f.write_str("Development Land"),
            Self::BLACKWING_LAIR => f.write_str("Blackwing Lair"),
            Self::WARSONG_GULCH => f.write_str("Warsong Gulch"),
            Self::RUINS_OF_AHN_QIRAJ => f.write_str("Ruins of Ahn'Qiraj"),
            Self::ARATHI_BASIN => f.write_str("Arathi Basin"),
            Self::AHN_QIRAJ_TEMPLE => f.write_str("Ahn'Qiraj Temple"),
            Self::NAXXRAMAS => f.write_str("Naxxramas"),
        }
    }
}

