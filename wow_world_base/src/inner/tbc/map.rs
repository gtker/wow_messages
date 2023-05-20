/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/map.wowm:140`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/map.wowm#L140):
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

}

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
            Self::RuinsOfLordaeron => f.write_str("Ruins of Lordaeron"),
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
            Self::TransportBootyBayToRatchet => f.write_str("Transport: Booty Bay to Ratchet"),
            Self::SunwellFixUnused => f.write_str("Sunwell Fix (Unused)"),
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
            v => Err(crate::errors::EnumError::new("Map", v as u64),)
        }
    }
}

