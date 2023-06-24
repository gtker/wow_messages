/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/map.wowm:395`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/map.wowm#L395):
/// ```text
/// enum Map : u32 {
///     EASTERN_KINGDOMS = 0;
///     KALIMDOR = 1;
///     TESTING = 13;
///     SCOTT_TEST = 25;
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
///     NORTHREND = 571;
///     RUINS_OF_LORDAERON = 572;
///     EXTERIORTEST = 573;
///     UTGARDE_KEEP = 574;
///     UTGARDE_PINNACLE = 575;
///     THE_NEXUS = 576;
///     THE_OCULUS = 578;
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
///     TRANSPORT_BOREAN_TUNDRA_TEST = 592;
///     TRANSPORT_BOOTY_BAY_TO_RATCHET = 593;
///     TRANSPORT_HOWLING_FJORD_SISTER_MERCY_QUEST = 594;
///     THE_CULLING_OF_STRATHOLME = 595;
///     TRANSPORT_NAGLFAR = 596;
///     CRAIG_TEST = 597;
///     SUNWELL_FIX_UNUSED = 598;
///     HALLS_OF_STONE = 599;
///     DRAK_THARON_KEEP = 600;
///     AZJOL_NERUB = 601;
///     HALLS_OF_LIGHTNING = 602;
///     ULDUAR = 603;
///     GUNDRAK = 604;
///     DEVELOPMENT_LAND_NON_WEIGHTED_TEXTURES = 605;
///     QA_AND_DVD = 606;
///     STRAND_OF_THE_ANCIENTS = 607;
///     VIOLET_HOLD = 608;
///     EBON_HOLD = 609;
///     TRANSPORT_TIRISFAL_TO_VENGEANCE_LANDING = 610;
///     TRANSPORT_MENETHIL_TO_VALGARDE = 612;
///     TRANSPORT_ORGRIMMAR_TO_WARSONG_HOLD = 613;
///     TRANSPORT_STORMWIND_TO_VALIANCE_KEEP = 614;
///     THE_OBSIDIAN_SANCTUM = 615;
///     THE_EYE_OF_ETERNITY = 616;
///     DALARAN_SEWERS = 617;
///     THE_RING_OF_VALOR = 618;
///     AHN_KAHET_THE_OLD_KINGDOM = 619;
///     TRANSPORT_MOA_KI_TO_UNU_PE = 620;
///     TRANSPORT_MOA_KI_TO_KAMAGUA = 621;
///     TRANSPORT_ORGRIMS_HAMMER = 622;
///     TRANSPORT_THE_SKYBREAKER = 623;
///     VAULT_OF_ARCHAVON = 624;
///     ISLE_OF_CONQUEST = 628;
///     ICECROWN_CITADEL = 631;
///     THE_FORGE_OF_SOULS = 632;
///     TRANSPORT_ALLIANCE_AIRSHIP_BG = 641;
///     TRANSPORT_HORDEAIRSHIPBG = 642;
///     TRANSPORT_ORGRIMMAR_TO_THUNDER_BLUFF = 647;
///     TRIAL_OF_THE_CRUSADER = 649;
///     TRIAL_OF_THE_CHAMPION = 650;
///     PIT_OF_SARON = 658;
///     HALLS_OF_REFLECTION = 668;
///     TRANSPORT_THE_SKYBREAKER_ICECROWN_CITADEL_RAID = 672;
///     TRANSPORT_ORGRIMS_HAMMER_ICECROWN_CITADEL_RAID = 673;
///     TRANSPORT_THE_SKYBREAKER_IC_DUNGEON = 712;
///     TRANSPORT_ORGRIMS_HAMMER_IC_DUNGEON = 713;
///     TRANSPORT_THE_MIGHTY_WIND_ICECROWN_CITADEL_RAID = 718;
///     STORMWIND = 723;
///     THE_RUBY_SANCTUM = 724;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Map {
    EasternKingdoms,
    Kalimdor,
    Testing,
    ScottTest,
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
    /// Map files not present on 3.3.5 client, so loading the map will crash the client.
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
    Northrend,
    RuinsOfLordaeron,
    Exteriortest,
    UtgardeKeep,
    UtgardePinnacle,
    TheNexus,
    TheOculus,
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
    TransportBoreanTundraTest,
    TransportBootyBayToRatchet,
    TransportHowlingFjordSisterMercyQuest,
    TheCullingOfStratholme,
    TransportNaglfar,
    CraigTest,
    SunwellFixUnused,
    HallsOfStone,
    DrakTharonKeep,
    AzjolNerub,
    HallsOfLightning,
    Ulduar,
    Gundrak,
    DevelopmentLandNonWeightedTextures,
    QaAndDvd,
    StrandOfTheAncients,
    VioletHold,
    EbonHold,
    TransportTirisfalToVengeanceLanding,
    TransportMenethilToValgarde,
    TransportOrgrimmarToWarsongHold,
    TransportStormwindToValianceKeep,
    TheObsidianSanctum,
    TheEyeOfEternity,
    DalaranSewers,
    TheRingOfValor,
    AhnKahetTheOldKingdom,
    TransportMoaKiToUnuPe,
    TransportMoaKiToKamagua,
    TransportOrgrimsHammer,
    TransportTheSkybreaker,
    VaultOfArchavon,
    IsleOfConquest,
    IcecrownCitadel,
    TheForgeOfSouls,
    TransportAllianceAirshipBg,
    TransportHordeairshipbg,
    TransportOrgrimmarToThunderBluff,
    TrialOfTheCrusader,
    TrialOfTheChampion,
    PitOfSaron,
    HallsOfReflection,
    TransportTheSkybreakerIcecrownCitadelRaid,
    TransportOrgrimsHammerIcecrownCitadelRaid,
    TransportTheSkybreakerIcDungeon,
    TransportOrgrimsHammerIcDungeon,
    TransportTheMightyWindIcecrownCitadelRaid,
    Stormwind,
    TheRubySanctum,
}

