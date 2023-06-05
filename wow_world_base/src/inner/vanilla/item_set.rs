/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/item_sets.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/item_sets.wowm#L1):
/// ```text
/// enum ItemSet : u16 {
///     NONE = 0;
///     THE_GLADIATOR = 1;
///     DAL_RENDS_ARMS = 41;
///     SPIDERS_KISS = 65;
///     THE_POSTMASTER = 81;
///     CADAVEROUS_GARB = 121;
///     NECROPILE_RAIMENT = 122;
///     BLOODMAIL_REGALIA = 123;
///     DEATHBONE_GUARDIAN = 124;
///     VOLCANIC_ARMOR = 141;
///     STORMSHROUD_ARMOR = 142;
///     DEVILSAUR_ARMOR = 143;
///     IRONFEATHER_ARMOR = 144;
///     DEFIAS_LEATHER = 161;
///     EMBRACE_OF_THE_VIPER = 162;
///     CHAIN_OF_THE_SCARLET_CRUSADE = 163;
///     MAGISTERS_REGALIA = 181;
///     VESTMENTS_OF_THE_DEVOUT = 182;
///     DREADMIST_RAIMENT = 183;
///     SHADOWCRAFT_ARMOR = 184;
///     WILDHEART_RAIMENT = 185;
///     BEASTSTALKER_ARMOR = 186;
///     THE_ELEMENTS = 187;
///     LIGHTFORGE_ARMOR = 188;
///     BATTLEGEAR_OF_VALOR = 189;
///     ARCANIST_REGALIA = 201;
///     VESTMENTS_OF_PROPHECY = 202;
///     FELHEART_RAIMENT = 203;
///     NIGHTSLAYER_ARMOR = 204;
///     CENARION_RAIMENT = 205;
///     GIANTSTALKER_ARMOR = 206;
///     THE_EARTHFURY = 207;
///     LAWBRINGER_ARMOR = 208;
///     BATTLEGEAR_OF_MIGHT = 209;
///     NETHERWIND_REGALIA = 210;
///     VESTMENTS_OF_TRANSCENDENCE = 211;
///     NEMESIS_RAIMENT = 212;
///     BLOODFANG_ARMOR = 213;
///     STORMRAGE_RAIMENT = 214;
///     DRAGONSTALKER_ARMOR = 215;
///     THE_TEN_STORMS = 216;
///     JUDGEMENT_ARMOR = 217;
///     BATTLEGEAR_OF_WRATH = 218;
///     GARB_OF_THERO_SHAN = 221;
///     SHARD_OF_THE_GODS = 241;
///     SPIRIT_OF_ESKHANDAR = 261;
///     CHAMPIONS_BATTLEGEAR = 281;
///     LIEUTENANT_COMMANDERS_BATTLEGEAR = 282;
///     CHAMPIONS_EARTHSHAKER = 301;
///     IMPERIAL_PLATE = 321;
///     CHAMPIONS_REGALIA = 341;
///     CHAMPIONS_RAIMENT = 342;
///     LIEUTENANT_COMMANDERS_REGALIA = 343;
///     LIEUTENANT_COMMANDERS_RAIMENT = 344;
///     CHAMPIONS_THREADS = 345;
///     LIEUTENANT_COMMANDERS_THREADS = 346;
///     CHAMPIONS_VESTMENTS = 347;
///     LIEUTENANT_COMMANDERS_VESTMENTS = 348;
///     CHAMPIONS_PURSUIT = 361;
///     LIEUTENANT_COMMANDERS_PURSUIT = 362;
///     LIEUTENANT_COMMANDERS_SANCTUARY = 381;
///     CHAMPIONS_SANCTUARY = 382;
///     WARLORDS_BATTLEGEAR = 383;
///     FIELD_MARSHALS_BATTLEGEAR = 384;
///     WARLORDS_EARTHSHAKER = 386;
///     WARLORDS_REGALIA = 387;
///     FIELD_MARSHALS_REGALIA = 388;
///     FIELD_MARSHALS_RAIMENT = 389;
///     WARLORDS_RAIMENT = 390;
///     WARLORDS_THREADS = 391;
///     FIELD_MARSHALS_THREADS = 392;
///     WARLORDS_VESTMENTS = 393;
///     FIELD_MARSHALS_VESTMENTS = 394;
///     FIELD_MARSHALS_PURSUIT = 395;
///     WARLORDS_PURSUIT = 396;
///     FIELD_MARSHALS_SANCTUARY = 397;
///     WARLORDS_SANCTUARY = 398;
///     LIEUTENANT_COMMANDERS_AEGIS = 401;
///     FIELD_MARSHALS_AEGIS = 402;
///     BLOODVINE_GARB = 421;
///     PRIMAL_BATSKIN = 441;
///     BLOOD_TIGER_HARNESS = 442;
///     BLOODSOUL_EMBRACE = 443;
///     THE_DARKSOUL = 444;
///     THE_TWIN_BLADES_OF_HAKKARI = 461;
///     ZANZILS_CONCENTRATION = 462;
///     PRIMAL_BLESSING = 463;
///     OVERLORDS_RESOLUTION = 464;
///     PRAYER_OF_THE_PRIMAL = 465;
///     MAJOR_MOJO_INFUSION = 466;
///     THE_HIGHLANDERS_RESOLUTION = 467;
///     THE_HIGHLANDERS_RESOLVE = 468;
///     THE_HIGHLANDERS_DETERMINATION = 469;
///     THE_HIGHLANDERS_FORTITUDE = 470;
///     THE_HIGHLANDERS_PURPOSE = 471;
///     THE_HIGHLANDERS_WILL = 472;
///     THE_HIGHLANDERS_INTENT = 473;
///     VINDICATORS_BATTLEGEAR = 474;
///     FREETHINKERS_ARMOR = 475;
///     AUGURS_REGALIA = 476;
///     PREDATORS_ARMOR = 477;
///     MADCAPS_OUTFIT = 478;
///     HARUSPEXS_GARB = 479;
///     CONFESSORS_RAIMENT = 480;
///     DEMONIACS_THREADS = 481;
///     ILLUSIONISTS_ATTIRE = 482;
///     THE_DEFILERS_DETERMINATION = 483;
///     THE_DEFILERS_FORTITUDE = 484;
///     THE_DEFILERS_INTENT = 485;
///     THE_DEFILERS_PURPOSE = 486;
///     THE_DEFILERS_RESOLUTION = 487;
///     THE_DEFILERS_WILL = 488;
///     BLACK_DRAGON_MAIL = 489;
///     GREEN_DRAGON_MAIL = 490;
///     BLUE_DRAGON_MAIL = 491;
///     TWILIGHT_TRAPPINGS = 492;
///     GENESIS_RAIMENT = 493;
///     SYMBOLS_OF_UNENDING_LIFE = 494;
///     BATTLEGEAR_OF_UNYIELDING_STRENGTH = 495;
///     CONQUERORS_BATTLEGEAR = 496;
///     DEATHDEALERS_EMBRACE = 497;
///     EMBLEMS_OF_VEILED_SHADOWS = 498;
///     DOOMCALLERS_ATTIRE = 499;
///     IMPLEMENTS_OF_UNSPOKEN_NAMES = 500;
///     STORMCALLERS_GARB = 501;
///     GIFT_OF_THE_GATHERING_STORM = 502;
///     ENIGMA_VESTMENTS = 503;
///     TRAPPINGS_OF_VAULTED_SECRETS = 504;
///     AVENGERS_BATTLEGEAR = 505;
///     BATTLEGEAR_OF_ETERNAL_JUSTICE = 506;
///     GARMENTS_OF_THE_ORACLE = 507;
///     FINERY_OF_INFINITE_WISDOM = 508;
///     STRIKERS_GARB = 509;
///     TRAPPINGS_OF_THE_UNSEEN_PATH = 510;
///     BATTLEGEAR_OF_HEROISM = 511;
///     DARKMANTLE_ARMOR = 512;
///     FERALHEART_RAIMENT = 513;
///     VESTMENTS_OF_THE_VIRTUOUS = 514;
///     BEASTMASTER_ARMOR = 515;
///     SOULFORGE_ARMOR = 516;
///     SORCERERS_REGALIA = 517;
///     DEATHMIST_RAIMENT = 518;
///     THE_FIVE_THUNDERS = 519;
///     IRONWEAVE_BATTLESUIT = 520;
///     DREAMWALKER_RAIMENT = 521;
///     CHAMPIONS_GUARD = 522;
///     DREADNAUGHTS_BATTLEGEAR = 523;
///     BONESCYTHE_ARMOR = 524;
///     VESTMENTS_OF_FAITH = 525;
///     FROSTFIRE_REGALIA = 526;
///     THE_EARTHSHATTERER = 527;
///     REDEMPTION_ARMOR = 528;
///     PLAGUEHEART_RAIMENT = 529;
///     CRYPTSTALKER_ARMOR = 530;
///     BATTLEGEAR_OF_UNDEAD_SLAYING = 533;
///     UNDEAD_SLAYERS_ARMOR = 534;
///     GARB_OF_THE_UNDEAD_SLAYER = 535;
///     REGALIA_OF_UNDEAD_CLEANSING = 536;
///     CHAMPIONS_BATTLEARMOR = 537;
///     CHAMPIONS_STORMCALLER = 538;
///     CHAMPIONS_REFUGE = 539;
///     CHAMPIONS_INVESTITURE = 540;
///     CHAMPIONS_DREADGEAR = 541;
///     CHAMPIONS_ARCANUM = 542;
///     CHAMPIONS_PURSUANCE = 543;
///     LIEUTENANT_COMMANDERS_REDOUBT = 544;
///     LIEUTENANT_COMMANDERS_BATTLEARMOR = 545;
///     LIEUTENANT_COMMANDERS_ARCANUM = 546;
///     LIEUTENANT_COMMANDERS_DREADGEAR = 547;
///     LIEUTENANT_COMMANDERS_GUARD = 548;
///     LIEUTENANT_COMMANDERS_INVESTITURE = 549;
///     LIEUTENANT_COMMANDERS_PURSUANCE = 550;
///     LIEUTENANT_COMMANDERS_REFUGE = 551;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemSet {
    None,
    TheGladiator,
    DalRendsArms,
    SpidersKiss,
    ThePostmaster,
    CadaverousGarb,
    NecropileRaiment,
    BloodmailRegalia,
    DeathboneGuardian,
    VolcanicArmor,
    StormshroudArmor,
    DevilsaurArmor,
    IronfeatherArmor,
    DefiasLeather,
    EmbraceOfTheViper,
    ChainOfTheScarletCrusade,
    MagistersRegalia,
    VestmentsOfTheDevout,
    DreadmistRaiment,
    ShadowcraftArmor,
    WildheartRaiment,
    BeaststalkerArmor,
    TheElements,
    LightforgeArmor,
    BattlegearOfValor,
    ArcanistRegalia,
    VestmentsOfProphecy,
    FelheartRaiment,
    NightslayerArmor,
    CenarionRaiment,
    GiantstalkerArmor,
    TheEarthfury,
    LawbringerArmor,
    BattlegearOfMight,
    NetherwindRegalia,
    VestmentsOfTranscendence,
    NemesisRaiment,
    BloodfangArmor,
    StormrageRaiment,
    DragonstalkerArmor,
    TheTenStorms,
    JudgementArmor,
    BattlegearOfWrath,
    GarbOfTheroShan,
    ShardOfTheGods,
    SpiritOfEskhandar,
    ChampionsBattlegear,
    LieutenantCommandersBattlegear,
    ChampionsEarthshaker,
    ImperialPlate,
    ChampionsRegalia,
    ChampionsRaiment,
    LieutenantCommandersRegalia,
    LieutenantCommandersRaiment,
    ChampionsThreads,
    LieutenantCommandersThreads,
    ChampionsVestments,
    LieutenantCommandersVestments,
    ChampionsPursuit,
    LieutenantCommandersPursuit,
    LieutenantCommandersSanctuary,
    ChampionsSanctuary,
    WarlordsBattlegear,
    FieldMarshalsBattlegear,
    WarlordsEarthshaker,
    WarlordsRegalia,
    FieldMarshalsRegalia,
    FieldMarshalsRaiment,
    WarlordsRaiment,
    WarlordsThreads,
    FieldMarshalsThreads,
    WarlordsVestments,
    FieldMarshalsVestments,
    FieldMarshalsPursuit,
    WarlordsPursuit,
    FieldMarshalsSanctuary,
    WarlordsSanctuary,
    LieutenantCommandersAegis,
    FieldMarshalsAegis,
    BloodvineGarb,
    PrimalBatskin,
    BloodTigerHarness,
    BloodsoulEmbrace,
    TheDarksoul,
    TheTwinBladesOfHakkari,
    ZanzilsConcentration,
    PrimalBlessing,
    OverlordsResolution,
    PrayerOfThePrimal,
    MajorMojoInfusion,
    TheHighlandersResolution,
    TheHighlandersResolve,
    TheHighlandersDetermination,
    TheHighlandersFortitude,
    TheHighlandersPurpose,
    TheHighlandersWill,
    TheHighlandersIntent,
    VindicatorsBattlegear,
    FreethinkersArmor,
    AugursRegalia,
    PredatorsArmor,
    MadcapsOutfit,
    HaruspexsGarb,
    ConfessorsRaiment,
    DemoniacsThreads,
    IllusionistsAttire,
    TheDefilersDetermination,
    TheDefilersFortitude,
    TheDefilersIntent,
    TheDefilersPurpose,
    TheDefilersResolution,
    TheDefilersWill,
    BlackDragonMail,
    GreenDragonMail,
    BlueDragonMail,
    TwilightTrappings,
    GenesisRaiment,
    SymbolsOfUnendingLife,
    BattlegearOfUnyieldingStrength,
    ConquerorsBattlegear,
    DeathdealersEmbrace,
    EmblemsOfVeiledShadows,
    DoomcallersAttire,
    ImplementsOfUnspokenNames,
    StormcallersGarb,
    GiftOfTheGatheringStorm,
    EnigmaVestments,
    TrappingsOfVaultedSecrets,
    AvengersBattlegear,
    BattlegearOfEternalJustice,
    GarmentsOfTheOracle,
    FineryOfInfiniteWisdom,
    StrikersGarb,
    TrappingsOfTheUnseenPath,
    BattlegearOfHeroism,
    DarkmantleArmor,
    FeralheartRaiment,
    VestmentsOfTheVirtuous,
    BeastmasterArmor,
    SoulforgeArmor,
    SorcerersRegalia,
    DeathmistRaiment,
    TheFiveThunders,
    IronweaveBattlesuit,
    DreamwalkerRaiment,
    ChampionsGuard,
    DreadnaughtsBattlegear,
    BonescytheArmor,
    VestmentsOfFaith,
    FrostfireRegalia,
    TheEarthshatterer,
    RedemptionArmor,
    PlagueheartRaiment,
    CryptstalkerArmor,
    BattlegearOfUndeadSlaying,
    UndeadSlayersArmor,
    GarbOfTheUndeadSlayer,
    RegaliaOfUndeadCleansing,
    ChampionsBattlearmor,
    ChampionsStormcaller,
    ChampionsRefuge,
    ChampionsInvestiture,
    ChampionsDreadgear,
    ChampionsArcanum,
    ChampionsPursuance,
    LieutenantCommandersRedoubt,
    LieutenantCommandersBattlearmor,
    LieutenantCommandersArcanum,
    LieutenantCommandersDreadgear,
    LieutenantCommandersGuard,
    LieutenantCommandersInvestiture,
    LieutenantCommandersPursuance,
    LieutenantCommandersRefuge,
}

