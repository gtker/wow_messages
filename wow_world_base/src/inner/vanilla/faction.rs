/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/faction.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/faction.wowm#L1):
/// ```text
/// enum Faction : u16 {
///     NONE = 0;
///     PLAYER_HUMAN = 1;
///     PLAYER_ORC = 2;
///     PLAYER_DWARF = 3;
///     PLAYER_NIGHT_ELF = 4;
///     PLAYER_UNDEAD = 5;
///     PLAYER_TAUREN = 6;
///     CREATURE = 7;
///     PLAYER_GNOME = 8;
///     PLAYER_TROLL = 9;
///     MONSTER = 14;
///     DEFIAS_BROTHERHOOD = 15;
///     GNOLL_RIVERPAW = 16;
///     GNOLL_REDRIDGE = 17;
///     GNOLL_SHADOWHIDE = 18;
///     MURLOC = 19;
///     UNDEAD_SCOURGE = 20;
///     BOOTY_BAY = 21;
///     BEAST_SPIDER = 22;
///     BEAST_BOAR = 23;
///     WORGEN = 24;
///     KOBOLD = 25;
///     TROLL_BLOODSCALP = 26;
///     TROLL_SKULLSPLITTER = 27;
///     PREY = 28;
///     BEAST_WOLF = 29;
///     DEFIAS_BROTHERHOOD_TRAITOR = 30;
///     FRIENDLY = 31;
///     TROGG = 32;
///     TROLL_FROSTMANE = 33;
///     ORC_BLACKROCK = 34;
///     VILLIAN = 35;
///     VICTIM = 36;
///     BEAST_BEAR = 37;
///     OGRE = 38;
///     KURZENS_MERCENARIES = 39;
///     ESCORTEE = 40;
///     VENTURE_COMPANY = 41;
///     BEAST_RAPTOR = 42;
///     BASILISK = 43;
///     DRAGONFLIGHT_GREEN = 44;
///     LOST_ONES = 45;
///     BLACKSMITHING_ARMORSMITHING = 46;
///     IRONFORGE = 47;
///     DARK_IRON_DWARVES = 48;
///     HUMAN_NIGHT_WATCH = 49;
///     DRAGONFLIGHT_RED = 50;
///     GNOLL_MOSSHIDE = 51;
///     ORC_DRAGONMAW = 52;
///     GNOME_LEPER = 53;
///     GNOMEREGAN_EXILES = 54;
///     LEOPARD = 55;
///     SCARLET_CRUSADE = 56;
///     GNOLL_ROTHIDE = 57;
///     BEAST_GORILLA = 58;
///     THORIUM_BROTHERHOOD = 59;
///     NAGA = 60;
///     DALARAN = 61;
///     FORLORN_SPIRIT = 62;
///     DARKHOWL = 63;
///     GRELL = 64;
///     FURBOLG = 65;
///     HORDE_GENERIC = 66;
///     HORDE = 67;
///     UNDERCITY = 68;
///     DARNASSUS = 69;
///     SYNDICATE = 70;
///     HILLSBRAD_MILITIA = 71;
///     STORMWIND = 72;
///     DEMON = 73;
///     ELEMENTAL = 74;
///     SPIRIT = 75;
///     ORGRIMMAR = 76;
///     TREASURE = 77;
///     GNOLL_MUDSNOUT = 78;
///     HILLSBRAD_SOUTHSHORE_MAYOR = 79;
///     DRAGONFLIGHT_BLACK = 80;
///     THUNDER_BLUFF = 81;
///     TROLL_WITHERBARK = 82;
///     LEATHERWORKING_ELEMENTAL = 83;
///     QUILBOAR_RAZORMANE = 84;
///     QUILBOAR_BRISTLEBACK = 85;
///     LEATHERWORKING_DRAGONSCALE = 86;
///     BLOODSAIL_BUCCANEERS = 87;
///     BLACKFATHOM = 88;
///     MAKRURA = 89;
///     CENTAUR_KOLKAR = 90;
///     CENTAUR_GALAK = 91;
///     GELKIS_CLAN_CENTAUR = 92;
///     MAGRAM_CLAN_CENTAUR = 93;
///     MARAUDINE = 94;
///     THERAMORE = 108;
///     QUILBOAR_RAZORFEN = 109;
///     QUILBOAR_RAZORMANE_2 = 110;
///     QUILBOAR_DEATHSHEAD = 111;
///     ENEMY = 128;
///     AMBIENT = 148;
///     NETHERGARDE_CARAVAN = 168;
///     STEAMWHEEDLE_CARTEL = 169;
///     ALLIANCE_GENERIC = 189;
///     NETHERGARDE = 209;
///     WAILING_CAVERNS = 229;
///     SILITHID = 249;
///     SILVERMOON_REMNANT = 269;
///     ZANDALAR_TRIBE = 270;
///     BLACKSMITHING_WEAPONSMITHING = 289;
///     SCORPID = 309;
///     BEAST_BAT = 310;
///     TITAN = 311;
///     TASKMASTER_FIZZULE = 329;
///     RAVENHOLDT = 349;
///     GADGETZAN = 369;
///     GNOMEREGAN_BUG = 389;
///     HARPY = 409;
///     BURNING_BLADE = 429;
///     SHADOWSILK_POACHER = 449;
///     SEARING_SPIDER = 450;
///     ALLIANCE = 469;
///     RATCHET = 470;
///     WILDHAMMER_CLAN = 471;
///     GOBLIN_DARK_IRON_BAR_PATRON = 489;
///     THE_LEAGUE_OF_ARATHOR = 509;
///     THE_DEFILERS = 510;
///     GIANT = 511;
///     ARGENT_DAWN = 529;
///     DARKSPEAR_TROLLS = 530;
///     DRAGONFLIGHT_BRONZE = 531;
///     DRAGONFLIGHT_BLUE = 532;
///     LEATHERWORKING_TRIBAL = 549;
///     ENGINEERING_GOBLIN = 550;
///     ENGINEERING_GNOME = 551;
///     BLACKSMITHING_HAMMERSMITHING = 569;
///     BLACKSMITHING_AXESMITHING = 570;
///     BLACKSMITHING_SWORDSMITHING = 571;
///     TROLL_VILEBRANCH = 572;
///     SOUTHSEA_FREEBOOTERS = 573;
///     CAER_DARROW = 574;
///     FURBOLG_UNCORRUPTED = 575;
///     TIMBERMAW_HOLD = 576;
///     EVERLOOK = 577;
///     WINTERSABER_TRAINERS = 589;
///     CENARION_CIRCLE = 609;
///     SHATTERSPEAR_TROLLS = 629;
///     RAVASAUR_TRAINERS = 630;
///     MAJORDOMO_EXECUTUS = 649;
///     BEAST_CARRION_BIRD = 669;
///     BEAST_CAT = 670;
///     BEAST_CRAB = 671;
///     BEAST_CROCILISK = 672;
///     BEAST_HYENA = 673;
///     BEAST_OWL = 674;
///     BEAST_SCORPID = 675;
///     BEAST_TALLSTRIDER = 676;
///     BEAST_TURTLE = 677;
///     BEAST_WIND_SERPENT = 678;
///     TRAINING_DUMMY = 679;
///     DRAGONFLIGHT_BLACK_BAIT = 689;
///     BATTLEGROUND_NEUTRAL = 709;
///     FROSTWOLF_CLAN = 729;
///     STORMPIKE_GUARD = 730;
///     HYDRAXIAN_WATERLORDS = 749;
///     SULFURON_FIRELORDS = 750;
///     GIZLOCKS_DUMMY = 769;
///     GIZLOCKS_CHARM = 770;
///     GIZLOCK = 771;
///     MORO_GAI = 789;
///     SPIRIT_GUIDE_ALLIANCE = 790;
///     SHEN_DRALAR = 809;
///     OGRE_CAPTAIN_KROMCRUSH = 829;
///     SPIRIT_GUIDE_HORDE = 849;
///     JAEDENAR = 869;
///     WARSONG_OUTRIDERS = 889;
///     SILVERWING_SENTINELS = 890;
///     ALLIANCE_FORCES = 891;
///     HORDE_FORCES = 892;
///     REVANTUSK_TROLLS = 893;
///     DARKMOON_FAIRE = 909;
///     BROOD_OF_NOZDORMU = 910;
///     MIGHT_OF_KALIMDOR = 912;
///     ARMIES_OF_C_THUN = 915;
///     SILITHID_ATTACKERS = 916;
///     THE_IRONFORGE_BRIGADE = 917;
///     RC_ENEMIES = 918;
///     RC_OBJECTS = 919;
///     RED = 920;
///     BLUE = 921;
///     SCOURGE_INVADERS = 928;
///     TEST_FACTION_NOT_A_REAL_FACTION = 931;
///     TOWOW_FLAG = 950;
///     TOWOW_FLAG_TRIGGER_ALLIANCE_DND = 951;
///     TOWOW_FLAG_TRIGGER_HORDE_DND = 954;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Faction {
    None,
    PlayerHuman,
    PlayerOrc,
    PlayerDwarf,
    PlayerNightElf,
    PlayerUndead,
    PlayerTauren,
    Creature,
    PlayerGnome,
    PlayerTroll,
    Monster,
    DefiasBrotherhood,
    GnollRiverpaw,
    GnollRedridge,
    GnollShadowhide,
    Murloc,
    UndeadScourge,
    BootyBay,
    BeastSpider,
    BeastBoar,
    Worgen,
    Kobold,
    TrollBloodscalp,
    TrollSkullsplitter,
    Prey,
    BeastWolf,
    DefiasBrotherhoodTraitor,
    Friendly,
    Trogg,
    TrollFrostmane,
    OrcBlackrock,
    Villian,
    Victim,
    BeastBear,
    Ogre,
    KurzensMercenaries,
    Escortee,
    VentureCompany,
    BeastRaptor,
    Basilisk,
    DragonflightGreen,
    LostOnes,
    BlacksmithingArmorsmithing,
    Ironforge,
    DarkIronDwarves,
    HumanNightWatch,
    DragonflightRed,
    GnollMosshide,
    OrcDragonmaw,
    GnomeLeper,
    GnomereganExiles,
    Leopard,
    ScarletCrusade,
    GnollRothide,
    BeastGorilla,
    ThoriumBrotherhood,
    Naga,
    Dalaran,
    ForlornSpirit,
    Darkhowl,
    Grell,
    Furbolg,
    HordeGeneric,
    Horde,
    Undercity,
    Darnassus,
    Syndicate,
    HillsbradMilitia,
    Stormwind,
    Demon,
    Elemental,
    Spirit,
    Orgrimmar,
    Treasure,
    GnollMudsnout,
    HillsbradSouthshoreMayor,
    DragonflightBlack,
    ThunderBluff,
    TrollWitherbark,
    LeatherworkingElemental,
    QuilboarRazormane,
    QuilboarBristleback,
    LeatherworkingDragonscale,
    BloodsailBuccaneers,
    Blackfathom,
    Makrura,
    CentaurKolkar,
    CentaurGalak,
    GelkisClanCentaur,
    MagramClanCentaur,
    Maraudine,
    Theramore,
    QuilboarRazorfen,
    QuilboarRazormane2,
    QuilboarDeathshead,
    Enemy,
    Ambient,
    NethergardeCaravan,
    SteamwheedleCartel,
    AllianceGeneric,
    Nethergarde,
    WailingCaverns,
    Silithid,
    SilvermoonRemnant,
    ZandalarTribe,
    BlacksmithingWeaponsmithing,
    Scorpid,
    BeastBat,
    Titan,
    TaskmasterFizzule,
    Ravenholdt,
    Gadgetzan,
    GnomereganBug,
    Harpy,
    BurningBlade,
    ShadowsilkPoacher,
    SearingSpider,
    Alliance,
    Ratchet,
    WildhammerClan,
    GoblinDarkIronBarPatron,
    TheLeagueOfArathor,
    TheDefilers,
    Giant,
    ArgentDawn,
    DarkspearTrolls,
    DragonflightBronze,
    DragonflightBlue,
    LeatherworkingTribal,
    EngineeringGoblin,
    EngineeringGnome,
    BlacksmithingHammersmithing,
    BlacksmithingAxesmithing,
    BlacksmithingSwordsmithing,
    TrollVilebranch,
    SouthseaFreebooters,
    CaerDarrow,
    FurbolgUncorrupted,
    TimbermawHold,
    Everlook,
    WintersaberTrainers,
    CenarionCircle,
    ShatterspearTrolls,
    RavasaurTrainers,
    MajordomoExecutus,
    BeastCarrionBird,
    BeastCat,
    BeastCrab,
    BeastCrocilisk,
    BeastHyena,
    BeastOwl,
    BeastScorpid,
    BeastTallstrider,
    BeastTurtle,
    BeastWindSerpent,
    TrainingDummy,
    DragonflightBlackBait,
    BattlegroundNeutral,
    FrostwolfClan,
    StormpikeGuard,
    HydraxianWaterlords,
    SulfuronFirelords,
    GizlocksDummy,
    GizlocksCharm,
    Gizlock,
    MoroGai,
    SpiritGuideAlliance,
    ShenDralar,
    OgreCaptainKromcrush,
    SpiritGuideHorde,
    Jaedenar,
    WarsongOutriders,
    SilverwingSentinels,
    AllianceForces,
    HordeForces,
    RevantuskTrolls,
    DarkmoonFaire,
    BroodOfNozdormu,
    MightOfKalimdor,
    ArmiesOfCThun,
    SilithidAttackers,
    TheIronforgeBrigade,
    RcEnemies,
    RcObjects,
    Red,
    Blue,
    ScourgeInvaders,
    TestFactionNotARealFaction,
    TowowFlag,
    TowowFlagTriggerAllianceDnd,
    TowowFlagTriggerHordeDnd,
}

