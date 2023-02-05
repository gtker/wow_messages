use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/item_sets.wowm:350`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/item_sets.wowm#L350):
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
///     WRATH_OF_SPELLFIRE = 552;
///     SHADOWS_EMBRACE = 553;
///     PRIMAL_MOONCLOTH = 554;
///     NETHERWEAVE_VESTMENTS = 555;
///     IMBUED_NETHERWEAVE = 556;
///     SOULCLOTH_EMBRACE = 557;
///     ARCANOWEAVE_VESTMENTS = 558;
///     SPELLSTRIKE_INFUSION = 559;
///     FEL_IRON_PLATE = 560;
///     FEL_IRON_CHAIN = 561;
///     ADAMANTITE_BATTLEGEAR = 562;
///     ENCHANTED_ADAMANTITE_ARMOR = 563;
///     FLAME_GUARD = 564;
///     KHORIUM_WARD = 565;
///     BURNING_RAGE = 566;
///     GLADIATORS_BATTLEGEAR = 567;
///     GLADIATORS_DREADGEAR = 568;
///     FAITH_IN_FELSTEEL = 569;
///     THE_UNYIELDING = 570;
///     WHITEMEND_WISDOM = 571;
///     BATTLECAST_GARB = 572;
///     FEL_SKIN = 573;
///     STRENGTH_OF_THE_CLEFTHOOF = 574;
///     FELSTALKER_ARMOR = 575;
///     FURY_OF_THE_NETHER = 576;
///     GLADIATORS_VESTMENTS = 577;
///     GLADIATORS_EARTHSHAKER = 578;
///     GLADIATORS_REGALIA = 579;
///     GLADIATORS_THUNDERFIST = 580;
///     GLADIATORS_RAIMENT = 581;
///     GLADIATORS_AEGIS = 582;
///     GLADIATORS_VINDICATION = 583;
///     GLADIATORS_SANCTUARY = 584;
///     GLADIATORS_WILDHIDE = 585;
///     GLADIATORS_PURSUIT = 586;
///     HIGH_WARLORDS_AEGIS = 587;
///     HIGH_WARLORDS_BATTLEGEAR = 588;
///     GRAND_MARSHALS_AEGIS = 589;
///     GRAND_MARSHALS_BATTLEGEAR = 590;
///     GRAND_MARSHALS_DREADGEAR = 591;
///     HIGH_WARLORDS_DREADGEAR = 592;
///     GRAND_MARSHALS_EARTHSHAKER = 593;
///     HIGH_WARLORDS_EARTHSHAKER = 594;
///     GRAND_MARSHALS_PURSUIT = 595;
///     HIGH_WARLORDS_PURSUIT = 596;
///     GRAND_MARSHALS_RAIMENT = 597;
///     HIGH_WARLORDS_RAIMENT = 598;
///     GRAND_MARSHALS_REGALIA = 599;
///     HIGH_WARLORDS_REGALIA = 600;
///     GRAND_MARSHALS_SANCTUARY = 601;
///     HIGH_WARLORDS_SANCTUARY = 602;
///     GRAND_MARSHALS_THUNDERFIST = 603;
///     HIGH_WARLORDS_THUNDERFIST = 604;
///     GRAND_MARSHALS_VESTMENTS = 605;
///     HIGH_WARLORDS_VESTMENTS = 606;
///     GRAND_MARSHALS_VINDICATION = 607;
///     HIGH_WARLORDS_VINDICATION = 608;
///     GRAND_MARSHALS_WILDHIDE = 609;
///     HIGH_WARLORDS_WILDHIDE = 610;
///     FELSCALE_ARMOR = 611;
///     SCALED_DRAENIC_ARMOR = 612;
///     THICK_DRAENIC_ARMOR = 613;
///     WILD_DRAENISH_ARMOR = 614;
///     GLADIATORS_FELSHROUD = 615;
///     NETHERSCALE_ARMOR = 616;
///     NETHERSTRIKE_ARMOR = 617;
///     WINDHAWK_ARMOR = 618;
///     PRIMAL_INTENT = 619;
///     ASSASSINATION_ARMOR = 620;
///     NETHERBLADE = 621;
///     DEATHMANTLE = 622;
///     RIGHTEOUS_ARMOR = 623;
///     JUSTICAR_RAIMENT = 624;
///     JUSTICAR_ARMOR = 625;
///     JUSTICAR_BATTLEGEAR = 626;
///     CRYSTALFORGE_RAIMENT = 627;
///     CRYSTALFORGE_ARMOR = 628;
///     CRYSTALFORGE_BATTLEGEAR = 629;
///     TIDEFURY_RAIMENT = 630;
///     CYCLONE_RAIMENT = 631;
///     CYCLONE_REGALIA = 632;
///     CYCLONE_HARNESS = 633;
///     CATACLYSM_RAIMENT = 634;
///     CATACLYSM_REGALIA = 635;
///     CATACLYSM_HARNESS = 636;
///     MOONGLADE_RAIMENT = 637;
///     MALORNE_RAIMENT = 638;
///     MALORNE_REGALIA = 639;
///     MALORNE_HARNESS = 640;
///     NORDRASSIL_HARNESS = 641;
///     NORDRASSIL_RAIMENT = 642;
///     NORDRASSIL_REGALIA = 643;
///     OBLIVION_RAIMENT = 644;
///     VOIDHEART_RAIMENT = 645;
///     CORRUPTOR_RAIMENT = 646;
///     INCANTERS_REGALIA = 647;
///     ALDOR_REGALIA = 648;
///     TIRISFAL_REGALIA = 649;
///     BEAST_LORD_ARMOR = 650;
///     DEMON_STALKER_ARMOR = 651;
///     RIFT_STALKER_ARMOR = 652;
///     BOLD_ARMOR = 653;
///     WARBRINGER_ARMOR = 654;
///     WARBRINGER_BATTLEGEAR = 655;
///     DESTROYER_ARMOR = 656;
///     DESTROYER_BATTLEGEAR = 657;
///     MANA_ETCHED_REGALIA = 658;
///     WASTEWALKER_ARMOR = 659;
///     DESOLATION_BATTLEGEAR = 660;
///     DOOMPLATE_BATTLEGEAR = 661;
///     HALLOWED_RAIMENT = 662;
///     INCARNATE_RAIMENT = 663;
///     INCARNATE_REGALIA = 664;
///     AVATAR_RAIMENT = 665;
///     AVATAR_REGALIA = 666;
///     THE_TWIN_STARS = 667;
///     SLAYERS_ARMOR = 668;
///     GRONNSTALKERS_ARMOR = 669;
///     MALEFIC_RAIMENT = 670;
///     TEMPEST_REGALIA = 671;
///     ONSLAUGHT_BATTLEGEAR = 672;
///     ONSLAUGHT_ARMOR = 673;
///     ABSOLUTION_REGALIA = 674;
///     VESTMENTS_OF_ABSOLUTION = 675;
///     THUNDERHEART_HARNESS = 676;
///     THUNDERHEART_REGALIA = 677;
///     THUNDERHEART_RAIMENT = 678;
///     LIGHTBRINGER_ARMOR = 679;
///     LIGHTBRINGER_BATTLEGEAR = 680;
///     LIGHTBRINGER_RAIMENT = 681;
///     SKYSHATTER_HARNESS = 682;
///     SKYSHATTER_RAIMENT = 683;
///     SKYSHATTER_REGALIA = 684;
///     GLADIATORS_REFUGE = 685;
///     GLADIATORS_WARTIDE = 686;
///     GLADIATORS_INVESTITURE = 687;
///     GRAND_MARSHALS_REFUGE = 688;
///     HIGH_WARLORDS_REFUGE = 689;
///     GLADIATORS_REDEMPTION = 690;
///     GRAND_MARSHALS_INVESTITURE = 691;
///     HIGH_WARLORDS_INVESTITURE = 692;
///     GRAND_MARSHALS_REDEMPTION = 693;
///     HIGH_WARLORDS_REDEMPTION = 694;
///     GRAND_MARSHALS_WARTIDE = 695;
///     HIGH_WARLORDS_WARTIDE = 696;
///     CHAMPIONS_REDOUBT = 697;
///     WARLORDS_AEGIS = 698;
///     THE_TWIN_BLADES_OF_AZZINOTH = 699;
///     MERCILESS_GLADIATORS_AEGIS = 700;
///     MERCILESS_GLADIATORS_BATTLEGEAR = 701;
///     MERCILESS_GLADIATORS_DREADGEAR = 702;
///     MERCILESS_GLADIATORS_EARTHSHAKER = 703;
///     MERCILESS_GLADIATORS_FELSHROUD = 704;
///     MERCILESS_GLADIATORS_INVESTITURE = 705;
///     MERCILESS_GLADIATORS_PURSUIT = 706;
///     MERCILESS_GLADIATORS_RAIMENT = 707;
///     MERCILESS_GLADIATORS_REDEMPTION = 708;
///     MERCILESS_GLADIATORS_REFUGE = 709;
///     MERCILESS_GLADIATORS_REGALIA = 710;
///     MERCILESS_GLADIATORS_SANCTUARY = 711;
///     MERCILESS_GLADIATORS_THUNDERFIST = 712;
///     MERCILESS_GLADIATORS_VESTMENTS = 713;
///     MERCILESS_GLADIATORS_VINDICATION = 714;
///     MERCILESS_GLADIATORS_WARTIDE = 715;
///     MERCILESS_GLADIATORS_WILDHIDE = 716;
///     FIELD_MARSHALS_EARTHSHAKER = 717;
///     LIEUTENANT_COMMANDERS_EARTHSHAKER = 718;
///     THE_FISTS_OF_FURY = 719;
///     VENGEFUL_GLADIATORS_REFUGE = 720;
///     VENGEFUL_GLADIATORS_SANCTUARY = 721;
///     VENGEFUL_GLADIATORS_WILDHIDE = 722;
///     VENGEFUL_GLADIATORS_PURSUIT = 723;
///     VENGEFUL_GLADIATORS_REGALIA = 724;
///     VENGEFUL_GLADIATORS_REDEMPTION = 725;
///     VENGEFUL_GLADIATORS_VINDICATION = 726;
///     VENGEFUL_GLADIATORS_AEGIS = 727;
///     VENGEFUL_GLADIATORS_INVESTITURE = 728;
///     VENGEFUL_GLADIATORS_RAIMENT = 729;
///     VENGEFUL_GLADIATORS_VESTMENTS = 730;
///     VENGEFUL_GLADIATORS_WARTIDE = 731;
///     VENGEFUL_GLADIATORS_EARTHSHAKER = 732;
///     VENGEFUL_GLADIATORS_THUNDERFIST = 733;
///     VENGEFUL_GLADIATORS_DREADGEAR = 734;
///     VENGEFUL_GLADIATORS_FELSHROUD = 735;
///     VENGEFUL_GLADIATORS_BATTLEGEAR = 736;
///     LATROS_FLURRY = 737;
///     DREADWEAVE_BATTLEGEAR = 738;
///     MOONCLOTH_BATTLEGEAR = 739;
///     SATIN_BATTLEGEAR = 740;
///     EVOKERS_SILK_BATTLEGEAR = 741;
///     DRAGONHIDE_BATTLEGEAR = 742;
///     WYRMHIDE_BATTLEGEAR = 743;
///     KODOHIDE_BATTLEGEAR = 744;
///     OPPORTUNISTS_BATTLEGEAR = 745;
///     SEERS_MAIL_BATTLEGEAR = 746;
///     SEERS_RINGMAIL_BATTLEGEAR = 747;
///     SEERS_LINKED_BATTLEGEAR = 748;
///     STALKERS_CHAIN_BATTLEGEAR = 749;
///     SAVAGE_PLATE_BATTLEGEAR = 750;
///     CRUSADERS_ORNAMENTED_BATTLEGEAR = 751;
///     CRUSADERS_SCALED_BATTLEGEAR = 752;
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
    WrathOfSpellfire,
    ShadowsEmbrace,
    PrimalMooncloth,
    NetherweaveVestments,
    ImbuedNetherweave,
    SoulclothEmbrace,
    ArcanoweaveVestments,
    SpellstrikeInfusion,
    FelIronPlate,
    FelIronChain,
    AdamantiteBattlegear,
    EnchantedAdamantiteArmor,
    FlameGuard,
    KhoriumWard,
    BurningRage,
    GladiatorsBattlegear,
    GladiatorsDreadgear,
    FaithInFelsteel,
    TheUnyielding,
    WhitemendWisdom,
    BattlecastGarb,
    FelSkin,
    StrengthOfTheClefthoof,
    FelstalkerArmor,
    FuryOfTheNether,
    GladiatorsVestments,
    GladiatorsEarthshaker,
    GladiatorsRegalia,
    GladiatorsThunderfist,
    GladiatorsRaiment,
    GladiatorsAegis,
    GladiatorsVindication,
    GladiatorsSanctuary,
    GladiatorsWildhide,
    GladiatorsPursuit,
    HighWarlordsAegis,
    HighWarlordsBattlegear,
    GrandMarshalsAegis,
    GrandMarshalsBattlegear,
    GrandMarshalsDreadgear,
    HighWarlordsDreadgear,
    GrandMarshalsEarthshaker,
    HighWarlordsEarthshaker,
    GrandMarshalsPursuit,
    HighWarlordsPursuit,
    GrandMarshalsRaiment,
    HighWarlordsRaiment,
    GrandMarshalsRegalia,
    HighWarlordsRegalia,
    GrandMarshalsSanctuary,
    HighWarlordsSanctuary,
    GrandMarshalsThunderfist,
    HighWarlordsThunderfist,
    GrandMarshalsVestments,
    HighWarlordsVestments,
    GrandMarshalsVindication,
    HighWarlordsVindication,
    GrandMarshalsWildhide,
    HighWarlordsWildhide,
    FelscaleArmor,
    ScaledDraenicArmor,
    ThickDraenicArmor,
    WildDraenishArmor,
    GladiatorsFelshroud,
    NetherscaleArmor,
    NetherstrikeArmor,
    WindhawkArmor,
    PrimalIntent,
    AssassinationArmor,
    Netherblade,
    Deathmantle,
    RighteousArmor,
    JusticarRaiment,
    JusticarArmor,
    JusticarBattlegear,
    CrystalforgeRaiment,
    CrystalforgeArmor,
    CrystalforgeBattlegear,
    TidefuryRaiment,
    CycloneRaiment,
    CycloneRegalia,
    CycloneHarness,
    CataclysmRaiment,
    CataclysmRegalia,
    CataclysmHarness,
    MoongladeRaiment,
    MalorneRaiment,
    MalorneRegalia,
    MalorneHarness,
    NordrassilHarness,
    NordrassilRaiment,
    NordrassilRegalia,
    OblivionRaiment,
    VoidheartRaiment,
    CorruptorRaiment,
    IncantersRegalia,
    AldorRegalia,
    TirisfalRegalia,
    BeastLordArmor,
    DemonStalkerArmor,
    RiftStalkerArmor,
    BoldArmor,
    WarbringerArmor,
    WarbringerBattlegear,
    DestroyerArmor,
    DestroyerBattlegear,
    ManaEtchedRegalia,
    WastewalkerArmor,
    DesolationBattlegear,
    DoomplateBattlegear,
    HallowedRaiment,
    IncarnateRaiment,
    IncarnateRegalia,
    AvatarRaiment,
    AvatarRegalia,
    TheTwinStars,
    SlayersArmor,
    GronnstalkersArmor,
    MaleficRaiment,
    TempestRegalia,
    OnslaughtBattlegear,
    OnslaughtArmor,
    AbsolutionRegalia,
    VestmentsOfAbsolution,
    ThunderheartHarness,
    ThunderheartRegalia,
    ThunderheartRaiment,
    LightbringerArmor,
    LightbringerBattlegear,
    LightbringerRaiment,
    SkyshatterHarness,
    SkyshatterRaiment,
    SkyshatterRegalia,
    GladiatorsRefuge,
    GladiatorsWartide,
    GladiatorsInvestiture,
    GrandMarshalsRefuge,
    HighWarlordsRefuge,
    GladiatorsRedemption,
    GrandMarshalsInvestiture,
    HighWarlordsInvestiture,
    GrandMarshalsRedemption,
    HighWarlordsRedemption,
    GrandMarshalsWartide,
    HighWarlordsWartide,
    ChampionsRedoubt,
    WarlordsAegis,
    TheTwinBladesOfAzzinoth,
    MercilessGladiatorsAegis,
    MercilessGladiatorsBattlegear,
    MercilessGladiatorsDreadgear,
    MercilessGladiatorsEarthshaker,
    MercilessGladiatorsFelshroud,
    MercilessGladiatorsInvestiture,
    MercilessGladiatorsPursuit,
    MercilessGladiatorsRaiment,
    MercilessGladiatorsRedemption,
    MercilessGladiatorsRefuge,
    MercilessGladiatorsRegalia,
    MercilessGladiatorsSanctuary,
    MercilessGladiatorsThunderfist,
    MercilessGladiatorsVestments,
    MercilessGladiatorsVindication,
    MercilessGladiatorsWartide,
    MercilessGladiatorsWildhide,
    FieldMarshalsEarthshaker,
    LieutenantCommandersEarthshaker,
    TheFistsOfFury,
    VengefulGladiatorsRefuge,
    VengefulGladiatorsSanctuary,
    VengefulGladiatorsWildhide,
    VengefulGladiatorsPursuit,
    VengefulGladiatorsRegalia,
    VengefulGladiatorsRedemption,
    VengefulGladiatorsVindication,
    VengefulGladiatorsAegis,
    VengefulGladiatorsInvestiture,
    VengefulGladiatorsRaiment,
    VengefulGladiatorsVestments,
    VengefulGladiatorsWartide,
    VengefulGladiatorsEarthshaker,
    VengefulGladiatorsThunderfist,
    VengefulGladiatorsDreadgear,
    VengefulGladiatorsFelshroud,
    VengefulGladiatorsBattlegear,
    LatrosFlurry,
    DreadweaveBattlegear,
    MoonclothBattlegear,
    SatinBattlegear,
    EvokersSilkBattlegear,
    DragonhideBattlegear,
    WyrmhideBattlegear,
    KodohideBattlegear,
    OpportunistsBattlegear,
    SeersMailBattlegear,
    SeersRingmailBattlegear,
    SeersLinkedBattlegear,
    StalkersChainBattlegear,
    SavagePlateBattlegear,
    CrusadersOrnamentedBattlegear,
    CrusadersScaledBattlegear,
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
            Self::WrathOfSpellfire => 0x228,
            Self::ShadowsEmbrace => 0x229,
            Self::PrimalMooncloth => 0x22a,
            Self::NetherweaveVestments => 0x22b,
            Self::ImbuedNetherweave => 0x22c,
            Self::SoulclothEmbrace => 0x22d,
            Self::ArcanoweaveVestments => 0x22e,
            Self::SpellstrikeInfusion => 0x22f,
            Self::FelIronPlate => 0x230,
            Self::FelIronChain => 0x231,
            Self::AdamantiteBattlegear => 0x232,
            Self::EnchantedAdamantiteArmor => 0x233,
            Self::FlameGuard => 0x234,
            Self::KhoriumWard => 0x235,
            Self::BurningRage => 0x236,
            Self::GladiatorsBattlegear => 0x237,
            Self::GladiatorsDreadgear => 0x238,
            Self::FaithInFelsteel => 0x239,
            Self::TheUnyielding => 0x23a,
            Self::WhitemendWisdom => 0x23b,
            Self::BattlecastGarb => 0x23c,
            Self::FelSkin => 0x23d,
            Self::StrengthOfTheClefthoof => 0x23e,
            Self::FelstalkerArmor => 0x23f,
            Self::FuryOfTheNether => 0x240,
            Self::GladiatorsVestments => 0x241,
            Self::GladiatorsEarthshaker => 0x242,
            Self::GladiatorsRegalia => 0x243,
            Self::GladiatorsThunderfist => 0x244,
            Self::GladiatorsRaiment => 0x245,
            Self::GladiatorsAegis => 0x246,
            Self::GladiatorsVindication => 0x247,
            Self::GladiatorsSanctuary => 0x248,
            Self::GladiatorsWildhide => 0x249,
            Self::GladiatorsPursuit => 0x24a,
            Self::HighWarlordsAegis => 0x24b,
            Self::HighWarlordsBattlegear => 0x24c,
            Self::GrandMarshalsAegis => 0x24d,
            Self::GrandMarshalsBattlegear => 0x24e,
            Self::GrandMarshalsDreadgear => 0x24f,
            Self::HighWarlordsDreadgear => 0x250,
            Self::GrandMarshalsEarthshaker => 0x251,
            Self::HighWarlordsEarthshaker => 0x252,
            Self::GrandMarshalsPursuit => 0x253,
            Self::HighWarlordsPursuit => 0x254,
            Self::GrandMarshalsRaiment => 0x255,
            Self::HighWarlordsRaiment => 0x256,
            Self::GrandMarshalsRegalia => 0x257,
            Self::HighWarlordsRegalia => 0x258,
            Self::GrandMarshalsSanctuary => 0x259,
            Self::HighWarlordsSanctuary => 0x25a,
            Self::GrandMarshalsThunderfist => 0x25b,
            Self::HighWarlordsThunderfist => 0x25c,
            Self::GrandMarshalsVestments => 0x25d,
            Self::HighWarlordsVestments => 0x25e,
            Self::GrandMarshalsVindication => 0x25f,
            Self::HighWarlordsVindication => 0x260,
            Self::GrandMarshalsWildhide => 0x261,
            Self::HighWarlordsWildhide => 0x262,
            Self::FelscaleArmor => 0x263,
            Self::ScaledDraenicArmor => 0x264,
            Self::ThickDraenicArmor => 0x265,
            Self::WildDraenishArmor => 0x266,
            Self::GladiatorsFelshroud => 0x267,
            Self::NetherscaleArmor => 0x268,
            Self::NetherstrikeArmor => 0x269,
            Self::WindhawkArmor => 0x26a,
            Self::PrimalIntent => 0x26b,
            Self::AssassinationArmor => 0x26c,
            Self::Netherblade => 0x26d,
            Self::Deathmantle => 0x26e,
            Self::RighteousArmor => 0x26f,
            Self::JusticarRaiment => 0x270,
            Self::JusticarArmor => 0x271,
            Self::JusticarBattlegear => 0x272,
            Self::CrystalforgeRaiment => 0x273,
            Self::CrystalforgeArmor => 0x274,
            Self::CrystalforgeBattlegear => 0x275,
            Self::TidefuryRaiment => 0x276,
            Self::CycloneRaiment => 0x277,
            Self::CycloneRegalia => 0x278,
            Self::CycloneHarness => 0x279,
            Self::CataclysmRaiment => 0x27a,
            Self::CataclysmRegalia => 0x27b,
            Self::CataclysmHarness => 0x27c,
            Self::MoongladeRaiment => 0x27d,
            Self::MalorneRaiment => 0x27e,
            Self::MalorneRegalia => 0x27f,
            Self::MalorneHarness => 0x280,
            Self::NordrassilHarness => 0x281,
            Self::NordrassilRaiment => 0x282,
            Self::NordrassilRegalia => 0x283,
            Self::OblivionRaiment => 0x284,
            Self::VoidheartRaiment => 0x285,
            Self::CorruptorRaiment => 0x286,
            Self::IncantersRegalia => 0x287,
            Self::AldorRegalia => 0x288,
            Self::TirisfalRegalia => 0x289,
            Self::BeastLordArmor => 0x28a,
            Self::DemonStalkerArmor => 0x28b,
            Self::RiftStalkerArmor => 0x28c,
            Self::BoldArmor => 0x28d,
            Self::WarbringerArmor => 0x28e,
            Self::WarbringerBattlegear => 0x28f,
            Self::DestroyerArmor => 0x290,
            Self::DestroyerBattlegear => 0x291,
            Self::ManaEtchedRegalia => 0x292,
            Self::WastewalkerArmor => 0x293,
            Self::DesolationBattlegear => 0x294,
            Self::DoomplateBattlegear => 0x295,
            Self::HallowedRaiment => 0x296,
            Self::IncarnateRaiment => 0x297,
            Self::IncarnateRegalia => 0x298,
            Self::AvatarRaiment => 0x299,
            Self::AvatarRegalia => 0x29a,
            Self::TheTwinStars => 0x29b,
            Self::SlayersArmor => 0x29c,
            Self::GronnstalkersArmor => 0x29d,
            Self::MaleficRaiment => 0x29e,
            Self::TempestRegalia => 0x29f,
            Self::OnslaughtBattlegear => 0x2a0,
            Self::OnslaughtArmor => 0x2a1,
            Self::AbsolutionRegalia => 0x2a2,
            Self::VestmentsOfAbsolution => 0x2a3,
            Self::ThunderheartHarness => 0x2a4,
            Self::ThunderheartRegalia => 0x2a5,
            Self::ThunderheartRaiment => 0x2a6,
            Self::LightbringerArmor => 0x2a7,
            Self::LightbringerBattlegear => 0x2a8,
            Self::LightbringerRaiment => 0x2a9,
            Self::SkyshatterHarness => 0x2aa,
            Self::SkyshatterRaiment => 0x2ab,
            Self::SkyshatterRegalia => 0x2ac,
            Self::GladiatorsRefuge => 0x2ad,
            Self::GladiatorsWartide => 0x2ae,
            Self::GladiatorsInvestiture => 0x2af,
            Self::GrandMarshalsRefuge => 0x2b0,
            Self::HighWarlordsRefuge => 0x2b1,
            Self::GladiatorsRedemption => 0x2b2,
            Self::GrandMarshalsInvestiture => 0x2b3,
            Self::HighWarlordsInvestiture => 0x2b4,
            Self::GrandMarshalsRedemption => 0x2b5,
            Self::HighWarlordsRedemption => 0x2b6,
            Self::GrandMarshalsWartide => 0x2b7,
            Self::HighWarlordsWartide => 0x2b8,
            Self::ChampionsRedoubt => 0x2b9,
            Self::WarlordsAegis => 0x2ba,
            Self::TheTwinBladesOfAzzinoth => 0x2bb,
            Self::MercilessGladiatorsAegis => 0x2bc,
            Self::MercilessGladiatorsBattlegear => 0x2bd,
            Self::MercilessGladiatorsDreadgear => 0x2be,
            Self::MercilessGladiatorsEarthshaker => 0x2bf,
            Self::MercilessGladiatorsFelshroud => 0x2c0,
            Self::MercilessGladiatorsInvestiture => 0x2c1,
            Self::MercilessGladiatorsPursuit => 0x2c2,
            Self::MercilessGladiatorsRaiment => 0x2c3,
            Self::MercilessGladiatorsRedemption => 0x2c4,
            Self::MercilessGladiatorsRefuge => 0x2c5,
            Self::MercilessGladiatorsRegalia => 0x2c6,
            Self::MercilessGladiatorsSanctuary => 0x2c7,
            Self::MercilessGladiatorsThunderfist => 0x2c8,
            Self::MercilessGladiatorsVestments => 0x2c9,
            Self::MercilessGladiatorsVindication => 0x2ca,
            Self::MercilessGladiatorsWartide => 0x2cb,
            Self::MercilessGladiatorsWildhide => 0x2cc,
            Self::FieldMarshalsEarthshaker => 0x2cd,
            Self::LieutenantCommandersEarthshaker => 0x2ce,
            Self::TheFistsOfFury => 0x2cf,
            Self::VengefulGladiatorsRefuge => 0x2d0,
            Self::VengefulGladiatorsSanctuary => 0x2d1,
            Self::VengefulGladiatorsWildhide => 0x2d2,
            Self::VengefulGladiatorsPursuit => 0x2d3,
            Self::VengefulGladiatorsRegalia => 0x2d4,
            Self::VengefulGladiatorsRedemption => 0x2d5,
            Self::VengefulGladiatorsVindication => 0x2d6,
            Self::VengefulGladiatorsAegis => 0x2d7,
            Self::VengefulGladiatorsInvestiture => 0x2d8,
            Self::VengefulGladiatorsRaiment => 0x2d9,
            Self::VengefulGladiatorsVestments => 0x2da,
            Self::VengefulGladiatorsWartide => 0x2db,
            Self::VengefulGladiatorsEarthshaker => 0x2dc,
            Self::VengefulGladiatorsThunderfist => 0x2dd,
            Self::VengefulGladiatorsDreadgear => 0x2de,
            Self::VengefulGladiatorsFelshroud => 0x2df,
            Self::VengefulGladiatorsBattlegear => 0x2e0,
            Self::LatrosFlurry => 0x2e1,
            Self::DreadweaveBattlegear => 0x2e2,
            Self::MoonclothBattlegear => 0x2e3,
            Self::SatinBattlegear => 0x2e4,
            Self::EvokersSilkBattlegear => 0x2e5,
            Self::DragonhideBattlegear => 0x2e6,
            Self::WyrmhideBattlegear => 0x2e7,
            Self::KodohideBattlegear => 0x2e8,
            Self::OpportunistsBattlegear => 0x2e9,
            Self::SeersMailBattlegear => 0x2ea,
            Self::SeersRingmailBattlegear => 0x2eb,
            Self::SeersLinkedBattlegear => 0x2ec,
            Self::StalkersChainBattlegear => 0x2ed,
            Self::SavagePlateBattlegear => 0x2ee,
            Self::CrusadersOrnamentedBattlegear => 0x2ef,
            Self::CrusadersScaledBattlegear => 0x2f0,
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
            Self::WrathOfSpellfire => f.write_str("WrathOfSpellfire"),
            Self::ShadowsEmbrace => f.write_str("Shadow's Embrace"),
            Self::PrimalMooncloth => f.write_str("PrimalMooncloth"),
            Self::NetherweaveVestments => f.write_str("NetherweaveVestments"),
            Self::ImbuedNetherweave => f.write_str("ImbuedNetherweave"),
            Self::SoulclothEmbrace => f.write_str("SoulclothEmbrace"),
            Self::ArcanoweaveVestments => f.write_str("ArcanoweaveVestments"),
            Self::SpellstrikeInfusion => f.write_str("SpellstrikeInfusion"),
            Self::FelIronPlate => f.write_str("FelIronPlate"),
            Self::FelIronChain => f.write_str("FelIronChain"),
            Self::AdamantiteBattlegear => f.write_str("AdamantiteBattlegear"),
            Self::EnchantedAdamantiteArmor => f.write_str("EnchantedAdamantiteArmor"),
            Self::FlameGuard => f.write_str("FlameGuard"),
            Self::KhoriumWard => f.write_str("KhoriumWard"),
            Self::BurningRage => f.write_str("BurningRage"),
            Self::GladiatorsBattlegear => f.write_str("Gladiator's Battlegear"),
            Self::GladiatorsDreadgear => f.write_str("Gladiator's Dreadgear"),
            Self::FaithInFelsteel => f.write_str("FaithInFelsteel"),
            Self::TheUnyielding => f.write_str("TheUnyielding"),
            Self::WhitemendWisdom => f.write_str("WhitemendWisdom"),
            Self::BattlecastGarb => f.write_str("BattlecastGarb"),
            Self::FelSkin => f.write_str("FelSkin"),
            Self::StrengthOfTheClefthoof => f.write_str("StrengthOfTheClefthoof"),
            Self::FelstalkerArmor => f.write_str("FelstalkerArmor"),
            Self::FuryOfTheNether => f.write_str("FuryOfTheNether"),
            Self::GladiatorsVestments => f.write_str("Gladiator's Vestments"),
            Self::GladiatorsEarthshaker => f.write_str("Gladiator's Earthshaker"),
            Self::GladiatorsRegalia => f.write_str("Gladiator's Regalia"),
            Self::GladiatorsThunderfist => f.write_str("Gladiator's Thunderfist"),
            Self::GladiatorsRaiment => f.write_str("Gladiator's Raiment"),
            Self::GladiatorsAegis => f.write_str("Gladiator's Aegis"),
            Self::GladiatorsVindication => f.write_str("Gladiator's Vindication"),
            Self::GladiatorsSanctuary => f.write_str("Gladiator's Sanctuary"),
            Self::GladiatorsWildhide => f.write_str("Gladiator's Wildhide"),
            Self::GladiatorsPursuit => f.write_str("Gladiator's Pursuit"),
            Self::HighWarlordsAegis => f.write_str("High Warlord's Aegis"),
            Self::HighWarlordsBattlegear => f.write_str("High Warlord's Battlegear"),
            Self::GrandMarshalsAegis => f.write_str("Grand Marshal's Aegis"),
            Self::GrandMarshalsBattlegear => f.write_str("Grand Marshal's Battlegear"),
            Self::GrandMarshalsDreadgear => f.write_str("Grand Marshal's Dreadgear"),
            Self::HighWarlordsDreadgear => f.write_str("High Warlord's Dreadgear"),
            Self::GrandMarshalsEarthshaker => f.write_str("Grand Marshal's Earthshaker"),
            Self::HighWarlordsEarthshaker => f.write_str("High Warlord's Earthshaker"),
            Self::GrandMarshalsPursuit => f.write_str("Grand Marshal's Pursuit"),
            Self::HighWarlordsPursuit => f.write_str("High Warlord's Pursuit"),
            Self::GrandMarshalsRaiment => f.write_str("Grand Marshal's Raiment"),
            Self::HighWarlordsRaiment => f.write_str("High Warlord's Raiment"),
            Self::GrandMarshalsRegalia => f.write_str("Grand Marshal's Regalia"),
            Self::HighWarlordsRegalia => f.write_str("High Warlord's Regalia"),
            Self::GrandMarshalsSanctuary => f.write_str("Grand Marshal's Sanctuary"),
            Self::HighWarlordsSanctuary => f.write_str("High Warlord's Sanctuary"),
            Self::GrandMarshalsThunderfist => f.write_str("Grand Marshal's Thunderfist"),
            Self::HighWarlordsThunderfist => f.write_str("High Warlord's Thunderfist"),
            Self::GrandMarshalsVestments => f.write_str("Grand Marshal's Vestments"),
            Self::HighWarlordsVestments => f.write_str("High Warlord's Vestments"),
            Self::GrandMarshalsVindication => f.write_str("Grand Marshal's Vindication"),
            Self::HighWarlordsVindication => f.write_str("High Warlord's Vindication"),
            Self::GrandMarshalsWildhide => f.write_str("Grand Marshal's Wildhide"),
            Self::HighWarlordsWildhide => f.write_str("High Warlord's Wildhide"),
            Self::FelscaleArmor => f.write_str("FelscaleArmor"),
            Self::ScaledDraenicArmor => f.write_str("ScaledDraenicArmor"),
            Self::ThickDraenicArmor => f.write_str("ThickDraenicArmor"),
            Self::WildDraenishArmor => f.write_str("WildDraenishArmor"),
            Self::GladiatorsFelshroud => f.write_str("Gladiator's Felshroud"),
            Self::NetherscaleArmor => f.write_str("NetherscaleArmor"),
            Self::NetherstrikeArmor => f.write_str("NetherstrikeArmor"),
            Self::WindhawkArmor => f.write_str("WindhawkArmor"),
            Self::PrimalIntent => f.write_str("PrimalIntent"),
            Self::AssassinationArmor => f.write_str("AssassinationArmor"),
            Self::Netherblade => f.write_str("Netherblade"),
            Self::Deathmantle => f.write_str("Deathmantle"),
            Self::RighteousArmor => f.write_str("RighteousArmor"),
            Self::JusticarRaiment => f.write_str("JusticarRaiment"),
            Self::JusticarArmor => f.write_str("JusticarArmor"),
            Self::JusticarBattlegear => f.write_str("JusticarBattlegear"),
            Self::CrystalforgeRaiment => f.write_str("CrystalforgeRaiment"),
            Self::CrystalforgeArmor => f.write_str("CrystalforgeArmor"),
            Self::CrystalforgeBattlegear => f.write_str("CrystalforgeBattlegear"),
            Self::TidefuryRaiment => f.write_str("TidefuryRaiment"),
            Self::CycloneRaiment => f.write_str("CycloneRaiment"),
            Self::CycloneRegalia => f.write_str("CycloneRegalia"),
            Self::CycloneHarness => f.write_str("CycloneHarness"),
            Self::CataclysmRaiment => f.write_str("CataclysmRaiment"),
            Self::CataclysmRegalia => f.write_str("CataclysmRegalia"),
            Self::CataclysmHarness => f.write_str("CataclysmHarness"),
            Self::MoongladeRaiment => f.write_str("MoongladeRaiment"),
            Self::MalorneRaiment => f.write_str("MalorneRaiment"),
            Self::MalorneRegalia => f.write_str("MalorneRegalia"),
            Self::MalorneHarness => f.write_str("MalorneHarness"),
            Self::NordrassilHarness => f.write_str("NordrassilHarness"),
            Self::NordrassilRaiment => f.write_str("NordrassilRaiment"),
            Self::NordrassilRegalia => f.write_str("NordrassilRegalia"),
            Self::OblivionRaiment => f.write_str("OblivionRaiment"),
            Self::VoidheartRaiment => f.write_str("VoidheartRaiment"),
            Self::CorruptorRaiment => f.write_str("CorruptorRaiment"),
            Self::IncantersRegalia => f.write_str("Incanter's Regalia"),
            Self::AldorRegalia => f.write_str("AldorRegalia"),
            Self::TirisfalRegalia => f.write_str("TirisfalRegalia"),
            Self::BeastLordArmor => f.write_str("BeastLordArmor"),
            Self::DemonStalkerArmor => f.write_str("DemonStalkerArmor"),
            Self::RiftStalkerArmor => f.write_str("RiftStalkerArmor"),
            Self::BoldArmor => f.write_str("BoldArmor"),
            Self::WarbringerArmor => f.write_str("WarbringerArmor"),
            Self::WarbringerBattlegear => f.write_str("WarbringerBattlegear"),
            Self::DestroyerArmor => f.write_str("DestroyerArmor"),
            Self::DestroyerBattlegear => f.write_str("DestroyerBattlegear"),
            Self::ManaEtchedRegalia => f.write_str("Mana-Etched Regalia"),
            Self::WastewalkerArmor => f.write_str("WastewalkerArmor"),
            Self::DesolationBattlegear => f.write_str("DesolationBattlegear"),
            Self::DoomplateBattlegear => f.write_str("DoomplateBattlegear"),
            Self::HallowedRaiment => f.write_str("HallowedRaiment"),
            Self::IncarnateRaiment => f.write_str("IncarnateRaiment"),
            Self::IncarnateRegalia => f.write_str("IncarnateRegalia"),
            Self::AvatarRaiment => f.write_str("AvatarRaiment"),
            Self::AvatarRegalia => f.write_str("AvatarRegalia"),
            Self::TheTwinStars => f.write_str("TheTwinStars"),
            Self::SlayersArmor => f.write_str("Slayer's Armor"),
            Self::GronnstalkersArmor => f.write_str("Gronnstalker's Armor"),
            Self::MaleficRaiment => f.write_str("MaleficRaiment"),
            Self::TempestRegalia => f.write_str("TempestRegalia"),
            Self::OnslaughtBattlegear => f.write_str("OnslaughtBattlegear"),
            Self::OnslaughtArmor => f.write_str("OnslaughtArmor"),
            Self::AbsolutionRegalia => f.write_str("AbsolutionRegalia"),
            Self::VestmentsOfAbsolution => f.write_str("VestmentsOfAbsolution"),
            Self::ThunderheartHarness => f.write_str("ThunderheartHarness"),
            Self::ThunderheartRegalia => f.write_str("ThunderheartRegalia"),
            Self::ThunderheartRaiment => f.write_str("ThunderheartRaiment"),
            Self::LightbringerArmor => f.write_str("LightbringerArmor"),
            Self::LightbringerBattlegear => f.write_str("LightbringerBattlegear"),
            Self::LightbringerRaiment => f.write_str("LightbringerRaiment"),
            Self::SkyshatterHarness => f.write_str("SkyshatterHarness"),
            Self::SkyshatterRaiment => f.write_str("SkyshatterRaiment"),
            Self::SkyshatterRegalia => f.write_str("SkyshatterRegalia"),
            Self::GladiatorsRefuge => f.write_str("Gladiator's Refuge"),
            Self::GladiatorsWartide => f.write_str("Gladiator's Wartide"),
            Self::GladiatorsInvestiture => f.write_str("Gladiator's Investiture"),
            Self::GrandMarshalsRefuge => f.write_str("Grand Marshal's Refuge"),
            Self::HighWarlordsRefuge => f.write_str("High Warlord's Refuge"),
            Self::GladiatorsRedemption => f.write_str("Gladiator's Redemption"),
            Self::GrandMarshalsInvestiture => f.write_str("Grand Marshal's Investiture"),
            Self::HighWarlordsInvestiture => f.write_str("High Warlord's Investiture"),
            Self::GrandMarshalsRedemption => f.write_str("Grand Marshal's Redemption"),
            Self::HighWarlordsRedemption => f.write_str("High Warlord's Redemption"),
            Self::GrandMarshalsWartide => f.write_str("Grand Marshal's Wartide"),
            Self::HighWarlordsWartide => f.write_str("High Warlord's Wartide"),
            Self::ChampionsRedoubt => f.write_str("Champion's Redoubt"),
            Self::WarlordsAegis => f.write_str("Warlord's Aegis"),
            Self::TheTwinBladesOfAzzinoth => f.write_str("TheTwinBladesOfAzzinoth"),
            Self::MercilessGladiatorsAegis => f.write_str("Merciless Gladiator's Aegis"),
            Self::MercilessGladiatorsBattlegear => f.write_str("Merciless Gladiator's Battlegear"),
            Self::MercilessGladiatorsDreadgear => f.write_str("Merciless Gladiator's Dreadgear"),
            Self::MercilessGladiatorsEarthshaker => f.write_str("Merciless Gladiator's Earthshaker"),
            Self::MercilessGladiatorsFelshroud => f.write_str("Merciless Gladiator's Felshroud"),
            Self::MercilessGladiatorsInvestiture => f.write_str("Merciless Gladiator's Investiture"),
            Self::MercilessGladiatorsPursuit => f.write_str("Merciless Gladiator's Pursuit"),
            Self::MercilessGladiatorsRaiment => f.write_str("Merciless Gladiator's Raiment"),
            Self::MercilessGladiatorsRedemption => f.write_str("Merciless Gladiator's Redemption"),
            Self::MercilessGladiatorsRefuge => f.write_str("Merciless Gladiator's Refuge"),
            Self::MercilessGladiatorsRegalia => f.write_str("Merciless Gladiator's Regalia"),
            Self::MercilessGladiatorsSanctuary => f.write_str("Merciless Gladiator's Sanctuary"),
            Self::MercilessGladiatorsThunderfist => f.write_str("Merciless Gladiator's Thunderfist"),
            Self::MercilessGladiatorsVestments => f.write_str("Merciless Gladiator's Vestments"),
            Self::MercilessGladiatorsVindication => f.write_str("Merciless Gladiator's Vindication"),
            Self::MercilessGladiatorsWartide => f.write_str("Merciless Gladiator's Wartide"),
            Self::MercilessGladiatorsWildhide => f.write_str("Merciless Gladiator's Wildhide"),
            Self::FieldMarshalsEarthshaker => f.write_str("Field Marshal's Earthshaker"),
            Self::LieutenantCommandersEarthshaker => f.write_str("Lieutenant Commander's Earthshaker"),
            Self::TheFistsOfFury => f.write_str("TheFistsOfFury"),
            Self::VengefulGladiatorsRefuge => f.write_str("Vengeful Gladiator's Refuge"),
            Self::VengefulGladiatorsSanctuary => f.write_str("Vengeful Gladiator's Sanctuary"),
            Self::VengefulGladiatorsWildhide => f.write_str("Vengeful Gladiator's Wildhide"),
            Self::VengefulGladiatorsPursuit => f.write_str("Vengeful Gladiator's Pursuit"),
            Self::VengefulGladiatorsRegalia => f.write_str("Vengeful Gladiator's Regalia"),
            Self::VengefulGladiatorsRedemption => f.write_str("Vengeful Gladiator's Redemption"),
            Self::VengefulGladiatorsVindication => f.write_str("Vengeful Gladiator's Vindication"),
            Self::VengefulGladiatorsAegis => f.write_str("Vengeful Gladiator's Aegis"),
            Self::VengefulGladiatorsInvestiture => f.write_str("Vengeful Gladiator's Investiture"),
            Self::VengefulGladiatorsRaiment => f.write_str("Vengeful Gladiator's Raiment"),
            Self::VengefulGladiatorsVestments => f.write_str("Vengeful Gladiator's Vestments"),
            Self::VengefulGladiatorsWartide => f.write_str("Vengeful Gladiator's Wartide"),
            Self::VengefulGladiatorsEarthshaker => f.write_str("Vengeful Gladiator's Earthshaker"),
            Self::VengefulGladiatorsThunderfist => f.write_str("Vengeful Gladiator's Thunderfist"),
            Self::VengefulGladiatorsDreadgear => f.write_str("Vengeful Gladiator's Dreadgear"),
            Self::VengefulGladiatorsFelshroud => f.write_str("Vengeful Gladiator's Felshroud"),
            Self::VengefulGladiatorsBattlegear => f.write_str("Vengeful Gladiator's Battlegear"),
            Self::LatrosFlurry => f.write_str("Latro's Flurry"),
            Self::DreadweaveBattlegear => f.write_str("DreadweaveBattlegear"),
            Self::MoonclothBattlegear => f.write_str("MoonclothBattlegear"),
            Self::SatinBattlegear => f.write_str("SatinBattlegear"),
            Self::EvokersSilkBattlegear => f.write_str("Evoker's Silk Battlegear"),
            Self::DragonhideBattlegear => f.write_str("DragonhideBattlegear"),
            Self::WyrmhideBattlegear => f.write_str("WyrmhideBattlegear"),
            Self::KodohideBattlegear => f.write_str("KodohideBattlegear"),
            Self::OpportunistsBattlegear => f.write_str("Opportunist's Battlegear"),
            Self::SeersMailBattlegear => f.write_str("Seer's Mail Battlegear"),
            Self::SeersRingmailBattlegear => f.write_str("Seer's Ringmail Battlegear"),
            Self::SeersLinkedBattlegear => f.write_str("Seer's Linked Battlegear"),
            Self::StalkersChainBattlegear => f.write_str("Stalker's Chain Battlegear"),
            Self::SavagePlateBattlegear => f.write_str("SavagePlateBattlegear"),
            Self::CrusadersOrnamentedBattlegear => f.write_str("Crusader's Ornamented Battlegear"),
            Self::CrusadersScaledBattlegear => f.write_str("Crusader's Scaled Battlegear"),
        }
    }
}

impl TryFrom<u16> for ItemSet {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
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
            552 => Ok(Self::WrathOfSpellfire),
            553 => Ok(Self::ShadowsEmbrace),
            554 => Ok(Self::PrimalMooncloth),
            555 => Ok(Self::NetherweaveVestments),
            556 => Ok(Self::ImbuedNetherweave),
            557 => Ok(Self::SoulclothEmbrace),
            558 => Ok(Self::ArcanoweaveVestments),
            559 => Ok(Self::SpellstrikeInfusion),
            560 => Ok(Self::FelIronPlate),
            561 => Ok(Self::FelIronChain),
            562 => Ok(Self::AdamantiteBattlegear),
            563 => Ok(Self::EnchantedAdamantiteArmor),
            564 => Ok(Self::FlameGuard),
            565 => Ok(Self::KhoriumWard),
            566 => Ok(Self::BurningRage),
            567 => Ok(Self::GladiatorsBattlegear),
            568 => Ok(Self::GladiatorsDreadgear),
            569 => Ok(Self::FaithInFelsteel),
            570 => Ok(Self::TheUnyielding),
            571 => Ok(Self::WhitemendWisdom),
            572 => Ok(Self::BattlecastGarb),
            573 => Ok(Self::FelSkin),
            574 => Ok(Self::StrengthOfTheClefthoof),
            575 => Ok(Self::FelstalkerArmor),
            576 => Ok(Self::FuryOfTheNether),
            577 => Ok(Self::GladiatorsVestments),
            578 => Ok(Self::GladiatorsEarthshaker),
            579 => Ok(Self::GladiatorsRegalia),
            580 => Ok(Self::GladiatorsThunderfist),
            581 => Ok(Self::GladiatorsRaiment),
            582 => Ok(Self::GladiatorsAegis),
            583 => Ok(Self::GladiatorsVindication),
            584 => Ok(Self::GladiatorsSanctuary),
            585 => Ok(Self::GladiatorsWildhide),
            586 => Ok(Self::GladiatorsPursuit),
            587 => Ok(Self::HighWarlordsAegis),
            588 => Ok(Self::HighWarlordsBattlegear),
            589 => Ok(Self::GrandMarshalsAegis),
            590 => Ok(Self::GrandMarshalsBattlegear),
            591 => Ok(Self::GrandMarshalsDreadgear),
            592 => Ok(Self::HighWarlordsDreadgear),
            593 => Ok(Self::GrandMarshalsEarthshaker),
            594 => Ok(Self::HighWarlordsEarthshaker),
            595 => Ok(Self::GrandMarshalsPursuit),
            596 => Ok(Self::HighWarlordsPursuit),
            597 => Ok(Self::GrandMarshalsRaiment),
            598 => Ok(Self::HighWarlordsRaiment),
            599 => Ok(Self::GrandMarshalsRegalia),
            600 => Ok(Self::HighWarlordsRegalia),
            601 => Ok(Self::GrandMarshalsSanctuary),
            602 => Ok(Self::HighWarlordsSanctuary),
            603 => Ok(Self::GrandMarshalsThunderfist),
            604 => Ok(Self::HighWarlordsThunderfist),
            605 => Ok(Self::GrandMarshalsVestments),
            606 => Ok(Self::HighWarlordsVestments),
            607 => Ok(Self::GrandMarshalsVindication),
            608 => Ok(Self::HighWarlordsVindication),
            609 => Ok(Self::GrandMarshalsWildhide),
            610 => Ok(Self::HighWarlordsWildhide),
            611 => Ok(Self::FelscaleArmor),
            612 => Ok(Self::ScaledDraenicArmor),
            613 => Ok(Self::ThickDraenicArmor),
            614 => Ok(Self::WildDraenishArmor),
            615 => Ok(Self::GladiatorsFelshroud),
            616 => Ok(Self::NetherscaleArmor),
            617 => Ok(Self::NetherstrikeArmor),
            618 => Ok(Self::WindhawkArmor),
            619 => Ok(Self::PrimalIntent),
            620 => Ok(Self::AssassinationArmor),
            621 => Ok(Self::Netherblade),
            622 => Ok(Self::Deathmantle),
            623 => Ok(Self::RighteousArmor),
            624 => Ok(Self::JusticarRaiment),
            625 => Ok(Self::JusticarArmor),
            626 => Ok(Self::JusticarBattlegear),
            627 => Ok(Self::CrystalforgeRaiment),
            628 => Ok(Self::CrystalforgeArmor),
            629 => Ok(Self::CrystalforgeBattlegear),
            630 => Ok(Self::TidefuryRaiment),
            631 => Ok(Self::CycloneRaiment),
            632 => Ok(Self::CycloneRegalia),
            633 => Ok(Self::CycloneHarness),
            634 => Ok(Self::CataclysmRaiment),
            635 => Ok(Self::CataclysmRegalia),
            636 => Ok(Self::CataclysmHarness),
            637 => Ok(Self::MoongladeRaiment),
            638 => Ok(Self::MalorneRaiment),
            639 => Ok(Self::MalorneRegalia),
            640 => Ok(Self::MalorneHarness),
            641 => Ok(Self::NordrassilHarness),
            642 => Ok(Self::NordrassilRaiment),
            643 => Ok(Self::NordrassilRegalia),
            644 => Ok(Self::OblivionRaiment),
            645 => Ok(Self::VoidheartRaiment),
            646 => Ok(Self::CorruptorRaiment),
            647 => Ok(Self::IncantersRegalia),
            648 => Ok(Self::AldorRegalia),
            649 => Ok(Self::TirisfalRegalia),
            650 => Ok(Self::BeastLordArmor),
            651 => Ok(Self::DemonStalkerArmor),
            652 => Ok(Self::RiftStalkerArmor),
            653 => Ok(Self::BoldArmor),
            654 => Ok(Self::WarbringerArmor),
            655 => Ok(Self::WarbringerBattlegear),
            656 => Ok(Self::DestroyerArmor),
            657 => Ok(Self::DestroyerBattlegear),
            658 => Ok(Self::ManaEtchedRegalia),
            659 => Ok(Self::WastewalkerArmor),
            660 => Ok(Self::DesolationBattlegear),
            661 => Ok(Self::DoomplateBattlegear),
            662 => Ok(Self::HallowedRaiment),
            663 => Ok(Self::IncarnateRaiment),
            664 => Ok(Self::IncarnateRegalia),
            665 => Ok(Self::AvatarRaiment),
            666 => Ok(Self::AvatarRegalia),
            667 => Ok(Self::TheTwinStars),
            668 => Ok(Self::SlayersArmor),
            669 => Ok(Self::GronnstalkersArmor),
            670 => Ok(Self::MaleficRaiment),
            671 => Ok(Self::TempestRegalia),
            672 => Ok(Self::OnslaughtBattlegear),
            673 => Ok(Self::OnslaughtArmor),
            674 => Ok(Self::AbsolutionRegalia),
            675 => Ok(Self::VestmentsOfAbsolution),
            676 => Ok(Self::ThunderheartHarness),
            677 => Ok(Self::ThunderheartRegalia),
            678 => Ok(Self::ThunderheartRaiment),
            679 => Ok(Self::LightbringerArmor),
            680 => Ok(Self::LightbringerBattlegear),
            681 => Ok(Self::LightbringerRaiment),
            682 => Ok(Self::SkyshatterHarness),
            683 => Ok(Self::SkyshatterRaiment),
            684 => Ok(Self::SkyshatterRegalia),
            685 => Ok(Self::GladiatorsRefuge),
            686 => Ok(Self::GladiatorsWartide),
            687 => Ok(Self::GladiatorsInvestiture),
            688 => Ok(Self::GrandMarshalsRefuge),
            689 => Ok(Self::HighWarlordsRefuge),
            690 => Ok(Self::GladiatorsRedemption),
            691 => Ok(Self::GrandMarshalsInvestiture),
            692 => Ok(Self::HighWarlordsInvestiture),
            693 => Ok(Self::GrandMarshalsRedemption),
            694 => Ok(Self::HighWarlordsRedemption),
            695 => Ok(Self::GrandMarshalsWartide),
            696 => Ok(Self::HighWarlordsWartide),
            697 => Ok(Self::ChampionsRedoubt),
            698 => Ok(Self::WarlordsAegis),
            699 => Ok(Self::TheTwinBladesOfAzzinoth),
            700 => Ok(Self::MercilessGladiatorsAegis),
            701 => Ok(Self::MercilessGladiatorsBattlegear),
            702 => Ok(Self::MercilessGladiatorsDreadgear),
            703 => Ok(Self::MercilessGladiatorsEarthshaker),
            704 => Ok(Self::MercilessGladiatorsFelshroud),
            705 => Ok(Self::MercilessGladiatorsInvestiture),
            706 => Ok(Self::MercilessGladiatorsPursuit),
            707 => Ok(Self::MercilessGladiatorsRaiment),
            708 => Ok(Self::MercilessGladiatorsRedemption),
            709 => Ok(Self::MercilessGladiatorsRefuge),
            710 => Ok(Self::MercilessGladiatorsRegalia),
            711 => Ok(Self::MercilessGladiatorsSanctuary),
            712 => Ok(Self::MercilessGladiatorsThunderfist),
            713 => Ok(Self::MercilessGladiatorsVestments),
            714 => Ok(Self::MercilessGladiatorsVindication),
            715 => Ok(Self::MercilessGladiatorsWartide),
            716 => Ok(Self::MercilessGladiatorsWildhide),
            717 => Ok(Self::FieldMarshalsEarthshaker),
            718 => Ok(Self::LieutenantCommandersEarthshaker),
            719 => Ok(Self::TheFistsOfFury),
            720 => Ok(Self::VengefulGladiatorsRefuge),
            721 => Ok(Self::VengefulGladiatorsSanctuary),
            722 => Ok(Self::VengefulGladiatorsWildhide),
            723 => Ok(Self::VengefulGladiatorsPursuit),
            724 => Ok(Self::VengefulGladiatorsRegalia),
            725 => Ok(Self::VengefulGladiatorsRedemption),
            726 => Ok(Self::VengefulGladiatorsVindication),
            727 => Ok(Self::VengefulGladiatorsAegis),
            728 => Ok(Self::VengefulGladiatorsInvestiture),
            729 => Ok(Self::VengefulGladiatorsRaiment),
            730 => Ok(Self::VengefulGladiatorsVestments),
            731 => Ok(Self::VengefulGladiatorsWartide),
            732 => Ok(Self::VengefulGladiatorsEarthshaker),
            733 => Ok(Self::VengefulGladiatorsThunderfist),
            734 => Ok(Self::VengefulGladiatorsDreadgear),
            735 => Ok(Self::VengefulGladiatorsFelshroud),
            736 => Ok(Self::VengefulGladiatorsBattlegear),
            737 => Ok(Self::LatrosFlurry),
            738 => Ok(Self::DreadweaveBattlegear),
            739 => Ok(Self::MoonclothBattlegear),
            740 => Ok(Self::SatinBattlegear),
            741 => Ok(Self::EvokersSilkBattlegear),
            742 => Ok(Self::DragonhideBattlegear),
            743 => Ok(Self::WyrmhideBattlegear),
            744 => Ok(Self::KodohideBattlegear),
            745 => Ok(Self::OpportunistsBattlegear),
            746 => Ok(Self::SeersMailBattlegear),
            747 => Ok(Self::SeersRingmailBattlegear),
            748 => Ok(Self::SeersLinkedBattlegear),
            749 => Ok(Self::StalkersChainBattlegear),
            750 => Ok(Self::SavagePlateBattlegear),
            751 => Ok(Self::CrusadersOrnamentedBattlegear),
            752 => Ok(Self::CrusadersScaledBattlegear),
            v => Err(crate::errors::EnumError::new("ItemSet", v as u64),)
        }
    }
}