impl ItemSet {
    pub const fn as_int(&self) -> u16 {
        match self {
            Self::None => 0x0,
            Self::TheGladiator => 0x1,
            Self::DalRendsArms => 0x29,
            Self::SpidersKiss => 0x41,
            Self::ThePostmaster => 0x51,
            Self::CadaverousGarb => 0x79,
            Self::NecropileRaiment => 0x7a,
            Self::BloodmailRegalia => 0x7b,
            Self::DeathboneGuardian => 0x7c,
            Self::VolcanicArmor => 0x8d,
            Self::StormshroudArmor => 0x8e,
            Self::DevilsaurArmor => 0x8f,
            Self::IronfeatherArmor => 0x90,
            Self::DefiasLeather => 0xa1,
            Self::EmbraceOfTheViper => 0xa2,
            Self::ChainOfTheScarletCrusade => 0xa3,
            Self::MagistersRegalia => 0xb5,
            Self::VestmentsOfTheDevout => 0xb6,
            Self::DreadmistRaiment => 0xb7,
            Self::ShadowcraftArmor => 0xb8,
            Self::WildheartRaiment => 0xb9,
            Self::BeaststalkerArmor => 0xba,
            Self::TheElements => 0xbb,
            Self::LightforgeArmor => 0xbc,
            Self::BattlegearOfValor => 0xbd,
            Self::ArcanistRegalia => 0xc9,
            Self::VestmentsOfProphecy => 0xca,
            Self::FelheartRaiment => 0xcb,
            Self::NightslayerArmor => 0xcc,
            Self::CenarionRaiment => 0xcd,
            Self::GiantstalkerArmor => 0xce,
            Self::TheEarthfury => 0xcf,
            Self::LawbringerArmor => 0xd0,
            Self::BattlegearOfMight => 0xd1,
            Self::NetherwindRegalia => 0xd2,
            Self::VestmentsOfTranscendence => 0xd3,
            Self::NemesisRaiment => 0xd4,
            Self::BloodfangArmor => 0xd5,
            Self::StormrageRaiment => 0xd6,
            Self::DragonstalkerArmor => 0xd7,
            Self::TheTenStorms => 0xd8,
            Self::JudgementArmor => 0xd9,
            Self::BattlegearOfWrath => 0xda,
            Self::GarbOfTheroShan => 0xdd,
            Self::ShardOfTheGods => 0xf1,
            Self::SpiritOfEskhandar => 0x105,
            Self::ChampionsBattlegear => 0x119,
            Self::LieutenantCommandersBattlegear => 0x11a,
            Self::ChampionsEarthshaker => 0x12d,
            Self::ImperialPlate => 0x141,
            Self::ChampionsRegalia => 0x155,
            Self::ChampionsRaiment => 0x156,
            Self::LieutenantCommandersRegalia => 0x157,
            Self::LieutenantCommandersRaiment => 0x158,
            Self::ChampionsThreads => 0x159,
            Self::LieutenantCommandersThreads => 0x15a,
            Self::ChampionsVestments => 0x15b,
            Self::LieutenantCommandersVestments => 0x15c,
            Self::ChampionsPursuit => 0x169,
            Self::LieutenantCommandersPursuit => 0x16a,
            Self::LieutenantCommandersSanctuary => 0x17d,
            Self::ChampionsSanctuary => 0x17e,
            Self::WarlordsBattlegear => 0x17f,
            Self::FieldMarshalsBattlegear => 0x180,
            Self::WarlordsEarthshaker => 0x182,
            Self::WarlordsRegalia => 0x183,
            Self::FieldMarshalsRegalia => 0x184,
            Self::FieldMarshalsRaiment => 0x185,
            Self::WarlordsRaiment => 0x186,
            Self::WarlordsThreads => 0x187,
            Self::FieldMarshalsThreads => 0x188,
            Self::WarlordsVestments => 0x189,
            Self::FieldMarshalsVestments => 0x18a,
            Self::FieldMarshalsPursuit => 0x18b,
            Self::WarlordsPursuit => 0x18c,
            Self::FieldMarshalsSanctuary => 0x18d,
            Self::WarlordsSanctuary => 0x18e,
            Self::LieutenantCommandersAegis => 0x191,
            Self::FieldMarshalsAegis => 0x192,
            Self::BloodvineGarb => 0x1a5,
            Self::PrimalBatskin => 0x1b9,
            Self::BloodTigerHarness => 0x1ba,
            Self::BloodsoulEmbrace => 0x1bb,
            Self::TheDarksoul => 0x1bc,
            Self::TheTwinBladesOfHakkari => 0x1cd,
            Self::ZanzilsConcentration => 0x1ce,
            Self::PrimalBlessing => 0x1cf,
            Self::OverlordsResolution => 0x1d0,
            Self::PrayerOfThePrimal => 0x1d1,
            Self::MajorMojoInfusion => 0x1d2,
            Self::TheHighlandersResolution => 0x1d3,
            Self::TheHighlandersResolve => 0x1d4,
            Self::TheHighlandersDetermination => 0x1d5,
            Self::TheHighlandersFortitude => 0x1d6,
            Self::TheHighlandersPurpose => 0x1d7,
            Self::TheHighlandersWill => 0x1d8,
            Self::TheHighlandersIntent => 0x1d9,
            Self::VindicatorsBattlegear => 0x1da,
            Self::FreethinkersArmor => 0x1db,
            Self::AugursRegalia => 0x1dc,
            Self::PredatorsArmor => 0x1dd,
            Self::MadcapsOutfit => 0x1de,
            Self::HaruspexsGarb => 0x1df,
            Self::ConfessorsRaiment => 0x1e0,
            Self::DemoniacsThreads => 0x1e1,
            Self::IllusionistsAttire => 0x1e2,
            Self::TheDefilersDetermination => 0x1e3,
            Self::TheDefilersFortitude => 0x1e4,
            Self::TheDefilersIntent => 0x1e5,
            Self::TheDefilersPurpose => 0x1e6,
            Self::TheDefilersResolution => 0x1e7,
            Self::TheDefilersWill => 0x1e8,
            Self::BlackDragonMail => 0x1e9,
            Self::GreenDragonMail => 0x1ea,
            Self::BlueDragonMail => 0x1eb,
            Self::TwilightTrappings => 0x1ec,
            Self::GenesisRaiment => 0x1ed,
            Self::SymbolsOfUnendingLife => 0x1ee,
            Self::BattlegearOfUnyieldingStrength => 0x1ef,
            Self::ConquerorsBattlegear => 0x1f0,
            Self::DeathdealersEmbrace => 0x1f1,
            Self::EmblemsOfVeiledShadows => 0x1f2,
            Self::DoomcallersAttire => 0x1f3,
            Self::ImplementsOfUnspokenNames => 0x1f4,
            Self::StormcallersGarb => 0x1f5,
            Self::GiftOfTheGatheringStorm => 0x1f6,
            Self::EnigmaVestments => 0x1f7,
            Self::TrappingsOfVaultedSecrets => 0x1f8,
            Self::AvengersBattlegear => 0x1f9,
            Self::BattlegearOfEternalJustice => 0x1fa,
            Self::GarmentsOfTheOracle => 0x1fb,
            Self::FineryOfInfiniteWisdom => 0x1fc,
            Self::StrikersGarb => 0x1fd,
            Self::TrappingsOfTheUnseenPath => 0x1fe,
            Self::BattlegearOfHeroism => 0x1ff,
            Self::DarkmantleArmor => 0x200,
            Self::FeralheartRaiment => 0x201,
            Self::VestmentsOfTheVirtuous => 0x202,
            Self::BeastmasterArmor => 0x203,
            Self::SoulforgeArmor => 0x204,
            Self::SorcerersRegalia => 0x205,
            Self::DeathmistRaiment => 0x206,
            Self::TheFiveThunders => 0x207,
            Self::IronweaveBattlesuit => 0x208,
            Self::DreamwalkerRaiment => 0x209,
            Self::ChampionsGuard => 0x20a,
            Self::DreadnaughtsBattlegear => 0x20b,
            Self::BonescytheArmor => 0x20c,
            Self::VestmentsOfFaith => 0x20d,
            Self::FrostfireRegalia => 0x20e,
            Self::TheEarthshatterer => 0x20f,
            Self::RedemptionArmor => 0x210,
            Self::PlagueheartRaiment => 0x211,
            Self::CryptstalkerArmor => 0x212,
            Self::BattlegearOfUndeadSlaying => 0x215,
            Self::UndeadSlayersArmor => 0x216,
            Self::GarbOfTheUndeadSlayer => 0x217,
            Self::RegaliaOfUndeadCleansing => 0x218,
            Self::ChampionsBattlearmor => 0x219,
            Self::ChampionsStormcaller => 0x21a,
            Self::ChampionsRefuge => 0x21b,
            Self::ChampionsInvestiture => 0x21c,
            Self::ChampionsDreadgear => 0x21d,
            Self::ChampionsArcanum => 0x21e,
            Self::ChampionsPursuance => 0x21f,
            Self::LieutenantCommandersRedoubt => 0x220,
            Self::LieutenantCommandersBattlearmor => 0x221,
            Self::LieutenantCommandersArcanum => 0x222,
            Self::LieutenantCommandersDreadgear => 0x223,
            Self::LieutenantCommandersGuard => 0x224,
            Self::LieutenantCommandersInvestiture => 0x225,
            Self::LieutenantCommandersPursuance => 0x226,
            Self::LieutenantCommandersRefuge => 0x227,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl ItemSet {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::TheGladiator => "THE_GLADIATOR",
            Self::DalRendsArms => "DAL_RENDS_ARMS",
            Self::SpidersKiss => "SPIDERS_KISS",
            Self::ThePostmaster => "THE_POSTMASTER",
            Self::CadaverousGarb => "CADAVEROUS_GARB",
            Self::NecropileRaiment => "NECROPILE_RAIMENT",
            Self::BloodmailRegalia => "BLOODMAIL_REGALIA",
            Self::DeathboneGuardian => "DEATHBONE_GUARDIAN",
            Self::VolcanicArmor => "VOLCANIC_ARMOR",
            Self::StormshroudArmor => "STORMSHROUD_ARMOR",
            Self::DevilsaurArmor => "DEVILSAUR_ARMOR",
            Self::IronfeatherArmor => "IRONFEATHER_ARMOR",
            Self::DefiasLeather => "DEFIAS_LEATHER",
            Self::EmbraceOfTheViper => "EMBRACE_OF_THE_VIPER",
            Self::ChainOfTheScarletCrusade => "CHAIN_OF_THE_SCARLET_CRUSADE",
            Self::MagistersRegalia => "MAGISTERS_REGALIA",
            Self::VestmentsOfTheDevout => "VESTMENTS_OF_THE_DEVOUT",
            Self::DreadmistRaiment => "DREADMIST_RAIMENT",
            Self::ShadowcraftArmor => "SHADOWCRAFT_ARMOR",
            Self::WildheartRaiment => "WILDHEART_RAIMENT",
            Self::BeaststalkerArmor => "BEASTSTALKER_ARMOR",
            Self::TheElements => "THE_ELEMENTS",
            Self::LightforgeArmor => "LIGHTFORGE_ARMOR",
            Self::BattlegearOfValor => "BATTLEGEAR_OF_VALOR",
            Self::ArcanistRegalia => "ARCANIST_REGALIA",
            Self::VestmentsOfProphecy => "VESTMENTS_OF_PROPHECY",
            Self::FelheartRaiment => "FELHEART_RAIMENT",
            Self::NightslayerArmor => "NIGHTSLAYER_ARMOR",
            Self::CenarionRaiment => "CENARION_RAIMENT",
            Self::GiantstalkerArmor => "GIANTSTALKER_ARMOR",
            Self::TheEarthfury => "THE_EARTHFURY",
            Self::LawbringerArmor => "LAWBRINGER_ARMOR",
            Self::BattlegearOfMight => "BATTLEGEAR_OF_MIGHT",
            Self::NetherwindRegalia => "NETHERWIND_REGALIA",
            Self::VestmentsOfTranscendence => "VESTMENTS_OF_TRANSCENDENCE",
            Self::NemesisRaiment => "NEMESIS_RAIMENT",
            Self::BloodfangArmor => "BLOODFANG_ARMOR",
            Self::StormrageRaiment => "STORMRAGE_RAIMENT",
            Self::DragonstalkerArmor => "DRAGONSTALKER_ARMOR",
            Self::TheTenStorms => "THE_TEN_STORMS",
            Self::JudgementArmor => "JUDGEMENT_ARMOR",
            Self::BattlegearOfWrath => "BATTLEGEAR_OF_WRATH",
            Self::GarbOfTheroShan => "GARB_OF_THERO_SHAN",
            Self::ShardOfTheGods => "SHARD_OF_THE_GODS",
            Self::SpiritOfEskhandar => "SPIRIT_OF_ESKHANDAR",
            Self::ChampionsBattlegear => "CHAMPIONS_BATTLEGEAR",
            Self::LieutenantCommandersBattlegear => "LIEUTENANT_COMMANDERS_BATTLEGEAR",
            Self::ChampionsEarthshaker => "CHAMPIONS_EARTHSHAKER",
            Self::ImperialPlate => "IMPERIAL_PLATE",
            Self::ChampionsRegalia => "CHAMPIONS_REGALIA",
            Self::ChampionsRaiment => "CHAMPIONS_RAIMENT",
            Self::LieutenantCommandersRegalia => "LIEUTENANT_COMMANDERS_REGALIA",
            Self::LieutenantCommandersRaiment => "LIEUTENANT_COMMANDERS_RAIMENT",
            Self::ChampionsThreads => "CHAMPIONS_THREADS",
            Self::LieutenantCommandersThreads => "LIEUTENANT_COMMANDERS_THREADS",
            Self::ChampionsVestments => "CHAMPIONS_VESTMENTS",
            Self::LieutenantCommandersVestments => "LIEUTENANT_COMMANDERS_VESTMENTS",
            Self::ChampionsPursuit => "CHAMPIONS_PURSUIT",
            Self::LieutenantCommandersPursuit => "LIEUTENANT_COMMANDERS_PURSUIT",
            Self::LieutenantCommandersSanctuary => "LIEUTENANT_COMMANDERS_SANCTUARY",
            Self::ChampionsSanctuary => "CHAMPIONS_SANCTUARY",
            Self::WarlordsBattlegear => "WARLORDS_BATTLEGEAR",
            Self::FieldMarshalsBattlegear => "FIELD_MARSHALS_BATTLEGEAR",
            Self::WarlordsEarthshaker => "WARLORDS_EARTHSHAKER",
            Self::WarlordsRegalia => "WARLORDS_REGALIA",
            Self::FieldMarshalsRegalia => "FIELD_MARSHALS_REGALIA",
            Self::FieldMarshalsRaiment => "FIELD_MARSHALS_RAIMENT",
            Self::WarlordsRaiment => "WARLORDS_RAIMENT",
            Self::WarlordsThreads => "WARLORDS_THREADS",
            Self::FieldMarshalsThreads => "FIELD_MARSHALS_THREADS",
            Self::WarlordsVestments => "WARLORDS_VESTMENTS",
            Self::FieldMarshalsVestments => "FIELD_MARSHALS_VESTMENTS",
            Self::FieldMarshalsPursuit => "FIELD_MARSHALS_PURSUIT",
            Self::WarlordsPursuit => "WARLORDS_PURSUIT",
            Self::FieldMarshalsSanctuary => "FIELD_MARSHALS_SANCTUARY",
            Self::WarlordsSanctuary => "WARLORDS_SANCTUARY",
            Self::LieutenantCommandersAegis => "LIEUTENANT_COMMANDERS_AEGIS",
            Self::FieldMarshalsAegis => "FIELD_MARSHALS_AEGIS",
            Self::BloodvineGarb => "BLOODVINE_GARB",
            Self::PrimalBatskin => "PRIMAL_BATSKIN",
            Self::BloodTigerHarness => "BLOOD_TIGER_HARNESS",
            Self::BloodsoulEmbrace => "BLOODSOUL_EMBRACE",
            Self::TheDarksoul => "THE_DARKSOUL",
            Self::TheTwinBladesOfHakkari => "THE_TWIN_BLADES_OF_HAKKARI",
            Self::ZanzilsConcentration => "ZANZILS_CONCENTRATION",
            Self::PrimalBlessing => "PRIMAL_BLESSING",
            Self::OverlordsResolution => "OVERLORDS_RESOLUTION",
            Self::PrayerOfThePrimal => "PRAYER_OF_THE_PRIMAL",
            Self::MajorMojoInfusion => "MAJOR_MOJO_INFUSION",
            Self::TheHighlandersResolution => "THE_HIGHLANDERS_RESOLUTION",
            Self::TheHighlandersResolve => "THE_HIGHLANDERS_RESOLVE",
            Self::TheHighlandersDetermination => "THE_HIGHLANDERS_DETERMINATION",
            Self::TheHighlandersFortitude => "THE_HIGHLANDERS_FORTITUDE",
            Self::TheHighlandersPurpose => "THE_HIGHLANDERS_PURPOSE",
            Self::TheHighlandersWill => "THE_HIGHLANDERS_WILL",
            Self::TheHighlandersIntent => "THE_HIGHLANDERS_INTENT",
            Self::VindicatorsBattlegear => "VINDICATORS_BATTLEGEAR",
            Self::FreethinkersArmor => "FREETHINKERS_ARMOR",
            Self::AugursRegalia => "AUGURS_REGALIA",
            Self::PredatorsArmor => "PREDATORS_ARMOR",
            Self::MadcapsOutfit => "MADCAPS_OUTFIT",
            Self::HaruspexsGarb => "HARUSPEXS_GARB",
            Self::ConfessorsRaiment => "CONFESSORS_RAIMENT",
            Self::DemoniacsThreads => "DEMONIACS_THREADS",
            Self::IllusionistsAttire => "ILLUSIONISTS_ATTIRE",
            Self::TheDefilersDetermination => "THE_DEFILERS_DETERMINATION",
            Self::TheDefilersFortitude => "THE_DEFILERS_FORTITUDE",
            Self::TheDefilersIntent => "THE_DEFILERS_INTENT",
            Self::TheDefilersPurpose => "THE_DEFILERS_PURPOSE",
            Self::TheDefilersResolution => "THE_DEFILERS_RESOLUTION",
            Self::TheDefilersWill => "THE_DEFILERS_WILL",
            Self::BlackDragonMail => "BLACK_DRAGON_MAIL",
            Self::GreenDragonMail => "GREEN_DRAGON_MAIL",
            Self::BlueDragonMail => "BLUE_DRAGON_MAIL",
            Self::TwilightTrappings => "TWILIGHT_TRAPPINGS",
            Self::GenesisRaiment => "GENESIS_RAIMENT",
            Self::SymbolsOfUnendingLife => "SYMBOLS_OF_UNENDING_LIFE",
            Self::BattlegearOfUnyieldingStrength => "BATTLEGEAR_OF_UNYIELDING_STRENGTH",
            Self::ConquerorsBattlegear => "CONQUERORS_BATTLEGEAR",
            Self::DeathdealersEmbrace => "DEATHDEALERS_EMBRACE",
            Self::EmblemsOfVeiledShadows => "EMBLEMS_OF_VEILED_SHADOWS",
            Self::DoomcallersAttire => "DOOMCALLERS_ATTIRE",
            Self::ImplementsOfUnspokenNames => "IMPLEMENTS_OF_UNSPOKEN_NAMES",
            Self::StormcallersGarb => "STORMCALLERS_GARB",
            Self::GiftOfTheGatheringStorm => "GIFT_OF_THE_GATHERING_STORM",
            Self::EnigmaVestments => "ENIGMA_VESTMENTS",
            Self::TrappingsOfVaultedSecrets => "TRAPPINGS_OF_VAULTED_SECRETS",
            Self::AvengersBattlegear => "AVENGERS_BATTLEGEAR",
            Self::BattlegearOfEternalJustice => "BATTLEGEAR_OF_ETERNAL_JUSTICE",
            Self::GarmentsOfTheOracle => "GARMENTS_OF_THE_ORACLE",
            Self::FineryOfInfiniteWisdom => "FINERY_OF_INFINITE_WISDOM",
            Self::StrikersGarb => "STRIKERS_GARB",
            Self::TrappingsOfTheUnseenPath => "TRAPPINGS_OF_THE_UNSEEN_PATH",
            Self::BattlegearOfHeroism => "BATTLEGEAR_OF_HEROISM",
            Self::DarkmantleArmor => "DARKMANTLE_ARMOR",
            Self::FeralheartRaiment => "FERALHEART_RAIMENT",
            Self::VestmentsOfTheVirtuous => "VESTMENTS_OF_THE_VIRTUOUS",
            Self::BeastmasterArmor => "BEASTMASTER_ARMOR",
            Self::SoulforgeArmor => "SOULFORGE_ARMOR",
            Self::SorcerersRegalia => "SORCERERS_REGALIA",
            Self::DeathmistRaiment => "DEATHMIST_RAIMENT",
            Self::TheFiveThunders => "THE_FIVE_THUNDERS",
            Self::IronweaveBattlesuit => "IRONWEAVE_BATTLESUIT",
            Self::DreamwalkerRaiment => "DREAMWALKER_RAIMENT",
            Self::ChampionsGuard => "CHAMPIONS_GUARD",
            Self::DreadnaughtsBattlegear => "DREADNAUGHTS_BATTLEGEAR",
            Self::BonescytheArmor => "BONESCYTHE_ARMOR",
            Self::VestmentsOfFaith => "VESTMENTS_OF_FAITH",
            Self::FrostfireRegalia => "FROSTFIRE_REGALIA",
            Self::TheEarthshatterer => "THE_EARTHSHATTERER",
            Self::RedemptionArmor => "REDEMPTION_ARMOR",
            Self::PlagueheartRaiment => "PLAGUEHEART_RAIMENT",
            Self::CryptstalkerArmor => "CRYPTSTALKER_ARMOR",
            Self::BattlegearOfUndeadSlaying => "BATTLEGEAR_OF_UNDEAD_SLAYING",
            Self::UndeadSlayersArmor => "UNDEAD_SLAYERS_ARMOR",
            Self::GarbOfTheUndeadSlayer => "GARB_OF_THE_UNDEAD_SLAYER",
            Self::RegaliaOfUndeadCleansing => "REGALIA_OF_UNDEAD_CLEANSING",
            Self::ChampionsBattlearmor => "CHAMPIONS_BATTLEARMOR",
            Self::ChampionsStormcaller => "CHAMPIONS_STORMCALLER",
            Self::ChampionsRefuge => "CHAMPIONS_REFUGE",
            Self::ChampionsInvestiture => "CHAMPIONS_INVESTITURE",
            Self::ChampionsDreadgear => "CHAMPIONS_DREADGEAR",
            Self::ChampionsArcanum => "CHAMPIONS_ARCANUM",
            Self::ChampionsPursuance => "CHAMPIONS_PURSUANCE",
            Self::LieutenantCommandersRedoubt => "LIEUTENANT_COMMANDERS_REDOUBT",
            Self::LieutenantCommandersBattlearmor => "LIEUTENANT_COMMANDERS_BATTLEARMOR",
            Self::LieutenantCommandersArcanum => "LIEUTENANT_COMMANDERS_ARCANUM",
            Self::LieutenantCommandersDreadgear => "LIEUTENANT_COMMANDERS_DREADGEAR",
            Self::LieutenantCommandersGuard => "LIEUTENANT_COMMANDERS_GUARD",
            Self::LieutenantCommandersInvestiture => "LIEUTENANT_COMMANDERS_INVESTITURE",
            Self::LieutenantCommandersPursuance => "LIEUTENANT_COMMANDERS_PURSUANCE",
            Self::LieutenantCommandersRefuge => "LIEUTENANT_COMMANDERS_REFUGE",
        }
    }

}

impl Default for ItemSet {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for ItemSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::TheGladiator => f.write_str("TheGladiator"),
            Self::DalRendsArms => f.write_str("Dal'Rend's Arms"),
            Self::SpidersKiss => f.write_str("Spider's Kiss"),
            Self::ThePostmaster => f.write_str("ThePostmaster"),
            Self::CadaverousGarb => f.write_str("CadaverousGarb"),
            Self::NecropileRaiment => f.write_str("NecropileRaiment"),
            Self::BloodmailRegalia => f.write_str("BloodmailRegalia"),
            Self::DeathboneGuardian => f.write_str("DeathboneGuardian"),
            Self::VolcanicArmor => f.write_str("VolcanicArmor"),
            Self::StormshroudArmor => f.write_str("StormshroudArmor"),
            Self::DevilsaurArmor => f.write_str("DevilsaurArmor"),
            Self::IronfeatherArmor => f.write_str("IronfeatherArmor"),
            Self::DefiasLeather => f.write_str("DefiasLeather"),
            Self::EmbraceOfTheViper => f.write_str("EmbraceOfTheViper"),
            Self::ChainOfTheScarletCrusade => f.write_str("ChainOfTheScarletCrusade"),
            Self::MagistersRegalia => f.write_str("Magister's Regalia"),
            Self::VestmentsOfTheDevout => f.write_str("VestmentsOfTheDevout"),
            Self::DreadmistRaiment => f.write_str("DreadmistRaiment"),
            Self::ShadowcraftArmor => f.write_str("ShadowcraftArmor"),
            Self::WildheartRaiment => f.write_str("WildheartRaiment"),
            Self::BeaststalkerArmor => f.write_str("BeaststalkerArmor"),
            Self::TheElements => f.write_str("TheElements"),
            Self::LightforgeArmor => f.write_str("LightforgeArmor"),
            Self::BattlegearOfValor => f.write_str("BattlegearOfValor"),
            Self::ArcanistRegalia => f.write_str("ArcanistRegalia"),
            Self::VestmentsOfProphecy => f.write_str("VestmentsOfProphecy"),
            Self::FelheartRaiment => f.write_str("FelheartRaiment"),
            Self::NightslayerArmor => f.write_str("NightslayerArmor"),
            Self::CenarionRaiment => f.write_str("CenarionRaiment"),
            Self::GiantstalkerArmor => f.write_str("GiantstalkerArmor"),
            Self::TheEarthfury => f.write_str("TheEarthfury"),
            Self::LawbringerArmor => f.write_str("LawbringerArmor"),
            Self::BattlegearOfMight => f.write_str("BattlegearOfMight"),
            Self::NetherwindRegalia => f.write_str("NetherwindRegalia"),
            Self::VestmentsOfTranscendence => f.write_str("VestmentsOfTranscendence"),
            Self::NemesisRaiment => f.write_str("NemesisRaiment"),
            Self::BloodfangArmor => f.write_str("BloodfangArmor"),
            Self::StormrageRaiment => f.write_str("StormrageRaiment"),
            Self::DragonstalkerArmor => f.write_str("DragonstalkerArmor"),
            Self::TheTenStorms => f.write_str("TheTenStorms"),
            Self::JudgementArmor => f.write_str("JudgementArmor"),
            Self::BattlegearOfWrath => f.write_str("BattlegearOfWrath"),
            Self::GarbOfTheroShan => f.write_str("Garb of Thero-shan"),
            Self::ShardOfTheGods => f.write_str("ShardOfTheGods"),
            Self::SpiritOfEskhandar => f.write_str("SpiritOfEskhandar"),
            Self::ChampionsBattlegear => f.write_str("Champion's Battlegear"),
            Self::LieutenantCommandersBattlegear => f.write_str("Lieutenant Commander's Battlegear"),
            Self::ChampionsEarthshaker => f.write_str("Champion's Earthshaker"),
            Self::ImperialPlate => f.write_str("ImperialPlate"),
            Self::ChampionsRegalia => f.write_str("Champion's Regalia"),
            Self::ChampionsRaiment => f.write_str("Champion's Raiment"),
            Self::LieutenantCommandersRegalia => f.write_str("Lieutenant Commander's Regalia"),
            Self::LieutenantCommandersRaiment => f.write_str("Lieutenant Commander's Raiment"),
            Self::ChampionsThreads => f.write_str("Champion's Threads"),
            Self::LieutenantCommandersThreads => f.write_str("Lieutenant Commander's Threads"),
            Self::ChampionsVestments => f.write_str("Champion's Vestments"),
            Self::LieutenantCommandersVestments => f.write_str("Lieutenant Commander's Vestments"),
            Self::ChampionsPursuit => f.write_str("Champion's Pursuit"),
            Self::LieutenantCommandersPursuit => f.write_str("Lieutenant Commander's Pursuit"),
            Self::LieutenantCommandersSanctuary => f.write_str("Lieutenant Commander's Sanctuary"),
            Self::ChampionsSanctuary => f.write_str("Champion's Sanctuary"),
            Self::WarlordsBattlegear => f.write_str("Warlord's Battlegear"),
            Self::FieldMarshalsBattlegear => f.write_str("Field Marshal's Battlegear"),
            Self::WarlordsEarthshaker => f.write_str("Warlord's Earthshaker"),
            Self::WarlordsRegalia => f.write_str("Warlord's Regalia"),
            Self::FieldMarshalsRegalia => f.write_str("Field Marshal's Regalia"),
            Self::FieldMarshalsRaiment => f.write_str("Field Marshal's Raiment"),
            Self::WarlordsRaiment => f.write_str("Warlord's Raiment"),
            Self::WarlordsThreads => f.write_str("Warlord's Threads"),
            Self::FieldMarshalsThreads => f.write_str("Field Marshal's Threads"),
            Self::WarlordsVestments => f.write_str("Warlord's Vestments"),
            Self::FieldMarshalsVestments => f.write_str("Field Marshal's Vestments"),
            Self::FieldMarshalsPursuit => f.write_str("Field Marshal's Pursuit"),
            Self::WarlordsPursuit => f.write_str("Warlord's Pursuit"),
            Self::FieldMarshalsSanctuary => f.write_str("Field Marshal's Sanctuary"),
            Self::WarlordsSanctuary => f.write_str("Warlord's Sanctuary"),
            Self::LieutenantCommandersAegis => f.write_str("Lieutenant Commander's Aegis"),
            Self::FieldMarshalsAegis => f.write_str("Field Marshal's Aegis"),
            Self::BloodvineGarb => f.write_str("BloodvineGarb"),
            Self::PrimalBatskin => f.write_str("PrimalBatskin"),
            Self::BloodTigerHarness => f.write_str("BloodTigerHarness"),
            Self::BloodsoulEmbrace => f.write_str("BloodsoulEmbrace"),
            Self::TheDarksoul => f.write_str("TheDarksoul"),
            Self::TheTwinBladesOfHakkari => f.write_str("TheTwinBladesOfHakkari"),
            Self::ZanzilsConcentration => f.write_str("Zanzil's Concentration"),
            Self::PrimalBlessing => f.write_str("PrimalBlessing"),
            Self::OverlordsResolution => f.write_str("Overlord's Resolution"),
            Self::PrayerOfThePrimal => f.write_str("PrayerOfThePrimal"),
            Self::MajorMojoInfusion => f.write_str("MajorMojoInfusion"),
            Self::TheHighlandersResolution => f.write_str("The Highlander's Resolution"),
            Self::TheHighlandersResolve => f.write_str("The Highlander's Resolve"),
            Self::TheHighlandersDetermination => f.write_str("The Highlander's Determination"),
            Self::TheHighlandersFortitude => f.write_str("The Highlander's Fortitude"),
            Self::TheHighlandersPurpose => f.write_str("The Highlander's Purpose"),
            Self::TheHighlandersWill => f.write_str("The Highlander's Will"),
            Self::TheHighlandersIntent => f.write_str("The Highlander's Intent"),
            Self::VindicatorsBattlegear => f.write_str("Vindicator's Battlegear"),
            Self::FreethinkersArmor => f.write_str("Freethinker's Armor"),
            Self::AugursRegalia => f.write_str("Augur's Regalia"),
            Self::PredatorsArmor => f.write_str("Predator's Armor"),
            Self::MadcapsOutfit => f.write_str("Madcap's Outfit"),
            Self::HaruspexsGarb => f.write_str("Haruspex's Garb"),
            Self::ConfessorsRaiment => f.write_str("Confessor's Raiment"),
            Self::DemoniacsThreads => f.write_str("Demoniac's Threads"),
            Self::IllusionistsAttire => f.write_str("Illusionist's Attire"),
            Self::TheDefilersDetermination => f.write_str("The Defiler's Determination"),
            Self::TheDefilersFortitude => f.write_str("The Defiler's Fortitude"),
            Self::TheDefilersIntent => f.write_str("The Defiler's Intent"),
            Self::TheDefilersPurpose => f.write_str("The Defiler's Purpose"),
            Self::TheDefilersResolution => f.write_str("The Defiler's Resolution"),
            Self::TheDefilersWill => f.write_str("The Defiler's Will"),
            Self::BlackDragonMail => f.write_str("BlackDragonMail"),
            Self::GreenDragonMail => f.write_str("GreenDragonMail"),
            Self::BlueDragonMail => f.write_str("BlueDragonMail"),
            Self::TwilightTrappings => f.write_str("TwilightTrappings"),
            Self::GenesisRaiment => f.write_str("GenesisRaiment"),
            Self::SymbolsOfUnendingLife => f.write_str("SymbolsOfUnendingLife"),
            Self::BattlegearOfUnyieldingStrength => f.write_str("BattlegearOfUnyieldingStrength"),
            Self::ConquerorsBattlegear => f.write_str("Conqueror's Battlegear"),
            Self::DeathdealersEmbrace => f.write_str("Deathdealer's Embrace"),
            Self::EmblemsOfVeiledShadows => f.write_str("EmblemsOfVeiledShadows"),
            Self::DoomcallersAttire => f.write_str("Doomcaller's Attire"),
            Self::ImplementsOfUnspokenNames => f.write_str("ImplementsOfUnspokenNames"),
            Self::StormcallersGarb => f.write_str("Stormcaller's Garb"),
            Self::GiftOfTheGatheringStorm => f.write_str("GiftOfTheGatheringStorm"),
            Self::EnigmaVestments => f.write_str("EnigmaVestments"),
            Self::TrappingsOfVaultedSecrets => f.write_str("TrappingsOfVaultedSecrets"),
            Self::AvengersBattlegear => f.write_str("Avenger's Battlegear"),
            Self::BattlegearOfEternalJustice => f.write_str("BattlegearOfEternalJustice"),
            Self::GarmentsOfTheOracle => f.write_str("GarmentsOfTheOracle"),
            Self::FineryOfInfiniteWisdom => f.write_str("FineryOfInfiniteWisdom"),
            Self::StrikersGarb => f.write_str("Striker's Garb"),
            Self::TrappingsOfTheUnseenPath => f.write_str("TrappingsOfTheUnseenPath"),
            Self::BattlegearOfHeroism => f.write_str("BattlegearOfHeroism"),
            Self::DarkmantleArmor => f.write_str("DarkmantleArmor"),
            Self::FeralheartRaiment => f.write_str("FeralheartRaiment"),
            Self::VestmentsOfTheVirtuous => f.write_str("VestmentsOfTheVirtuous"),
            Self::BeastmasterArmor => f.write_str("BeastmasterArmor"),
            Self::SoulforgeArmor => f.write_str("SoulforgeArmor"),
            Self::SorcerersRegalia => f.write_str("Sorcerer's Regalia"),
            Self::DeathmistRaiment => f.write_str("DeathmistRaiment"),
            Self::TheFiveThunders => f.write_str("TheFiveThunders"),
            Self::IronweaveBattlesuit => f.write_str("IronweaveBattlesuit"),
            Self::DreamwalkerRaiment => f.write_str("DreamwalkerRaiment"),
            Self::ChampionsGuard => f.write_str("Champion's Guard"),
            Self::DreadnaughtsBattlegear => f.write_str("Dreadnaught's Battlegear"),
            Self::BonescytheArmor => f.write_str("BonescytheArmor"),
            Self::VestmentsOfFaith => f.write_str("VestmentsOfFaith"),
            Self::FrostfireRegalia => f.write_str("FrostfireRegalia"),
            Self::TheEarthshatterer => f.write_str("TheEarthshatterer"),
            Self::RedemptionArmor => f.write_str("RedemptionArmor"),
            Self::PlagueheartRaiment => f.write_str("PlagueheartRaiment"),
            Self::CryptstalkerArmor => f.write_str("CryptstalkerArmor"),
            Self::BattlegearOfUndeadSlaying => f.write_str("BattlegearOfUndeadSlaying"),
            Self::UndeadSlayersArmor => f.write_str("Undead Slayer's Armor"),
            Self::GarbOfTheUndeadSlayer => f.write_str("GarbOfTheUndeadSlayer"),
            Self::RegaliaOfUndeadCleansing => f.write_str("RegaliaOfUndeadCleansing"),
            Self::ChampionsBattlearmor => f.write_str("Champion's Battlearmor"),
            Self::ChampionsStormcaller => f.write_str("Champion's Stormcaller"),
            Self::ChampionsRefuge => f.write_str("Champion's Refuge"),
            Self::ChampionsInvestiture => f.write_str("Champion's Investiture"),
            Self::ChampionsDreadgear => f.write_str("Champion's Dreadgear"),
            Self::ChampionsArcanum => f.write_str("Champion's Arcanum"),
            Self::ChampionsPursuance => f.write_str("Champion's Pursuance"),
            Self::LieutenantCommandersRedoubt => f.write_str("Lieutenant Commander's Redoubt"),
            Self::LieutenantCommandersBattlearmor => f.write_str("Lieutenant Commander's Battlearmor"),
            Self::LieutenantCommandersArcanum => f.write_str("Lieutenant Commander's Arcanum"),
            Self::LieutenantCommandersDreadgear => f.write_str("Lieutenant Commander's Dreadgear"),
            Self::LieutenantCommandersGuard => f.write_str("Lieutenant Commander's Guard"),
            Self::LieutenantCommandersInvestiture => f.write_str("Lieutenant Commander's Investiture"),
            Self::LieutenantCommandersPursuance => f.write_str("Lieutenant Commander's Pursuance"),
            Self::LieutenantCommandersRefuge => f.write_str("Lieutenant Commander's Refuge"),
        }
    }
}