impl Faction {
    pub const fn as_int(&self) -> u16 {
        match self {
            Self::None => 0x0,
            Self::PlayerHuman => 0x1,
            Self::PlayerOrc => 0x2,
            Self::PlayerDwarf => 0x3,
            Self::PlayerNightElf => 0x4,
            Self::PlayerUndead => 0x5,
            Self::PlayerTauren => 0x6,
            Self::Creature => 0x7,
            Self::PlayerGnome => 0x8,
            Self::PlayerTroll => 0x9,
            Self::Monster => 0xe,
            Self::DefiasBrotherhood => 0xf,
            Self::GnollRiverpaw => 0x10,
            Self::GnollRedridge => 0x11,
            Self::GnollShadowhide => 0x12,
            Self::Murloc => 0x13,
            Self::UndeadScourge => 0x14,
            Self::BootyBay => 0x15,
            Self::BeastSpider => 0x16,
            Self::BeastBoar => 0x17,
            Self::Worgen => 0x18,
            Self::Kobold => 0x19,
            Self::TrollBloodscalp => 0x1a,
            Self::TrollSkullsplitter => 0x1b,
            Self::Prey => 0x1c,
            Self::BeastWolf => 0x1d,
            Self::DefiasBrotherhoodTraitor => 0x1e,
            Self::Friendly => 0x1f,
            Self::Trogg => 0x20,
            Self::TrollFrostmane => 0x21,
            Self::OrcBlackrock => 0x22,
            Self::Villian => 0x23,
            Self::Victim => 0x24,
            Self::BeastBear => 0x25,
            Self::Ogre => 0x26,
            Self::KurzensMercenaries => 0x27,
            Self::Escortee => 0x28,
            Self::VentureCompany => 0x29,
            Self::BeastRaptor => 0x2a,
            Self::Basilisk => 0x2b,
            Self::DragonflightGreen => 0x2c,
            Self::LostOnes => 0x2d,
            Self::BlacksmithingArmorsmithing => 0x2e,
            Self::Ironforge => 0x2f,
            Self::DarkIronDwarves => 0x30,
            Self::HumanNightWatch => 0x31,
            Self::DragonflightRed => 0x32,
            Self::GnollMosshide => 0x33,
            Self::OrcDragonmaw => 0x34,
            Self::GnomeLeper => 0x35,
            Self::GnomereganExiles => 0x36,
            Self::Leopard => 0x37,
            Self::ScarletCrusade => 0x38,
            Self::GnollRothide => 0x39,
            Self::BeastGorilla => 0x3a,
            Self::ThoriumBrotherhood => 0x3b,
            Self::Naga => 0x3c,
            Self::Dalaran => 0x3d,
            Self::ForlornSpirit => 0x3e,
            Self::Darkhowl => 0x3f,
            Self::Grell => 0x40,
            Self::Furbolg => 0x41,
            Self::HordeGeneric => 0x42,
            Self::Horde => 0x43,
            Self::Undercity => 0x44,
            Self::Darnassus => 0x45,
            Self::Syndicate => 0x46,
            Self::HillsbradMilitia => 0x47,
            Self::Stormwind => 0x48,
            Self::Demon => 0x49,
            Self::Elemental => 0x4a,
            Self::Spirit => 0x4b,
            Self::Orgrimmar => 0x4c,
            Self::Treasure => 0x4d,
            Self::GnollMudsnout => 0x4e,
            Self::HillsbradSouthshoreMayor => 0x4f,
            Self::DragonflightBlack => 0x50,
            Self::ThunderBluff => 0x51,
            Self::TrollWitherbark => 0x52,
            Self::LeatherworkingElemental => 0x53,
            Self::QuilboarRazormane => 0x54,
            Self::QuilboarBristleback => 0x55,
            Self::LeatherworkingDragonscale => 0x56,
            Self::BloodsailBuccaneers => 0x57,
            Self::Blackfathom => 0x58,
            Self::Makrura => 0x59,
            Self::CentaurKolkar => 0x5a,
            Self::CentaurGalak => 0x5b,
            Self::GelkisClanCentaur => 0x5c,
            Self::MagramClanCentaur => 0x5d,
            Self::Maraudine => 0x5e,
            Self::Theramore => 0x6c,
            Self::QuilboarRazorfen => 0x6d,
            Self::QuilboarRazormane2 => 0x6e,
            Self::QuilboarDeathshead => 0x6f,
            Self::Enemy => 0x80,
            Self::Ambient => 0x94,
            Self::NethergardeCaravan => 0xa8,
            Self::SteamwheedleCartel => 0xa9,
            Self::AllianceGeneric => 0xbd,
            Self::Nethergarde => 0xd1,
            Self::WailingCaverns => 0xe5,
            Self::Silithid => 0xf9,
            Self::SilvermoonRemnant => 0x10d,
            Self::ZandalarTribe => 0x10e,
            Self::BlacksmithingWeaponsmithing => 0x121,
            Self::Scorpid => 0x135,
            Self::BeastBat => 0x136,
            Self::Titan => 0x137,
            Self::TaskmasterFizzule => 0x149,
            Self::Ravenholdt => 0x15d,
            Self::Gadgetzan => 0x171,
            Self::GnomereganBug => 0x185,
            Self::Harpy => 0x199,
            Self::BurningBlade => 0x1ad,
            Self::ShadowsilkPoacher => 0x1c1,
            Self::SearingSpider => 0x1c2,
            Self::Alliance => 0x1d5,
            Self::Ratchet => 0x1d6,
            Self::WildhammerClan => 0x1d7,
            Self::GoblinDarkIronBarPatron => 0x1e9,
            Self::TheLeagueOfArathor => 0x1fd,
            Self::TheDefilers => 0x1fe,
            Self::Giant => 0x1ff,
            Self::ArgentDawn => 0x211,
            Self::DarkspearTrolls => 0x212,
            Self::DragonflightBronze => 0x213,
            Self::DragonflightBlue => 0x214,
            Self::LeatherworkingTribal => 0x225,
            Self::EngineeringGoblin => 0x226,
            Self::EngineeringGnome => 0x227,
            Self::BlacksmithingHammersmithing => 0x239,
            Self::BlacksmithingAxesmithing => 0x23a,
            Self::BlacksmithingSwordsmithing => 0x23b,
            Self::TrollVilebranch => 0x23c,
            Self::SouthseaFreebooters => 0x23d,
            Self::CaerDarrow => 0x23e,
            Self::FurbolgUncorrupted => 0x23f,
            Self::TimbermawHold => 0x240,
            Self::Everlook => 0x241,
            Self::WintersaberTrainers => 0x24d,
            Self::CenarionCircle => 0x261,
            Self::ShatterspearTrolls => 0x275,
            Self::RavasaurTrainers => 0x276,
            Self::MajordomoExecutus => 0x289,
            Self::BeastCarrionBird => 0x29d,
            Self::BeastCat => 0x29e,
            Self::BeastCrab => 0x29f,
            Self::BeastCrocilisk => 0x2a0,
            Self::BeastHyena => 0x2a1,
            Self::BeastOwl => 0x2a2,
            Self::BeastScorpid => 0x2a3,
            Self::BeastTallstrider => 0x2a4,
            Self::BeastTurtle => 0x2a5,
            Self::BeastWindSerpent => 0x2a6,
            Self::TrainingDummy => 0x2a7,
            Self::DragonflightBlackBait => 0x2b1,
            Self::BattlegroundNeutral => 0x2c5,
            Self::FrostwolfClan => 0x2d9,
            Self::StormpikeGuard => 0x2da,
            Self::HydraxianWaterlords => 0x2ed,
            Self::SulfuronFirelords => 0x2ee,
            Self::GizlocksDummy => 0x301,
            Self::GizlocksCharm => 0x302,
            Self::Gizlock => 0x303,
            Self::MoroGai => 0x315,
            Self::SpiritGuideAlliance => 0x316,
            Self::ShenDralar => 0x329,
            Self::OgreCaptainKromcrush => 0x33d,
            Self::SpiritGuideHorde => 0x351,
            Self::Jaedenar => 0x365,
            Self::WarsongOutriders => 0x379,
            Self::SilverwingSentinels => 0x37a,
            Self::AllianceForces => 0x37b,
            Self::HordeForces => 0x37c,
            Self::RevantuskTrolls => 0x37d,
            Self::DarkmoonFaire => 0x38d,
            Self::BroodOfNozdormu => 0x38e,
            Self::MightOfKalimdor => 0x390,
            Self::ArmiesOfCThun => 0x393,
            Self::SilithidAttackers => 0x394,
            Self::TheIronforgeBrigade => 0x395,
            Self::RcEnemies => 0x396,
            Self::RcObjects => 0x397,
            Self::Red => 0x398,
            Self::Blue => 0x399,
            Self::ScourgeInvaders => 0x3a0,
            Self::TestFactionNotARealFaction => 0x3a3,
            Self::TowowFlag => 0x3b6,
            Self::TowowFlagTriggerAllianceDnd => 0x3b7,
            Self::TowowFlagTriggerHordeDnd => 0x3ba,
        }
    }

