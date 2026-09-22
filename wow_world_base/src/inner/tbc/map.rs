/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/map.wowm:141`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/map.wowm#L141):
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
///     OUTLAND = 530;
///     AHN_QIRAJ_TEMPLE = 531;
///     KARAZHAN = 532;
///     NAXXRAMAS = 533;
///     THE_BATTLE_FOR_MOUNT_HYJAL = 534;
///     HELLFIRE_CITADEL_THE_SHATTERED_HALLS = 540;
///     HELLFIRE_CITADEL_THE_BLOOD_FURNACE = 542;
///     HELLFIRE_CITADEL_RAMPARTS = 543;
///     MAGTHERIDONS_LAIR = 544;
///     COILFANG_THE_STEAMVAULT = 545;
///     COILFANG_THE_UNDERBOG = 546;
///     COILFANG_THE_SLAVE_PENS = 547;
///     COILFANG_SERPENTSHRINE_CAVERN = 548;
///     TEMPEST_KEEP = 550;
///     TEMPEST_KEEP_THE_ARCATRAZ = 552;
///     TEMPEST_KEEP_THE_BOTANICA = 553;
///     TEMPEST_KEEP_THE_MECHANAR = 554;
///     AUCHINDOUN_SHADOW_LABYRINTH = 555;
///     AUCHINDOUN_SETHEKK_HALLS = 556;
///     AUCHINDOUN_MANA_TOMBS = 557;
///     AUCHINDOUN_AUCHENAI_CRYPTS = 558;
///     NAGRAND_ARENA = 559;
///     THE_ESCAPE_FROM_DURNHOLDE = 560;
///     BLADES_EDGE_ARENA = 562;
///     BLACK_TEMPLE = 564;
///     GRUULS_LAIR = 565;
///     EYE_OF_THE_STORM = 566;
///     ZUL_AMAN = 568;
///     RUINS_OF_LORDAERON = 572;
///     THE_SUNWELL = 580;
///     TRANSPORT_RUT_THERAN_TO_AUBERDINE = 582;
///     TRANSPORT_MENETHIL_TO_THERAMORE = 584;
///     MAGISTERS_TERRACE = 585;
///     TRANSPORT_EXODAR_TO_AUBERDINE = 586;
///     TRANSPORT_FEATHERMOON_FERRY = 587;
///     TRANSPORT_MENETHIL_TO_AUBERDINE = 588;
///     TRANSPORT_ORGRIMMAR_TO_GROM_GOL = 589;
///     TRANSPORT_GROM_GOL_TO_UNDERCITY = 590;
///     TRANSPORT_UNDERCITY_TO_ORGRIMMAR = 591;
///     TRANSPORT_BOOTY_BAY_TO_RATCHET = 593;
///     SUNWELL_FIX_UNUSED = 598;
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
    Outland,
    AhnQirajTemple,
    Karazhan,
    Naxxramas,
    TheBattleForMountHyjal,
    HellfireCitadelTheShatteredHalls,
    HellfireCitadelTheBloodFurnace,
    HellfireCitadelRamparts,
    MagtheridonsLair,
    CoilfangTheSteamvault,
    CoilfangTheUnderbog,
    CoilfangTheSlavePens,
    CoilfangSerpentshrineCavern,
    TempestKeep,
    TempestKeepTheArcatraz,
    TempestKeepTheBotanica,
    TempestKeepTheMechanar,
    AuchindounShadowLabyrinth,
    AuchindounSethekkHalls,
    AuchindounManaTombs,
    AuchindounAuchenaiCrypts,
    NagrandArena,
    TheEscapeFromDurnholde,
    BladesEdgeArena,
    BlackTemple,
    GruulsLair,
    EyeOfTheStorm,
    ZulAman,
    RuinsOfLordaeron,
    TheSunwell,
    TransportRutTheranToAuberdine,
    TransportMenethilToTheramore,
    MagistersTerrace,
    TransportExodarToAuberdine,
    TransportFeathermoonFerry,
    TransportMenethilToAuberdine,
    TransportOrgrimmarToGromGol,
    TransportGromGolToUndercity,
    TransportUndercityToOrgrimmar,
    TransportBootyBayToRatchet,
    SunwellFixUnused,
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
            Self::Outland => 0x212,
            Self::AhnQirajTemple => 0x213,
            Self::Karazhan => 0x214,
            Self::Naxxramas => 0x215,
            Self::TheBattleForMountHyjal => 0x216,
            Self::HellfireCitadelTheShatteredHalls => 0x21c,
            Self::HellfireCitadelTheBloodFurnace => 0x21e,
            Self::HellfireCitadelRamparts => 0x21f,
            Self::MagtheridonsLair => 0x220,
            Self::CoilfangTheSteamvault => 0x221,
            Self::CoilfangTheUnderbog => 0x222,
            Self::CoilfangTheSlavePens => 0x223,
            Self::CoilfangSerpentshrineCavern => 0x224,
            Self::TempestKeep => 0x226,
            Self::TempestKeepTheArcatraz => 0x228,
            Self::TempestKeepTheBotanica => 0x229,
            Self::TempestKeepTheMechanar => 0x22a,
            Self::AuchindounShadowLabyrinth => 0x22b,
            Self::AuchindounSethekkHalls => 0x22c,
            Self::AuchindounManaTombs => 0x22d,
            Self::AuchindounAuchenaiCrypts => 0x22e,
            Self::NagrandArena => 0x22f,
            Self::TheEscapeFromDurnholde => 0x230,
            Self::BladesEdgeArena => 0x232,
            Self::BlackTemple => 0x234,
            Self::GruulsLair => 0x235,
            Self::EyeOfTheStorm => 0x236,
            Self::ZulAman => 0x238,
            Self::RuinsOfLordaeron => 0x23c,
            Self::TheSunwell => 0x244,
            Self::TransportRutTheranToAuberdine => 0x246,
            Self::TransportMenethilToTheramore => 0x248,
            Self::MagistersTerrace => 0x249,
            Self::TransportExodarToAuberdine => 0x24a,
            Self::TransportFeathermoonFerry => 0x24b,
            Self::TransportMenethilToAuberdine => 0x24c,
            Self::TransportOrgrimmarToGromGol => 0x24d,
            Self::TransportGromGolToUndercity => 0x24e,
            Self::TransportUndercityToOrgrimmar => 0x24f,
            Self::TransportBootyBayToRatchet => 0x251,
            Self::SunwellFixUnused => 0x256,
        }
    }

    pub const fn variants() -> [Self; 83] {
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
            Self::Outland,
            Self::AhnQirajTemple,
            Self::Karazhan,
            Self::Naxxramas,
            Self::TheBattleForMountHyjal,
            Self::HellfireCitadelTheShatteredHalls,
            Self::HellfireCitadelTheBloodFurnace,
            Self::HellfireCitadelRamparts,
            Self::MagtheridonsLair,
            Self::CoilfangTheSteamvault,
            Self::CoilfangTheUnderbog,
            Self::CoilfangTheSlavePens,
            Self::CoilfangSerpentshrineCavern,
            Self::TempestKeep,
            Self::TempestKeepTheArcatraz,
            Self::TempestKeepTheBotanica,
            Self::TempestKeepTheMechanar,
            Self::AuchindounShadowLabyrinth,
            Self::AuchindounSethekkHalls,
            Self::AuchindounManaTombs,
            Self::AuchindounAuchenaiCrypts,
            Self::NagrandArena,
            Self::TheEscapeFromDurnholde,
            Self::BladesEdgeArena,
            Self::BlackTemple,
            Self::GruulsLair,
            Self::EyeOfTheStorm,
            Self::ZulAman,
            Self::RuinsOfLordaeron,
            Self::TheSunwell,
            Self::TransportRutTheranToAuberdine,
            Self::TransportMenethilToTheramore,
            Self::MagistersTerrace,
            Self::TransportExodarToAuberdine,
            Self::TransportFeathermoonFerry,
            Self::TransportMenethilToAuberdine,
            Self::TransportOrgrimmarToGromGol,
            Self::TransportGromGolToUndercity,
            Self::TransportUndercityToOrgrimmar,
            Self::TransportBootyBayToRatchet,
            Self::SunwellFixUnused,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
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
            530 => Ok(Self::Outland),
            531 => Ok(Self::AhnQirajTemple),
            532 => Ok(Self::Karazhan),
            533 => Ok(Self::Naxxramas),
            534 => Ok(Self::TheBattleForMountHyjal),
            540 => Ok(Self::HellfireCitadelTheShatteredHalls),
            542 => Ok(Self::HellfireCitadelTheBloodFurnace),
            543 => Ok(Self::HellfireCitadelRamparts),
            544 => Ok(Self::MagtheridonsLair),
            545 => Ok(Self::CoilfangTheSteamvault),
            546 => Ok(Self::CoilfangTheUnderbog),
            547 => Ok(Self::CoilfangTheSlavePens),
            548 => Ok(Self::CoilfangSerpentshrineCavern),
            550 => Ok(Self::TempestKeep),
            552 => Ok(Self::TempestKeepTheArcatraz),
            553 => Ok(Self::TempestKeepTheBotanica),
            554 => Ok(Self::TempestKeepTheMechanar),
            555 => Ok(Self::AuchindounShadowLabyrinth),
            556 => Ok(Self::AuchindounSethekkHalls),
            557 => Ok(Self::AuchindounManaTombs),
            558 => Ok(Self::AuchindounAuchenaiCrypts),
            559 => Ok(Self::NagrandArena),
            560 => Ok(Self::TheEscapeFromDurnholde),
            562 => Ok(Self::BladesEdgeArena),
            564 => Ok(Self::BlackTemple),
            565 => Ok(Self::GruulsLair),
            566 => Ok(Self::EyeOfTheStorm),
            568 => Ok(Self::ZulAman),
            572 => Ok(Self::RuinsOfLordaeron),
            580 => Ok(Self::TheSunwell),
            582 => Ok(Self::TransportRutTheranToAuberdine),
            584 => Ok(Self::TransportMenethilToTheramore),
            585 => Ok(Self::MagistersTerrace),
            586 => Ok(Self::TransportExodarToAuberdine),
            587 => Ok(Self::TransportFeathermoonFerry),
            588 => Ok(Self::TransportMenethilToAuberdine),
            589 => Ok(Self::TransportOrgrimmarToGromGol),
            590 => Ok(Self::TransportGromGolToUndercity),
            591 => Ok(Self::TransportUndercityToOrgrimmar),
            593 => Ok(Self::TransportBootyBayToRatchet),
            598 => Ok(Self::SunwellFixUnused),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
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
            Self::Outland => "OUTLAND",
            Self::AhnQirajTemple => "AHN_QIRAJ_TEMPLE",
            Self::Karazhan => "KARAZHAN",
            Self::Naxxramas => "NAXXRAMAS",
            Self::TheBattleForMountHyjal => "THE_BATTLE_FOR_MOUNT_HYJAL",
            Self::HellfireCitadelTheShatteredHalls => "HELLFIRE_CITADEL_THE_SHATTERED_HALLS",
            Self::HellfireCitadelTheBloodFurnace => "HELLFIRE_CITADEL_THE_BLOOD_FURNACE",
            Self::HellfireCitadelRamparts => "HELLFIRE_CITADEL_RAMPARTS",
            Self::MagtheridonsLair => "MAGTHERIDONS_LAIR",
            Self::CoilfangTheSteamvault => "COILFANG_THE_STEAMVAULT",
            Self::CoilfangTheUnderbog => "COILFANG_THE_UNDERBOG",
            Self::CoilfangTheSlavePens => "COILFANG_THE_SLAVE_PENS",
            Self::CoilfangSerpentshrineCavern => "COILFANG_SERPENTSHRINE_CAVERN",
            Self::TempestKeep => "TEMPEST_KEEP",
            Self::TempestKeepTheArcatraz => "TEMPEST_KEEP_THE_ARCATRAZ",
            Self::TempestKeepTheBotanica => "TEMPEST_KEEP_THE_BOTANICA",
            Self::TempestKeepTheMechanar => "TEMPEST_KEEP_THE_MECHANAR",
            Self::AuchindounShadowLabyrinth => "AUCHINDOUN_SHADOW_LABYRINTH",
            Self::AuchindounSethekkHalls => "AUCHINDOUN_SETHEKK_HALLS",
            Self::AuchindounManaTombs => "AUCHINDOUN_MANA_TOMBS",
            Self::AuchindounAuchenaiCrypts => "AUCHINDOUN_AUCHENAI_CRYPTS",
            Self::NagrandArena => "NAGRAND_ARENA",
            Self::TheEscapeFromDurnholde => "THE_ESCAPE_FROM_DURNHOLDE",
            Self::BladesEdgeArena => "BLADES_EDGE_ARENA",
            Self::BlackTemple => "BLACK_TEMPLE",
            Self::GruulsLair => "GRUULS_LAIR",
            Self::EyeOfTheStorm => "EYE_OF_THE_STORM",
            Self::ZulAman => "ZUL_AMAN",
            Self::RuinsOfLordaeron => "RUINS_OF_LORDAERON",
            Self::TheSunwell => "THE_SUNWELL",
            Self::TransportRutTheranToAuberdine => "TRANSPORT_RUT_THERAN_TO_AUBERDINE",
            Self::TransportMenethilToTheramore => "TRANSPORT_MENETHIL_TO_THERAMORE",
            Self::MagistersTerrace => "MAGISTERS_TERRACE",
            Self::TransportExodarToAuberdine => "TRANSPORT_EXODAR_TO_AUBERDINE",
            Self::TransportFeathermoonFerry => "TRANSPORT_FEATHERMOON_FERRY",
            Self::TransportMenethilToAuberdine => "TRANSPORT_MENETHIL_TO_AUBERDINE",
            Self::TransportOrgrimmarToGromGol => "TRANSPORT_ORGRIMMAR_TO_GROM_GOL",
            Self::TransportGromGolToUndercity => "TRANSPORT_GROM_GOL_TO_UNDERCITY",
            Self::TransportUndercityToOrgrimmar => "TRANSPORT_UNDERCITY_TO_ORGRIMMAR",
            Self::TransportBootyBayToRatchet => "TRANSPORT_BOOTY_BAY_TO_RATCHET",
            Self::SunwellFixUnused => "SUNWELL_FIX_UNUSED",
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
        f.write_str(match self {
            Self::EasternKingdoms => "Eastern Kingdoms",
            Self::Kalimdor => "Kalimdor",
            Self::Testing => "Testing",
            Self::ScottTest => "Scott Test",
            Self::CashTest => "CashTest",
            Self::AlteracValley => "Alterac Valley",
            Self::ShadowfangKeep => "Shadowfang Keep",
            Self::StormwindStockade => "Stormwind Stockade",
            Self::StormwindPrison => "Stormwind Prison",
            Self::Deadmines => "Deadmines",
            Self::AzsharaCrater => "Azshara Crater",
            Self::CollinsTest => "Collin's Test",
            Self::WailingCaverns => "Wailing Caverns",
            Self::MonasteryUnused => "<Unused> Monastery",
            Self::RazorfenKraul => "Razorfen Kraul",
            Self::BlackfathomDeeps => "Blackfathom Deeps",
            Self::Uldaman => "Uldaman",
            Self::Gnomeregan => "Gnomeregan",
            Self::SunkenTemple => "Sunken Temple",
            Self::RazorfenDowns => "Razorfen Downs",
            Self::EmeraldDream => "Emerald Dream",
            Self::ScarletMonastery => "Scarlet Monastery",
            Self::ZulFarrak => "Zul'Farrak",
            Self::BlackrockSpire => "Blackrock Spire",
            Self::BlackrockDepths => "Blackrock Depths",
            Self::OnyxiasLair => "Onyxia's Lair",
            Self::OpeningOfTheDarkPortal => "Opening of the Dark Portal",
            Self::Scholomance => "Scholomance",
            Self::ZulGurub => "Zul'Gurub",
            Self::Stratholme => "Stratholme",
            Self::Maraudon => "Maraudon",
            Self::DeeprunTram => "Deeprun Tram",
            Self::RagefireChasm => "Ragefire Chasm",
            Self::MoltenCore => "Molten Core",
            Self::DireMaul => "Dire Maul",
            Self::AlliancePvpBarracks => "Alliance PVP Barracks",
            Self::HordePvpBarracks => "Horde PVP Barracks",
            Self::DevelopmentLand => "Development Land",
            Self::BlackwingLair => "Blackwing Lair",
            Self::WarsongGulch => "Warsong Gulch",
            Self::RuinsOfAhnQiraj => "Ruins of Ahn'Qiraj",
            Self::ArathiBasin => "Arathi Basin",
            Self::Outland => "Outland",
            Self::AhnQirajTemple => "Ahn'Qiraj Temple",
            Self::Karazhan => "Karazhan",
            Self::Naxxramas => "Naxxramas",
            Self::TheBattleForMountHyjal => "The Battle for Mount Hyjal",
            Self::HellfireCitadelTheShatteredHalls => "Hellfire Citadel: The Shattered Halls",
            Self::HellfireCitadelTheBloodFurnace => "Hellfire Citadel: The Blood Furnace",
            Self::HellfireCitadelRamparts => "Hellfire Citadel: Ramparts",
            Self::MagtheridonsLair => "Magtheridon's Lair",
            Self::CoilfangTheSteamvault => "Coilfang: The Steamvault",
            Self::CoilfangTheUnderbog => "Coilfang: The Underbog",
            Self::CoilfangTheSlavePens => "Coilfang: The Slave Pens",
            Self::CoilfangSerpentshrineCavern => "Coilfang: Serpentshrine Cavern",
            Self::TempestKeep => "Tempest Keep",
            Self::TempestKeepTheArcatraz => "Tempest Keep: The Arcatraz",
            Self::TempestKeepTheBotanica => "Tempest Keep: The Botanica",
            Self::TempestKeepTheMechanar => "Tempest Keep: The Mechanar",
            Self::AuchindounShadowLabyrinth => "Auchindoun: Shadow Labyrinth",
            Self::AuchindounSethekkHalls => "Auchindoun: Sethekk Halls",
            Self::AuchindounManaTombs => "Auchindoun: Mana-Tombs",
            Self::AuchindounAuchenaiCrypts => "Auchindoun: Auchenai Crypts",
            Self::NagrandArena => "Nagrand Arena",
            Self::TheEscapeFromDurnholde => "The Escape From Durnholde",
            Self::BladesEdgeArena => "Blade's Edge Arena",
            Self::BlackTemple => "Black Temple",
            Self::GruulsLair => "Gruul's Lair",
            Self::EyeOfTheStorm => "Eye of the Storm",
            Self::ZulAman => "Zul'Aman",
            Self::RuinsOfLordaeron => "Ruins of Lordaeron",
            Self::TheSunwell => "The Sunwell",
            Self::TransportRutTheranToAuberdine => "Transport: Rut'theran to Auberdine",
            Self::TransportMenethilToTheramore => "Transport: Menethil to Theramore",
            Self::MagistersTerrace => "Magister's Terrace",
            Self::TransportExodarToAuberdine => "Transport: Exodar to Auberdine",
            Self::TransportFeathermoonFerry => "Transport: Feathermoon Ferry",
            Self::TransportMenethilToAuberdine => "Transport: Menethil to Auberdine",
            Self::TransportOrgrimmarToGromGol => "Transport: Orgrimmar to Grom'Gol",
            Self::TransportGromGolToUndercity => "Transport: Grom'Gol to Undercity",
            Self::TransportUndercityToOrgrimmar => "Transport: Undercity to Orgrimmar",
            Self::TransportBootyBayToRatchet => "Transport: Booty Bay to Ratchet",
            Self::SunwellFixUnused => "Sunwell Fix (Unused)",
        })
    }
}

impl TryFrom<u32> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for Map {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
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
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
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