impl TryFrom<u16> for ItemSet {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::TheGladiator),
            41 => Ok(Self::DalRendsArms),
            65 => Ok(Self::SpidersKiss),
            81 => Ok(Self::ThePostmaster),
            121 => Ok(Self::CadaverousGarb),
            122 => Ok(Self::NecropileRaiment),
            123 => Ok(Self::BloodmailRegalia),
            124 => Ok(Self::DeathboneGuardian),
            141 => Ok(Self::VolcanicArmor),
            142 => Ok(Self::StormshroudArmor),
            143 => Ok(Self::DevilsaurArmor),
            144 => Ok(Self::IronfeatherArmor),
            161 => Ok(Self::DefiasLeather),
            162 => Ok(Self::EmbraceOfTheViper),
            163 => Ok(Self::ChainOfTheScarletCrusade),
            181 => Ok(Self::MagistersRegalia),
            182 => Ok(Self::VestmentsOfTheDevout),
            183 => Ok(Self::DreadmistRaiment),
            184 => Ok(Self::ShadowcraftArmor),
            185 => Ok(Self::WildheartRaiment),
            186 => Ok(Self::BeaststalkerArmor),
            187 => Ok(Self::TheElements),
            188 => Ok(Self::LightforgeArmor),
            189 => Ok(Self::BattlegearOfValor),
            201 => Ok(Self::ArcanistRegalia),
            202 => Ok(Self::VestmentsOfProphecy),
            203 => Ok(Self::FelheartRaiment),
            204 => Ok(Self::NightslayerArmor),
            205 => Ok(Self::CenarionRaiment),
            206 => Ok(Self::GiantstalkerArmor),
            207 => Ok(Self::TheEarthfury),
            208 => Ok(Self::LawbringerArmor),
            209 => Ok(Self::BattlegearOfMight),
            210 => Ok(Self::NetherwindRegalia),
            211 => Ok(Self::VestmentsOfTranscendence),
            212 => Ok(Self::NemesisRaiment),
            213 => Ok(Self::BloodfangArmor),
            214 => Ok(Self::StormrageRaiment),
            215 => Ok(Self::DragonstalkerArmor),
            216 => Ok(Self::TheTenStorms),
            217 => Ok(Self::JudgementArmor),
            218 => Ok(Self::BattlegearOfWrath),
            221 => Ok(Self::GarbOfTheroShan),
            241 => Ok(Self::ShardOfTheGods),
            261 => Ok(Self::SpiritOfEskhandar),
            281 => Ok(Self::ChampionsBattlegear),
            282 => Ok(Self::LieutenantCommandersBattlegear),
            301 => Ok(Self::ChampionsEarthshaker),
            321 => Ok(Self::ImperialPlate),
            341 => Ok(Self::ChampionsRegalia),
            342 => Ok(Self::ChampionsRaiment),
            343 => Ok(Self::LieutenantCommandersRegalia),
            344 => Ok(Self::LieutenantCommandersRaiment),
            345 => Ok(Self::ChampionsThreads),
            346 => Ok(Self::LieutenantCommandersThreads),
            347 => Ok(Self::ChampionsVestments),
            348 => Ok(Self::LieutenantCommandersVestments),
            361 => Ok(Self::ChampionsPursuit),
            362 => Ok(Self::LieutenantCommandersPursuit),
            381 => Ok(Self::LieutenantCommandersSanctuary),
            382 => Ok(Self::ChampionsSanctuary),
            383 => Ok(Self::WarlordsBattlegear),
            384 => Ok(Self::FieldMarshalsBattlegear),
            386 => Ok(Self::WarlordsEarthshaker),
            387 => Ok(Self::WarlordsRegalia),
            388 => Ok(Self::FieldMarshalsRegalia),
            389 => Ok(Self::FieldMarshalsRaiment),
            390 => Ok(Self::WarlordsRaiment),
            391 => Ok(Self::WarlordsThreads),
            392 => Ok(Self::FieldMarshalsThreads),
            393 => Ok(Self::WarlordsVestments),
            394 => Ok(Self::FieldMarshalsVestments),
            395 => Ok(Self::FieldMarshalsPursuit),
            396 => Ok(Self::WarlordsPursuit),
            397 => Ok(Self::FieldMarshalsSanctuary),
            398 => Ok(Self::WarlordsSanctuary),
            401 => Ok(Self::LieutenantCommandersAegis),
            402 => Ok(Self::FieldMarshalsAegis),
            421 => Ok(Self::BloodvineGarb),
            441 => Ok(Self::PrimalBatskin),
            442 => Ok(Self::BloodTigerHarness),
            443 => Ok(Self::BloodsoulEmbrace),
            444 => Ok(Self::TheDarksoul),
            461 => Ok(Self::TheTwinBladesOfHakkari),
            462 => Ok(Self::ZanzilsConcentration),
            463 => Ok(Self::PrimalBlessing),
            464 => Ok(Self::OverlordsResolution),
            465 => Ok(Self::PrayerOfThePrimal),
            466 => Ok(Self::MajorMojoInfusion),
            467 => Ok(Self::TheHighlandersResolution),
            468 => Ok(Self::TheHighlandersResolve),
            469 => Ok(Self::TheHighlandersDetermination),
            470 => Ok(Self::TheHighlandersFortitude),
            471 => Ok(Self::TheHighlandersPurpose),
            472 => Ok(Self::TheHighlandersWill),
            473 => Ok(Self::TheHighlandersIntent),
            474 => Ok(Self::VindicatorsBattlegear),
            475 => Ok(Self::FreethinkersArmor),
            476 => Ok(Self::AugursRegalia),
            477 => Ok(Self::PredatorsArmor),
            478 => Ok(Self::MadcapsOutfit),
            479 => Ok(Self::HaruspexsGarb),
            480 => Ok(Self::ConfessorsRaiment),
            481 => Ok(Self::DemoniacsThreads),
            482 => Ok(Self::IllusionistsAttire),
            483 => Ok(Self::TheDefilersDetermination),
            484 => Ok(Self::TheDefilersFortitude),
            485 => Ok(Self::TheDefilersIntent),
            486 => Ok(Self::TheDefilersPurpose),
            487 => Ok(Self::TheDefilersResolution),
            488 => Ok(Self::TheDefilersWill),
            489 => Ok(Self::BlackDragonMail),
            490 => Ok(Self::GreenDragonMail),
            491 => Ok(Self::BlueDragonMail),
            492 => Ok(Self::TwilightTrappings),
            493 => Ok(Self::GenesisRaiment),
            494 => Ok(Self::SymbolsOfUnendingLife),
            495 => Ok(Self::BattlegearOfUnyieldingStrength),
            496 => Ok(Self::ConquerorsBattlegear),
            497 => Ok(Self::DeathdealersEmbrace),
            498 => Ok(Self::EmblemsOfVeiledShadows),
            499 => Ok(Self::DoomcallersAttire),
            500 => Ok(Self::ImplementsOfUnspokenNames),
            501 => Ok(Self::StormcallersGarb),
            502 => Ok(Self::GiftOfTheGatheringStorm),
            503 => Ok(Self::EnigmaVestments),
            504 => Ok(Self::TrappingsOfVaultedSecrets),
            505 => Ok(Self::AvengersBattlegear),
            506 => Ok(Self::BattlegearOfEternalJustice),
            507 => Ok(Self::GarmentsOfTheOracle),
            508 => Ok(Self::FineryOfInfiniteWisdom),
            509 => Ok(Self::StrikersGarb),
            510 => Ok(Self::TrappingsOfTheUnseenPath),
            511 => Ok(Self::BattlegearOfHeroism),
            512 => Ok(Self::DarkmantleArmor),
            513 => Ok(Self::FeralheartRaiment),
            514 => Ok(Self::VestmentsOfTheVirtuous),
            515 => Ok(Self::BeastmasterArmor),
            516 => Ok(Self::SoulforgeArmor),
            517 => Ok(Self::SorcerersRegalia),
            518 => Ok(Self::DeathmistRaiment),
            519 => Ok(Self::TheFiveThunders),
            520 => Ok(Self::IronweaveBattlesuit),
            521 => Ok(Self::DreamwalkerRaiment),
            522 => Ok(Self::ChampionsGuard),
            523 => Ok(Self::DreadnaughtsBattlegear),
            524 => Ok(Self::BonescytheArmor),
            525 => Ok(Self::VestmentsOfFaith),
            526 => Ok(Self::FrostfireRegalia),
            527 => Ok(Self::TheEarthshatterer),
            528 => Ok(Self::RedemptionArmor),
            529 => Ok(Self::PlagueheartRaiment),
            530 => Ok(Self::CryptstalkerArmor),
            533 => Ok(Self::BattlegearOfUndeadSlaying),
            534 => Ok(Self::UndeadSlayersArmor),
            535 => Ok(Self::GarbOfTheUndeadSlayer),
            536 => Ok(Self::RegaliaOfUndeadCleansing),
            537 => Ok(Self::ChampionsBattlearmor),
            538 => Ok(Self::ChampionsStormcaller),
            539 => Ok(Self::ChampionsRefuge),
            540 => Ok(Self::ChampionsInvestiture),
            541 => Ok(Self::ChampionsDreadgear),
            542 => Ok(Self::ChampionsArcanum),
            543 => Ok(Self::ChampionsPursuance),
            544 => Ok(Self::LieutenantCommandersRedoubt),
            545 => Ok(Self::LieutenantCommandersBattlearmor),
            546 => Ok(Self::LieutenantCommandersArcanum),
            547 => Ok(Self::LieutenantCommandersDreadgear),
            548 => Ok(Self::LieutenantCommandersGuard),
            549 => Ok(Self::LieutenantCommandersInvestiture),
            550 => Ok(Self::LieutenantCommandersPursuance),
            551 => Ok(Self::LieutenantCommandersRefuge),
            v => Err(crate::errors::EnumError::new("ItemSet", v as u64),)
        }
    }
}