    pub const fn variants() -> [Self; 191] {
        [
            Self::None,
            Self::PlayerHuman,
            Self::PlayerOrc,
            Self::PlayerDwarf,
            Self::PlayerNightElf,
            Self::PlayerUndead,
            Self::PlayerTauren,
            Self::Creature,
            Self::PlayerGnome,
            Self::PlayerTroll,
            Self::Monster,
            Self::DefiasBrotherhood,
            Self::GnollRiverpaw,
            Self::GnollRedridge,
            Self::GnollShadowhide,
            Self::Murloc,
            Self::UndeadScourge,
            Self::BootyBay,
            Self::BeastSpider,
            Self::BeastBoar,
            Self::Worgen,
            Self::Kobold,
            Self::TrollBloodscalp,
            Self::TrollSkullsplitter,
            Self::Prey,
            Self::BeastWolf,
            Self::DefiasBrotherhoodTraitor,
            Self::Friendly,
            Self::Trogg,
            Self::TrollFrostmane,
            Self::OrcBlackrock,
            Self::Villian,
            Self::Victim,
            Self::BeastBear,
            Self::Ogre,
            Self::KurzensMercenaries,
            Self::Escortee,
            Self::VentureCompany,
            Self::BeastRaptor,
            Self::Basilisk,
            Self::DragonflightGreen,
            Self::LostOnes,
            Self::BlacksmithingArmorsmithing,
            Self::Ironforge,
            Self::DarkIronDwarves,
            Self::HumanNightWatch,
            Self::DragonflightRed,
            Self::GnollMosshide,
            Self::OrcDragonmaw,
            Self::GnomeLeper,
            Self::GnomereganExiles,
            Self::Leopard,
            Self::ScarletCrusade,
            Self::GnollRothide,
            Self::BeastGorilla,
            Self::ThoriumBrotherhood,
            Self::Naga,
            Self::Dalaran,
            Self::ForlornSpirit,
            Self::Darkhowl,
            Self::Grell,
            Self::Furbolg,
            Self::HordeGeneric,
            Self::Horde,
            Self::Undercity,
            Self::Darnassus,
            Self::Syndicate,
            Self::HillsbradMilitia,
            Self::Stormwind,
            Self::Demon,
            Self::Elemental,
            Self::Spirit,
            Self::Orgrimmar,
            Self::Treasure,
            Self::GnollMudsnout,
            Self::HillsbradSouthshoreMayor,
            Self::DragonflightBlack,
            Self::ThunderBluff,
            Self::TrollWitherbark,
            Self::LeatherworkingElemental,
            Self::QuilboarRazormane,
            Self::QuilboarBristleback,
            Self::LeatherworkingDragonscale,
            Self::BloodsailBuccaneers,
            Self::Blackfathom,
            Self::Makrura,
            Self::CentaurKolkar,
            Self::CentaurGalak,
            Self::GelkisClanCentaur,
            Self::MagramClanCentaur,
            Self::Maraudine,
            Self::Theramore,
            Self::QuilboarRazorfen,
            Self::QuilboarRazormane2,
            Self::QuilboarDeathshead,
            Self::Enemy,
            Self::Ambient,
            Self::NethergardeCaravan,
            Self::SteamwheedleCartel,
            Self::AllianceGeneric,
            Self::Nethergarde,
            Self::WailingCaverns,
            Self::Silithid,
            Self::SilvermoonRemnant,
            Self::ZandalarTribe,
            Self::BlacksmithingWeaponsmithing,
            Self::Scorpid,
            Self::BeastBat,
            Self::Titan,
            Self::TaskmasterFizzule,
            Self::Ravenholdt,
            Self::Gadgetzan,
            Self::GnomereganBug,
            Self::Harpy,
            Self::BurningBlade,
            Self::ShadowsilkPoacher,
            Self::SearingSpider,
            Self::Alliance,
            Self::Ratchet,
            Self::WildhammerClan,
            Self::GoblinDarkIronBarPatron,
            Self::TheLeagueOfArathor,
            Self::TheDefilers,
            Self::Giant,
            Self::ArgentDawn,
            Self::DarkspearTrolls,
            Self::DragonflightBronze,
            Self::DragonflightBlue,
            Self::LeatherworkingTribal,
            Self::EngineeringGoblin,
            Self::EngineeringGnome,
            Self::BlacksmithingHammersmithing,
            Self::BlacksmithingAxesmithing,
            Self::BlacksmithingSwordsmithing,
            Self::TrollVilebranch,
            Self::SouthseaFreebooters,
            Self::CaerDarrow,
            Self::FurbolgUncorrupted,
            Self::TimbermawHold,
            Self::Everlook,
            Self::WintersaberTrainers,
            Self::CenarionCircle,
            Self::ShatterspearTrolls,
            Self::RavasaurTrainers,
            Self::MajordomoExecutus,
            Self::BeastCarrionBird,
            Self::BeastCat,
            Self::BeastCrab,
            Self::BeastCrocilisk,
            Self::BeastHyena,
            Self::BeastOwl,
            Self::BeastScorpid,
            Self::BeastTallstrider,
            Self::BeastTurtle,
            Self::BeastWindSerpent,
            Self::TrainingDummy,
            Self::DragonflightBlackBait,
            Self::BattlegroundNeutral,
            Self::FrostwolfClan,
            Self::StormpikeGuard,
            Self::HydraxianWaterlords,
            Self::SulfuronFirelords,
            Self::GizlocksDummy,
            Self::GizlocksCharm,
            Self::Gizlock,
            Self::MoroGai,
            Self::SpiritGuideAlliance,
            Self::ShenDralar,
            Self::OgreCaptainKromcrush,
            Self::SpiritGuideHorde,
            Self::Jaedenar,
            Self::WarsongOutriders,
            Self::SilverwingSentinels,
            Self::AllianceForces,
            Self::HordeForces,
            Self::RevantuskTrolls,
            Self::DarkmoonFaire,
            Self::BroodOfNozdormu,
            Self::MightOfKalimdor,
            Self::ArmiesOfCThun,
            Self::SilithidAttackers,
            Self::TheIronforgeBrigade,
            Self::RcEnemies,
            Self::RcObjects,
            Self::Red,
            Self::Blue,
            Self::ScourgeInvaders,
            Self::TestFactionNotARealFaction,
            Self::TowowFlag,
            Self::TowowFlagTriggerAllianceDnd,
            Self::TowowFlagTriggerHordeDnd,
        ]
    }