impl Map {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::EasternKingdoms => 0x0,
            Self::Kalimdor => 0x1,
            Self::Testing => 0xd,
            Self::ScottTest => 0x19,
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
            Self::Northrend => 0x23b,
            Self::RuinsOfLordaeron => 0x23c,
            Self::Exteriortest => 0x23d,
            Self::UtgardeKeep => 0x23e,
            Self::UtgardePinnacle => 0x23f,
            Self::TheNexus => 0x240,
            Self::TheOculus => 0x242,
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
            Self::TransportBoreanTundraTest => 0x250,
            Self::TransportBootyBayToRatchet => 0x251,
            Self::TransportHowlingFjordSisterMercyQuest => 0x252,
            Self::TheCullingOfStratholme => 0x253,
            Self::TransportNaglfar => 0x254,
            Self::CraigTest => 0x255,
            Self::SunwellFixUnused => 0x256,
            Self::HallsOfStone => 0x257,
            Self::DrakTharonKeep => 0x258,
            Self::AzjolNerub => 0x259,
            Self::HallsOfLightning => 0x25a,
            Self::Ulduar => 0x25b,
            Self::Gundrak => 0x25c,
            Self::DevelopmentLandNonWeightedTextures => 0x25d,
            Self::QaAndDvd => 0x25e,
            Self::StrandOfTheAncients => 0x25f,
            Self::VioletHold => 0x260,
            Self::EbonHold => 0x261,
            Self::TransportTirisfalToVengeanceLanding => 0x262,
            Self::TransportMenethilToValgarde => 0x264,
            Self::TransportOrgrimmarToWarsongHold => 0x265,
            Self::TransportStormwindToValianceKeep => 0x266,
            Self::TheObsidianSanctum => 0x267,
            Self::TheEyeOfEternity => 0x268,
            Self::DalaranSewers => 0x269,
            Self::TheRingOfValor => 0x26a,
            Self::AhnKahetTheOldKingdom => 0x26b,
            Self::TransportMoaKiToUnuPe => 0x26c,
            Self::TransportMoaKiToKamagua => 0x26d,
            Self::TransportOrgrimsHammer => 0x26e,
            Self::TransportTheSkybreaker => 0x26f,
            Self::VaultOfArchavon => 0x270,
            Self::IsleOfConquest => 0x274,
            Self::IcecrownCitadel => 0x277,
            Self::TheForgeOfSouls => 0x278,
            Self::TransportAllianceAirshipBg => 0x281,
            Self::TransportHordeairshipbg => 0x282,
            Self::TransportOrgrimmarToThunderBluff => 0x287,
            Self::TrialOfTheCrusader => 0x289,
            Self::TrialOfTheChampion => 0x28a,
            Self::PitOfSaron => 0x292,
            Self::HallsOfReflection => 0x29c,
            Self::TransportTheSkybreakerIcecrownCitadelRaid => 0x2a0,
            Self::TransportOrgrimsHammerIcecrownCitadelRaid => 0x2a1,
            Self::TransportTheSkybreakerIcDungeon => 0x2c8,
            Self::TransportOrgrimsHammerIcDungeon => 0x2c9,
            Self::TransportTheMightyWindIcecrownCitadelRaid => 0x2ce,
            Self::Stormwind => 0x2d3,
            Self::TheRubySanctum => 0x2d4,
        }
    }

    pub const fn variants() -> [Self; 135] {
        [
            Self::EasternKingdoms,
            Self::Kalimdor,
            Self::Testing,
            Self::ScottTest,
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
            Self::Northrend,
            Self::RuinsOfLordaeron,
            Self::Exteriortest,
            Self::UtgardeKeep,
            Self::UtgardePinnacle,
            Self::TheNexus,
            Self::TheOculus,
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
            Self::TransportBoreanTundraTest,
            Self::TransportBootyBayToRatchet,
            Self::TransportHowlingFjordSisterMercyQuest,
            Self::TheCullingOfStratholme,
            Self::TransportNaglfar,
            Self::CraigTest,
            Self::SunwellFixUnused,
            Self::HallsOfStone,
            Self::DrakTharonKeep,
            Self::AzjolNerub,
            Self::HallsOfLightning,
            Self::Ulduar,
            Self::Gundrak,
            Self::DevelopmentLandNonWeightedTextures,
            Self::QaAndDvd,
            Self::StrandOfTheAncients,
            Self::VioletHold,
            Self::EbonHold,
            Self::TransportTirisfalToVengeanceLanding,
            Self::TransportMenethilToValgarde,
            Self::TransportOrgrimmarToWarsongHold,
            Self::TransportStormwindToValianceKeep,
            Self::TheObsidianSanctum,
            Self::TheEyeOfEternity,
            Self::DalaranSewers,
            Self::TheRingOfValor,
            Self::AhnKahetTheOldKingdom,
            Self::TransportMoaKiToUnuPe,
            Self::TransportMoaKiToKamagua,
            Self::TransportOrgrimsHammer,
            Self::TransportTheSkybreaker,
            Self::VaultOfArchavon,
            Self::IsleOfConquest,
            Self::IcecrownCitadel,
            Self::TheForgeOfSouls,
            Self::TransportAllianceAirshipBg,
            Self::TransportHordeairshipbg,
            Self::TransportOrgrimmarToThunderBluff,
            Self::TrialOfTheCrusader,
            Self::TrialOfTheChampion,
            Self::PitOfSaron,
            Self::HallsOfReflection,
            Self::TransportTheSkybreakerIcecrownCitadelRaid,
            Self::TransportOrgrimsHammerIcecrownCitadelRaid,
            Self::TransportTheSkybreakerIcDungeon,
            Self::TransportOrgrimsHammerIcDungeon,
            Self::TransportTheMightyWindIcecrownCitadelRaid,
            Self::Stormwind,
            Self::TheRubySanctum,
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
            Self::Northrend => "NORTHREND",
            Self::RuinsOfLordaeron => "RUINS_OF_LORDAERON",
            Self::Exteriortest => "EXTERIORTEST",
            Self::UtgardeKeep => "UTGARDE_KEEP",
            Self::UtgardePinnacle => "UTGARDE_PINNACLE",
            Self::TheNexus => "THE_NEXUS",
            Self::TheOculus => "THE_OCULUS",
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
            Self::TransportBoreanTundraTest => "TRANSPORT_BOREAN_TUNDRA_TEST",
            Self::TransportBootyBayToRatchet => "TRANSPORT_BOOTY_BAY_TO_RATCHET",
            Self::TransportHowlingFjordSisterMercyQuest => "TRANSPORT_HOWLING_FJORD_SISTER_MERCY_QUEST",
            Self::TheCullingOfStratholme => "THE_CULLING_OF_STRATHOLME",
            Self::TransportNaglfar => "TRANSPORT_NAGLFAR",
            Self::CraigTest => "CRAIG_TEST",
            Self::SunwellFixUnused => "SUNWELL_FIX_UNUSED",
            Self::HallsOfStone => "HALLS_OF_STONE",
            Self::DrakTharonKeep => "DRAK_THARON_KEEP",
            Self::AzjolNerub => "AZJOL_NERUB",
            Self::HallsOfLightning => "HALLS_OF_LIGHTNING",
            Self::Ulduar => "ULDUAR",
            Self::Gundrak => "GUNDRAK",
            Self::DevelopmentLandNonWeightedTextures => "DEVELOPMENT_LAND_NON_WEIGHTED_TEXTURES",
            Self::QaAndDvd => "QA_AND_DVD",
            Self::StrandOfTheAncients => "STRAND_OF_THE_ANCIENTS",
            Self::VioletHold => "VIOLET_HOLD",
            Self::EbonHold => "EBON_HOLD",
            Self::TransportTirisfalToVengeanceLanding => "TRANSPORT_TIRISFAL_TO_VENGEANCE_LANDING",
            Self::TransportMenethilToValgarde => "TRANSPORT_MENETHIL_TO_VALGARDE",
            Self::TransportOrgrimmarToWarsongHold => "TRANSPORT_ORGRIMMAR_TO_WARSONG_HOLD",
            Self::TransportStormwindToValianceKeep => "TRANSPORT_STORMWIND_TO_VALIANCE_KEEP",
            Self::TheObsidianSanctum => "THE_OBSIDIAN_SANCTUM",
            Self::TheEyeOfEternity => "THE_EYE_OF_ETERNITY",
            Self::DalaranSewers => "DALARAN_SEWERS",
            Self::TheRingOfValor => "THE_RING_OF_VALOR",
            Self::AhnKahetTheOldKingdom => "AHN_KAHET_THE_OLD_KINGDOM",
            Self::TransportMoaKiToUnuPe => "TRANSPORT_MOA_KI_TO_UNU_PE",
            Self::TransportMoaKiToKamagua => "TRANSPORT_MOA_KI_TO_KAMAGUA",
            Self::TransportOrgrimsHammer => "TRANSPORT_ORGRIMS_HAMMER",
            Self::TransportTheSkybreaker => "TRANSPORT_THE_SKYBREAKER",
            Self::VaultOfArchavon => "VAULT_OF_ARCHAVON",
            Self::IsleOfConquest => "ISLE_OF_CONQUEST",
            Self::IcecrownCitadel => "ICECROWN_CITADEL",
            Self::TheForgeOfSouls => "THE_FORGE_OF_SOULS",
            Self::TransportAllianceAirshipBg => "TRANSPORT_ALLIANCE_AIRSHIP_BG",
            Self::TransportHordeairshipbg => "TRANSPORT_HORDEAIRSHIPBG",
            Self::TransportOrgrimmarToThunderBluff => "TRANSPORT_ORGRIMMAR_TO_THUNDER_BLUFF",
            Self::TrialOfTheCrusader => "TRIAL_OF_THE_CRUSADER",
            Self::TrialOfTheChampion => "TRIAL_OF_THE_CHAMPION",
            Self::PitOfSaron => "PIT_OF_SARON",
            Self::HallsOfReflection => "HALLS_OF_REFLECTION",
            Self::TransportTheSkybreakerIcecrownCitadelRaid => "TRANSPORT_THE_SKYBREAKER_ICECROWN_CITADEL_RAID",
            Self::TransportOrgrimsHammerIcecrownCitadelRaid => "TRANSPORT_ORGRIMS_HAMMER_ICECROWN_CITADEL_RAID",
            Self::TransportTheSkybreakerIcDungeon => "TRANSPORT_THE_SKYBREAKER_IC_DUNGEON",
            Self::TransportOrgrimsHammerIcDungeon => "TRANSPORT_ORGRIMS_HAMMER_IC_DUNGEON",
            Self::TransportTheMightyWindIcecrownCitadelRaid => "TRANSPORT_THE_MIGHTY_WIND_ICECROWN_CITADEL_RAID",
            Self::Stormwind => "STORMWIND",
            Self::TheRubySanctum => "THE_RUBY_SANCTUM",
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
            Self::Outland => f.write_str("Outland"),
            Self::AhnQirajTemple => f.write_str("Ahn'Qiraj Temple"),
            Self::Karazhan => f.write_str("Karazhan"),
            Self::Naxxramas => f.write_str("Naxxramas"),
            Self::TheBattleForMountHyjal => f.write_str("The Battle for Mount Hyjal"),
            Self::HellfireCitadelTheShatteredHalls => f.write_str("Hellfire Citadel: The Shattered Halls"),
            Self::HellfireCitadelTheBloodFurnace => f.write_str("Hellfire Citadel: The Blood Furnace"),
            Self::HellfireCitadelRamparts => f.write_str("Hellfire Citadel: Ramparts"),
            Self::MagtheridonsLair => f.write_str("Magtheridon's Lair"),
            Self::CoilfangTheSteamvault => f.write_str("Coilfang: The Steamvault"),
            Self::CoilfangTheUnderbog => f.write_str("Coilfang: The Underbog"),
            Self::CoilfangTheSlavePens => f.write_str("Coilfang: The Slave Pens"),
            Self::CoilfangSerpentshrineCavern => f.write_str("Coilfang: Serpentshrine Cavern"),
            Self::TempestKeep => f.write_str("Tempest Keep"),
            Self::TempestKeepTheArcatraz => f.write_str("Tempest Keep: The Arcatraz"),
            Self::TempestKeepTheBotanica => f.write_str("Tempest Keep: The Botanica"),
            Self::TempestKeepTheMechanar => f.write_str("Tempest Keep: The Mechanar"),
            Self::AuchindounShadowLabyrinth => f.write_str("Auchindoun: Shadow Labyrinth"),
            Self::AuchindounSethekkHalls => f.write_str("Auchindoun: Sethekk Halls"),
            Self::AuchindounManaTombs => f.write_str("Auchindoun: Mana-Tombs"),
            Self::AuchindounAuchenaiCrypts => f.write_str("Auchindoun: Auchenai Crypts"),
            Self::NagrandArena => f.write_str("Nagrand Arena"),
            Self::TheEscapeFromDurnholde => f.write_str("The Escape From Durnholde"),
            Self::BladesEdgeArena => f.write_str("Blade's Edge Arena"),
            Self::BlackTemple => f.write_str("Black Temple"),
            Self::GruulsLair => f.write_str("Gruul's Lair"),
            Self::EyeOfTheStorm => f.write_str("Eye of the Storm"),
            Self::ZulAman => f.write_str("Zul'Aman"),
            Self::Northrend => f.write_str("Northrend"),
            Self::RuinsOfLordaeron => f.write_str("Ruins of Lordaeron"),
            Self::Exteriortest => f.write_str("ExteriorTest"),
            Self::UtgardeKeep => f.write_str("Utgarde Keep"),
            Self::UtgardePinnacle => f.write_str("Utgarde Pinnacle"),
            Self::TheNexus => f.write_str("The Nexus"),
            Self::TheOculus => f.write_str("The Oculus"),
            Self::TheSunwell => f.write_str("The Sunwell"),
            Self::TransportRutTheranToAuberdine => f.write_str("Transport: Rut'theran to Auberdine"),
            Self::TransportMenethilToTheramore => f.write_str("Transport: Menethil to Theramore"),
            Self::MagistersTerrace => f.write_str("Magister's Terrace"),
            Self::TransportExodarToAuberdine => f.write_str("Transport: Exodar to Auberdine"),
            Self::TransportFeathermoonFerry => f.write_str("Transport: Feathermoon Ferry"),
            Self::TransportMenethilToAuberdine => f.write_str("Transport: Menethil to Auberdine"),
            Self::TransportOrgrimmarToGromGol => f.write_str("Transport: Orgrimmar to Grom'Gol"),
            Self::TransportGromGolToUndercity => f.write_str("Transport: Grom'Gol to Undercity"),
            Self::TransportUndercityToOrgrimmar => f.write_str("Transport: Undercity to Orgrimmar"),
            Self::TransportBoreanTundraTest => f.write_str("Transport: Borean Tundra Test"),
            Self::TransportBootyBayToRatchet => f.write_str("Transport: Booty Bay to Ratchet"),
            Self::TransportHowlingFjordSisterMercyQuest => f.write_str("Transport: Howling Fjord Sister Mercy (Quest)"),
            Self::TheCullingOfStratholme => f.write_str("The Culling of Stratholme"),
            Self::TransportNaglfar => f.write_str("Transport: Naglfar"),
            Self::CraigTest => f.write_str("Craig Test"),
            Self::SunwellFixUnused => f.write_str("Sunwell Fix (Unused)"),
            Self::HallsOfStone => f.write_str("Halls of Stone"),
            Self::DrakTharonKeep => f.write_str("Drak'Tharon Keep"),
            Self::AzjolNerub => f.write_str("Azjol-Nerub"),
            Self::HallsOfLightning => f.write_str("Halls of Lightning"),
            Self::Ulduar => f.write_str("Ulduar"),
            Self::Gundrak => f.write_str("Gundrak"),
            Self::DevelopmentLandNonWeightedTextures => f.write_str("Development Land (non-weighted textures)"),
            Self::QaAndDvd => f.write_str("QA and DVD"),
            Self::StrandOfTheAncients => f.write_str("Strand of the Ancients"),
            Self::VioletHold => f.write_str("Violet Hold"),
            Self::EbonHold => f.write_str("Ebon Hold"),
            Self::TransportTirisfalToVengeanceLanding => f.write_str("Transport: Tirisfal to Vengeance Landing"),
            Self::TransportMenethilToValgarde => f.write_str("Transport: Menethil to Valgarde"),
            Self::TransportOrgrimmarToWarsongHold => f.write_str("Transport: Orgrimmar to Warsong Hold"),
            Self::TransportStormwindToValianceKeep => f.write_str("Transport: Stormwind to Valiance Keep"),
            Self::TheObsidianSanctum => f.write_str("The Obsidian Sanctum"),
            Self::TheEyeOfEternity => f.write_str("The Eye of Eternity"),
            Self::DalaranSewers => f.write_str("Dalaran Sewers"),
            Self::TheRingOfValor => f.write_str("The Ring of Valor"),
            Self::AhnKahetTheOldKingdom => f.write_str("Ahn'kahet: The Old Kingdom"),
            Self::TransportMoaKiToUnuPe => f.write_str("Transport: Moa'ki to Unu'pe"),
            Self::TransportMoaKiToKamagua => f.write_str("Transport: Moa'ki to Kamagua"),
            Self::TransportOrgrimsHammer => f.write_str("Transport: Orgrim's Hammer"),
            Self::TransportTheSkybreaker => f.write_str("Transport: The Skybreaker"),
            Self::VaultOfArchavon => f.write_str("Vault of Archavon"),
            Self::IsleOfConquest => f.write_str("Isle of Conquest"),
            Self::IcecrownCitadel => f.write_str("Icecrown Citadel"),
            Self::TheForgeOfSouls => f.write_str("The Forge of Souls"),
            Self::TransportAllianceAirshipBg => f.write_str("Transport: Alliance Airship BG"),
            Self::TransportHordeairshipbg => f.write_str("Transport: HordeAirshipBG"),
            Self::TransportOrgrimmarToThunderBluff => f.write_str("Transport: Orgrimmar to Thunder Bluff"),
            Self::TrialOfTheCrusader => f.write_str("Trial of the Crusader"),
            Self::TrialOfTheChampion => f.write_str("Trial of the Champion"),
            Self::PitOfSaron => f.write_str("Pit of Saron"),
            Self::HallsOfReflection => f.write_str("Halls of Reflection"),
            Self::TransportTheSkybreakerIcecrownCitadelRaid => f.write_str("Transport: The Skybreaker (Icecrown Citadel Raid)"),
            Self::TransportOrgrimsHammerIcecrownCitadelRaid => f.write_str("Transport: Orgrim's Hammer (Icecrown Citadel Raid)"),
            Self::TransportTheSkybreakerIcDungeon => f.write_str("Transport: The Skybreaker (IC Dungeon)"),
            Self::TransportOrgrimsHammerIcDungeon => f.write_str("Transport: Orgrim's Hammer (IC Dungeon)"),
            Self::TransportTheMightyWindIcecrownCitadelRaid => f.write_str("Transport: The Mighty Wind (Icecrown Citadel Raid)"),
            Self::Stormwind => f.write_str("Stormwind"),
            Self::TheRubySanctum => f.write_str("The Ruby Sanctum"),
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
            571 => Ok(Self::Northrend),
            572 => Ok(Self::RuinsOfLordaeron),
            573 => Ok(Self::Exteriortest),
            574 => Ok(Self::UtgardeKeep),
            575 => Ok(Self::UtgardePinnacle),
            576 => Ok(Self::TheNexus),
            578 => Ok(Self::TheOculus),
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
            592 => Ok(Self::TransportBoreanTundraTest),
            593 => Ok(Self::TransportBootyBayToRatchet),
            594 => Ok(Self::TransportHowlingFjordSisterMercyQuest),
            595 => Ok(Self::TheCullingOfStratholme),
            596 => Ok(Self::TransportNaglfar),
            597 => Ok(Self::CraigTest),
            598 => Ok(Self::SunwellFixUnused),
            599 => Ok(Self::HallsOfStone),
            600 => Ok(Self::DrakTharonKeep),
            601 => Ok(Self::AzjolNerub),
            602 => Ok(Self::HallsOfLightning),
            603 => Ok(Self::Ulduar),
            604 => Ok(Self::Gundrak),
            605 => Ok(Self::DevelopmentLandNonWeightedTextures),
            606 => Ok(Self::QaAndDvd),
            607 => Ok(Self::StrandOfTheAncients),
            608 => Ok(Self::VioletHold),
            609 => Ok(Self::EbonHold),
            610 => Ok(Self::TransportTirisfalToVengeanceLanding),
            612 => Ok(Self::TransportMenethilToValgarde),
            613 => Ok(Self::TransportOrgrimmarToWarsongHold),
            614 => Ok(Self::TransportStormwindToValianceKeep),
            615 => Ok(Self::TheObsidianSanctum),
            616 => Ok(Self::TheEyeOfEternity),
            617 => Ok(Self::DalaranSewers),
            618 => Ok(Self::TheRingOfValor),
            619 => Ok(Self::AhnKahetTheOldKingdom),
            620 => Ok(Self::TransportMoaKiToUnuPe),
            621 => Ok(Self::TransportMoaKiToKamagua),
            622 => Ok(Self::TransportOrgrimsHammer),
            623 => Ok(Self::TransportTheSkybreaker),
            624 => Ok(Self::VaultOfArchavon),
            628 => Ok(Self::IsleOfConquest),
            631 => Ok(Self::IcecrownCitadel),
            632 => Ok(Self::TheForgeOfSouls),
            641 => Ok(Self::TransportAllianceAirshipBg),
            642 => Ok(Self::TransportHordeairshipbg),
            647 => Ok(Self::TransportOrgrimmarToThunderBluff),
            649 => Ok(Self::TrialOfTheCrusader),
            650 => Ok(Self::TrialOfTheChampion),
            658 => Ok(Self::PitOfSaron),
            668 => Ok(Self::HallsOfReflection),
            672 => Ok(Self::TransportTheSkybreakerIcecrownCitadelRaid),
            673 => Ok(Self::TransportOrgrimsHammerIcecrownCitadelRaid),
            712 => Ok(Self::TransportTheSkybreakerIcDungeon),
            713 => Ok(Self::TransportOrgrimsHammerIcDungeon),
            718 => Ok(Self::TransportTheMightyWindIcecrownCitadelRaid),
            723 => Ok(Self::Stormwind),
            724 => Ok(Self::TheRubySanctum),
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