    pub const fn from_int(value: u16) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::PlayerHuman),
            2 => Ok(Self::PlayerOrc),
            3 => Ok(Self::PlayerDwarf),
            4 => Ok(Self::PlayerNightElf),
            5 => Ok(Self::PlayerUndead),
            6 => Ok(Self::PlayerTauren),
            7 => Ok(Self::Creature),
            8 => Ok(Self::PlayerGnome),
            9 => Ok(Self::PlayerTroll),
            14 => Ok(Self::Monster),
            15 => Ok(Self::DefiasBrotherhood),
            16 => Ok(Self::GnollRiverpaw),
            17 => Ok(Self::GnollRedridge),
            18 => Ok(Self::GnollShadowhide),
            19 => Ok(Self::Murloc),
            20 => Ok(Self::UndeadScourge),
            21 => Ok(Self::BootyBay),
            22 => Ok(Self::BeastSpider),
            23 => Ok(Self::BeastBoar),
            24 => Ok(Self::Worgen),
            25 => Ok(Self::Kobold),
            26 => Ok(Self::TrollBloodscalp),
            27 => Ok(Self::TrollSkullsplitter),
            28 => Ok(Self::Prey),
            29 => Ok(Self::BeastWolf),
            30 => Ok(Self::DefiasBrotherhoodTraitor),
            31 => Ok(Self::Friendly),
            32 => Ok(Self::Trogg),
            33 => Ok(Self::TrollFrostmane),
            34 => Ok(Self::OrcBlackrock),
            35 => Ok(Self::Villian),
            36 => Ok(Self::Victim),
            37 => Ok(Self::BeastBear),
            38 => Ok(Self::Ogre),
            39 => Ok(Self::KurzensMercenaries),
            40 => Ok(Self::Escortee),
            41 => Ok(Self::VentureCompany),
            42 => Ok(Self::BeastRaptor),
            43 => Ok(Self::Basilisk),
            44 => Ok(Self::DragonflightGreen),
            45 => Ok(Self::LostOnes),
            46 => Ok(Self::BlacksmithingArmorsmithing),
            47 => Ok(Self::Ironforge),
            48 => Ok(Self::DarkIronDwarves),
            49 => Ok(Self::HumanNightWatch),
            50 => Ok(Self::DragonflightRed),
            51 => Ok(Self::GnollMosshide),
            52 => Ok(Self::OrcDragonmaw),
            53 => Ok(Self::GnomeLeper),
            54 => Ok(Self::GnomereganExiles),
            55 => Ok(Self::Leopard),
            56 => Ok(Self::ScarletCrusade),
            57 => Ok(Self::GnollRothide),
            58 => Ok(Self::BeastGorilla),
            59 => Ok(Self::ThoriumBrotherhood),
            60 => Ok(Self::Naga),
            61 => Ok(Self::Dalaran),
            62 => Ok(Self::ForlornSpirit),
            63 => Ok(Self::Darkhowl),
            64 => Ok(Self::Grell),
            65 => Ok(Self::Furbolg),
            66 => Ok(Self::HordeGeneric),
            67 => Ok(Self::Horde),
            68 => Ok(Self::Undercity),
            69 => Ok(Self::Darnassus),
            70 => Ok(Self::Syndicate),
            71 => Ok(Self::HillsbradMilitia),
            72 => Ok(Self::Stormwind),
            73 => Ok(Self::Demon),
            74 => Ok(Self::Elemental),
            75 => Ok(Self::Spirit),
            76 => Ok(Self::Orgrimmar),
            77 => Ok(Self::Treasure),
            78 => Ok(Self::GnollMudsnout),
            79 => Ok(Self::HillsbradSouthshoreMayor),
            80 => Ok(Self::DragonflightBlack),
            81 => Ok(Self::ThunderBluff),
            82 => Ok(Self::TrollWitherbark),
            83 => Ok(Self::LeatherworkingElemental),
            84 => Ok(Self::QuilboarRazormane),
            85 => Ok(Self::QuilboarBristleback),
            86 => Ok(Self::LeatherworkingDragonscale),
            87 => Ok(Self::BloodsailBuccaneers),
            88 => Ok(Self::Blackfathom),
            89 => Ok(Self::Makrura),
            90 => Ok(Self::CentaurKolkar),
            91 => Ok(Self::CentaurGalak),
            92 => Ok(Self::GelkisClanCentaur),
            93 => Ok(Self::MagramClanCentaur),
            94 => Ok(Self::Maraudine),
            108 => Ok(Self::Theramore),
            109 => Ok(Self::QuilboarRazorfen),
            110 => Ok(Self::QuilboarRazormane2),
            111 => Ok(Self::QuilboarDeathshead),
            128 => Ok(Self::Enemy),
            148 => Ok(Self::Ambient),
            168 => Ok(Self::NethergardeCaravan),
            169 => Ok(Self::SteamwheedleCartel),
            189 => Ok(Self::AllianceGeneric),
            209 => Ok(Self::Nethergarde),
            229 => Ok(Self::WailingCaverns),
            249 => Ok(Self::Silithid),
            269 => Ok(Self::SilvermoonRemnant),
            270 => Ok(Self::ZandalarTribe),
            289 => Ok(Self::BlacksmithingWeaponsmithing),
            309 => Ok(Self::Scorpid),
            310 => Ok(Self::BeastBat),
            311 => Ok(Self::Titan),
            329 => Ok(Self::TaskmasterFizzule),
            349 => Ok(Self::Ravenholdt),
            369 => Ok(Self::Gadgetzan),
            389 => Ok(Self::GnomereganBug),
            409 => Ok(Self::Harpy),
            429 => Ok(Self::BurningBlade),
            449 => Ok(Self::ShadowsilkPoacher),
            450 => Ok(Self::SearingSpider),
            469 => Ok(Self::Alliance),
            470 => Ok(Self::Ratchet),
            471 => Ok(Self::WildhammerClan),
            489 => Ok(Self::GoblinDarkIronBarPatron),
            509 => Ok(Self::TheLeagueOfArathor),
            510 => Ok(Self::TheDefilers),
            511 => Ok(Self::Giant),
            529 => Ok(Self::ArgentDawn),
            530 => Ok(Self::DarkspearTrolls),
            531 => Ok(Self::DragonflightBronze),
            532 => Ok(Self::DragonflightBlue),
            549 => Ok(Self::LeatherworkingTribal),
            550 => Ok(Self::EngineeringGoblin),
            551 => Ok(Self::EngineeringGnome),
            569 => Ok(Self::BlacksmithingHammersmithing),
            570 => Ok(Self::BlacksmithingAxesmithing),
            571 => Ok(Self::BlacksmithingSwordsmithing),
            572 => Ok(Self::TrollVilebranch),
            573 => Ok(Self::SouthseaFreebooters),
            574 => Ok(Self::CaerDarrow),
            575 => Ok(Self::FurbolgUncorrupted),
            576 => Ok(Self::TimbermawHold),
            577 => Ok(Self::Everlook),
            589 => Ok(Self::WintersaberTrainers),
            609 => Ok(Self::CenarionCircle),
            629 => Ok(Self::ShatterspearTrolls),
            630 => Ok(Self::RavasaurTrainers),
            649 => Ok(Self::MajordomoExecutus),
            669 => Ok(Self::BeastCarrionBird),
            670 => Ok(Self::BeastCat),
            671 => Ok(Self::BeastCrab),
            672 => Ok(Self::BeastCrocilisk),
            673 => Ok(Self::BeastHyena),
            674 => Ok(Self::BeastOwl),
            675 => Ok(Self::BeastScorpid),
            676 => Ok(Self::BeastTallstrider),
            677 => Ok(Self::BeastTurtle),
            678 => Ok(Self::BeastWindSerpent),
            679 => Ok(Self::TrainingDummy),
            689 => Ok(Self::DragonflightBlackBait),
            709 => Ok(Self::BattlegroundNeutral),
            729 => Ok(Self::FrostwolfClan),
            730 => Ok(Self::StormpikeGuard),
            749 => Ok(Self::HydraxianWaterlords),
            750 => Ok(Self::SulfuronFirelords),
            769 => Ok(Self::GizlocksDummy),
            770 => Ok(Self::GizlocksCharm),
            771 => Ok(Self::Gizlock),
            789 => Ok(Self::MoroGai),
            790 => Ok(Self::SpiritGuideAlliance),
            809 => Ok(Self::ShenDralar),
            829 => Ok(Self::OgreCaptainKromcrush),
            849 => Ok(Self::SpiritGuideHorde),
            869 => Ok(Self::Jaedenar),
            889 => Ok(Self::WarsongOutriders),
            890 => Ok(Self::SilverwingSentinels),
            891 => Ok(Self::AllianceForces),
            892 => Ok(Self::HordeForces),
            893 => Ok(Self::RevantuskTrolls),
            909 => Ok(Self::DarkmoonFaire),
            910 => Ok(Self::BroodOfNozdormu),
            912 => Ok(Self::MightOfKalimdor),
            915 => Ok(Self::ArmiesOfCThun),
            916 => Ok(Self::SilithidAttackers),
            917 => Ok(Self::TheIronforgeBrigade),
            918 => Ok(Self::RcEnemies),
            919 => Ok(Self::RcObjects),
            920 => Ok(Self::Red),
            921 => Ok(Self::Blue),
            928 => Ok(Self::ScourgeInvaders),
            931 => Ok(Self::TestFactionNotARealFaction),
            950 => Ok(Self::TowowFlag),
            951 => Ok(Self::TowowFlagTriggerAllianceDnd),
            954 => Ok(Self::TowowFlagTriggerHordeDnd),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl Faction {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::PlayerHuman => "PLAYER_HUMAN",
            Self::PlayerOrc => "PLAYER_ORC",
            Self::PlayerDwarf => "PLAYER_DWARF",
            Self::PlayerNightElf => "PLAYER_NIGHT_ELF",
            Self::PlayerUndead => "PLAYER_UNDEAD",
            Self::PlayerTauren => "PLAYER_TAUREN",
            Self::Creature => "CREATURE",
            Self::PlayerGnome => "PLAYER_GNOME",
            Self::PlayerTroll => "PLAYER_TROLL",
            Self::Monster => "MONSTER",
            Self::DefiasBrotherhood => "DEFIAS_BROTHERHOOD",
            Self::GnollRiverpaw => "GNOLL_RIVERPAW",
            Self::GnollRedridge => "GNOLL_REDRIDGE",
            Self::GnollShadowhide => "GNOLL_SHADOWHIDE",
            Self::Murloc => "MURLOC",
            Self::UndeadScourge => "UNDEAD_SCOURGE",
            Self::BootyBay => "BOOTY_BAY",
            Self::BeastSpider => "BEAST_SPIDER",
            Self::BeastBoar => "BEAST_BOAR",
            Self::Worgen => "WORGEN",
            Self::Kobold => "KOBOLD",
            Self::TrollBloodscalp => "TROLL_BLOODSCALP",
            Self::TrollSkullsplitter => "TROLL_SKULLSPLITTER",
            Self::Prey => "PREY",
            Self::BeastWolf => "BEAST_WOLF",
            Self::DefiasBrotherhoodTraitor => "DEFIAS_BROTHERHOOD_TRAITOR",
            Self::Friendly => "FRIENDLY",
            Self::Trogg => "TROGG",
            Self::TrollFrostmane => "TROLL_FROSTMANE",
            Self::OrcBlackrock => "ORC_BLACKROCK",
            Self::Villian => "VILLIAN",
            Self::Victim => "VICTIM",
            Self::BeastBear => "BEAST_BEAR",
            Self::Ogre => "OGRE",
            Self::KurzensMercenaries => "KURZENS_MERCENARIES",
            Self::Escortee => "ESCORTEE",
            Self::VentureCompany => "VENTURE_COMPANY",
            Self::BeastRaptor => "BEAST_RAPTOR",
            Self::Basilisk => "BASILISK",
            Self::DragonflightGreen => "DRAGONFLIGHT_GREEN",
            Self::LostOnes => "LOST_ONES",
            Self::BlacksmithingArmorsmithing => "BLACKSMITHING_ARMORSMITHING",
            Self::Ironforge => "IRONFORGE",
            Self::DarkIronDwarves => "DARK_IRON_DWARVES",
            Self::HumanNightWatch => "HUMAN_NIGHT_WATCH",
            Self::DragonflightRed => "DRAGONFLIGHT_RED",
            Self::GnollMosshide => "GNOLL_MOSSHIDE",
            Self::OrcDragonmaw => "ORC_DRAGONMAW",
            Self::GnomeLeper => "GNOME_LEPER",
            Self::GnomereganExiles => "GNOMEREGAN_EXILES",
            Self::Leopard => "LEOPARD",
            Self::ScarletCrusade => "SCARLET_CRUSADE",
            Self::GnollRothide => "GNOLL_ROTHIDE",
            Self::BeastGorilla => "BEAST_GORILLA",
            Self::ThoriumBrotherhood => "THORIUM_BROTHERHOOD",
            Self::Naga => "NAGA",
            Self::Dalaran => "DALARAN",
            Self::ForlornSpirit => "FORLORN_SPIRIT",
            Self::Darkhowl => "DARKHOWL",
            Self::Grell => "GRELL",
            Self::Furbolg => "FURBOLG",
            Self::HordeGeneric => "HORDE_GENERIC",
            Self::Horde => "HORDE",
            Self::Undercity => "UNDERCITY",
            Self::Darnassus => "DARNASSUS",
            Self::Syndicate => "SYNDICATE",
            Self::HillsbradMilitia => "HILLSBRAD_MILITIA",
            Self::Stormwind => "STORMWIND",
            Self::Demon => "DEMON",
            Self::Elemental => "ELEMENTAL",
            Self::Spirit => "SPIRIT",
            Self::Orgrimmar => "ORGRIMMAR",
            Self::Treasure => "TREASURE",
            Self::GnollMudsnout => "GNOLL_MUDSNOUT",
            Self::HillsbradSouthshoreMayor => "HILLSBRAD_SOUTHSHORE_MAYOR",
            Self::DragonflightBlack => "DRAGONFLIGHT_BLACK",
            Self::ThunderBluff => "THUNDER_BLUFF",
            Self::TrollWitherbark => "TROLL_WITHERBARK",
            Self::LeatherworkingElemental => "LEATHERWORKING_ELEMENTAL",
            Self::QuilboarRazormane => "QUILBOAR_RAZORMANE",
            Self::QuilboarBristleback => "QUILBOAR_BRISTLEBACK",
            Self::LeatherworkingDragonscale => "LEATHERWORKING_DRAGONSCALE",
            Self::BloodsailBuccaneers => "BLOODSAIL_BUCCANEERS",
            Self::Blackfathom => "BLACKFATHOM",
            Self::Makrura => "MAKRURA",
            Self::CentaurKolkar => "CENTAUR_KOLKAR",
            Self::CentaurGalak => "CENTAUR_GALAK",
            Self::GelkisClanCentaur => "GELKIS_CLAN_CENTAUR",
            Self::MagramClanCentaur => "MAGRAM_CLAN_CENTAUR",
            Self::Maraudine => "MARAUDINE",
            Self::Theramore => "THERAMORE",
            Self::QuilboarRazorfen => "QUILBOAR_RAZORFEN",
            Self::QuilboarRazormane2 => "QUILBOAR_RAZORMANE_2",
            Self::QuilboarDeathshead => "QUILBOAR_DEATHSHEAD",
            Self::Enemy => "ENEMY",
            Self::Ambient => "AMBIENT",
            Self::NethergardeCaravan => "NETHERGARDE_CARAVAN",
            Self::SteamwheedleCartel => "STEAMWHEEDLE_CARTEL",
            Self::AllianceGeneric => "ALLIANCE_GENERIC",
            Self::Nethergarde => "NETHERGARDE",
            Self::WailingCaverns => "WAILING_CAVERNS",
            Self::Silithid => "SILITHID",
            Self::SilvermoonRemnant => "SILVERMOON_REMNANT",
            Self::ZandalarTribe => "ZANDALAR_TRIBE",
            Self::BlacksmithingWeaponsmithing => "BLACKSMITHING_WEAPONSMITHING",
            Self::Scorpid => "SCORPID",
            Self::BeastBat => "BEAST_BAT",
            Self::Titan => "TITAN",
            Self::TaskmasterFizzule => "TASKMASTER_FIZZULE",
            Self::Ravenholdt => "RAVENHOLDT",
            Self::Gadgetzan => "GADGETZAN",
            Self::GnomereganBug => "GNOMEREGAN_BUG",
            Self::Harpy => "HARPY",
            Self::BurningBlade => "BURNING_BLADE",
            Self::ShadowsilkPoacher => "SHADOWSILK_POACHER",
            Self::SearingSpider => "SEARING_SPIDER",
            Self::Alliance => "ALLIANCE",
            Self::Ratchet => "RATCHET",
            Self::WildhammerClan => "WILDHAMMER_CLAN",
            Self::GoblinDarkIronBarPatron => "GOBLIN_DARK_IRON_BAR_PATRON",
            Self::TheLeagueOfArathor => "THE_LEAGUE_OF_ARATHOR",
            Self::TheDefilers => "THE_DEFILERS",
            Self::Giant => "GIANT",
            Self::ArgentDawn => "ARGENT_DAWN",
            Self::DarkspearTrolls => "DARKSPEAR_TROLLS",
            Self::DragonflightBronze => "DRAGONFLIGHT_BRONZE",
            Self::DragonflightBlue => "DRAGONFLIGHT_BLUE",
            Self::LeatherworkingTribal => "LEATHERWORKING_TRIBAL",
            Self::EngineeringGoblin => "ENGINEERING_GOBLIN",
            Self::EngineeringGnome => "ENGINEERING_GNOME",
            Self::BlacksmithingHammersmithing => "BLACKSMITHING_HAMMERSMITHING",
            Self::BlacksmithingAxesmithing => "BLACKSMITHING_AXESMITHING",
            Self::BlacksmithingSwordsmithing => "BLACKSMITHING_SWORDSMITHING",
            Self::TrollVilebranch => "TROLL_VILEBRANCH",
            Self::SouthseaFreebooters => "SOUTHSEA_FREEBOOTERS",
            Self::CaerDarrow => "CAER_DARROW",
            Self::FurbolgUncorrupted => "FURBOLG_UNCORRUPTED",
            Self::TimbermawHold => "TIMBERMAW_HOLD",
            Self::Everlook => "EVERLOOK",
            Self::WintersaberTrainers => "WINTERSABER_TRAINERS",
            Self::CenarionCircle => "CENARION_CIRCLE",
            Self::ShatterspearTrolls => "SHATTERSPEAR_TROLLS",
            Self::RavasaurTrainers => "RAVASAUR_TRAINERS",
            Self::MajordomoExecutus => "MAJORDOMO_EXECUTUS",
            Self::BeastCarrionBird => "BEAST_CARRION_BIRD",
            Self::BeastCat => "BEAST_CAT",
            Self::BeastCrab => "BEAST_CRAB",
            Self::BeastCrocilisk => "BEAST_CROCILISK",
            Self::BeastHyena => "BEAST_HYENA",
            Self::BeastOwl => "BEAST_OWL",
            Self::BeastScorpid => "BEAST_SCORPID",
            Self::BeastTallstrider => "BEAST_TALLSTRIDER",
            Self::BeastTurtle => "BEAST_TURTLE",
            Self::BeastWindSerpent => "BEAST_WIND_SERPENT",
            Self::TrainingDummy => "TRAINING_DUMMY",
            Self::DragonflightBlackBait => "DRAGONFLIGHT_BLACK_BAIT",
            Self::BattlegroundNeutral => "BATTLEGROUND_NEUTRAL",
            Self::FrostwolfClan => "FROSTWOLF_CLAN",
            Self::StormpikeGuard => "STORMPIKE_GUARD",
            Self::HydraxianWaterlords => "HYDRAXIAN_WATERLORDS",
            Self::SulfuronFirelords => "SULFURON_FIRELORDS",
            Self::GizlocksDummy => "GIZLOCKS_DUMMY",
            Self::GizlocksCharm => "GIZLOCKS_CHARM",
            Self::Gizlock => "GIZLOCK",
            Self::MoroGai => "MORO_GAI",
            Self::SpiritGuideAlliance => "SPIRIT_GUIDE_ALLIANCE",
            Self::ShenDralar => "SHEN_DRALAR",
            Self::OgreCaptainKromcrush => "OGRE_CAPTAIN_KROMCRUSH",
            Self::SpiritGuideHorde => "SPIRIT_GUIDE_HORDE",
            Self::Jaedenar => "JAEDENAR",
            Self::WarsongOutriders => "WARSONG_OUTRIDERS",
            Self::SilverwingSentinels => "SILVERWING_SENTINELS",
            Self::AllianceForces => "ALLIANCE_FORCES",
            Self::HordeForces => "HORDE_FORCES",
            Self::RevantuskTrolls => "REVANTUSK_TROLLS",
            Self::DarkmoonFaire => "DARKMOON_FAIRE",
            Self::BroodOfNozdormu => "BROOD_OF_NOZDORMU",
            Self::MightOfKalimdor => "MIGHT_OF_KALIMDOR",
            Self::ArmiesOfCThun => "ARMIES_OF_C_THUN",
            Self::SilithidAttackers => "SILITHID_ATTACKERS",
            Self::TheIronforgeBrigade => "THE_IRONFORGE_BRIGADE",
            Self::RcEnemies => "RC_ENEMIES",
            Self::RcObjects => "RC_OBJECTS",
            Self::Red => "RED",
            Self::Blue => "BLUE",
            Self::ScourgeInvaders => "SCOURGE_INVADERS",
            Self::TestFactionNotARealFaction => "TEST_FACTION_NOT_A_REAL_FACTION",
            Self::TowowFlag => "TOWOW_FLAG",
            Self::TowowFlagTriggerAllianceDnd => "TOWOW_FLAG_TRIGGER_ALLIANCE_DND",
            Self::TowowFlagTriggerHordeDnd => "TOWOW_FLAG_TRIGGER_HORDE_DND",
        }
    }

}

const NAME: &str = "Faction";

impl Default for Faction {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for Faction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::PlayerHuman => f.write_str("PLAYER, Human"),
            Self::PlayerOrc => f.write_str("PLAYER, Orc"),
            Self::PlayerDwarf => f.write_str("PLAYER, Dwarf"),
            Self::PlayerNightElf => f.write_str("PLAYER, Night Elf"),
            Self::PlayerUndead => f.write_str("PLAYER, Undead"),
            Self::PlayerTauren => f.write_str("PLAYER, Tauren"),
            Self::Creature => f.write_str("Creature"),
            Self::PlayerGnome => f.write_str("PLAYER, Gnome"),
            Self::PlayerTroll => f.write_str("PLAYER, Troll"),
            Self::Monster => f.write_str("Monster"),
            Self::DefiasBrotherhood => f.write_str("DefiasBrotherhood"),
            Self::GnollRiverpaw => f.write_str("Gnoll - Riverpaw"),
            Self::GnollRedridge => f.write_str("Gnoll - Redridge"),
            Self::GnollShadowhide => f.write_str("Gnoll - Shadowhide"),
            Self::Murloc => f.write_str("Murloc"),
            Self::UndeadScourge => f.write_str("Undead, Scourge"),
            Self::BootyBay => f.write_str("BootyBay"),
            Self::BeastSpider => f.write_str("Beast - Spider"),
            Self::BeastBoar => f.write_str("Beast - Boar"),
            Self::Worgen => f.write_str("Worgen"),
            Self::Kobold => f.write_str("Kobold"),
            Self::TrollBloodscalp => f.write_str("Troll, Bloodscalp"),
            Self::TrollSkullsplitter => f.write_str("Troll, Skullsplitter"),
            Self::Prey => f.write_str("Prey"),
            Self::BeastWolf => f.write_str("Beast - Wolf"),
            Self::DefiasBrotherhoodTraitor => f.write_str("DefiasBrotherhoodTraitor"),
            Self::Friendly => f.write_str("Friendly"),
            Self::Trogg => f.write_str("Trogg"),
            Self::TrollFrostmane => f.write_str("Troll, Frostmane"),
            Self::OrcBlackrock => f.write_str("Orc, Blackrock"),
            Self::Villian => f.write_str("Villian"),
            Self::Victim => f.write_str("Victim"),
            Self::BeastBear => f.write_str("Beast - Bear"),
            Self::Ogre => f.write_str("Ogre"),
            Self::KurzensMercenaries => f.write_str("Kurzen's Mercenaries"),
            Self::Escortee => f.write_str("Escortee"),
            Self::VentureCompany => f.write_str("VentureCompany"),
            Self::BeastRaptor => f.write_str("Beast - Raptor"),
            Self::Basilisk => f.write_str("Basilisk"),
            Self::DragonflightGreen => f.write_str("Dragonflight, Green"),
            Self::LostOnes => f.write_str("LostOnes"),
            Self::BlacksmithingArmorsmithing => f.write_str("Blacksmithing - Armorsmithing"),
            Self::Ironforge => f.write_str("Ironforge"),
            Self::DarkIronDwarves => f.write_str("DarkIronDwarves"),
            Self::HumanNightWatch => f.write_str("Human, Night Watch"),
            Self::DragonflightRed => f.write_str("Dragonflight, Red"),
            Self::GnollMosshide => f.write_str("Gnoll - Mosshide"),
            Self::OrcDragonmaw => f.write_str("Orc, Dragonmaw"),
            Self::GnomeLeper => f.write_str("Gnome - Leper"),
            Self::GnomereganExiles => f.write_str("GnomereganExiles"),
            Self::Leopard => f.write_str("Leopard"),
            Self::ScarletCrusade => f.write_str("ScarletCrusade"),
            Self::GnollRothide => f.write_str("Gnoll - Rothide"),
            Self::BeastGorilla => f.write_str("Beast - Gorilla"),
            Self::ThoriumBrotherhood => f.write_str("ThoriumBrotherhood"),
            Self::Naga => f.write_str("Naga"),
            Self::Dalaran => f.write_str("Dalaran"),
            Self::ForlornSpirit => f.write_str("ForlornSpirit"),
            Self::Darkhowl => f.write_str("Darkhowl"),
            Self::Grell => f.write_str("Grell"),
            Self::Furbolg => f.write_str("Furbolg"),
            Self::HordeGeneric => f.write_str("HordeGeneric"),
            Self::Horde => f.write_str("Horde"),
            Self::Undercity => f.write_str("Undercity"),
            Self::Darnassus => f.write_str("Darnassus"),
            Self::Syndicate => f.write_str("Syndicate"),
            Self::HillsbradMilitia => f.write_str("HillsbradMilitia"),
            Self::Stormwind => f.write_str("Stormwind"),
            Self::Demon => f.write_str("Demon"),
            Self::Elemental => f.write_str("Elemental"),
            Self::Spirit => f.write_str("Spirit"),
            Self::Orgrimmar => f.write_str("Orgrimmar"),
            Self::Treasure => f.write_str("Treasure"),
            Self::GnollMudsnout => f.write_str("Gnoll - Mudsnout"),
            Self::HillsbradSouthshoreMayor => f.write_str("HIllsbrad, Southshore Mayor"),
            Self::DragonflightBlack => f.write_str("Dragonflight, Black"),
            Self::ThunderBluff => f.write_str("ThunderBluff"),
            Self::TrollWitherbark => f.write_str("Troll, Witherbark"),
            Self::LeatherworkingElemental => f.write_str("Leatherworking - Elemental"),
            Self::QuilboarRazormane => f.write_str("Quilboar, Razormane"),
            Self::QuilboarBristleback => f.write_str("Quilboar, Bristleback"),
            Self::LeatherworkingDragonscale => f.write_str("Leatherworking - Dragonscale"),
            Self::BloodsailBuccaneers => f.write_str("BloodsailBuccaneers"),
            Self::Blackfathom => f.write_str("Blackfathom"),
            Self::Makrura => f.write_str("Makrura"),
            Self::CentaurKolkar => f.write_str("Centaur, Kolkar"),
            Self::CentaurGalak => f.write_str("Centaur, Galak"),
            Self::GelkisClanCentaur => f.write_str("GelkisClanCentaur"),
            Self::MagramClanCentaur => f.write_str("MagramClanCentaur"),
            Self::Maraudine => f.write_str("Maraudine"),
            Self::Theramore => f.write_str("Theramore"),
            Self::QuilboarRazorfen => f.write_str("Quilboar, Razorfen"),
            Self::QuilboarRazormane2 => f.write_str("Quilboar, Razormane 2"),
            Self::QuilboarDeathshead => f.write_str("Quilboar, Deathshead"),
            Self::Enemy => f.write_str("Enemy"),
            Self::Ambient => f.write_str("Ambient"),
            Self::NethergardeCaravan => f.write_str("NethergardeCaravan"),
            Self::SteamwheedleCartel => f.write_str("SteamwheedleCartel"),
            Self::AllianceGeneric => f.write_str("AllianceGeneric"),
            Self::Nethergarde => f.write_str("Nethergarde"),
            Self::WailingCaverns => f.write_str("WailingCaverns"),
            Self::Silithid => f.write_str("Silithid"),
            Self::SilvermoonRemnant => f.write_str("SilvermoonRemnant"),
            Self::ZandalarTribe => f.write_str("ZandalarTribe"),
            Self::BlacksmithingWeaponsmithing => f.write_str("Blacksmithing - Weaponsmithing"),
            Self::Scorpid => f.write_str("Scorpid"),
            Self::BeastBat => f.write_str("Beast - Bat"),
            Self::Titan => f.write_str("Titan"),
            Self::TaskmasterFizzule => f.write_str("TaskmasterFizzule"),
            Self::Ravenholdt => f.write_str("Ravenholdt"),
            Self::Gadgetzan => f.write_str("Gadgetzan"),
            Self::GnomereganBug => f.write_str("GnomereganBug"),
            Self::Harpy => f.write_str("Harpy"),
            Self::BurningBlade => f.write_str("BurningBlade"),
            Self::ShadowsilkPoacher => f.write_str("ShadowsilkPoacher"),
            Self::SearingSpider => f.write_str("SearingSpider"),
            Self::Alliance => f.write_str("Alliance"),
            Self::Ratchet => f.write_str("Ratchet"),
            Self::WildhammerClan => f.write_str("WildhammerClan"),
            Self::GoblinDarkIronBarPatron => f.write_str("Goblin, Dark Iron Bar Patron"),
            Self::TheLeagueOfArathor => f.write_str("TheLeagueOfArathor"),
            Self::TheDefilers => f.write_str("TheDefilers"),
            Self::Giant => f.write_str("Giant"),
            Self::ArgentDawn => f.write_str("ArgentDawn"),
            Self::DarkspearTrolls => f.write_str("DarkspearTrolls"),
            Self::DragonflightBronze => f.write_str("Dragonflight, Bronze"),
            Self::DragonflightBlue => f.write_str("Dragonflight, Blue"),
            Self::LeatherworkingTribal => f.write_str("Leatherworking - Tribal"),
            Self::EngineeringGoblin => f.write_str("Engineering - Goblin"),
            Self::EngineeringGnome => f.write_str("Engineering - Gnome"),
            Self::BlacksmithingHammersmithing => f.write_str("Blacksmithing - Hammersmithing"),
            Self::BlacksmithingAxesmithing => f.write_str("Blacksmithing - Axesmithing"),
            Self::BlacksmithingSwordsmithing => f.write_str("Blacksmithing - Swordsmithing"),
            Self::TrollVilebranch => f.write_str("Troll, Vilebranch"),
            Self::SouthseaFreebooters => f.write_str("SouthseaFreebooters"),
            Self::CaerDarrow => f.write_str("CaerDarrow"),
            Self::FurbolgUncorrupted => f.write_str("Furbolg, Uncorrupted"),
            Self::TimbermawHold => f.write_str("TimbermawHold"),
            Self::Everlook => f.write_str("Everlook"),
            Self::WintersaberTrainers => f.write_str("WintersaberTrainers"),
            Self::CenarionCircle => f.write_str("CenarionCircle"),
            Self::ShatterspearTrolls => f.write_str("ShatterspearTrolls"),
            Self::RavasaurTrainers => f.write_str("RavasaurTrainers"),
            Self::MajordomoExecutus => f.write_str("MajordomoExecutus"),
            Self::BeastCarrionBird => f.write_str("Beast - Carrion Bird"),
            Self::BeastCat => f.write_str("Beast - Cat"),
            Self::BeastCrab => f.write_str("Beast - Crab"),
            Self::BeastCrocilisk => f.write_str("Beast - Crocilisk"),
            Self::BeastHyena => f.write_str("Beast - Hyena"),
            Self::BeastOwl => f.write_str("Beast - Owl"),
            Self::BeastScorpid => f.write_str("Beast - Scorpid"),
            Self::BeastTallstrider => f.write_str("Beast - Tallstrider"),
            Self::BeastTurtle => f.write_str("Beast - Turtle"),
            Self::BeastWindSerpent => f.write_str("Beast - Wind Serpent"),
            Self::TrainingDummy => f.write_str("TrainingDummy"),
            Self::DragonflightBlackBait => f.write_str("Dragonflight, Black - Bait"),
            Self::BattlegroundNeutral => f.write_str("BattlegroundNeutral"),
            Self::FrostwolfClan => f.write_str("FrostwolfClan"),
            Self::StormpikeGuard => f.write_str("StormpikeGuard"),
            Self::HydraxianWaterlords => f.write_str("HydraxianWaterlords"),
            Self::SulfuronFirelords => f.write_str("SulfuronFirelords"),
            Self::GizlocksDummy => f.write_str("Gizlock's Dummy"),
            Self::GizlocksCharm => f.write_str("Gizlock's Charm"),
            Self::Gizlock => f.write_str("Gizlock"),
            Self::MoroGai => f.write_str("Moro'gai"),
            Self::SpiritGuideAlliance => f.write_str("Spirit Guide - Alliance"),
            Self::ShenDralar => f.write_str("Shen'dralar"),
            Self::OgreCaptainKromcrush => f.write_str("Ogre (Captain Kromcrush)"),
            Self::SpiritGuideHorde => f.write_str("Spirit Guide - Horde"),
            Self::Jaedenar => f.write_str("Jaedenar"),
            Self::WarsongOutriders => f.write_str("WarsongOutriders"),
            Self::SilverwingSentinels => f.write_str("SilverwingSentinels"),
            Self::AllianceForces => f.write_str("AllianceForces"),
            Self::HordeForces => f.write_str("HordeForces"),
            Self::RevantuskTrolls => f.write_str("RevantuskTrolls"),
            Self::DarkmoonFaire => f.write_str("DarkmoonFaire"),
            Self::BroodOfNozdormu => f.write_str("BroodOfNozdormu"),
            Self::MightOfKalimdor => f.write_str("MightOfKalimdor"),
            Self::ArmiesOfCThun => f.write_str("Armies of C'Thun"),
            Self::SilithidAttackers => f.write_str("SilithidAttackers"),
            Self::TheIronforgeBrigade => f.write_str("TheIronforgeBrigade"),
            Self::RcEnemies => f.write_str("RcEnemies"),
            Self::RcObjects => f.write_str("RcObjects"),
            Self::Red => f.write_str("Red"),
            Self::Blue => f.write_str("Blue"),
            Self::ScourgeInvaders => f.write_str("ScourgeInvaders"),
            Self::TestFactionNotARealFaction => f.write_str("Test Faction (not a real faction)"),
            Self::TowowFlag => f.write_str("ToWoW - Flag"),
            Self::TowowFlagTriggerAllianceDnd => f.write_str("ToWoW - Flag Trigger Alliance (DND)"),
            Self::TowowFlagTriggerHordeDnd => f.write_str("ToWoW - Flag Trigger Horde (DND)"),
        }
    }
}

impl TryFrom<u16> for Faction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for Faction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for Faction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for Faction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for Faction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for Faction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for Faction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for Faction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for Faction {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

