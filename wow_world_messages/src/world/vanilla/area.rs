use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/area.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/area.wowm#L1):
/// ```text
/// enum Area : u32 {
///     NONE = 0;
///     DUN_MOROGH = 1;
///     LONGSHORE = 2;
///     BADLANDS = 3;
///     BLASTED_LANDS = 4;
///     BLACKWATER_COVE = 7;
///     SWAMP_OF_SORROWS = 8;
///     NORTHSHIRE_VALLEY = 9;
///     DUSKWOOD = 10;
///     WETLANDS = 11;
///     ELWYNN_FOREST = 12;
///     THE_WORLD_TREE = 13;
///     DUROTAR = 14;
///     DUSTWALLOW_MARSH = 15;
///     AZSHARA = 16;
///     THE_BARRENS = 17;
///     CRYSTAL_LAKE = 18;
///     ZUL_GURUB0 = 19;
///     MOONBROOK = 20;
///     KUL_TIRAS = 21;
///     PROGRAMMER_ISLE = 22;
///     NORTHSHIRE_RIVER = 23;
///     NORTHSHIRE_ABBEY = 24;
///     BLACKROCK_MOUNTAIN0 = 25;
///     LIGHTHOUSE = 26;
///     WESTERN_PLAGUELANDS = 28;
///     NINE = 30;
///     THE_CEMETARY = 32;
///     STRANGLETHORN_VALE = 33;
///     ECHO_RIDGE_MINE = 34;
///     BOOTY_BAY = 35;
///     ALTERAC_MOUNTAINS = 36;
///     LAKE_NAZFERITI = 37;
///     LOCH_MODAN = 38;
///     WESTFALL0 = 40;
///     DEADWIND_PASS = 41;
///     DARKSHIRE = 42;
///     WILD_SHORE = 43;
///     REDRIDGE_MOUNTAINS = 44;
///     ARATHI_HIGHLANDS = 45;
///     BURNING_STEPPES = 46;
///     THE_HINTERLANDS = 47;
///     DEAD_MANS_HOLE = 49;
///     SEARING_GORGE = 51;
///     THIEVES_CAMP = 53;
///     JASPERLODE_MINE = 54;
///     VALLEY_OF_HEROES_UNUSED = 55;
///     HEROES_VIGIL = 56;
///     FARGODEEP_MINE = 57;
///     NORTHSHIRE_VINEYARDS = 59;
///     FORESTS_EDGE = 60;
///     THUNDER_FALLS = 61;
///     BRACKWELL_PUMPKIN_PATCH = 62;
///     THE_STONEFIELD_FARM = 63;
///     THE_MACLURE_VINEYARDS = 64;
///     ON_MAP_DUNGEON0 = 65;
///     ON_MAP_DUNGEON1 = 66;
///     ON_MAP_DUNGEON2 = 67;
///     LAKE_EVERSTILL = 68;
///     LAKESHIRE = 69;
///     STONEWATCH = 70;
///     STONEWATCH_FALLS = 71;
///     THE_DARK_PORTAL = 72;
///     THE_TAINTED_SCAR = 73;
///     POOL_OF_TEARS = 74;
///     STONARD = 75;
///     FALLOW_SANCTUARY = 76;
///     ANVILMAR = 77;
///     STORMWIND_MOUNTAINS = 80;
///     JEFF_NE_QUADRANT_CHANGED = 81;
///     JEFF_NW_QUADRANT = 82;
///     JEFF_SE_QUADRANT = 83;
///     JEFF_SW_QUADRANT = 84;
///     TIRISFAL_GLADES = 85;
///     STONE_CAIRN_LAKE = 86;
///     GOLDSHIRE = 87;
///     EASTVALE_LOGGING_CAMP = 88;
///     MIRROR_LAKE_ORCHARD = 89;
///     TOWER_OF_AZORA = 91;
///     MIRROR_LAKE = 92;
///     VUL_GOL_OGRE_MOUND = 93;
///     RAVEN_HILL = 94;
///     REDRIDGE_CANYONS = 95;
///     TOWER_OF_ILGALAR = 96;
///     ALTHERS_MILL = 97;
///     RETHBAN_CAVERNS = 98;
///     REBEL_CAMP = 99;
///     NESINGWARYS_EXPEDITION = 100;
///     KURZENS_COMPOUND = 101;
///     RUINS_OF_ZUL_KUNDA = 102;
///     RUINS_OF_ZUL_MAMWE = 103;
///     THE_VILE_REEF = 104;
///     MOSH_OGG_OGRE_MOUND = 105;
///     THE_STOCKPILE = 106;
///     SALDEANS_FARM = 107;
///     SENTINEL_HILL = 108;
///     FURLBROWS_PUMPKIN_FARM = 109;
///     JANGOLODE_MINE = 111;
///     GOLD_COAST_QUARRY = 113;
///     WESTFALL_LIGHTHOUSE = 115;
///     MISTY_VALLEY = 116;
///     GROMGOL_BASE_CAMP = 117;
///     WHELGARS_EXCAVATION_SITE = 118;
///     WESTBROOK_GARRISON = 120;
///     TRANQUIL_GARDENS_CEMETERY = 121;
///     ZUULDAIA_RUINS = 122;
///     BALLAL_RUINS = 123;
///     KALAI_RUINS = 125;
///     TKASHI_RUINS = 126;
///     BALIAMAH_RUINS = 127;
///     ZIATAJAI_RUINS = 128;
///     MIZJAH_RUINS = 129;
///     SILVERPINE_FOREST = 130;
///     KHARANOS = 131;
///     COLDRIDGE_VALLEY = 132;
///     GNOMEREGAN0 = 133;
///     GOL_BOLAR_QUARRY = 134;
///     FROSTMANE_HOLD = 135;
///     THE_GRIZZLED_DEN = 136;
///     BREWNALL_VILLAGE = 137;
///     MISTY_PINE_REFUGE = 138;
///     EASTERN_PLAGUELANDS = 139;
///     TELDRASSIL = 141;
///     IRONBANDS_EXCAVATION_SITE = 142;
///     MOGROSH_STRONGHOLD = 143;
///     THELSAMAR = 144;
///     ALGAZ_GATE = 145;
///     STONEWROUGHT_DAM = 146;
///     THE_FARSTRIDER_LODGE = 147;
///     DARKSHORE = 148;
///     SILVER_STREAM_MINE = 149;
///     MENETHIL_HARBOR = 150;
///     DESIGNER_ISLAND = 151;
///     THE_BULWARK0 = 152;
///     RUINS_OF_LORDAERON = 153;
///     DEATHKNELL = 154;
///     NIGHT_WEBS_HOLLOW = 155;
///     SOLLIDEN_FARMSTEAD = 156;
///     AGAMAND_MILLS = 157;
///     AGAMAND_FAMILY_CRYPT = 158;
///     BRILL = 159;
///     WHISPERING_GARDENS = 160;
///     TERRACE_OF_REPOSE = 161;
///     BRIGHTWATER_LAKE = 162;
///     GUNTHERS_RETREAT = 163;
///     GARRENS_HAUNT = 164;
///     BALNIR_FARMSTEAD = 165;
///     COLD_HEARTH_MANOR = 166;
///     CRUSADER_OUTPOST = 167;
///     THE_NORTH_COAST = 168;
///     WHISPERING_SHORE = 169;
///     LORDAMERE_LAKE0 = 170;
///     FENRIS_ISLE = 172;
///     FAOLS_REST = 173;
///     DOLANAAR = 186;
///     DARNASSUS_UNUSED = 187;
///     SHADOWGLEN = 188;
///     STEELGRILLS_DEPOT = 189;
///     HEARTHGLEN = 190;
///     NORTHRIDGE_LUMBER_CAMP = 192;
///     RUINS_OF_ANDORHAL = 193;
///     SCHOOL_OF_NECROMANCY = 195;
///     UTHERS_TOMB = 196;
///     SORROW_HILL = 197;
///     THE_WEEPING_CAVE = 198;
///     FELSTONE_FIELD = 199;
///     DALSONS_TEARS = 200;
///     GAHRRONS_WITHERING = 201;
///     THE_WRITHING_HAUNT = 202;
///     MARDENHOLDE_KEEP = 203;
///     PYREWOOD_VILLAGE = 204;
///     DUN_MODR = 205;
///     WESTFALL1 = 206;
///     THE_GREAT_SEA0 = 207;
///     UNUSED_IRONCLADCOVE = 208;
///     SHADOWFANG_KEEP0 = 209;
///     ON_MAP_DUNGEON3 = 210;
///     ICEFLOW_LAKE = 211;
///     HELMS_BED_LAKE = 212;
///     DEEP_ELEM_MINE = 213;
///     THE_GREAT_SEA1 = 214;
///     MULGORE = 215;
///     ALEXSTON_FARMSTEAD = 219;
///     RED_CLOUD_MESA = 220;
///     CAMP_NARACHE = 221;
///     BLOODHOOF_VILLAGE = 222;
///     STONEBULL_LAKE = 223;
///     RAVAGED_CARAVAN = 224;
///     RED_ROCKS = 225;
///     THE_SKITTERING_DARK = 226;
///     VALGANS_FIELD = 227;
///     THE_SEPULCHER = 228;
///     OLSENS_FARTHING = 229;
///     THE_GREYMANE_WALL = 230;
///     BERENS_PERIL = 231;
///     THE_DAWNING_ISLES = 232;
///     AMBERMILL = 233;
///     FENRIS_KEEP = 235;
///     SHADOWFANG_KEEP1 = 236;
///     THE_DECREPIT_FERRY = 237;
///     MALDENS_ORCHARD = 238;
///     THE_IVAR_PATCH = 239;
///     THE_DEAD_FIELD = 240;
///     THE_ROTTING_ORCHARD = 241;
///     BRIGHTWOOD_GROVE = 242;
///     FORLORN_ROWE = 243;
///     THE_WHIPPLE_ESTATE = 244;
///     THE_YORGEN_FARMSTEAD = 245;
///     THE_CAULDRON = 246;
///     GRIMESILT_DIG_SITE = 247;
///     DREADMAUL_ROCK = 249;
///     RUINS_OF_THAURISSAN = 250;
///     FLAME_CREST = 251;
///     BLACKROCK_STRONGHOLD = 252;
///     THE_PILLAR_OF_ASH = 253;
///     BLACKROCK_MOUNTAIN1 = 254;
///     ALTAR_OF_STORMS0 = 255;
///     ALDRASSIL = 256;
///     SHADOWTHREAD_CAVE = 257;
///     FEL_ROCK = 258;
///     LAKE_AL_AMETH = 259;
///     STARBREEZE_VILLAGE = 260;
///     GNARLPINE_HOLD = 261;
///     BANETHIL_BARROW_DEN = 262;
///     THE_CLEFT = 263;
///     THE_ORACLE_GLADE = 264;
///     WELLSPRING_RIVER = 265;
///     WELLSPRING_LAKE = 266;
///     HILLSBRAD_FOOTHILLS = 267;
///     AZSHARA_CRATER = 268;
///     DUN_ALGAZ0 = 269;
///     SOUTHSHORE0 = 271;
///     TARREN_MILL0 = 272;
///     DURNHOLDE_KEEP0 = 275;
///     UNUSED_STONEWROUGHT_PASS = 276;
///     THE_FOOTHILL_CAVERNS = 277;
///     LORDAMERE_INTERNMENT_CAMP = 278;
///     DALARAN = 279;
///     STRAHNBRAD = 280;
///     RUINS_OF_ALTERAC = 281;
///     CRUSHRIDGE_HOLD = 282;
///     SLAUGHTER_HOLLOW = 283;
///     THE_UPLANDS = 284;
///     SOUTHPOINT_TOWER0 = 285;
///     HILLSBRAD_FIELDS0 = 286;
///     HILLSBRAD = 287;
///     AZURELODE_MINE0 = 288;
///     NETHANDER_STEAD0 = 289;
///     DUN_GAROK0 = 290;
///     THORADINS_WALL0 = 293;
///     EASTERN_STRAND0 = 294;
///     WESTERN_STRAND0 = 295;
///     SOUTH_SEAS_UNUSED = 296;
///     JAGUERO_ISLE = 297;
///     BARADIN_BAY = 298;
///     MENETHIL_BAY = 299;
///     MISTY_REED_STRAND = 300;
///     THE_SAVAGE_COAST = 301;
///     THE_CRYSTAL_SHORE = 302;
///     SHELL_BEACH = 303;
///     NORTH_TIDES_RUN = 305;
///     SOUTH_TIDES_RUN = 306;
///     THE_OVERLOOK_CLIFFS = 307;
///     THE_FORBIDDING_SEA0 = 308;
///     IRONBEARDS_TOMB = 309;
///     CRYSTALVEIN_MINE = 310;
///     RUINS_OF_ABORAZ = 311;
///     JANEIROS_POINT = 312;
///     NORTHFOLD_MANOR = 313;
///     GO_SHEK_FARM = 314;
///     DABYRIES_FARMSTEAD = 315;
///     BOULDERFIST_HALL = 316;
///     WITHERBARK_VILLAGE = 317;
///     DRYWHISKER_GORGE = 318;
///     REFUGE_POINTE = 320;
///     HAMMERFALL = 321;
///     BLACKWATER_SHIPWRECKS = 322;
///     O_BREENS_CAMP = 323;
///     STROMGARDE_KEEP = 324;
///     THE_TOWER_OF_ARATHOR = 325;
///     THE_SANCTUM = 326;
///     FALDIRS_COVE = 327;
///     THE_DROWNED_REEF = 328;
///     THANDOL_SPAN0 = 330;
///     ASHENVALE = 331;
///     THE_GREAT_SEA2 = 332;
///     CIRCLE_OF_EAST_BINDING = 333;
///     CIRCLE_OF_WEST_BINDING = 334;
///     CIRCLE_OF_INNER_BINDING = 335;
///     CIRCLE_OF_OUTER_BINDING = 336;
///     APOCRYPHANS_REST = 337;
///     ANGOR_FORTRESS = 338;
///     LETHLOR_RAVINE = 339;
///     KARGATH = 340;
///     CAMP_KOSH = 341;
///     CAMP_BOFF = 342;
///     CAMP_WURG = 343;
///     CAMP_CAGG = 344;
///     AGMONDS_END = 345;
///     HAMMERTOES_DIGSITE = 346;
///     DUSTBELCH_GROTTO = 347;
///     AERIE_PEAK = 348;
///     WILDHAMMER_KEEP = 349;
///     QUEL_DANIL_LODGE = 350;
///     SKULK_ROCK = 351;
///     ZUNWATHA = 352;
///     SHADRA_ALOR = 353;
///     JINTHA_ALOR = 354;
///     THE_ALTAR_OF_ZUL = 355;
///     SERADANE = 356;
///     FERALAS = 357;
///     BRAMBLEBLADE_RAVINE = 358;
///     BAEL_MODAN = 359;
///     THE_VENTURE_CO_MINE = 360;
///     FELWOOD = 361;
///     RAZOR_HILL = 362;
///     VALLEY_OF_TRIALS = 363;
///     THE_DEN = 364;
///     BURNING_BLADE_COVEN = 365;
///     KOLKAR_CRAG = 366;
///     SENJIN_VILLAGE = 367;
///     ECHO_ISLES = 368;
///     THUNDER_RIDGE = 369;
///     DRYGULCH_RAVINE = 370;
///     DUSTWIND_CAVE = 371;
///     TIRAGARDE_KEEP = 372;
///     SCUTTLE_COAST = 373;
///     BLADEFIST_BAY = 374;
///     DEADEYE_SHORE = 375;
///     SOUTHFURY_RIVER0 = 377;
///     CAMP_TAURAJO = 378;
///     FAR_WATCH_POST = 379;
///     THE_CROSSROADS = 380;
///     BOULDER_LODE_MINE = 381;
///     THE_SLUDGE_FEN = 382;
///     THE_DRY_HILLS = 383;
///     DREADMIST_PEAK = 384;
///     NORTHWATCH_HOLD = 385;
///     THE_FORGOTTEN_POOLS = 386;
///     LUSHWATER_OASIS = 387;
///     THE_STAGNANT_OASIS = 388;
///     FIELD_OF_GIANTS = 390;
///     THE_MERCHANT_COAST = 391;
///     RATCHET = 392;
///     DARKSPEAR_STRAND = 393;
///     DARROWMERE_LAKE_UNUSED = 394;
///     CAER_DARROW_UNUSED = 395;
///     WINTERHOOF_WATER_WELL = 396;
///     THUNDERHORN_WATER_WELL = 397;
///     WILDMANE_WATER_WELL = 398;
///     SKYLINE_RIDGE = 399;
///     THOUSAND_NEEDLES = 400;
///     THE_TIDUS_STAIR = 401;
///     SHADY_REST_INN = 403;
///     BAELDUN_DIGSITE = 404;
///     DESOLACE = 405;
///     STONETALON_MOUNTAINS = 406;
///     ORGRIMMAR_UNUSED = 407;
///     GILLIJIMS_ISLE = 408;
///     ISLAND_OF_DOCTOR_LAPIDIS = 409;
///     RAZORWIND_CANYON = 410;
///     BATHRANS_HAUNT = 411;
///     THE_RUINS_OF_ORDIL_ARAN = 412;
///     MAESTRAS_POST = 413;
///     THE_ZORAM_STRAND = 414;
///     ASTRANAAR = 415;
///     THE_SHRINE_OF_AESSINA = 416;
///     FIRE_SCAR_SHRINE = 417;
///     THE_RUINS_OF_STARDUST = 418;
///     THE_HOWLING_VALE = 419;
///     SILVERWIND_REFUGE = 420;
///     MYSTRAL_LAKE = 421;
///     FALLEN_SKY_LAKE = 422;
///     IRIS_LAKE = 424;
///     MOONWELL = 425;
///     RAYNEWOOD_RETREAT = 426;
///     THE_SHADY_NOOK = 427;
///     NIGHT_RUN = 428;
///     XAVIAN = 429;
///     SATYRNAAR = 430;
///     SPLINTERTREE_POST = 431;
///     THE_DOR_DANIL_BARROW_DEN = 432;
///     FALFARREN_RIVER = 433;
///     FELFIRE_HILL = 434;
///     DEMON_FALL_CANYON = 435;
///     DEMON_FALL_RIDGE = 436;
///     WARSONG_LUMBER_CAMP = 437;
///     BOUGH_SHADOW = 438;
///     THE_SHIMMERING_FLATS = 439;
///     TANARIS = 440;
///     LAKE_FALATHIM = 441;
///     AUBERDINE = 442;
///     RUINS_OF_MATHYSTRA = 443;
///     TOWER_OF_ALTHALAXX = 444;
///     CLIFFSPRING_FALLS = 445;
///     BASHAL_ARAN = 446;
///     AMETH_ARAN = 447;
///     GROVE_OF_THE_ANCIENTS = 448;
///     THE_MASTERS_GLAIVE = 449;
///     REMTRAVELS_EXCAVATION = 450;
///     MISTS_EDGE = 452;
///     THE_LONG_WASH = 453;
///     WILDBEND_RIVER = 454;
///     BLACKWOOD_DEN = 455;
///     CLIFFSPRING_RIVER = 456;
///     THE_VEILED_SEA0 = 457;
///     GOLD_ROAD = 458;
///     SCARLET_WATCH_POST = 459;
///     SUN_ROCK_RETREAT = 460;
///     WINDSHEAR_CRAG = 461;
///     CRAGPOOL_LAKE = 463;
///     MIRKFALLON_LAKE = 464;
///     THE_CHARRED_VALE = 465;
///     VALLEY_OF_THE_BLOODFURIES = 466;
///     STONETALON_PEAK = 467;
///     THE_TALON_DEN = 468;
///     GREATWOOD_VALE = 469;
///     THUNDER_BLUFF_UNUSED = 470;
///     BRAVE_WIND_MESA = 471;
///     FIRE_STONE_MESA = 472;
///     MANTLE_ROCK = 473;
///     HUNTER_RISE_UNUSED = 474;
///     SPIRIT_RISE_UNUSED = 475;
///     ELDER_RISE_UNUSED = 476;
///     RUINS_OF_JUBUWAL = 477;
///     POOLS_OF_ARLITHRIEN = 478;
///     THE_RUSTMAUL_DIG_SITE = 479;
///     CAMP_ETHOK = 480;
///     SPLITHOOF_CRAG = 481;
///     HIGHPERCH = 482;
///     THE_SCREECHING_CANYON = 483;
///     FREEWIND_POST = 484;
///     THE_GREAT_LIFT0 = 485;
///     GALAK_HOLD = 486;
///     ROGUEFEATHER_DEN = 487;
///     THE_WEATHERED_NOOK = 488;
///     THALANAAR = 489;
///     UN_GORO_CRATER = 490;
///     RAZORFEN_KRAUL0 = 491;
///     RAVEN_HILL_CEMETERY = 492;
///     MOONGLADE = 493;
///     DELETE_ME0 = 495;
///     BRACKENWALL_VILLAGE = 496;
///     SWAMPLIGHT_MANOR = 497;
///     BLOODFEN_BURROW = 498;
///     DARKMIST_CAVERN = 499;
///     MOGGLE_POINT = 500;
///     BEEZILS_WRECK = 501;
///     WITCH_HILL = 502;
///     SENTRY_POINT = 503;
///     NORTH_POINT_TOWER = 504;
///     WEST_POINT_TOWER = 505;
///     LOST_POINT = 506;
///     BLUEFEN = 507;
///     STONEMAUL_RUINS = 508;
///     THE_DEN_OF_FLAME = 509;
///     THE_DRAGONMURK = 510;
///     WYRMBOG = 511;
///     ONYXIAS_LAIR_UNUSED = 512;
///     THERAMORE_ISLE = 513;
///     FOOTHOLD_CITADEL = 514;
///     IRONCLAD_PRISON = 515;
///     DUSTWALLOW_BAY = 516;
///     TIDEFURY_COVE = 517;
///     DREADMURK_SHORE = 518;
///     ADDLES_STEAD = 536;
///     FIRE_PLUME_RIDGE = 537;
///     LAKKARI_TAR_PITS = 538;
///     TERROR_RUN = 539;
///     THE_SLITHERING_SCAR = 540;
///     MARSHALS_REFUGE = 541;
///     FUNGAL_ROCK = 542;
///     GOLAKKA_HOT_SPRINGS = 543;
///     THE_LOCH = 556;
///     BEGGARS_HAUNT = 576;
///     KODO_GRAVEYARD = 596;
///     GHOST_WALKER_POST = 597;
///     SARTHERIS_STRAND = 598;
///     THUNDER_AXE_FORTRESS = 599;
///     BOLGANS_HOLE = 600;
///     MANNOROC_COVEN = 602;
///     SARGERON = 603;
///     MAGRAM_VILLAGE = 604;
///     GELKIS_VILLAGE = 606;
///     VALLEY_OF_SPEARS = 607;
///     NIJELS_POINT = 608;
///     KOLKAR_VILLAGE = 609;
///     HYJAL = 616;
///     WINTERSPRING = 618;
///     BLACKWOLF_RIVER = 636;
///     KODO_ROCK = 637;
///     HIDDEN_PATH = 638;
///     SPIRIT_ROCK = 639;
///     SHRINE_OF_THE_DORMANT_FLAME = 640;
///     LAKE_ELUNEARA = 656;
///     THE_HARBORAGE = 657;
///     OUTLAND = 676;
///     CRAFTSMENS_TERRACE_UNUSED = 696;
///     TRADESMENS_TERRACE_UNUSED = 697;
///     THE_TEMPLE_GARDENS_UNUSED = 698;
///     TEMPLE_OF_ELUNE_UNUSED = 699;
///     CENARION_ENCLAVE_UNUSED = 700;
///     WARRIORS_TERRACE_UNUSED = 701;
///     RUTTHERAN_VILLAGE = 702;
///     IRONBANDS_COMPOUND = 716;
///     THE_STOCKADE = 717;
///     WAILING_CAVERNS = 718;
///     BLACKFATHOM_DEEPS0 = 719;
///     FRAY_ISLAND = 720;
///     GNOMEREGAN1 = 721;
///     RAZORFEN_DOWNS0 = 722;
///     BANETHIL_HOLLOW = 736;
///     SCARLET_MONASTERY = 796;
///     JERODS_LANDING = 797;
///     RIDGEPOINT_TOWER = 798;
///     THE_DARKENED_BANK = 799;
///     COLDRIDGE_PASS = 800;
///     CHILL_BREEZE_VALLEY = 801;
///     SHIMMER_RIDGE = 802;
///     AMBERSTILL_RANCH = 803;
///     THE_TUNDRID_HILLS = 804;
///     SOUTH_GATE_PASS0 = 805;
///     SOUTH_GATE_OUTPOST = 806;
///     NORTH_GATE_PASS0 = 807;
///     NORTH_GATE_OUTPOST = 808;
///     GATES_OF_IRONFORGE = 809;
///     STILLWATER_POND = 810;
///     NIGHTMARE_VALE = 811;
///     VENOMWEB_VALE = 812;
///     THE_BULWARK1 = 813;
///     SOUTHFURY_RIVER1 = 814;
///     SOUTHFURY_RIVER2 = 815;
///     RAZORMANE_GROUNDS = 816;
///     SKULL_ROCK = 817;
///     PALEMANE_ROCK = 818;
///     WINDFURY_RIDGE = 819;
///     THE_GOLDEN_PLAINS = 820;
///     THE_ROLLING_PLAINS = 821;
///     DUN_ALGAZ1 = 836;
///     DUN_ALGAZ2 = 837;
///     NORTH_GATE_PASS1 = 838;
///     SOUTH_GATE_PASS1 = 839;
///     TWILIGHT_GROVE = 856;
///     GM_ISLAND = 876;
///     DELETE_ME1 = 877;
///     SOUTHFURY_RIVER3 = 878;
///     SOUTHFURY_RIVER4 = 879;
///     THANDOL_SPAN1 = 880;
///     THANDOL_SPAN2 = 881;
///     PURGATION_ISLE = 896;
///     THE_JANSEN_STEAD = 916;
///     THE_DEAD_ACRE = 917;
///     THE_MOLSEN_FARM = 918;
///     STENDELS_POND = 919;
///     THE_DAGGER_HILLS = 920;
///     DEMONTS_PLACE = 921;
///     THE_DUST_PLAINS = 922;
///     STONESPLINTER_VALLEY = 923;
///     VALLEY_OF_KINGS = 924;
///     ALGAZ_STATION = 925;
///     BUCKLEBREE_FARM = 926;
///     THE_SHINING_STRAND = 927;
///     NORTH_TIDES_HOLLOW = 928;
///     GRIZZLEPAW_RIDGE = 936;
///     THE_VERDANT_FIELDS = 956;
///     GADGETZAN = 976;
///     STEAMWHEEDLE_PORT = 977;
///     ZUL_FARRAK0 = 978;
///     SANDSORROW_WATCH = 979;
///     THISTLESHRUB_VALLEY = 980;
///     THE_GAPING_CHASM = 981;
///     THE_NOXIOUS_LAIR = 982;
///     DUNEMAUL_COMPOUND = 983;
///     EASTMOON_RUINS = 984;
///     WATERSPRING_FIELD = 985;
///     ZALASHJIS_DEN = 986;
///     LANDS_END_BEACH = 987;
///     WAVESTRIDER_BEACH = 988;
///     ULDUM = 989;
///     VALLEY_OF_THE_WATCHERS = 990;
///     GUNSTANS_POST = 991;
///     SOUTHMOON_RUINS = 992;
///     RENDERS_CAMP = 996;
///     RENDERS_VALLEY = 997;
///     RENDERS_ROCK = 998;
///     STONEWATCH_TOWER = 999;
///     GALARDELL_VALLEY = 1000;
///     LAKERIDGE_HIGHWAY = 1001;
///     THREE_CORNERS = 1002;
///     DIREFORGE_HILL = 1016;
///     RAPTOR_RIDGE = 1017;
///     BLACK_CHANNEL_MARSH = 1018;
///     THE_GREEN_BELT0 = 1019;
///     MOSSHIDE_FEN = 1020;
///     THELGEN_ROCK = 1021;
///     BLUEGILL_MARSH = 1022;
///     SALTSPRAY_GLEN = 1023;
///     SUNDOWN_MARSH = 1024;
///     THE_GREEN_BELT1 = 1025;
///     ANGERFANG_ENCAMPMENT = 1036;
///     GRIM_BATOL = 1037;
///     DRAGONMAW_GATES = 1038;
///     THE_LOST_FLEET = 1039;
///     DARROW_HILL0 = 1056;
///     THORADINS_WALL1 = 1057;
///     WEBWINDER_PATH = 1076;
///     THE_HUSHED_BANK = 1097;
///     MANOR_MISTMANTLE = 1098;
///     CAMP_MOJACHE = 1099;
///     GRIMTOTEM_COMPOUND = 1100;
///     THE_WRITHING_DEEP = 1101;
///     WILDWIND_LAKE = 1102;
///     GORDUNNI_OUTPOST = 1103;
///     MOK_GORDUN = 1104;
///     FERAL_SCAR_VALE = 1105;
///     FRAYFEATHER_HIGHLANDS = 1106;
///     IDLEWIND_LAKE = 1107;
///     THE_FORGOTTEN_COAST = 1108;
///     EAST_PILLAR = 1109;
///     WEST_PILLAR = 1110;
///     DREAM_BOUGH = 1111;
///     JADEMIR_LAKE = 1112;
///     ONEIROS = 1113;
///     RUINS_OF_RAVENWIND = 1114;
///     RAGE_SCAR_HOLD = 1115;
///     FEATHERMOON_STRONGHOLD = 1116;
///     RUINS_OF_SOLARSAL = 1117;
///     LOWER_WILDS_UNUSED = 1118;
///     THE_TWIN_COLOSSALS = 1119;
///     SARDOR_ISLE = 1120;
///     ISLE_OF_DREAD = 1121;
///     HIGH_WILDERNESS = 1136;
///     LOWER_WILDS = 1137;
///     SOUTHERN_BARRENS = 1156;
///     SOUTHERN_GOLD_ROAD = 1157;
///     ZUL_FARRAK1 = 1176;
///     UNUSED_ALCAZ_ISLAND = 1196;
///     TIMBERMAW_HOLD0 = 1216;
///     VANNDIR_ENCAMPMENT = 1217;
///     TEST_AZSHARA = 1218;
///     LEGASH_ENCAMPMENT = 1219;
///     THALASSIAN_BASE_CAMP = 1220;
///     RUINS_OF_ELDARATH = 1221;
///     HETAERAS_CLUTCH = 1222;
///     TEMPLE_OF_ZIN_MALOR = 1223;
///     BEARS_HEAD = 1224;
///     URSOLAN = 1225;
///     TEMPLE_OF_ARKKORAN = 1226;
///     BAY_OF_STORMS = 1227;
///     THE_SHATTERED_STRAND = 1228;
///     TOWER_OF_ELDARA = 1229;
///     JAGGED_REEF = 1230;
///     SOUTHRIDGE_BEACH = 1231;
///     RAVENCREST_MONUMENT = 1232;
///     FORLORN_RIDGE = 1233;
///     LAKE_MENNAR = 1234;
///     SHADOWSONG_SHRINE = 1235;
///     HALDARR_ENCAMPMENT = 1236;
///     VALORMOK = 1237;
///     THE_RUINED_REACHES = 1256;
///     THE_TALONDEEP_PATH0 = 1276;
///     THE_TALONDEEP_PATH1 = 1277;
///     ROCKTUSK_FARM = 1296;
///     JAGGEDSWINE_FARM = 1297;
///     RAZORFEN_DOWNS1 = 1316;
///     LOST_RIGGER_COVE = 1336;
///     ULDAMAN0 = 1337;
///     LORDAMERE_LAKE1 = 1338;
///     LORDAMERE_LAKE2 = 1339;
///     GALLOWS_CORNER = 1357;
///     SILITHUS = 1377;
///     EMERALD_FOREST = 1397;
///     SUNKEN_TEMPLE = 1417;
///     DREADMAUL_HOLD = 1437;
///     NETHERGARDE_KEEP = 1438;
///     DREADMAUL_POST = 1439;
///     SERPENTS_COIL = 1440;
///     ALTAR_OF_STORMS1 = 1441;
///     FIREWATCH_RIDGE = 1442;
///     THE_SLAG_PIT = 1443;
///     THE_SEA_OF_CINDERS = 1444;
///     BLACKROCK_MOUNTAIN2 = 1445;
///     THORIUM_POINT = 1446;
///     GARRISON_ARMORY = 1457;
///     THE_TEMPLE_OF_ATAL_HAKKAR = 1477;
///     UNDERCITY = 1497;
///     ULDAMAN1 = 1517;
///     NOT_USED_DEADMINES = 1518;
///     STORMWIND_CITY = 1519;
///     IRONFORGE = 1537;
///     SPLITHOOF_HOLD = 1557;
///     THE_CAPE_OF_STRANGLETHORN = 1577;
///     SOUTHERN_SAVAGE_COAST = 1578;
///     UNUSED_THE_DEADMINES_002 = 1579;
///     UNUSED_IRONCLAD_COVE_003 = 1580;
///     THE_DEADMINES = 1581;
///     IRONCLAD_COVE = 1582;
///     BLACKROCK_SPIRE = 1583;
///     BLACKROCK_DEPTHS = 1584;
///     RAPTOR_GROUNDS_UNUSED = 1597;
///     GROLDOM_FARM_UNUSED = 1598;
///     MORSHAN_BASE_CAMP = 1599;
///     HONORS_STAND_UNUSED = 1600;
///     BLACKTHORN_RIDGE_UNUSED = 1601;
///     BRAMBLESCAR_UNUSED = 1602;
///     AGAMAGOR_UNUSED = 1603;
///     VALLEY_OF_HEROES = 1617;
///     ORGRIMMAR = 1637;
///     THUNDER_BLUFF = 1638;
///     ELDER_RISE = 1639;
///     SPIRIT_RISE = 1640;
///     HUNTER_RISE = 1641;
///     DARNASSUS = 1657;
///     CENARION_ENCLAVE = 1658;
///     CRAFTSMENS_TERRACE = 1659;
///     WARRIORS_TERRACE = 1660;
///     THE_TEMPLE_GARDENS = 1661;
///     TRADESMENS_TERRACE = 1662;
///     GAVINS_NAZE = 1677;
///     SOFERAS_NAZE = 1678;
///     CORRAHNS_DAGGER = 1679;
///     THE_HEADLAND = 1680;
///     MISTY_SHORE = 1681;
///     DANDREDS_FOLD = 1682;
///     GROWLESS_CAVE = 1683;
///     CHILLWIND_POINT = 1684;
///     RAPTOR_GROUNDS = 1697;
///     BRAMBLESCAR = 1698;
///     THORN_HILL = 1699;
///     AGAMAGOR = 1700;
///     BLACKTHORN_RIDGE = 1701;
///     HONORS_STAND = 1702;
///     THE_MORSHAN_RAMPART = 1703;
///     GROLDOM_FARM = 1704;
///     RAZORFEN_KRAUL1 = 1717;
///     THE_GREAT_LIFT1 = 1718;
///     MISTVALE_VALLEY = 1737;
///     NEKMANI_WELLSPRING = 1738;
///     BLOODSAIL_COMPOUND = 1739;
///     VENTURE_CO_BASE_CAMP = 1740;
///     GURUBASHI_ARENA = 1741;
///     SPIRIT_DEN = 1742;
///     THE_CRIMSON_VEIL = 1757;
///     THE_RIPTIDE = 1758;
///     THE_DAMSELS_LUCK = 1759;
///     VENTURE_CO_OPERATIONS_CENTER = 1760;
///     DEADWOOD_VILLAGE = 1761;
///     FELPAW_VILLAGE = 1762;
///     JAEDENAR = 1763;
///     BLOODVENOM_RIVER = 1764;
///     BLOODVENOM_FALLS = 1765;
///     SHATTER_SCAR_VALE = 1766;
///     IRONTREE_WOODS = 1767;
///     IRONTREE_CAVERN = 1768;
///     TIMBERMAW_HOLD1 = 1769;
///     SHADOW_HOLD = 1770;
///     SHRINE_OF_THE_DECEIVER = 1771;
///     ITHARIUSS_CAVE = 1777;
///     SORROWMURK = 1778;
///     DRAENILDUR_VILLAGE = 1779;
///     SPLINTERSPEAR_JUNCTION = 1780;
///     STAGALBOG = 1797;
///     THE_SHIFTING_MIRE = 1798;
///     STAGALBOG_CAVE = 1817;
///     WITHERBARK_CAVERNS = 1837;
///     THORADINS_WALL2 = 1857;
///     BOULDERGOR = 1858;
///     VALLEY_OF_FANGS = 1877;
///     THE_DUSTBOWL = 1878;
///     MIRAGE_FLATS = 1879;
///     FEATHERBEARDS_HOVEL = 1880;
///     SHINDIGGERS_CAMP = 1881;
///     PLAGUEMIST_RAVINE = 1882;
///     VALORWIND_LAKE = 1883;
///     AGOLWATHA = 1884;
///     HIRIWATHA = 1885;
///     THE_CREEPING_RUIN = 1886;
///     BOGENS_LEDGE = 1887;
///     THE_MAKERS_TERRACE = 1897;
///     DUSTWIND_GULCH = 1898;
///     SHAOLWATHA = 1917;
///     NOONSHADE_RUINS = 1937;
///     BROKEN_PILLAR = 1938;
///     ABYSSAL_SANDS = 1939;
///     SOUTHBREAK_SHORE = 1940;
///     CAVERNS_OF_TIME0 = 1941;
///     THE_MARSHLANDS = 1942;
///     IRONSTONE_PLATEAU = 1943;
///     BLACKCHAR_CAVE = 1957;
///     TANNER_CAMP = 1958;
///     DUSTFIRE_VALLEY = 1959;
///     ZUL_GURUB1 = 1977;
///     MISTY_REED_POST = 1978;
///     BLOODVENOM_POST = 1997;
///     TALONBRANCH_GLADE = 1998;
///     STRATHOLME0 = 2017;
///     UNUSED_SHADOWFANG_KEEP_003 = 2037;
///     SCHOLOMANCE = 2057;
///     TWILIGHT_VALE = 2077;
///     TWILIGHT_SHORE = 2078;
///     ALCAZ_ISLAND = 2079;
///     DARKCLOUD_PINNACLE = 2097;
///     DAWNING_WOOD_CATACOMBS = 2098;
///     STONEWATCH_KEEP = 2099;
///     MARAUDON = 2100;
///     STOUTLAGER_INN = 2101;
///     THUNDERBREW_DISTILLERY = 2102;
///     MENETHIL_KEEP = 2103;
///     DEEPWATER_TAVERN = 2104;
///     SHADOW_GRAVE = 2117;
///     BRILL_TOWN_HALL = 2118;
///     GALLOWS_END_TAVERN = 2119;
///     THE_POOLS_OF_VISION_UNUSED = 2137;
///     DREADMIST_DEN = 2138;
///     BAELDUN_KEEP = 2157;
///     EMBERSTRIFES_DEN = 2158;
///     ONYXIAS_LAIR = 2159;
///     WINDSHEAR_MINE = 2160;
///     ROLANDS_DOOM = 2161;
///     BATTLE_RING = 2177;
///     THE_POOLS_OF_VISION = 2197;
///     SHADOWBREAK_RAVINE = 2198;
///     BROKEN_SPEAR_VILLAGE = 2217;
///     WHITEREACH_POST = 2237;
///     GORNIA = 2238;
///     ZANES_EYE_CRATER = 2239;
///     MIRAGE_RACEWAY = 2240;
///     FROSTSABER_ROCK = 2241;
///     THE_HIDDEN_GROVE = 2242;
///     TIMBERMAW_POST = 2243;
///     WINTERFALL_VILLAGE = 2244;
///     MAZTHORIL = 2245;
///     FROSTFIRE_HOT_SPRINGS = 2246;
///     ICE_THISTLE_HILLS = 2247;
///     DUN_MANDARR = 2248;
///     FROSTWHISPER_GORGE = 2249;
///     OWL_WING_THICKET = 2250;
///     LAKE_KEL_THERIL = 2251;
///     THE_RUINS_OF_KEL_THERIL = 2252;
///     STARFALL_VILLAGE = 2253;
///     BAN_THALLOW_BARROW_DEN = 2254;
///     EVERLOOK = 2255;
///     DARKWHISPER_GORGE = 2256;
///     DEEPRUN_TRAM = 2257;
///     THE_FUNGAL_VALE = 2258;
///     UNUSED_THE_MARRIS_STEAD = 2259;
///     THE_MARRIS_STEAD = 2260;
///     THE_UNDERCROFT = 2261;
///     DARROWSHIRE = 2262;
///     CROWN_GUARD_TOWER = 2263;
///     CORINS_CROSSING = 2264;
///     SCARLET_BASE_CAMP = 2265;
///     TYRS_HAND = 2266;
///     THE_SCARLET_BASILICA = 2267;
///     LIGHTS_HOPE_CHAPEL = 2268;
///     BROWMAN_MILL = 2269;
///     THE_NOXIOUS_GLADE = 2270;
///     EASTWALL_TOWER = 2271;
///     NORTHDALE = 2272;
///     ZUL_MASHAR = 2273;
///     MAZRA_ALOR = 2274;
///     NORTHPASS_TOWER = 2275;
///     QUEL_LITHIEN_LODGE = 2276;
///     PLAGUEWOOD = 2277;
///     SCOURGEHOLD = 2278;
///     STRATHOLME1 = 2279;
///     UNUSED_STRATHOLME = 2280;
///     DARROWMERE_LAKE0 = 2297;
///     CAER_DARROW = 2298;
///     DARROWMERE_LAKE1 = 2299;
///     CAVERNS_OF_TIME1 = 2300;
///     THISTLEFUR_VILLAGE = 2301;
///     THE_QUAGMIRE = 2302;
///     WINDBREAK_CANYON = 2303;
///     SOUTH_SEAS0 = 2317;
///     THE_GREAT_SEA3 = 2318;
///     THE_GREAT_SEA4 = 2319;
///     THE_GREAT_SEA5 = 2320;
///     THE_GREAT_SEA6 = 2321;
///     THE_VEILED_SEA1 = 2322;
///     THE_VEILED_SEA2 = 2323;
///     THE_VEILED_SEA3 = 2324;
///     THE_VEILED_SEA4 = 2325;
///     THE_VEILED_SEA5 = 2326;
///     RAZOR_HILL_BARRACKS = 2337;
///     SOUTH_SEAS1 = 2338;
///     THE_GREAT_SEA7 = 2339;
///     BLOODTOOTH_CAMP = 2357;
///     FOREST_SONG = 2358;
///     GREENPAW_VILLAGE = 2359;
///     SILVERWING_OUTPOST = 2360;
///     NIGHTHAVEN = 2361;
///     SHRINE_OF_REMULOS = 2362;
///     STORMRAGE_BARROW_DENS = 2363;
///     THE_GREAT_SEA8 = 2364;
///     THE_GREAT_SEA9 = 2365;
///     THE_BLACK_MORASS = 2366;
///     OLD_HILLSBRAD_FOOTHILLS = 2367;
///     TARREN_MILL1 = 2368;
///     SOUTHSHORE1 = 2369;
///     DURNHOLDE_KEEP1 = 2370;
///     DUN_GAROK1 = 2371;
///     HILLSBRAD_FIELDS1 = 2372;
///     EASTERN_STRAND1 = 2373;
///     NETHANDER_STEAD1 = 2374;
///     DARROW_HILL1 = 2375;
///     SOUTHPOINT_TOWER1 = 2376;
///     THORADINS_WALL3 = 2377;
///     WESTERN_STRAND1 = 2378;
///     AZURELODE_MINE1 = 2379;
///     THE_GREAT_SEA10 = 2397;
///     THE_GREAT_SEA11 = 2398;
///     THE_GREAT_SEA12 = 2399;
///     THE_FORBIDDING_SEA1 = 2400;
///     THE_FORBIDDING_SEA2 = 2401;
///     THE_FORBIDDING_SEA3 = 2402;
///     THE_FORBIDDING_SEA4 = 2403;
///     TETHRIS_ARAN = 2404;
///     ETHEL_RETHOR = 2405;
///     RANAZJAR_ISLE = 2406;
///     KORMEKS_HUT = 2407;
///     SHADOWPREY_VILLAGE = 2408;
///     BLACKROCK_PASS = 2417;
///     MORGANS_VIGIL = 2418;
///     SLITHER_ROCK = 2419;
///     TERROR_WING_PATH = 2420;
///     DRACODAR = 2421;
///     RAGEFIRE_CHASM = 2437;
///     NIGHTSONG_WOODS = 2457;
///     THE_VEILED_SEA6 = 2477;
///     MORLOS_ARAN = 2478;
///     EMERALD_SANCTUARY = 2479;
///     JADEFIRE_GLEN = 2480;
///     RUINS_OF_CONSTELLAS = 2481;
///     BITTER_REACHES = 2497;
///     RISE_OF_THE_DEFILER = 2517;
///     LARISS_PAVILION = 2518;
///     WOODPAW_HILLS = 2519;
///     WOODPAW_DEN = 2520;
///     VERDANTIS_RIVER = 2521;
///     RUINS_OF_ISILDIEN = 2522;
///     GRIMTOTEM_POST = 2537;
///     CAMP_APARAJE = 2538;
///     MALAKAJIN = 2539;
///     BOULDERSLIDE_RAVINE = 2540;
///     SISHIR_CANYON = 2541;
///     DIRE_MAUL0 = 2557;
///     DEADWIND_RAVINE = 2558;
///     DIAMONDHEAD_RIVER = 2559;
///     ARIDENS_CAMP = 2560;
///     THE_VICE = 2561;
///     KARAZHAN = 2562;
///     MORGANS_PLOT = 2563;
///     DIRE_MAUL1 = 2577;
///     ALTERAC_VALLEY0 = 2597;
///     SCRABBLESCREWS_CAMP = 2617;
///     JADEFIRE_RUN = 2618;
///     THONDRORIL_RIVER0 = 2619;
///     THONDRORIL_RIVER1 = 2620;
///     LAKE_MERELDAR = 2621;
///     PESTILENT_SCAR = 2622;
///     THE_INFECTIS_SCAR = 2623;
///     BLACKWOOD_LAKE = 2624;
///     EASTWALL_GATE = 2625;
///     TERRORWEB_TUNNEL = 2626;
///     TERRORDALE = 2627;
///     KARGATHIA_KEEP = 2637;
///     VALLEY_OF_BONES = 2657;
///     BLACKWING_LAIR = 2677;
///     DEADMANS_CROSSING = 2697;
///     MOLTEN_CORE = 2717;
///     THE_SCARAB_WALL = 2737;
///     SOUTHWIND_VILLAGE = 2738;
///     TWILIGHT_BASE_CAMP = 2739;
///     THE_CRYSTAL_VALE = 2740;
///     THE_SCARAB_DAIS = 2741;
///     HIVE_ASHI = 2742;
///     HIVE_ZORA = 2743;
///     HIVE_REGAL = 2744;
///     SHRINE_OF_THE_FALLEN_WARRIOR = 2757;
///     UNUSED_ALTERAC_VALLEY = 2777;
///     BLACKFATHOM_DEEPS1 = 2797;
///     ON_MAP_DUNGEON4 = 2817;
///     THE_MASTERS_CELLAR = 2837;
///     STONEWROUGHT_PASS = 2838;
///     ALTERAC_VALLEY1 = 2839;
///     THE_RUMBLE_CAGE = 2857;
///     CHUNK_TEST = 2877;
///     ZORAMGAR_OUTPOST = 2897;
///     HALL_OF_LEGENDS = 2917;
///     CHAMPIONS_HALL = 2918;
///     GROSHGOK_COMPOUND = 2937;
///     SLEEPING_GORGE = 2938;
///     IRONDEEP_MINE = 2957;
///     STONEHEARTH_OUTPOST = 2958;
///     DUN_BALDAR = 2959;
///     ICEWING_PASS = 2960;
///     FROSTWOLF_VILLAGE = 2961;
///     TOWER_POINT = 2962;
///     COLDTOOTH_MINE = 2963;
///     WINTERAX_HOLD = 2964;
///     ICEBLOOD_GARRISON = 2977;
///     FROSTWOLF_KEEP = 2978;
///     TORKREN_FARM = 2979;
///     FROST_DAGGER_PASS = 3017;
///     IRONSTONE_CAMP = 3037;
///     WEAZELS_CRATER = 3038;
///     TAHONDA_RUINS = 3039;
///     FIELD_OF_STRIFE = 3057;
///     ICEWING_CAVERN = 3058;
///     VALORS_REST = 3077;
///     THE_SWARMING_PILLAR = 3097;
///     TWILIGHT_POST = 3098;
///     TWILIGHT_OUTPOST = 3099;
///     RAVAGED_TWILIGHT_CAMP = 3100;
///     SHALZARUS_LAIR = 3117;
///     TALRENDIS_POINT = 3137;
///     RETHRESS_SANCTUM = 3138;
///     MOON_HORROR_DEN = 3139;
///     SCALEBEARDS_CAVE = 3140;
///     BOULDERSLIDE_CAVERN = 3157;
///     WARSONG_LABOR_CAMP = 3177;
///     CHILLWIND_CAMP = 3197;
///     THE_MAUL = 3217;
///     THE_MAUL_UNUSED = 3237;
///     BONES_OF_GRAKKAROND = 3257;
///     WARSONG_GULCH = 3277;
///     FROSTWOLF_GRAVEYARD = 3297;
///     FROSTWOLF_PASS = 3298;
///     DUN_BALDAR_PASS = 3299;
///     ICEBLOOD_GRAVEYARD = 3300;
///     SNOWFALL_GRAVEYARD = 3301;
///     STONEHEARTH_GRAVEYARD = 3302;
///     STORMPIKE_GRAVEYARD = 3303;
///     ICEWING_BUNKER = 3304;
///     STONEHEARTH_BUNKER = 3305;
///     WILDPAW_RIDGE = 3306;
///     REVANTUSK_VILLAGE = 3317;
///     ROCK_OF_DUROTAN = 3318;
///     SILVERWING_GROVE = 3319;
///     WARSONG_LUMBER_MILL = 3320;
///     SILVERWING_HOLD = 3321;
///     WILDPAW_CAVERN = 3337;
///     THE_VEILED_CLEFT = 3338;
///     YOJAMBA_ISLE = 3357;
///     ARATHI_BASIN = 3358;
///     THE_COIL = 3377;
///     ALTAR_OF_HIREEK = 3378;
///     SHADRAZAAR = 3379;
///     HAKKARI_GROUNDS = 3380;
///     NAZE_OF_SHIRVALLAH = 3381;
///     TEMPLE_OF_BETHEKK = 3382;
///     THE_BLOODFIRE_PIT = 3383;
///     ALTAR_OF_THE_BLOOD_GOD = 3384;
///     ZANZAS_RISE = 3397;
///     EDGE_OF_MADNESS = 3398;
///     TROLLBANE_HALL = 3417;
///     DEFILERS_DEN = 3418;
///     PAGLES_POINTE = 3419;
///     FARM = 3420;
///     BLACKSMITH = 3421;
///     LUMBER_MILL = 3422;
///     GOLD_MINE = 3423;
///     STABLES = 3424;
///     CENARION_HOLD = 3425;
///     STAGHELM_POINT = 3426;
///     BRONZEBEARD_ENCAMPMENT = 3427;
///     AHN_QIRAJ = 3428;
///     RUINS_OF_AHN_QIRAJ0 = 3429;
///     TWILIGHTS_RUN = 3446;
///     ORTELLS_HIDEOUT = 3447;
///     SCARAB_TERRACE = 3448;
///     GENERALS_TERRACE = 3449;
///     THE_RESERVOIR = 3450;
///     THE_HATCHERY = 3451;
///     THE_COMB = 3452;
///     WATCHERS_TERRACE = 3453;
///     RUINS_OF_AHN_QIRAJ1 = 3454;
///     NAXXRAMAS = 3456;
///     CITY = 3459;
///     GATES_OF_AHN_QIRAJ = 3478;
///     RAVENHOLDT_MANOR = 3486;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Area {
    None,
    DunMorogh,
    Longshore,
    Badlands,
    BlastedLands,
    BlackwaterCove,
    SwampOfSorrows,
    NorthshireValley,
    Duskwood,
    Wetlands,
    ElwynnForest,
    TheWorldTree,
    Durotar,
    DustwallowMarsh,
    Azshara,
    TheBarrens,
    CrystalLake,
    ZulGurub0,
    Moonbrook,
    KulTiras,
    ProgrammerIsle,
    NorthshireRiver,
    NorthshireAbbey,
    BlackrockMountain0,
    Lighthouse,
    WesternPlaguelands,
    Nine,
    TheCemetary,
    StranglethornVale,
    EchoRidgeMine,
    BootyBay,
    AlteracMountains,
    LakeNazferiti,
    LochModan,
    Westfall0,
    DeadwindPass,
    Darkshire,
    WildShore,
    RedridgeMountains,
    ArathiHighlands,
    BurningSteppes,
    TheHinterlands,
    DeadMansHole,
    SearingGorge,
    ThievesCamp,
    JasperlodeMine,
    ValleyOfHeroesUnused,
    HeroesVigil,
    FargodeepMine,
    NorthshireVineyards,
    ForestsEdge,
    ThunderFalls,
    BrackwellPumpkinPatch,
    TheStonefieldFarm,
    TheMaclureVineyards,
    OnMapDungeon0,
    OnMapDungeon1,
    OnMapDungeon2,
    LakeEverstill,
    Lakeshire,
    Stonewatch,
    StonewatchFalls,
    TheDarkPortal,
    TheTaintedScar,
    PoolOfTears,
    Stonard,
    FallowSanctuary,
    Anvilmar,
    StormwindMountains,
    JeffNeQuadrantChanged,
    JeffNwQuadrant,
    JeffSeQuadrant,
    JeffSwQuadrant,
    TirisfalGlades,
    StoneCairnLake,
    Goldshire,
    EastvaleLoggingCamp,
    MirrorLakeOrchard,
    TowerOfAzora,
    MirrorLake,
    VulGolOgreMound,
    RavenHill,
    RedridgeCanyons,
    TowerOfIlgalar,
    AlthersMill,
    RethbanCaverns,
    RebelCamp,
    NesingwarysExpedition,
    KurzensCompound,
    RuinsOfZulKunda,
    RuinsOfZulMamwe,
    TheVileReef,
    MoshOggOgreMound,
    TheStockpile,
    SaldeansFarm,
    SentinelHill,
    FurlbrowsPumpkinFarm,
    JangolodeMine,
    GoldCoastQuarry,
    WestfallLighthouse,
    MistyValley,
    GromgolBaseCamp,
    WhelgarsExcavationSite,
    WestbrookGarrison,
    TranquilGardensCemetery,
    ZuuldaiaRuins,
    BallalRuins,
    KalaiRuins,
    TkashiRuins,
    BaliamahRuins,
    ZiatajaiRuins,
    MizjahRuins,
    SilverpineForest,
    Kharanos,
    ColdridgeValley,
    Gnomeregan0,
    GolBolarQuarry,
    FrostmaneHold,
    TheGrizzledDen,
    BrewnallVillage,
    MistyPineRefuge,
    EasternPlaguelands,
    Teldrassil,
    IronbandsExcavationSite,
    MogroshStronghold,
    Thelsamar,
    AlgazGate,
    StonewroughtDam,
    TheFarstriderLodge,
    Darkshore,
    SilverStreamMine,
    MenethilHarbor,
    DesignerIsland,
    TheBulwark0,
    RuinsOfLordaeron,
    Deathknell,
    NightWebsHollow,
    SollidenFarmstead,
    AgamandMills,
    AgamandFamilyCrypt,
    Brill,
    WhisperingGardens,
    TerraceOfRepose,
    BrightwaterLake,
    GunthersRetreat,
    GarrensHaunt,
    BalnirFarmstead,
    ColdHearthManor,
    CrusaderOutpost,
    TheNorthCoast,
    WhisperingShore,
    LordamereLake0,
    FenrisIsle,
    FaolsRest,
    Dolanaar,
    DarnassusUnused,
    Shadowglen,
    SteelgrillsDepot,
    Hearthglen,
    NorthridgeLumberCamp,
    RuinsOfAndorhal,
    SchoolOfNecromancy,
    UthersTomb,
    SorrowHill,
    TheWeepingCave,
    FelstoneField,
    DalsonsTears,
    GahrronsWithering,
    TheWrithingHaunt,
    MardenholdeKeep,
    PyrewoodVillage,
    DunModr,
    Westfall1,
    TheGreatSea0,
    UnusedIroncladcove,
    ShadowfangKeep0,
    OnMapDungeon3,
    IceflowLake,
    HelmsBedLake,
    DeepElemMine,
    TheGreatSea1,
    Mulgore,
    AlexstonFarmstead,
    RedCloudMesa,
    CampNarache,
    BloodhoofVillage,
    StonebullLake,
    RavagedCaravan,
    RedRocks,
    TheSkitteringDark,
    ValgansField,
    TheSepulcher,
    OlsensFarthing,
    TheGreymaneWall,
    BerensPeril,
    TheDawningIsles,
    Ambermill,
    FenrisKeep,
    ShadowfangKeep1,
    TheDecrepitFerry,
    MaldensOrchard,
    TheIvarPatch,
    TheDeadField,
    TheRottingOrchard,
    BrightwoodGrove,
    ForlornRowe,
    TheWhippleEstate,
    TheYorgenFarmstead,
    TheCauldron,
    GrimesiltDigSite,
    DreadmaulRock,
    RuinsOfThaurissan,
    FlameCrest,
    BlackrockStronghold,
    ThePillarOfAsh,
    BlackrockMountain1,
    AltarOfStorms0,
    Aldrassil,
    ShadowthreadCave,
    FelRock,
    LakeAlAmeth,
    StarbreezeVillage,
    GnarlpineHold,
    BanethilBarrowDen,
    TheCleft,
    TheOracleGlade,
    WellspringRiver,
    WellspringLake,
    HillsbradFoothills,
    AzsharaCrater,
    DunAlgaz0,
    Southshore0,
    TarrenMill0,
    DurnholdeKeep0,
    UnusedStonewroughtPass,
    TheFoothillCaverns,
    LordamereInternmentCamp,
    Dalaran,
    Strahnbrad,
    RuinsOfAlterac,
    CrushridgeHold,
    SlaughterHollow,
    TheUplands,
    SouthpointTower0,
    HillsbradFields0,
    Hillsbrad,
    AzurelodeMine0,
    NethanderStead0,
    DunGarok0,
    ThoradinsWall0,
    EasternStrand0,
    WesternStrand0,
    SouthSeasUnused,
    JagueroIsle,
    BaradinBay,
    MenethilBay,
    MistyReedStrand,
    TheSavageCoast,
    TheCrystalShore,
    ShellBeach,
    NorthTidesRun,
    SouthTidesRun,
    TheOverlookCliffs,
    TheForbiddingSea0,
    IronbeardsTomb,
    CrystalveinMine,
    RuinsOfAboraz,
    JaneirosPoint,
    NorthfoldManor,
    GoShekFarm,
    DabyriesFarmstead,
    BoulderfistHall,
    WitherbarkVillage,
    DrywhiskerGorge,
    RefugePointe,
    Hammerfall,
    BlackwaterShipwrecks,
    OBreensCamp,
    StromgardeKeep,
    TheTowerOfArathor,
    TheSanctum,
    FaldirsCove,
    TheDrownedReef,
    ThandolSpan0,
    Ashenvale,
    TheGreatSea2,
    CircleOfEastBinding,
    CircleOfWestBinding,
    CircleOfInnerBinding,
    CircleOfOuterBinding,
    ApocryphansRest,
    AngorFortress,
    LethlorRavine,
    Kargath,
    CampKosh,
    CampBoff,
    CampWurg,
    CampCagg,
    AgmondsEnd,
    HammertoesDigsite,
    DustbelchGrotto,
    AeriePeak,
    WildhammerKeep,
    QuelDanilLodge,
    SkulkRock,
    Zunwatha,
    ShadraAlor,
    JinthaAlor,
    TheAltarOfZul,
    Seradane,
    Feralas,
    BramblebladeRavine,
    BaelModan,
    TheVentureCoMine,
    Felwood,
    RazorHill,
    ValleyOfTrials,
    TheDen,
    BurningBladeCoven,
    KolkarCrag,
    SenjinVillage,
    EchoIsles,
    ThunderRidge,
    DrygulchRavine,
    DustwindCave,
    TiragardeKeep,
    ScuttleCoast,
    BladefistBay,
    DeadeyeShore,
    SouthfuryRiver0,
    CampTaurajo,
    FarWatchPost,
    TheCrossroads,
    BoulderLodeMine,
    TheSludgeFen,
    TheDryHills,
    DreadmistPeak,
    NorthwatchHold,
    TheForgottenPools,
    LushwaterOasis,
    TheStagnantOasis,
    FieldOfGiants,
    TheMerchantCoast,
    Ratchet,
    DarkspearStrand,
    DarrowmereLakeUnused,
    CaerDarrowUnused,
    WinterhoofWaterWell,
    ThunderhornWaterWell,
    WildmaneWaterWell,
    SkylineRidge,
    ThousandNeedles,
    TheTidusStair,
    ShadyRestInn,
    BaeldunDigsite,
    Desolace,
    StonetalonMountains,
    OrgrimmarUnused,
    GillijimsIsle,
    IslandOfDoctorLapidis,
    RazorwindCanyon,
    BathransHaunt,
    TheRuinsOfOrdilAran,
    MaestrasPost,
    TheZoramStrand,
    Astranaar,
    TheShrineOfAessina,
    FireScarShrine,
    TheRuinsOfStardust,
    TheHowlingVale,
    SilverwindRefuge,
    MystralLake,
    FallenSkyLake,
    IrisLake,
    Moonwell,
    RaynewoodRetreat,
    TheShadyNook,
    NightRun,
    Xavian,
    Satyrnaar,
    SplintertreePost,
    TheDorDanilBarrowDen,
    FalfarrenRiver,
    FelfireHill,
    DemonFallCanyon,
    DemonFallRidge,
    WarsongLumberCamp,
    BoughShadow,
    TheShimmeringFlats,
    Tanaris,
    LakeFalathim,
    Auberdine,
    RuinsOfMathystra,
    TowerOfAlthalaxx,
    CliffspringFalls,
    BashalAran,
    AmethAran,
    GroveOfTheAncients,
    TheMastersGlaive,
    RemtravelsExcavation,
    MistsEdge,
    TheLongWash,
    WildbendRiver,
    BlackwoodDen,
    CliffspringRiver,
    TheVeiledSea0,
    GoldRoad,
    ScarletWatchPost,
    SunRockRetreat,
    WindshearCrag,
    CragpoolLake,
    MirkfallonLake,
    TheCharredVale,
    ValleyOfTheBloodfuries,
    StonetalonPeak,
    TheTalonDen,
    GreatwoodVale,
    ThunderBluffUnused,
    BraveWindMesa,
    FireStoneMesa,
    MantleRock,
    HunterRiseUnused,
    SpiritRiseUnused,
    ElderRiseUnused,
    RuinsOfJubuwal,
    PoolsOfArlithrien,
    TheRustmaulDigSite,
    CampEthok,
    SplithoofCrag,
    Highperch,
    TheScreechingCanyon,
    FreewindPost,
    TheGreatLift0,
    GalakHold,
    RoguefeatherDen,
    TheWeatheredNook,
    Thalanaar,
    UnGoroCrater,
    RazorfenKraul0,
    RavenHillCemetery,
    Moonglade,
    DeleteMe0,
    BrackenwallVillage,
    SwamplightManor,
    BloodfenBurrow,
    DarkmistCavern,
    MogglePoint,
    BeezilsWreck,
    WitchHill,
    SentryPoint,
    NorthPointTower,
    WestPointTower,
    LostPoint,
    Bluefen,
    StonemaulRuins,
    TheDenOfFlame,
    TheDragonmurk,
    Wyrmbog,
    OnyxiasLairUnused,
    TheramoreIsle,
    FootholdCitadel,
    IroncladPrison,
    DustwallowBay,
    TidefuryCove,
    DreadmurkShore,
    AddlesStead,
    FirePlumeRidge,
    LakkariTarPits,
    TerrorRun,
    TheSlitheringScar,
    MarshalsRefuge,
    FungalRock,
    GolakkaHotSprings,
    TheLoch,
    BeggarsHaunt,
    KodoGraveyard,
    GhostWalkerPost,
    SartherisStrand,
    ThunderAxeFortress,
    BolgansHole,
    MannorocCoven,
    Sargeron,
    MagramVillage,
    GelkisVillage,
    ValleyOfSpears,
    NijelsPoint,
    KolkarVillage,
    Hyjal,
    Winterspring,
    BlackwolfRiver,
    KodoRock,
    HiddenPath,
    SpiritRock,
    ShrineOfTheDormantFlame,
    LakeEluneara,
    TheHarborage,
    Outland,
    CraftsmensTerraceUnused,
    TradesmensTerraceUnused,
    TheTempleGardensUnused,
    TempleOfEluneUnused,
    CenarionEnclaveUnused,
    WarriorsTerraceUnused,
    RuttheranVillage,
    IronbandsCompound,
    TheStockade,
    WailingCaverns,
    BlackfathomDeeps0,
    FrayIsland,
    Gnomeregan1,
    RazorfenDowns0,
    BanethilHollow,
    ScarletMonastery,
    JerodsLanding,
    RidgepointTower,
    TheDarkenedBank,
    ColdridgePass,
    ChillBreezeValley,
    ShimmerRidge,
    AmberstillRanch,
    TheTundridHills,
    SouthGatePass0,
    SouthGateOutpost,
    NorthGatePass0,
    NorthGateOutpost,
    GatesOfIronforge,
    StillwaterPond,
    NightmareVale,
    VenomwebVale,
    TheBulwark1,
    SouthfuryRiver1,
    SouthfuryRiver2,
    RazormaneGrounds,
    SkullRock,
    PalemaneRock,
    WindfuryRidge,
    TheGoldenPlains,
    TheRollingPlains,
    DunAlgaz1,
    DunAlgaz2,
    NorthGatePass1,
    SouthGatePass1,
    TwilightGrove,
    GmIsland,
    DeleteMe1,
    SouthfuryRiver3,
    SouthfuryRiver4,
    ThandolSpan1,
    ThandolSpan2,
    PurgationIsle,
    TheJansenStead,
    TheDeadAcre,
    TheMolsenFarm,
    StendelsPond,
    TheDaggerHills,
    DemontsPlace,
    TheDustPlains,
    StonesplinterValley,
    ValleyOfKings,
    AlgazStation,
    BucklebreeFarm,
    TheShiningStrand,
    NorthTidesHollow,
    GrizzlepawRidge,
    TheVerdantFields,
    Gadgetzan,
    SteamwheedlePort,
    ZulFarrak0,
    SandsorrowWatch,
    ThistleshrubValley,
    TheGapingChasm,
    TheNoxiousLair,
    DunemaulCompound,
    EastmoonRuins,
    WaterspringField,
    ZalashjisDen,
    LandsEndBeach,
    WavestriderBeach,
    Uldum,
    ValleyOfTheWatchers,
    GunstansPost,
    SouthmoonRuins,
    RendersCamp,
    RendersValley,
    RendersRock,
    StonewatchTower,
    GalardellValley,
    LakeridgeHighway,
    ThreeCorners,
    DireforgeHill,
    RaptorRidge,
    BlackChannelMarsh,
    TheGreenBelt0,
    MosshideFen,
    ThelgenRock,
    BluegillMarsh,
    SaltsprayGlen,
    SundownMarsh,
    TheGreenBelt1,
    AngerfangEncampment,
    GrimBatol,
    DragonmawGates,
    TheLostFleet,
    DarrowHill0,
    ThoradinsWall1,
    WebwinderPath,
    TheHushedBank,
    ManorMistmantle,
    CampMojache,
    GrimtotemCompound,
    TheWrithingDeep,
    WildwindLake,
    GordunniOutpost,
    MokGordun,
    FeralScarVale,
    FrayfeatherHighlands,
    IdlewindLake,
    TheForgottenCoast,
    EastPillar,
    WestPillar,
    DreamBough,
    JademirLake,
    Oneiros,
    RuinsOfRavenwind,
    RageScarHold,
    FeathermoonStronghold,
    RuinsOfSolarsal,
    LowerWildsUnused,
    TheTwinColossals,
    SardorIsle,
    IsleOfDread,
    HighWilderness,
    LowerWilds,
    SouthernBarrens,
    SouthernGoldRoad,
    ZulFarrak1,
    UnusedAlcazIsland,
    TimbermawHold0,
    VanndirEncampment,
    TestAzshara,
    LegashEncampment,
    ThalassianBaseCamp,
    RuinsOfEldarath,
    HetaerasClutch,
    TempleOfZinMalor,
    BearsHead,
    Ursolan,
    TempleOfArkkoran,
    BayOfStorms,
    TheShatteredStrand,
    TowerOfEldara,
    JaggedReef,
    SouthridgeBeach,
    RavencrestMonument,
    ForlornRidge,
    LakeMennar,
    ShadowsongShrine,
    HaldarrEncampment,
    Valormok,
    TheRuinedReaches,
    TheTalondeepPath0,
    TheTalondeepPath1,
    RocktuskFarm,
    JaggedswineFarm,
    RazorfenDowns1,
    LostRiggerCove,
    Uldaman0,
    LordamereLake1,
    LordamereLake2,
    GallowsCorner,
    Silithus,
    EmeraldForest,
    SunkenTemple,
    DreadmaulHold,
    NethergardeKeep,
    DreadmaulPost,
    SerpentsCoil,
    AltarOfStorms1,
    FirewatchRidge,
    TheSlagPit,
    TheSeaOfCinders,
    BlackrockMountain2,
    ThoriumPoint,
    GarrisonArmory,
    TheTempleOfAtalHakkar,
    Undercity,
    Uldaman1,
    NotUsedDeadmines,
    StormwindCity,
    Ironforge,
    SplithoofHold,
    TheCapeOfStranglethorn,
    SouthernSavageCoast,
    UnusedTheDeadmines002,
    UnusedIroncladCove003,
    TheDeadmines,
    IroncladCove,
    BlackrockSpire,
    BlackrockDepths,
    RaptorGroundsUnused,
    GroldomFarmUnused,
    MorshanBaseCamp,
    HonorsStandUnused,
    BlackthornRidgeUnused,
    BramblescarUnused,
    AgamagorUnused,
    ValleyOfHeroes,
    Orgrimmar,
    ThunderBluff,
    ElderRise,
    SpiritRise,
    HunterRise,
    Darnassus,
    CenarionEnclave,
    CraftsmensTerrace,
    WarriorsTerrace,
    TheTempleGardens,
    TradesmensTerrace,
    GavinsNaze,
    SoferasNaze,
    CorrahnsDagger,
    TheHeadland,
    MistyShore,
    DandredsFold,
    GrowlessCave,
    ChillwindPoint,
    RaptorGrounds,
    Bramblescar,
    ThornHill,
    Agamagor,
    BlackthornRidge,
    HonorsStand,
    TheMorshanRampart,
    GroldomFarm,
    RazorfenKraul1,
    TheGreatLift1,
    MistvaleValley,
    NekmaniWellspring,
    BloodsailCompound,
    VentureCoBaseCamp,
    GurubashiArena,
    SpiritDen,
    TheCrimsonVeil,
    TheRiptide,
    TheDamselsLuck,
    VentureCoOperationsCenter,
    DeadwoodVillage,
    FelpawVillage,
    Jaedenar,
    BloodvenomRiver,
    BloodvenomFalls,
    ShatterScarVale,
    IrontreeWoods,
    IrontreeCavern,
    TimbermawHold1,
    ShadowHold,
    ShrineOfTheDeceiver,
    IthariussCave,
    Sorrowmurk,
    DraenildurVillage,
    SplinterspearJunction,
    Stagalbog,
    TheShiftingMire,
    StagalbogCave,
    WitherbarkCaverns,
    ThoradinsWall2,
    Bouldergor,
    ValleyOfFangs,
    TheDustbowl,
    MirageFlats,
    FeatherbeardsHovel,
    ShindiggersCamp,
    PlaguemistRavine,
    ValorwindLake,
    Agolwatha,
    Hiriwatha,
    TheCreepingRuin,
    BogensLedge,
    TheMakersTerrace,
    DustwindGulch,
    Shaolwatha,
    NoonshadeRuins,
    BrokenPillar,
    AbyssalSands,
    SouthbreakShore,
    CavernsOfTime0,
    TheMarshlands,
    IronstonePlateau,
    BlackcharCave,
    TannerCamp,
    DustfireValley,
    ZulGurub1,
    MistyReedPost,
    BloodvenomPost,
    TalonbranchGlade,
    Stratholme0,
    UnusedShadowfangKeep003,
    Scholomance,
    TwilightVale,
    TwilightShore,
    AlcazIsland,
    DarkcloudPinnacle,
    DawningWoodCatacombs,
    StonewatchKeep,
    Maraudon,
    StoutlagerInn,
    ThunderbrewDistillery,
    MenethilKeep,
    DeepwaterTavern,
    ShadowGrave,
    BrillTownHall,
    GallowsEndTavern,
    ThePoolsOfVisionUnused,
    DreadmistDen,
    BaeldunKeep,
    EmberstrifesDen,
    OnyxiasLair,
    WindshearMine,
    RolandsDoom,
    BattleRing,
    ThePoolsOfVision,
    ShadowbreakRavine,
    BrokenSpearVillage,
    WhitereachPost,
    Gornia,
    ZanesEyeCrater,
    MirageRaceway,
    FrostsaberRock,
    TheHiddenGrove,
    TimbermawPost,
    WinterfallVillage,
    Mazthoril,
    FrostfireHotSprings,
    IceThistleHills,
    DunMandarr,
    FrostwhisperGorge,
    OwlWingThicket,
    LakeKelTheril,
    TheRuinsOfKelTheril,
    StarfallVillage,
    BanThallowBarrowDen,
    Everlook,
    DarkwhisperGorge,
    DeeprunTram,
    TheFungalVale,
    UnusedTheMarrisStead,
    TheMarrisStead,
    TheUndercroft,
    Darrowshire,
    CrownGuardTower,
    CorinsCrossing,
    ScarletBaseCamp,
    TyrsHand,
    TheScarletBasilica,
    LightsHopeChapel,
    BrowmanMill,
    TheNoxiousGlade,
    EastwallTower,
    Northdale,
    ZulMashar,
    MazraAlor,
    NorthpassTower,
    QuelLithienLodge,
    Plaguewood,
    Scourgehold,
    Stratholme1,
    UnusedStratholme,
    DarrowmereLake0,
    CaerDarrow,
    DarrowmereLake1,
    CavernsOfTime1,
    ThistlefurVillage,
    TheQuagmire,
    WindbreakCanyon,
    SouthSeas0,
    TheGreatSea3,
    TheGreatSea4,
    TheGreatSea5,
    TheGreatSea6,
    TheVeiledSea1,
    TheVeiledSea2,
    TheVeiledSea3,
    TheVeiledSea4,
    TheVeiledSea5,
    RazorHillBarracks,
    SouthSeas1,
    TheGreatSea7,
    BloodtoothCamp,
    ForestSong,
    GreenpawVillage,
    SilverwingOutpost,
    Nighthaven,
    ShrineOfRemulos,
    StormrageBarrowDens,
    TheGreatSea8,
    TheGreatSea9,
    TheBlackMorass,
    OldHillsbradFoothills,
    TarrenMill1,
    Southshore1,
    DurnholdeKeep1,
    DunGarok1,
    HillsbradFields1,
    EasternStrand1,
    NethanderStead1,
    DarrowHill1,
    SouthpointTower1,
    ThoradinsWall3,
    WesternStrand1,
    AzurelodeMine1,
    TheGreatSea10,
    TheGreatSea11,
    TheGreatSea12,
    TheForbiddingSea1,
    TheForbiddingSea2,
    TheForbiddingSea3,
    TheForbiddingSea4,
    TethrisAran,
    EthelRethor,
    RanazjarIsle,
    KormeksHut,
    ShadowpreyVillage,
    BlackrockPass,
    MorgansVigil,
    SlitherRock,
    TerrorWingPath,
    Dracodar,
    RagefireChasm,
    NightsongWoods,
    TheVeiledSea6,
    MorlosAran,
    EmeraldSanctuary,
    JadefireGlen,
    RuinsOfConstellas,
    BitterReaches,
    RiseOfTheDefiler,
    LarissPavilion,
    WoodpawHills,
    WoodpawDen,
    VerdantisRiver,
    RuinsOfIsildien,
    GrimtotemPost,
    CampAparaje,
    Malakajin,
    BoulderslideRavine,
    SishirCanyon,
    DireMaul0,
    DeadwindRavine,
    DiamondheadRiver,
    AridensCamp,
    TheVice,
    Karazhan,
    MorgansPlot,
    DireMaul1,
    AlteracValley0,
    ScrabblescrewsCamp,
    JadefireRun,
    ThondrorilRiver0,
    ThondrorilRiver1,
    LakeMereldar,
    PestilentScar,
    TheInfectisScar,
    BlackwoodLake,
    EastwallGate,
    TerrorwebTunnel,
    Terrordale,
    KargathiaKeep,
    ValleyOfBones,
    BlackwingLair,
    DeadmansCrossing,
    MoltenCore,
    TheScarabWall,
    SouthwindVillage,
    TwilightBaseCamp,
    TheCrystalVale,
    TheScarabDais,
    HiveAshi,
    HiveZora,
    HiveRegal,
    ShrineOfTheFallenWarrior,
    UnusedAlteracValley,
    BlackfathomDeeps1,
    OnMapDungeon4,
    TheMastersCellar,
    StonewroughtPass,
    AlteracValley1,
    TheRumbleCage,
    ChunkTest,
    ZoramgarOutpost,
    HallOfLegends,
    ChampionsHall,
    GroshgokCompound,
    SleepingGorge,
    IrondeepMine,
    StonehearthOutpost,
    DunBaldar,
    IcewingPass,
    FrostwolfVillage,
    TowerPoint,
    ColdtoothMine,
    WinteraxHold,
    IcebloodGarrison,
    FrostwolfKeep,
    TorkrenFarm,
    FrostDaggerPass,
    IronstoneCamp,
    WeazelsCrater,
    TahondaRuins,
    FieldOfStrife,
    IcewingCavern,
    ValorsRest,
    TheSwarmingPillar,
    TwilightPost,
    TwilightOutpost,
    RavagedTwilightCamp,
    ShalzarusLair,
    TalrendisPoint,
    RethressSanctum,
    MoonHorrorDen,
    ScalebeardsCave,
    BoulderslideCavern,
    WarsongLaborCamp,
    ChillwindCamp,
    TheMaul,
    TheMaulUnused,
    BonesOfGrakkarond,
    WarsongGulch,
    FrostwolfGraveyard,
    FrostwolfPass,
    DunBaldarPass,
    IcebloodGraveyard,
    SnowfallGraveyard,
    StonehearthGraveyard,
    StormpikeGraveyard,
    IcewingBunker,
    StonehearthBunker,
    WildpawRidge,
    RevantuskVillage,
    RockOfDurotan,
    SilverwingGrove,
    WarsongLumberMill,
    SilverwingHold,
    WildpawCavern,
    TheVeiledCleft,
    YojambaIsle,
    ArathiBasin,
    TheCoil,
    AltarOfHireek,
    Shadrazaar,
    HakkariGrounds,
    NazeOfShirvallah,
    TempleOfBethekk,
    TheBloodfirePit,
    AltarOfTheBloodGod,
    ZanzasRise,
    EdgeOfMadness,
    TrollbaneHall,
    DefilersDen,
    PaglesPointe,
    Farm,
    Blacksmith,
    LumberMill,
    GoldMine,
    Stables,
    CenarionHold,
    StaghelmPoint,
    BronzebeardEncampment,
    AhnQiraj,
    RuinsOfAhnQiraj0,
    TwilightsRun,
    OrtellsHideout,
    ScarabTerrace,
    GeneralsTerrace,
    TheReservoir,
    TheHatchery,
    TheComb,
    WatchersTerrace,
    RuinsOfAhnQiraj1,
    Naxxramas,
    City,
    GatesOfAhnQiraj,
    RavenholdtManor,
}

impl Area {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0x0,
            Self::DunMorogh => 0x1,
            Self::Longshore => 0x2,
            Self::Badlands => 0x3,
            Self::BlastedLands => 0x4,
            Self::BlackwaterCove => 0x7,
            Self::SwampOfSorrows => 0x8,
            Self::NorthshireValley => 0x9,
            Self::Duskwood => 0xa,
            Self::Wetlands => 0xb,
            Self::ElwynnForest => 0xc,
            Self::TheWorldTree => 0xd,
            Self::Durotar => 0xe,
            Self::DustwallowMarsh => 0xf,
            Self::Azshara => 0x10,
            Self::TheBarrens => 0x11,
            Self::CrystalLake => 0x12,
            Self::ZulGurub0 => 0x13,
            Self::Moonbrook => 0x14,
            Self::KulTiras => 0x15,
            Self::ProgrammerIsle => 0x16,
            Self::NorthshireRiver => 0x17,
            Self::NorthshireAbbey => 0x18,
            Self::BlackrockMountain0 => 0x19,
            Self::Lighthouse => 0x1a,
            Self::WesternPlaguelands => 0x1c,
            Self::Nine => 0x1e,
            Self::TheCemetary => 0x20,
            Self::StranglethornVale => 0x21,
            Self::EchoRidgeMine => 0x22,
            Self::BootyBay => 0x23,
            Self::AlteracMountains => 0x24,
            Self::LakeNazferiti => 0x25,
            Self::LochModan => 0x26,
            Self::Westfall0 => 0x28,
            Self::DeadwindPass => 0x29,
            Self::Darkshire => 0x2a,
            Self::WildShore => 0x2b,
            Self::RedridgeMountains => 0x2c,
            Self::ArathiHighlands => 0x2d,
            Self::BurningSteppes => 0x2e,
            Self::TheHinterlands => 0x2f,
            Self::DeadMansHole => 0x31,
            Self::SearingGorge => 0x33,
            Self::ThievesCamp => 0x35,
            Self::JasperlodeMine => 0x36,
            Self::ValleyOfHeroesUnused => 0x37,
            Self::HeroesVigil => 0x38,
            Self::FargodeepMine => 0x39,
            Self::NorthshireVineyards => 0x3b,
            Self::ForestsEdge => 0x3c,
            Self::ThunderFalls => 0x3d,
            Self::BrackwellPumpkinPatch => 0x3e,
            Self::TheStonefieldFarm => 0x3f,
            Self::TheMaclureVineyards => 0x40,
            Self::OnMapDungeon0 => 0x41,
            Self::OnMapDungeon1 => 0x42,
            Self::OnMapDungeon2 => 0x43,
            Self::LakeEverstill => 0x44,
            Self::Lakeshire => 0x45,
            Self::Stonewatch => 0x46,
            Self::StonewatchFalls => 0x47,
            Self::TheDarkPortal => 0x48,
            Self::TheTaintedScar => 0x49,
            Self::PoolOfTears => 0x4a,
            Self::Stonard => 0x4b,
            Self::FallowSanctuary => 0x4c,
            Self::Anvilmar => 0x4d,
            Self::StormwindMountains => 0x50,
            Self::JeffNeQuadrantChanged => 0x51,
            Self::JeffNwQuadrant => 0x52,
            Self::JeffSeQuadrant => 0x53,
            Self::JeffSwQuadrant => 0x54,
            Self::TirisfalGlades => 0x55,
            Self::StoneCairnLake => 0x56,
            Self::Goldshire => 0x57,
            Self::EastvaleLoggingCamp => 0x58,
            Self::MirrorLakeOrchard => 0x59,
            Self::TowerOfAzora => 0x5b,
            Self::MirrorLake => 0x5c,
            Self::VulGolOgreMound => 0x5d,
            Self::RavenHill => 0x5e,
            Self::RedridgeCanyons => 0x5f,
            Self::TowerOfIlgalar => 0x60,
            Self::AlthersMill => 0x61,
            Self::RethbanCaverns => 0x62,
            Self::RebelCamp => 0x63,
            Self::NesingwarysExpedition => 0x64,
            Self::KurzensCompound => 0x65,
            Self::RuinsOfZulKunda => 0x66,
            Self::RuinsOfZulMamwe => 0x67,
            Self::TheVileReef => 0x68,
            Self::MoshOggOgreMound => 0x69,
            Self::TheStockpile => 0x6a,
            Self::SaldeansFarm => 0x6b,
            Self::SentinelHill => 0x6c,
            Self::FurlbrowsPumpkinFarm => 0x6d,
            Self::JangolodeMine => 0x6f,
            Self::GoldCoastQuarry => 0x71,
            Self::WestfallLighthouse => 0x73,
            Self::MistyValley => 0x74,
            Self::GromgolBaseCamp => 0x75,
            Self::WhelgarsExcavationSite => 0x76,
            Self::WestbrookGarrison => 0x78,
            Self::TranquilGardensCemetery => 0x79,
            Self::ZuuldaiaRuins => 0x7a,
            Self::BallalRuins => 0x7b,
            Self::KalaiRuins => 0x7d,
            Self::TkashiRuins => 0x7e,
            Self::BaliamahRuins => 0x7f,
            Self::ZiatajaiRuins => 0x80,
            Self::MizjahRuins => 0x81,
            Self::SilverpineForest => 0x82,
            Self::Kharanos => 0x83,
            Self::ColdridgeValley => 0x84,
            Self::Gnomeregan0 => 0x85,
            Self::GolBolarQuarry => 0x86,
            Self::FrostmaneHold => 0x87,
            Self::TheGrizzledDen => 0x88,
            Self::BrewnallVillage => 0x89,
            Self::MistyPineRefuge => 0x8a,
            Self::EasternPlaguelands => 0x8b,
            Self::Teldrassil => 0x8d,
            Self::IronbandsExcavationSite => 0x8e,
            Self::MogroshStronghold => 0x8f,
            Self::Thelsamar => 0x90,
            Self::AlgazGate => 0x91,
            Self::StonewroughtDam => 0x92,
            Self::TheFarstriderLodge => 0x93,
            Self::Darkshore => 0x94,
            Self::SilverStreamMine => 0x95,
            Self::MenethilHarbor => 0x96,
            Self::DesignerIsland => 0x97,
            Self::TheBulwark0 => 0x98,
            Self::RuinsOfLordaeron => 0x99,
            Self::Deathknell => 0x9a,
            Self::NightWebsHollow => 0x9b,
            Self::SollidenFarmstead => 0x9c,
            Self::AgamandMills => 0x9d,
            Self::AgamandFamilyCrypt => 0x9e,
            Self::Brill => 0x9f,
            Self::WhisperingGardens => 0xa0,
            Self::TerraceOfRepose => 0xa1,
            Self::BrightwaterLake => 0xa2,
            Self::GunthersRetreat => 0xa3,
            Self::GarrensHaunt => 0xa4,
            Self::BalnirFarmstead => 0xa5,
            Self::ColdHearthManor => 0xa6,
            Self::CrusaderOutpost => 0xa7,
            Self::TheNorthCoast => 0xa8,
            Self::WhisperingShore => 0xa9,
            Self::LordamereLake0 => 0xaa,
            Self::FenrisIsle => 0xac,
            Self::FaolsRest => 0xad,
            Self::Dolanaar => 0xba,
            Self::DarnassusUnused => 0xbb,
            Self::Shadowglen => 0xbc,
            Self::SteelgrillsDepot => 0xbd,
            Self::Hearthglen => 0xbe,
            Self::NorthridgeLumberCamp => 0xc0,
            Self::RuinsOfAndorhal => 0xc1,
            Self::SchoolOfNecromancy => 0xc3,
            Self::UthersTomb => 0xc4,
            Self::SorrowHill => 0xc5,
            Self::TheWeepingCave => 0xc6,
            Self::FelstoneField => 0xc7,
            Self::DalsonsTears => 0xc8,
            Self::GahrronsWithering => 0xc9,
            Self::TheWrithingHaunt => 0xca,
            Self::MardenholdeKeep => 0xcb,
            Self::PyrewoodVillage => 0xcc,
            Self::DunModr => 0xcd,
            Self::Westfall1 => 0xce,
            Self::TheGreatSea0 => 0xcf,
            Self::UnusedIroncladcove => 0xd0,
            Self::ShadowfangKeep0 => 0xd1,
            Self::OnMapDungeon3 => 0xd2,
            Self::IceflowLake => 0xd3,
            Self::HelmsBedLake => 0xd4,
            Self::DeepElemMine => 0xd5,
            Self::TheGreatSea1 => 0xd6,
            Self::Mulgore => 0xd7,
            Self::AlexstonFarmstead => 0xdb,
            Self::RedCloudMesa => 0xdc,
            Self::CampNarache => 0xdd,
            Self::BloodhoofVillage => 0xde,
            Self::StonebullLake => 0xdf,
            Self::RavagedCaravan => 0xe0,
            Self::RedRocks => 0xe1,
            Self::TheSkitteringDark => 0xe2,
            Self::ValgansField => 0xe3,
            Self::TheSepulcher => 0xe4,
            Self::OlsensFarthing => 0xe5,
            Self::TheGreymaneWall => 0xe6,
            Self::BerensPeril => 0xe7,
            Self::TheDawningIsles => 0xe8,
            Self::Ambermill => 0xe9,
            Self::FenrisKeep => 0xeb,
            Self::ShadowfangKeep1 => 0xec,
            Self::TheDecrepitFerry => 0xed,
            Self::MaldensOrchard => 0xee,
            Self::TheIvarPatch => 0xef,
            Self::TheDeadField => 0xf0,
            Self::TheRottingOrchard => 0xf1,
            Self::BrightwoodGrove => 0xf2,
            Self::ForlornRowe => 0xf3,
            Self::TheWhippleEstate => 0xf4,
            Self::TheYorgenFarmstead => 0xf5,
            Self::TheCauldron => 0xf6,
            Self::GrimesiltDigSite => 0xf7,
            Self::DreadmaulRock => 0xf9,
            Self::RuinsOfThaurissan => 0xfa,
            Self::FlameCrest => 0xfb,
            Self::BlackrockStronghold => 0xfc,
            Self::ThePillarOfAsh => 0xfd,
            Self::BlackrockMountain1 => 0xfe,
            Self::AltarOfStorms0 => 0xff,
            Self::Aldrassil => 0x100,
            Self::ShadowthreadCave => 0x101,
            Self::FelRock => 0x102,
            Self::LakeAlAmeth => 0x103,
            Self::StarbreezeVillage => 0x104,
            Self::GnarlpineHold => 0x105,
            Self::BanethilBarrowDen => 0x106,
            Self::TheCleft => 0x107,
            Self::TheOracleGlade => 0x108,
            Self::WellspringRiver => 0x109,
            Self::WellspringLake => 0x10a,
            Self::HillsbradFoothills => 0x10b,
            Self::AzsharaCrater => 0x10c,
            Self::DunAlgaz0 => 0x10d,
            Self::Southshore0 => 0x10f,
            Self::TarrenMill0 => 0x110,
            Self::DurnholdeKeep0 => 0x113,
            Self::UnusedStonewroughtPass => 0x114,
            Self::TheFoothillCaverns => 0x115,
            Self::LordamereInternmentCamp => 0x116,
            Self::Dalaran => 0x117,
            Self::Strahnbrad => 0x118,
            Self::RuinsOfAlterac => 0x119,
            Self::CrushridgeHold => 0x11a,
            Self::SlaughterHollow => 0x11b,
            Self::TheUplands => 0x11c,
            Self::SouthpointTower0 => 0x11d,
            Self::HillsbradFields0 => 0x11e,
            Self::Hillsbrad => 0x11f,
            Self::AzurelodeMine0 => 0x120,
            Self::NethanderStead0 => 0x121,
            Self::DunGarok0 => 0x122,
            Self::ThoradinsWall0 => 0x125,
            Self::EasternStrand0 => 0x126,
            Self::WesternStrand0 => 0x127,
            Self::SouthSeasUnused => 0x128,
            Self::JagueroIsle => 0x129,
            Self::BaradinBay => 0x12a,
            Self::MenethilBay => 0x12b,
            Self::MistyReedStrand => 0x12c,
            Self::TheSavageCoast => 0x12d,
            Self::TheCrystalShore => 0x12e,
            Self::ShellBeach => 0x12f,
            Self::NorthTidesRun => 0x131,
            Self::SouthTidesRun => 0x132,
            Self::TheOverlookCliffs => 0x133,
            Self::TheForbiddingSea0 => 0x134,
            Self::IronbeardsTomb => 0x135,
            Self::CrystalveinMine => 0x136,
            Self::RuinsOfAboraz => 0x137,
            Self::JaneirosPoint => 0x138,
            Self::NorthfoldManor => 0x139,
            Self::GoShekFarm => 0x13a,
            Self::DabyriesFarmstead => 0x13b,
            Self::BoulderfistHall => 0x13c,
            Self::WitherbarkVillage => 0x13d,
            Self::DrywhiskerGorge => 0x13e,
            Self::RefugePointe => 0x140,
            Self::Hammerfall => 0x141,
            Self::BlackwaterShipwrecks => 0x142,
            Self::OBreensCamp => 0x143,
            Self::StromgardeKeep => 0x144,
            Self::TheTowerOfArathor => 0x145,
            Self::TheSanctum => 0x146,
            Self::FaldirsCove => 0x147,
            Self::TheDrownedReef => 0x148,
            Self::ThandolSpan0 => 0x14a,
            Self::Ashenvale => 0x14b,
            Self::TheGreatSea2 => 0x14c,
            Self::CircleOfEastBinding => 0x14d,
            Self::CircleOfWestBinding => 0x14e,
            Self::CircleOfInnerBinding => 0x14f,
            Self::CircleOfOuterBinding => 0x150,
            Self::ApocryphansRest => 0x151,
            Self::AngorFortress => 0x152,
            Self::LethlorRavine => 0x153,
            Self::Kargath => 0x154,
            Self::CampKosh => 0x155,
            Self::CampBoff => 0x156,
            Self::CampWurg => 0x157,
            Self::CampCagg => 0x158,
            Self::AgmondsEnd => 0x159,
            Self::HammertoesDigsite => 0x15a,
            Self::DustbelchGrotto => 0x15b,
            Self::AeriePeak => 0x15c,
            Self::WildhammerKeep => 0x15d,
            Self::QuelDanilLodge => 0x15e,
            Self::SkulkRock => 0x15f,
            Self::Zunwatha => 0x160,
            Self::ShadraAlor => 0x161,
            Self::JinthaAlor => 0x162,
            Self::TheAltarOfZul => 0x163,
            Self::Seradane => 0x164,
            Self::Feralas => 0x165,
            Self::BramblebladeRavine => 0x166,
            Self::BaelModan => 0x167,
            Self::TheVentureCoMine => 0x168,
            Self::Felwood => 0x169,
            Self::RazorHill => 0x16a,
            Self::ValleyOfTrials => 0x16b,
            Self::TheDen => 0x16c,
            Self::BurningBladeCoven => 0x16d,
            Self::KolkarCrag => 0x16e,
            Self::SenjinVillage => 0x16f,
            Self::EchoIsles => 0x170,
            Self::ThunderRidge => 0x171,
            Self::DrygulchRavine => 0x172,
            Self::DustwindCave => 0x173,
            Self::TiragardeKeep => 0x174,
            Self::ScuttleCoast => 0x175,
            Self::BladefistBay => 0x176,
            Self::DeadeyeShore => 0x177,
            Self::SouthfuryRiver0 => 0x179,
            Self::CampTaurajo => 0x17a,
            Self::FarWatchPost => 0x17b,
            Self::TheCrossroads => 0x17c,
            Self::BoulderLodeMine => 0x17d,
            Self::TheSludgeFen => 0x17e,
            Self::TheDryHills => 0x17f,
            Self::DreadmistPeak => 0x180,
            Self::NorthwatchHold => 0x181,
            Self::TheForgottenPools => 0x182,
            Self::LushwaterOasis => 0x183,
            Self::TheStagnantOasis => 0x184,
            Self::FieldOfGiants => 0x186,
            Self::TheMerchantCoast => 0x187,
            Self::Ratchet => 0x188,
            Self::DarkspearStrand => 0x189,
            Self::DarrowmereLakeUnused => 0x18a,
            Self::CaerDarrowUnused => 0x18b,
            Self::WinterhoofWaterWell => 0x18c,
            Self::ThunderhornWaterWell => 0x18d,
            Self::WildmaneWaterWell => 0x18e,
            Self::SkylineRidge => 0x18f,
            Self::ThousandNeedles => 0x190,
            Self::TheTidusStair => 0x191,
            Self::ShadyRestInn => 0x193,
            Self::BaeldunDigsite => 0x194,
            Self::Desolace => 0x195,
            Self::StonetalonMountains => 0x196,
            Self::OrgrimmarUnused => 0x197,
            Self::GillijimsIsle => 0x198,
            Self::IslandOfDoctorLapidis => 0x199,
            Self::RazorwindCanyon => 0x19a,
            Self::BathransHaunt => 0x19b,
            Self::TheRuinsOfOrdilAran => 0x19c,
            Self::MaestrasPost => 0x19d,
            Self::TheZoramStrand => 0x19e,
            Self::Astranaar => 0x19f,
            Self::TheShrineOfAessina => 0x1a0,
            Self::FireScarShrine => 0x1a1,
            Self::TheRuinsOfStardust => 0x1a2,
            Self::TheHowlingVale => 0x1a3,
            Self::SilverwindRefuge => 0x1a4,
            Self::MystralLake => 0x1a5,
            Self::FallenSkyLake => 0x1a6,
            Self::IrisLake => 0x1a8,
            Self::Moonwell => 0x1a9,
            Self::RaynewoodRetreat => 0x1aa,
            Self::TheShadyNook => 0x1ab,
            Self::NightRun => 0x1ac,
            Self::Xavian => 0x1ad,
            Self::Satyrnaar => 0x1ae,
            Self::SplintertreePost => 0x1af,
            Self::TheDorDanilBarrowDen => 0x1b0,
            Self::FalfarrenRiver => 0x1b1,
            Self::FelfireHill => 0x1b2,
            Self::DemonFallCanyon => 0x1b3,
            Self::DemonFallRidge => 0x1b4,
            Self::WarsongLumberCamp => 0x1b5,
            Self::BoughShadow => 0x1b6,
            Self::TheShimmeringFlats => 0x1b7,
            Self::Tanaris => 0x1b8,
            Self::LakeFalathim => 0x1b9,
            Self::Auberdine => 0x1ba,
            Self::RuinsOfMathystra => 0x1bb,
            Self::TowerOfAlthalaxx => 0x1bc,
            Self::CliffspringFalls => 0x1bd,
            Self::BashalAran => 0x1be,
            Self::AmethAran => 0x1bf,
            Self::GroveOfTheAncients => 0x1c0,
            Self::TheMastersGlaive => 0x1c1,
            Self::RemtravelsExcavation => 0x1c2,
            Self::MistsEdge => 0x1c4,
            Self::TheLongWash => 0x1c5,
            Self::WildbendRiver => 0x1c6,
            Self::BlackwoodDen => 0x1c7,
            Self::CliffspringRiver => 0x1c8,
            Self::TheVeiledSea0 => 0x1c9,
            Self::GoldRoad => 0x1ca,
            Self::ScarletWatchPost => 0x1cb,
            Self::SunRockRetreat => 0x1cc,
            Self::WindshearCrag => 0x1cd,
            Self::CragpoolLake => 0x1cf,
            Self::MirkfallonLake => 0x1d0,
            Self::TheCharredVale => 0x1d1,
            Self::ValleyOfTheBloodfuries => 0x1d2,
            Self::StonetalonPeak => 0x1d3,
            Self::TheTalonDen => 0x1d4,
            Self::GreatwoodVale => 0x1d5,
            Self::ThunderBluffUnused => 0x1d6,
            Self::BraveWindMesa => 0x1d7,
            Self::FireStoneMesa => 0x1d8,
            Self::MantleRock => 0x1d9,
            Self::HunterRiseUnused => 0x1da,
            Self::SpiritRiseUnused => 0x1db,
            Self::ElderRiseUnused => 0x1dc,
            Self::RuinsOfJubuwal => 0x1dd,
            Self::PoolsOfArlithrien => 0x1de,
            Self::TheRustmaulDigSite => 0x1df,
            Self::CampEthok => 0x1e0,
            Self::SplithoofCrag => 0x1e1,
            Self::Highperch => 0x1e2,
            Self::TheScreechingCanyon => 0x1e3,
            Self::FreewindPost => 0x1e4,
            Self::TheGreatLift0 => 0x1e5,
            Self::GalakHold => 0x1e6,
            Self::RoguefeatherDen => 0x1e7,
            Self::TheWeatheredNook => 0x1e8,
            Self::Thalanaar => 0x1e9,
            Self::UnGoroCrater => 0x1ea,
            Self::RazorfenKraul0 => 0x1eb,
            Self::RavenHillCemetery => 0x1ec,
            Self::Moonglade => 0x1ed,
            Self::DeleteMe0 => 0x1ef,
            Self::BrackenwallVillage => 0x1f0,
            Self::SwamplightManor => 0x1f1,
            Self::BloodfenBurrow => 0x1f2,
            Self::DarkmistCavern => 0x1f3,
            Self::MogglePoint => 0x1f4,
            Self::BeezilsWreck => 0x1f5,
            Self::WitchHill => 0x1f6,
            Self::SentryPoint => 0x1f7,
            Self::NorthPointTower => 0x1f8,
            Self::WestPointTower => 0x1f9,
            Self::LostPoint => 0x1fa,
            Self::Bluefen => 0x1fb,
            Self::StonemaulRuins => 0x1fc,
            Self::TheDenOfFlame => 0x1fd,
            Self::TheDragonmurk => 0x1fe,
            Self::Wyrmbog => 0x1ff,
            Self::OnyxiasLairUnused => 0x200,
            Self::TheramoreIsle => 0x201,
            Self::FootholdCitadel => 0x202,
            Self::IroncladPrison => 0x203,
            Self::DustwallowBay => 0x204,
            Self::TidefuryCove => 0x205,
            Self::DreadmurkShore => 0x206,
            Self::AddlesStead => 0x218,
            Self::FirePlumeRidge => 0x219,
            Self::LakkariTarPits => 0x21a,
            Self::TerrorRun => 0x21b,
            Self::TheSlitheringScar => 0x21c,
            Self::MarshalsRefuge => 0x21d,
            Self::FungalRock => 0x21e,
            Self::GolakkaHotSprings => 0x21f,
            Self::TheLoch => 0x22c,
            Self::BeggarsHaunt => 0x240,
            Self::KodoGraveyard => 0x254,
            Self::GhostWalkerPost => 0x255,
            Self::SartherisStrand => 0x256,
            Self::ThunderAxeFortress => 0x257,
            Self::BolgansHole => 0x258,
            Self::MannorocCoven => 0x25a,
            Self::Sargeron => 0x25b,
            Self::MagramVillage => 0x25c,
            Self::GelkisVillage => 0x25e,
            Self::ValleyOfSpears => 0x25f,
            Self::NijelsPoint => 0x260,
            Self::KolkarVillage => 0x261,
            Self::Hyjal => 0x268,
            Self::Winterspring => 0x26a,
            Self::BlackwolfRiver => 0x27c,
            Self::KodoRock => 0x27d,
            Self::HiddenPath => 0x27e,
            Self::SpiritRock => 0x27f,
            Self::ShrineOfTheDormantFlame => 0x280,
            Self::LakeEluneara => 0x290,
            Self::TheHarborage => 0x291,
            Self::Outland => 0x2a4,
            Self::CraftsmensTerraceUnused => 0x2b8,
            Self::TradesmensTerraceUnused => 0x2b9,
            Self::TheTempleGardensUnused => 0x2ba,
            Self::TempleOfEluneUnused => 0x2bb,
            Self::CenarionEnclaveUnused => 0x2bc,
            Self::WarriorsTerraceUnused => 0x2bd,
            Self::RuttheranVillage => 0x2be,
            Self::IronbandsCompound => 0x2cc,
            Self::TheStockade => 0x2cd,
            Self::WailingCaverns => 0x2ce,
            Self::BlackfathomDeeps0 => 0x2cf,
            Self::FrayIsland => 0x2d0,
            Self::Gnomeregan1 => 0x2d1,
            Self::RazorfenDowns0 => 0x2d2,
            Self::BanethilHollow => 0x2e0,
            Self::ScarletMonastery => 0x31c,
            Self::JerodsLanding => 0x31d,
            Self::RidgepointTower => 0x31e,
            Self::TheDarkenedBank => 0x31f,
            Self::ColdridgePass => 0x320,
            Self::ChillBreezeValley => 0x321,
            Self::ShimmerRidge => 0x322,
            Self::AmberstillRanch => 0x323,
            Self::TheTundridHills => 0x324,
            Self::SouthGatePass0 => 0x325,
            Self::SouthGateOutpost => 0x326,
            Self::NorthGatePass0 => 0x327,
            Self::NorthGateOutpost => 0x328,
            Self::GatesOfIronforge => 0x329,
            Self::StillwaterPond => 0x32a,
            Self::NightmareVale => 0x32b,
            Self::VenomwebVale => 0x32c,
            Self::TheBulwark1 => 0x32d,
            Self::SouthfuryRiver1 => 0x32e,
            Self::SouthfuryRiver2 => 0x32f,
            Self::RazormaneGrounds => 0x330,
            Self::SkullRock => 0x331,
            Self::PalemaneRock => 0x332,
            Self::WindfuryRidge => 0x333,
            Self::TheGoldenPlains => 0x334,
            Self::TheRollingPlains => 0x335,
            Self::DunAlgaz1 => 0x344,
            Self::DunAlgaz2 => 0x345,
            Self::NorthGatePass1 => 0x346,
            Self::SouthGatePass1 => 0x347,
            Self::TwilightGrove => 0x358,
            Self::GmIsland => 0x36c,
            Self::DeleteMe1 => 0x36d,
            Self::SouthfuryRiver3 => 0x36e,
            Self::SouthfuryRiver4 => 0x36f,
            Self::ThandolSpan1 => 0x370,
            Self::ThandolSpan2 => 0x371,
            Self::PurgationIsle => 0x380,
            Self::TheJansenStead => 0x394,
            Self::TheDeadAcre => 0x395,
            Self::TheMolsenFarm => 0x396,
            Self::StendelsPond => 0x397,
            Self::TheDaggerHills => 0x398,
            Self::DemontsPlace => 0x399,
            Self::TheDustPlains => 0x39a,
            Self::StonesplinterValley => 0x39b,
            Self::ValleyOfKings => 0x39c,
            Self::AlgazStation => 0x39d,
            Self::BucklebreeFarm => 0x39e,
            Self::TheShiningStrand => 0x39f,
            Self::NorthTidesHollow => 0x3a0,
            Self::GrizzlepawRidge => 0x3a8,
            Self::TheVerdantFields => 0x3bc,
            Self::Gadgetzan => 0x3d0,
            Self::SteamwheedlePort => 0x3d1,
            Self::ZulFarrak0 => 0x3d2,
            Self::SandsorrowWatch => 0x3d3,
            Self::ThistleshrubValley => 0x3d4,
            Self::TheGapingChasm => 0x3d5,
            Self::TheNoxiousLair => 0x3d6,
            Self::DunemaulCompound => 0x3d7,
            Self::EastmoonRuins => 0x3d8,
            Self::WaterspringField => 0x3d9,
            Self::ZalashjisDen => 0x3da,
            Self::LandsEndBeach => 0x3db,
            Self::WavestriderBeach => 0x3dc,
            Self::Uldum => 0x3dd,
            Self::ValleyOfTheWatchers => 0x3de,
            Self::GunstansPost => 0x3df,
            Self::SouthmoonRuins => 0x3e0,
            Self::RendersCamp => 0x3e4,
            Self::RendersValley => 0x3e5,
            Self::RendersRock => 0x3e6,
            Self::StonewatchTower => 0x3e7,
            Self::GalardellValley => 0x3e8,
            Self::LakeridgeHighway => 0x3e9,
            Self::ThreeCorners => 0x3ea,
            Self::DireforgeHill => 0x3f8,
            Self::RaptorRidge => 0x3f9,
            Self::BlackChannelMarsh => 0x3fa,
            Self::TheGreenBelt0 => 0x3fb,
            Self::MosshideFen => 0x3fc,
            Self::ThelgenRock => 0x3fd,
            Self::BluegillMarsh => 0x3fe,
            Self::SaltsprayGlen => 0x3ff,
            Self::SundownMarsh => 0x400,
            Self::TheGreenBelt1 => 0x401,
            Self::AngerfangEncampment => 0x40c,
            Self::GrimBatol => 0x40d,
            Self::DragonmawGates => 0x40e,
            Self::TheLostFleet => 0x40f,
            Self::DarrowHill0 => 0x420,
            Self::ThoradinsWall1 => 0x421,
            Self::WebwinderPath => 0x434,
            Self::TheHushedBank => 0x449,
            Self::ManorMistmantle => 0x44a,
            Self::CampMojache => 0x44b,
            Self::GrimtotemCompound => 0x44c,
            Self::TheWrithingDeep => 0x44d,
            Self::WildwindLake => 0x44e,
            Self::GordunniOutpost => 0x44f,
            Self::MokGordun => 0x450,
            Self::FeralScarVale => 0x451,
            Self::FrayfeatherHighlands => 0x452,
            Self::IdlewindLake => 0x453,
            Self::TheForgottenCoast => 0x454,
            Self::EastPillar => 0x455,
            Self::WestPillar => 0x456,
            Self::DreamBough => 0x457,
            Self::JademirLake => 0x458,
            Self::Oneiros => 0x459,
            Self::RuinsOfRavenwind => 0x45a,
            Self::RageScarHold => 0x45b,
            Self::FeathermoonStronghold => 0x45c,
            Self::RuinsOfSolarsal => 0x45d,
            Self::LowerWildsUnused => 0x45e,
            Self::TheTwinColossals => 0x45f,
            Self::SardorIsle => 0x460,
            Self::IsleOfDread => 0x461,
            Self::HighWilderness => 0x470,
            Self::LowerWilds => 0x471,
            Self::SouthernBarrens => 0x484,
            Self::SouthernGoldRoad => 0x485,
            Self::ZulFarrak1 => 0x498,
            Self::UnusedAlcazIsland => 0x4ac,
            Self::TimbermawHold0 => 0x4c0,
            Self::VanndirEncampment => 0x4c1,
            Self::TestAzshara => 0x4c2,
            Self::LegashEncampment => 0x4c3,
            Self::ThalassianBaseCamp => 0x4c4,
            Self::RuinsOfEldarath => 0x4c5,
            Self::HetaerasClutch => 0x4c6,
            Self::TempleOfZinMalor => 0x4c7,
            Self::BearsHead => 0x4c8,
            Self::Ursolan => 0x4c9,
            Self::TempleOfArkkoran => 0x4ca,
            Self::BayOfStorms => 0x4cb,
            Self::TheShatteredStrand => 0x4cc,
            Self::TowerOfEldara => 0x4cd,
            Self::JaggedReef => 0x4ce,
            Self::SouthridgeBeach => 0x4cf,
            Self::RavencrestMonument => 0x4d0,
            Self::ForlornRidge => 0x4d1,
            Self::LakeMennar => 0x4d2,
            Self::ShadowsongShrine => 0x4d3,
            Self::HaldarrEncampment => 0x4d4,
            Self::Valormok => 0x4d5,
            Self::TheRuinedReaches => 0x4e8,
            Self::TheTalondeepPath0 => 0x4fc,
            Self::TheTalondeepPath1 => 0x4fd,
            Self::RocktuskFarm => 0x510,
            Self::JaggedswineFarm => 0x511,
            Self::RazorfenDowns1 => 0x524,
            Self::LostRiggerCove => 0x538,
            Self::Uldaman0 => 0x539,
            Self::LordamereLake1 => 0x53a,
            Self::LordamereLake2 => 0x53b,
            Self::GallowsCorner => 0x54d,
            Self::Silithus => 0x561,
            Self::EmeraldForest => 0x575,
            Self::SunkenTemple => 0x589,
            Self::DreadmaulHold => 0x59d,
            Self::NethergardeKeep => 0x59e,
            Self::DreadmaulPost => 0x59f,
            Self::SerpentsCoil => 0x5a0,
            Self::AltarOfStorms1 => 0x5a1,
            Self::FirewatchRidge => 0x5a2,
            Self::TheSlagPit => 0x5a3,
            Self::TheSeaOfCinders => 0x5a4,
            Self::BlackrockMountain2 => 0x5a5,
            Self::ThoriumPoint => 0x5a6,
            Self::GarrisonArmory => 0x5b1,
            Self::TheTempleOfAtalHakkar => 0x5c5,
            Self::Undercity => 0x5d9,
            Self::Uldaman1 => 0x5ed,
            Self::NotUsedDeadmines => 0x5ee,
            Self::StormwindCity => 0x5ef,
            Self::Ironforge => 0x601,
            Self::SplithoofHold => 0x615,
            Self::TheCapeOfStranglethorn => 0x629,
            Self::SouthernSavageCoast => 0x62a,
            Self::UnusedTheDeadmines002 => 0x62b,
            Self::UnusedIroncladCove003 => 0x62c,
            Self::TheDeadmines => 0x62d,
            Self::IroncladCove => 0x62e,
            Self::BlackrockSpire => 0x62f,
            Self::BlackrockDepths => 0x630,
            Self::RaptorGroundsUnused => 0x63d,
            Self::GroldomFarmUnused => 0x63e,
            Self::MorshanBaseCamp => 0x63f,
            Self::HonorsStandUnused => 0x640,
            Self::BlackthornRidgeUnused => 0x641,
            Self::BramblescarUnused => 0x642,
            Self::AgamagorUnused => 0x643,
            Self::ValleyOfHeroes => 0x651,
            Self::Orgrimmar => 0x665,
            Self::ThunderBluff => 0x666,
            Self::ElderRise => 0x667,
            Self::SpiritRise => 0x668,
            Self::HunterRise => 0x669,
            Self::Darnassus => 0x679,
            Self::CenarionEnclave => 0x67a,
            Self::CraftsmensTerrace => 0x67b,
            Self::WarriorsTerrace => 0x67c,
            Self::TheTempleGardens => 0x67d,
            Self::TradesmensTerrace => 0x67e,
            Self::GavinsNaze => 0x68d,
            Self::SoferasNaze => 0x68e,
            Self::CorrahnsDagger => 0x68f,
            Self::TheHeadland => 0x690,
            Self::MistyShore => 0x691,
            Self::DandredsFold => 0x692,
            Self::GrowlessCave => 0x693,
            Self::ChillwindPoint => 0x694,
            Self::RaptorGrounds => 0x6a1,
            Self::Bramblescar => 0x6a2,
            Self::ThornHill => 0x6a3,
            Self::Agamagor => 0x6a4,
            Self::BlackthornRidge => 0x6a5,
            Self::HonorsStand => 0x6a6,
            Self::TheMorshanRampart => 0x6a7,
            Self::GroldomFarm => 0x6a8,
            Self::RazorfenKraul1 => 0x6b5,
            Self::TheGreatLift1 => 0x6b6,
            Self::MistvaleValley => 0x6c9,
            Self::NekmaniWellspring => 0x6ca,
            Self::BloodsailCompound => 0x6cb,
            Self::VentureCoBaseCamp => 0x6cc,
            Self::GurubashiArena => 0x6cd,
            Self::SpiritDen => 0x6ce,
            Self::TheCrimsonVeil => 0x6dd,
            Self::TheRiptide => 0x6de,
            Self::TheDamselsLuck => 0x6df,
            Self::VentureCoOperationsCenter => 0x6e0,
            Self::DeadwoodVillage => 0x6e1,
            Self::FelpawVillage => 0x6e2,
            Self::Jaedenar => 0x6e3,
            Self::BloodvenomRiver => 0x6e4,
            Self::BloodvenomFalls => 0x6e5,
            Self::ShatterScarVale => 0x6e6,
            Self::IrontreeWoods => 0x6e7,
            Self::IrontreeCavern => 0x6e8,
            Self::TimbermawHold1 => 0x6e9,
            Self::ShadowHold => 0x6ea,
            Self::ShrineOfTheDeceiver => 0x6eb,
            Self::IthariussCave => 0x6f1,
            Self::Sorrowmurk => 0x6f2,
            Self::DraenildurVillage => 0x6f3,
            Self::SplinterspearJunction => 0x6f4,
            Self::Stagalbog => 0x705,
            Self::TheShiftingMire => 0x706,
            Self::StagalbogCave => 0x719,
            Self::WitherbarkCaverns => 0x72d,
            Self::ThoradinsWall2 => 0x741,
            Self::Bouldergor => 0x742,
            Self::ValleyOfFangs => 0x755,
            Self::TheDustbowl => 0x756,
            Self::MirageFlats => 0x757,
            Self::FeatherbeardsHovel => 0x758,
            Self::ShindiggersCamp => 0x759,
            Self::PlaguemistRavine => 0x75a,
            Self::ValorwindLake => 0x75b,
            Self::Agolwatha => 0x75c,
            Self::Hiriwatha => 0x75d,
            Self::TheCreepingRuin => 0x75e,
            Self::BogensLedge => 0x75f,
            Self::TheMakersTerrace => 0x769,
            Self::DustwindGulch => 0x76a,
            Self::Shaolwatha => 0x77d,
            Self::NoonshadeRuins => 0x791,
            Self::BrokenPillar => 0x792,
            Self::AbyssalSands => 0x793,
            Self::SouthbreakShore => 0x794,
            Self::CavernsOfTime0 => 0x795,
            Self::TheMarshlands => 0x796,
            Self::IronstonePlateau => 0x797,
            Self::BlackcharCave => 0x7a5,
            Self::TannerCamp => 0x7a6,
            Self::DustfireValley => 0x7a7,
            Self::ZulGurub1 => 0x7b9,
            Self::MistyReedPost => 0x7ba,
            Self::BloodvenomPost => 0x7cd,
            Self::TalonbranchGlade => 0x7ce,
            Self::Stratholme0 => 0x7e1,
            Self::UnusedShadowfangKeep003 => 0x7f5,
            Self::Scholomance => 0x809,
            Self::TwilightVale => 0x81d,
            Self::TwilightShore => 0x81e,
            Self::AlcazIsland => 0x81f,
            Self::DarkcloudPinnacle => 0x831,
            Self::DawningWoodCatacombs => 0x832,
            Self::StonewatchKeep => 0x833,
            Self::Maraudon => 0x834,
            Self::StoutlagerInn => 0x835,
            Self::ThunderbrewDistillery => 0x836,
            Self::MenethilKeep => 0x837,
            Self::DeepwaterTavern => 0x838,
            Self::ShadowGrave => 0x845,
            Self::BrillTownHall => 0x846,
            Self::GallowsEndTavern => 0x847,
            Self::ThePoolsOfVisionUnused => 0x859,
            Self::DreadmistDen => 0x85a,
            Self::BaeldunKeep => 0x86d,
            Self::EmberstrifesDen => 0x86e,
            Self::OnyxiasLair => 0x86f,
            Self::WindshearMine => 0x870,
            Self::RolandsDoom => 0x871,
            Self::BattleRing => 0x881,
            Self::ThePoolsOfVision => 0x895,
            Self::ShadowbreakRavine => 0x896,
            Self::BrokenSpearVillage => 0x8a9,
            Self::WhitereachPost => 0x8bd,
            Self::Gornia => 0x8be,
            Self::ZanesEyeCrater => 0x8bf,
            Self::MirageRaceway => 0x8c0,
            Self::FrostsaberRock => 0x8c1,
            Self::TheHiddenGrove => 0x8c2,
            Self::TimbermawPost => 0x8c3,
            Self::WinterfallVillage => 0x8c4,
            Self::Mazthoril => 0x8c5,
            Self::FrostfireHotSprings => 0x8c6,
            Self::IceThistleHills => 0x8c7,
            Self::DunMandarr => 0x8c8,
            Self::FrostwhisperGorge => 0x8c9,
            Self::OwlWingThicket => 0x8ca,
            Self::LakeKelTheril => 0x8cb,
            Self::TheRuinsOfKelTheril => 0x8cc,
            Self::StarfallVillage => 0x8cd,
            Self::BanThallowBarrowDen => 0x8ce,
            Self::Everlook => 0x8cf,
            Self::DarkwhisperGorge => 0x8d0,
            Self::DeeprunTram => 0x8d1,
            Self::TheFungalVale => 0x8d2,
            Self::UnusedTheMarrisStead => 0x8d3,
            Self::TheMarrisStead => 0x8d4,
            Self::TheUndercroft => 0x8d5,
            Self::Darrowshire => 0x8d6,
            Self::CrownGuardTower => 0x8d7,
            Self::CorinsCrossing => 0x8d8,
            Self::ScarletBaseCamp => 0x8d9,
            Self::TyrsHand => 0x8da,
            Self::TheScarletBasilica => 0x8db,
            Self::LightsHopeChapel => 0x8dc,
            Self::BrowmanMill => 0x8dd,
            Self::TheNoxiousGlade => 0x8de,
            Self::EastwallTower => 0x8df,
            Self::Northdale => 0x8e0,
            Self::ZulMashar => 0x8e1,
            Self::MazraAlor => 0x8e2,
            Self::NorthpassTower => 0x8e3,
            Self::QuelLithienLodge => 0x8e4,
            Self::Plaguewood => 0x8e5,
            Self::Scourgehold => 0x8e6,
            Self::Stratholme1 => 0x8e7,
            Self::UnusedStratholme => 0x8e8,
            Self::DarrowmereLake0 => 0x8f9,
            Self::CaerDarrow => 0x8fa,
            Self::DarrowmereLake1 => 0x8fb,
            Self::CavernsOfTime1 => 0x8fc,
            Self::ThistlefurVillage => 0x8fd,
            Self::TheQuagmire => 0x8fe,
            Self::WindbreakCanyon => 0x8ff,
            Self::SouthSeas0 => 0x90d,
            Self::TheGreatSea3 => 0x90e,
            Self::TheGreatSea4 => 0x90f,
            Self::TheGreatSea5 => 0x910,
            Self::TheGreatSea6 => 0x911,
            Self::TheVeiledSea1 => 0x912,
            Self::TheVeiledSea2 => 0x913,
            Self::TheVeiledSea3 => 0x914,
            Self::TheVeiledSea4 => 0x915,
            Self::TheVeiledSea5 => 0x916,
            Self::RazorHillBarracks => 0x921,
            Self::SouthSeas1 => 0x922,
            Self::TheGreatSea7 => 0x923,
            Self::BloodtoothCamp => 0x935,
            Self::ForestSong => 0x936,
            Self::GreenpawVillage => 0x937,
            Self::SilverwingOutpost => 0x938,
            Self::Nighthaven => 0x939,
            Self::ShrineOfRemulos => 0x93a,
            Self::StormrageBarrowDens => 0x93b,
            Self::TheGreatSea8 => 0x93c,
            Self::TheGreatSea9 => 0x93d,
            Self::TheBlackMorass => 0x93e,
            Self::OldHillsbradFoothills => 0x93f,
            Self::TarrenMill1 => 0x940,
            Self::Southshore1 => 0x941,
            Self::DurnholdeKeep1 => 0x942,
            Self::DunGarok1 => 0x943,
            Self::HillsbradFields1 => 0x944,
            Self::EasternStrand1 => 0x945,
            Self::NethanderStead1 => 0x946,
            Self::DarrowHill1 => 0x947,
            Self::SouthpointTower1 => 0x948,
            Self::ThoradinsWall3 => 0x949,
            Self::WesternStrand1 => 0x94a,
            Self::AzurelodeMine1 => 0x94b,
            Self::TheGreatSea10 => 0x95d,
            Self::TheGreatSea11 => 0x95e,
            Self::TheGreatSea12 => 0x95f,
            Self::TheForbiddingSea1 => 0x960,
            Self::TheForbiddingSea2 => 0x961,
            Self::TheForbiddingSea3 => 0x962,
            Self::TheForbiddingSea4 => 0x963,
            Self::TethrisAran => 0x964,
            Self::EthelRethor => 0x965,
            Self::RanazjarIsle => 0x966,
            Self::KormeksHut => 0x967,
            Self::ShadowpreyVillage => 0x968,
            Self::BlackrockPass => 0x971,
            Self::MorgansVigil => 0x972,
            Self::SlitherRock => 0x973,
            Self::TerrorWingPath => 0x974,
            Self::Dracodar => 0x975,
            Self::RagefireChasm => 0x985,
            Self::NightsongWoods => 0x999,
            Self::TheVeiledSea6 => 0x9ad,
            Self::MorlosAran => 0x9ae,
            Self::EmeraldSanctuary => 0x9af,
            Self::JadefireGlen => 0x9b0,
            Self::RuinsOfConstellas => 0x9b1,
            Self::BitterReaches => 0x9c1,
            Self::RiseOfTheDefiler => 0x9d5,
            Self::LarissPavilion => 0x9d6,
            Self::WoodpawHills => 0x9d7,
            Self::WoodpawDen => 0x9d8,
            Self::VerdantisRiver => 0x9d9,
            Self::RuinsOfIsildien => 0x9da,
            Self::GrimtotemPost => 0x9e9,
            Self::CampAparaje => 0x9ea,
            Self::Malakajin => 0x9eb,
            Self::BoulderslideRavine => 0x9ec,
            Self::SishirCanyon => 0x9ed,
            Self::DireMaul0 => 0x9fd,
            Self::DeadwindRavine => 0x9fe,
            Self::DiamondheadRiver => 0x9ff,
            Self::AridensCamp => 0xa00,
            Self::TheVice => 0xa01,
            Self::Karazhan => 0xa02,
            Self::MorgansPlot => 0xa03,
            Self::DireMaul1 => 0xa11,
            Self::AlteracValley0 => 0xa25,
            Self::ScrabblescrewsCamp => 0xa39,
            Self::JadefireRun => 0xa3a,
            Self::ThondrorilRiver0 => 0xa3b,
            Self::ThondrorilRiver1 => 0xa3c,
            Self::LakeMereldar => 0xa3d,
            Self::PestilentScar => 0xa3e,
            Self::TheInfectisScar => 0xa3f,
            Self::BlackwoodLake => 0xa40,
            Self::EastwallGate => 0xa41,
            Self::TerrorwebTunnel => 0xa42,
            Self::Terrordale => 0xa43,
            Self::KargathiaKeep => 0xa4d,
            Self::ValleyOfBones => 0xa61,
            Self::BlackwingLair => 0xa75,
            Self::DeadmansCrossing => 0xa89,
            Self::MoltenCore => 0xa9d,
            Self::TheScarabWall => 0xab1,
            Self::SouthwindVillage => 0xab2,
            Self::TwilightBaseCamp => 0xab3,
            Self::TheCrystalVale => 0xab4,
            Self::TheScarabDais => 0xab5,
            Self::HiveAshi => 0xab6,
            Self::HiveZora => 0xab7,
            Self::HiveRegal => 0xab8,
            Self::ShrineOfTheFallenWarrior => 0xac5,
            Self::UnusedAlteracValley => 0xad9,
            Self::BlackfathomDeeps1 => 0xaed,
            Self::OnMapDungeon4 => 0xb01,
            Self::TheMastersCellar => 0xb15,
            Self::StonewroughtPass => 0xb16,
            Self::AlteracValley1 => 0xb17,
            Self::TheRumbleCage => 0xb29,
            Self::ChunkTest => 0xb3d,
            Self::ZoramgarOutpost => 0xb51,
            Self::HallOfLegends => 0xb65,
            Self::ChampionsHall => 0xb66,
            Self::GroshgokCompound => 0xb79,
            Self::SleepingGorge => 0xb7a,
            Self::IrondeepMine => 0xb8d,
            Self::StonehearthOutpost => 0xb8e,
            Self::DunBaldar => 0xb8f,
            Self::IcewingPass => 0xb90,
            Self::FrostwolfVillage => 0xb91,
            Self::TowerPoint => 0xb92,
            Self::ColdtoothMine => 0xb93,
            Self::WinteraxHold => 0xb94,
            Self::IcebloodGarrison => 0xba1,
            Self::FrostwolfKeep => 0xba2,
            Self::TorkrenFarm => 0xba3,
            Self::FrostDaggerPass => 0xbc9,
            Self::IronstoneCamp => 0xbdd,
            Self::WeazelsCrater => 0xbde,
            Self::TahondaRuins => 0xbdf,
            Self::FieldOfStrife => 0xbf1,
            Self::IcewingCavern => 0xbf2,
            Self::ValorsRest => 0xc05,
            Self::TheSwarmingPillar => 0xc19,
            Self::TwilightPost => 0xc1a,
            Self::TwilightOutpost => 0xc1b,
            Self::RavagedTwilightCamp => 0xc1c,
            Self::ShalzarusLair => 0xc2d,
            Self::TalrendisPoint => 0xc41,
            Self::RethressSanctum => 0xc42,
            Self::MoonHorrorDen => 0xc43,
            Self::ScalebeardsCave => 0xc44,
            Self::BoulderslideCavern => 0xc55,
            Self::WarsongLaborCamp => 0xc69,
            Self::ChillwindCamp => 0xc7d,
            Self::TheMaul => 0xc91,
            Self::TheMaulUnused => 0xca5,
            Self::BonesOfGrakkarond => 0xcb9,
            Self::WarsongGulch => 0xccd,
            Self::FrostwolfGraveyard => 0xce1,
            Self::FrostwolfPass => 0xce2,
            Self::DunBaldarPass => 0xce3,
            Self::IcebloodGraveyard => 0xce4,
            Self::SnowfallGraveyard => 0xce5,
            Self::StonehearthGraveyard => 0xce6,
            Self::StormpikeGraveyard => 0xce7,
            Self::IcewingBunker => 0xce8,
            Self::StonehearthBunker => 0xce9,
            Self::WildpawRidge => 0xcea,
            Self::RevantuskVillage => 0xcf5,
            Self::RockOfDurotan => 0xcf6,
            Self::SilverwingGrove => 0xcf7,
            Self::WarsongLumberMill => 0xcf8,
            Self::SilverwingHold => 0xcf9,
            Self::WildpawCavern => 0xd09,
            Self::TheVeiledCleft => 0xd0a,
            Self::YojambaIsle => 0xd1d,
            Self::ArathiBasin => 0xd1e,
            Self::TheCoil => 0xd31,
            Self::AltarOfHireek => 0xd32,
            Self::Shadrazaar => 0xd33,
            Self::HakkariGrounds => 0xd34,
            Self::NazeOfShirvallah => 0xd35,
            Self::TempleOfBethekk => 0xd36,
            Self::TheBloodfirePit => 0xd37,
            Self::AltarOfTheBloodGod => 0xd38,
            Self::ZanzasRise => 0xd45,
            Self::EdgeOfMadness => 0xd46,
            Self::TrollbaneHall => 0xd59,
            Self::DefilersDen => 0xd5a,
            Self::PaglesPointe => 0xd5b,
            Self::Farm => 0xd5c,
            Self::Blacksmith => 0xd5d,
            Self::LumberMill => 0xd5e,
            Self::GoldMine => 0xd5f,
            Self::Stables => 0xd60,
            Self::CenarionHold => 0xd61,
            Self::StaghelmPoint => 0xd62,
            Self::BronzebeardEncampment => 0xd63,
            Self::AhnQiraj => 0xd64,
            Self::RuinsOfAhnQiraj0 => 0xd65,
            Self::TwilightsRun => 0xd76,
            Self::OrtellsHideout => 0xd77,
            Self::ScarabTerrace => 0xd78,
            Self::GeneralsTerrace => 0xd79,
            Self::TheReservoir => 0xd7a,
            Self::TheHatchery => 0xd7b,
            Self::TheComb => 0xd7c,
            Self::WatchersTerrace => 0xd7d,
            Self::RuinsOfAhnQiraj1 => 0xd7e,
            Self::Naxxramas => 0xd80,
            Self::City => 0xd83,
            Self::GatesOfAhnQiraj => 0xd96,
            Self::RavenholdtManor => 0xd9e,
        }
    }

}

impl Default for Area {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::DunMorogh => f.write_str("Dun Morogh"),
            Self::Longshore => f.write_str("Longshore"),
            Self::Badlands => f.write_str("Badlands"),
            Self::BlastedLands => f.write_str("Blasted Lands"),
            Self::BlackwaterCove => f.write_str("Blackwater Cove"),
            Self::SwampOfSorrows => f.write_str("Swamp of Sorrows"),
            Self::NorthshireValley => f.write_str("Northshire Valley"),
            Self::Duskwood => f.write_str("Duskwood"),
            Self::Wetlands => f.write_str("Wetlands"),
            Self::ElwynnForest => f.write_str("Elwynn Forest"),
            Self::TheWorldTree => f.write_str("The World Tree"),
            Self::Durotar => f.write_str("Durotar"),
            Self::DustwallowMarsh => f.write_str("Dustwallow Marsh"),
            Self::Azshara => f.write_str("Azshara"),
            Self::TheBarrens => f.write_str("The Barrens"),
            Self::CrystalLake => f.write_str("Crystal Lake"),
            Self::ZulGurub0 => f.write_str("Zul'Gurub"),
            Self::Moonbrook => f.write_str("Moonbrook"),
            Self::KulTiras => f.write_str("Kul Tiras"),
            Self::ProgrammerIsle => f.write_str("Programmer Isle"),
            Self::NorthshireRiver => f.write_str("Northshire River"),
            Self::NorthshireAbbey => f.write_str("Northshire Abbey"),
            Self::BlackrockMountain0 => f.write_str("Blackrock Mountain"),
            Self::Lighthouse => f.write_str("Lighthouse"),
            Self::WesternPlaguelands => f.write_str("Western Plaguelands"),
            Self::Nine => f.write_str("Nine"),
            Self::TheCemetary => f.write_str("The Cemetary"),
            Self::StranglethornVale => f.write_str("Stranglethorn Vale"),
            Self::EchoRidgeMine => f.write_str("Echo Ridge Mine"),
            Self::BootyBay => f.write_str("Booty Bay"),
            Self::AlteracMountains => f.write_str("Alterac Mountains"),
            Self::LakeNazferiti => f.write_str("Lake Nazferiti"),
            Self::LochModan => f.write_str("Loch Modan"),
            Self::Westfall0 => f.write_str("Westfall"),
            Self::DeadwindPass => f.write_str("Deadwind Pass"),
            Self::Darkshire => f.write_str("Darkshire"),
            Self::WildShore => f.write_str("Wild Shore"),
            Self::RedridgeMountains => f.write_str("Redridge Mountains"),
            Self::ArathiHighlands => f.write_str("Arathi Highlands"),
            Self::BurningSteppes => f.write_str("Burning Steppes"),
            Self::TheHinterlands => f.write_str("The Hinterlands"),
            Self::DeadMansHole => f.write_str("Dead Man's Hole"),
            Self::SearingGorge => f.write_str("Searing Gorge"),
            Self::ThievesCamp => f.write_str("Thieves Camp"),
            Self::JasperlodeMine => f.write_str("Jasperlode Mine"),
            Self::ValleyOfHeroesUnused => f.write_str("Valley of Heroes UNUSED"),
            Self::HeroesVigil => f.write_str("Heroes' Vigil"),
            Self::FargodeepMine => f.write_str("Fargodeep Mine"),
            Self::NorthshireVineyards => f.write_str("Northshire Vineyards"),
            Self::ForestsEdge => f.write_str("Forest's Edge"),
            Self::ThunderFalls => f.write_str("Thunder Falls"),
            Self::BrackwellPumpkinPatch => f.write_str("Brackwell Pumpkin Patch"),
            Self::TheStonefieldFarm => f.write_str("The Stonefield Farm"),
            Self::TheMaclureVineyards => f.write_str("The Maclure Vineyards"),
            Self::OnMapDungeon0 => f.write_str("***On Map Dungeon***"),
            Self::OnMapDungeon1 => f.write_str("***On Map Dungeon***"),
            Self::OnMapDungeon2 => f.write_str("***On Map Dungeon***"),
            Self::LakeEverstill => f.write_str("Lake Everstill"),
            Self::Lakeshire => f.write_str("Lakeshire"),
            Self::Stonewatch => f.write_str("Stonewatch"),
            Self::StonewatchFalls => f.write_str("Stonewatch Falls"),
            Self::TheDarkPortal => f.write_str("The Dark Portal"),
            Self::TheTaintedScar => f.write_str("The Tainted Scar"),
            Self::PoolOfTears => f.write_str("Pool of Tears"),
            Self::Stonard => f.write_str("Stonard"),
            Self::FallowSanctuary => f.write_str("Fallow Sanctuary"),
            Self::Anvilmar => f.write_str("Anvilmar"),
            Self::StormwindMountains => f.write_str("Stormwind Mountains"),
            Self::JeffNeQuadrantChanged => f.write_str("Jeff NE Quadrant Changed"),
            Self::JeffNwQuadrant => f.write_str("Jeff NW Quadrant"),
            Self::JeffSeQuadrant => f.write_str("Jeff SE Quadrant"),
            Self::JeffSwQuadrant => f.write_str("Jeff SW Quadrant"),
            Self::TirisfalGlades => f.write_str("Tirisfal Glades"),
            Self::StoneCairnLake => f.write_str("Stone Cairn Lake"),
            Self::Goldshire => f.write_str("Goldshire"),
            Self::EastvaleLoggingCamp => f.write_str("Eastvale Logging Camp"),
            Self::MirrorLakeOrchard => f.write_str("Mirror Lake Orchard"),
            Self::TowerOfAzora => f.write_str("Tower of Azora"),
            Self::MirrorLake => f.write_str("Mirror Lake"),
            Self::VulGolOgreMound => f.write_str("Vul'Gol Ogre Mound"),
            Self::RavenHill => f.write_str("Raven Hill"),
            Self::RedridgeCanyons => f.write_str("Redridge Canyons"),
            Self::TowerOfIlgalar => f.write_str("Tower of Ilgalar"),
            Self::AlthersMill => f.write_str("Alther's Mill"),
            Self::RethbanCaverns => f.write_str("Rethban Caverns"),
            Self::RebelCamp => f.write_str("Rebel Camp"),
            Self::NesingwarysExpedition => f.write_str("Nesingwary's Expedition"),
            Self::KurzensCompound => f.write_str("Kurzen's Compound"),
            Self::RuinsOfZulKunda => f.write_str("Ruins of Zul'Kunda"),
            Self::RuinsOfZulMamwe => f.write_str("Ruins of Zul'Mamwe"),
            Self::TheVileReef => f.write_str("The Vile Reef"),
            Self::MoshOggOgreMound => f.write_str("Mosh'Ogg Ogre Mound"),
            Self::TheStockpile => f.write_str("The Stockpile"),
            Self::SaldeansFarm => f.write_str("Saldean's Farm"),
            Self::SentinelHill => f.write_str("Sentinel Hill"),
            Self::FurlbrowsPumpkinFarm => f.write_str("Furlbrow's Pumpkin Farm"),
            Self::JangolodeMine => f.write_str("Jangolode Mine"),
            Self::GoldCoastQuarry => f.write_str("Gold Coast Quarry"),
            Self::WestfallLighthouse => f.write_str("Westfall Lighthouse"),
            Self::MistyValley => f.write_str("Misty Valley"),
            Self::GromgolBaseCamp => f.write_str("Grom'gol Base Camp"),
            Self::WhelgarsExcavationSite => f.write_str("Whelgar's Excavation Site"),
            Self::WestbrookGarrison => f.write_str("Westbrook Garrison"),
            Self::TranquilGardensCemetery => f.write_str("Tranquil Gardens Cemetery"),
            Self::ZuuldaiaRuins => f.write_str("Zuuldaia Ruins"),
            Self::BallalRuins => f.write_str("Bal'lal Ruins"),
            Self::KalaiRuins => f.write_str("Kal'ai Ruins"),
            Self::TkashiRuins => f.write_str("Tkashi Ruins"),
            Self::BaliamahRuins => f.write_str("Balia'mah Ruins"),
            Self::ZiatajaiRuins => f.write_str("Ziata'jai Ruins"),
            Self::MizjahRuins => f.write_str("Mizjah Ruins"),
            Self::SilverpineForest => f.write_str("Silverpine Forest"),
            Self::Kharanos => f.write_str("Kharanos"),
            Self::ColdridgeValley => f.write_str("Coldridge Valley"),
            Self::Gnomeregan0 => f.write_str("Gnomeregan"),
            Self::GolBolarQuarry => f.write_str("Gol'Bolar Quarry"),
            Self::FrostmaneHold => f.write_str("Frostmane Hold"),
            Self::TheGrizzledDen => f.write_str("The Grizzled Den"),
            Self::BrewnallVillage => f.write_str("Brewnall Village"),
            Self::MistyPineRefuge => f.write_str("Misty Pine Refuge"),
            Self::EasternPlaguelands => f.write_str("Eastern Plaguelands"),
            Self::Teldrassil => f.write_str("Teldrassil"),
            Self::IronbandsExcavationSite => f.write_str("Ironband's Excavation Site"),
            Self::MogroshStronghold => f.write_str("Mo'grosh Stronghold"),
            Self::Thelsamar => f.write_str("Thelsamar"),
            Self::AlgazGate => f.write_str("Algaz Gate"),
            Self::StonewroughtDam => f.write_str("Stonewrought Dam"),
            Self::TheFarstriderLodge => f.write_str("The Farstrider Lodge"),
            Self::Darkshore => f.write_str("Darkshore"),
            Self::SilverStreamMine => f.write_str("Silver Stream Mine"),
            Self::MenethilHarbor => f.write_str("Menethil Harbor"),
            Self::DesignerIsland => f.write_str("Designer Island"),
            Self::TheBulwark0 => f.write_str("The Bulwark"),
            Self::RuinsOfLordaeron => f.write_str("Ruins of Lordaeron"),
            Self::Deathknell => f.write_str("Deathknell"),
            Self::NightWebsHollow => f.write_str("Night Web's Hollow"),
            Self::SollidenFarmstead => f.write_str("Solliden Farmstead"),
            Self::AgamandMills => f.write_str("Agamand Mills"),
            Self::AgamandFamilyCrypt => f.write_str("Agamand Family Crypt"),
            Self::Brill => f.write_str("Brill"),
            Self::WhisperingGardens => f.write_str("Whispering Gardens"),
            Self::TerraceOfRepose => f.write_str("Terrace of Repose"),
            Self::BrightwaterLake => f.write_str("Brightwater Lake"),
            Self::GunthersRetreat => f.write_str("Gunther's Retreat"),
            Self::GarrensHaunt => f.write_str("Garren's Haunt"),
            Self::BalnirFarmstead => f.write_str("Balnir Farmstead"),
            Self::ColdHearthManor => f.write_str("Cold Hearth Manor"),
            Self::CrusaderOutpost => f.write_str("Crusader Outpost"),
            Self::TheNorthCoast => f.write_str("The North Coast"),
            Self::WhisperingShore => f.write_str("Whispering Shore"),
            Self::LordamereLake0 => f.write_str("Lordamere Lake"),
            Self::FenrisIsle => f.write_str("Fenris Isle"),
            Self::FaolsRest => f.write_str("Faol's Rest"),
            Self::Dolanaar => f.write_str("Dolanaar"),
            Self::DarnassusUnused => f.write_str("Darnassus UNUSED"),
            Self::Shadowglen => f.write_str("Shadowglen"),
            Self::SteelgrillsDepot => f.write_str("Steelgrill's Depot"),
            Self::Hearthglen => f.write_str("Hearthglen"),
            Self::NorthridgeLumberCamp => f.write_str("Northridge Lumber Camp"),
            Self::RuinsOfAndorhal => f.write_str("Ruins of Andorhal"),
            Self::SchoolOfNecromancy => f.write_str("School of Necromancy"),
            Self::UthersTomb => f.write_str("Uther's Tomb"),
            Self::SorrowHill => f.write_str("Sorrow Hill"),
            Self::TheWeepingCave => f.write_str("The Weeping Cave"),
            Self::FelstoneField => f.write_str("Felstone Field"),
            Self::DalsonsTears => f.write_str("Dalson's Tears"),
            Self::GahrronsWithering => f.write_str("Gahrron's Withering"),
            Self::TheWrithingHaunt => f.write_str("The Writhing Haunt"),
            Self::MardenholdeKeep => f.write_str("Mardenholde Keep"),
            Self::PyrewoodVillage => f.write_str("Pyrewood Village"),
            Self::DunModr => f.write_str("Dun Modr"),
            Self::Westfall1 => f.write_str("Westfall"),
            Self::TheGreatSea0 => f.write_str("The Great Sea"),
            Self::UnusedIroncladcove => f.write_str("Unused Ironcladcove"),
            Self::ShadowfangKeep0 => f.write_str("Shadowfang Keep"),
            Self::OnMapDungeon3 => f.write_str("***On Map Dungeon***"),
            Self::IceflowLake => f.write_str("Iceflow Lake"),
            Self::HelmsBedLake => f.write_str("Helm's Bed Lake"),
            Self::DeepElemMine => f.write_str("Deep Elem Mine"),
            Self::TheGreatSea1 => f.write_str("The Great Sea"),
            Self::Mulgore => f.write_str("Mulgore"),
            Self::AlexstonFarmstead => f.write_str("Alexston Farmstead"),
            Self::RedCloudMesa => f.write_str("Red Cloud Mesa"),
            Self::CampNarache => f.write_str("Camp Narache"),
            Self::BloodhoofVillage => f.write_str("Bloodhoof Village"),
            Self::StonebullLake => f.write_str("Stonebull Lake"),
            Self::RavagedCaravan => f.write_str("Ravaged Caravan"),
            Self::RedRocks => f.write_str("Red Rocks"),
            Self::TheSkitteringDark => f.write_str("The Skittering Dark"),
            Self::ValgansField => f.write_str("Valgan's Field"),
            Self::TheSepulcher => f.write_str("The Sepulcher"),
            Self::OlsensFarthing => f.write_str("Olsen's Farthing"),
            Self::TheGreymaneWall => f.write_str("The Greymane Wall"),
            Self::BerensPeril => f.write_str("Beren's Peril"),
            Self::TheDawningIsles => f.write_str("The Dawning Isles"),
            Self::Ambermill => f.write_str("Ambermill"),
            Self::FenrisKeep => f.write_str("Fenris Keep"),
            Self::ShadowfangKeep1 => f.write_str("Shadowfang Keep"),
            Self::TheDecrepitFerry => f.write_str("The Decrepit Ferry"),
            Self::MaldensOrchard => f.write_str("Malden's Orchard"),
            Self::TheIvarPatch => f.write_str("The Ivar Patch"),
            Self::TheDeadField => f.write_str("The Dead Field"),
            Self::TheRottingOrchard => f.write_str("The Rotting Orchard"),
            Self::BrightwoodGrove => f.write_str("Brightwood Grove"),
            Self::ForlornRowe => f.write_str("Forlorn Rowe"),
            Self::TheWhippleEstate => f.write_str("The Whipple Estate"),
            Self::TheYorgenFarmstead => f.write_str("The Yorgen Farmstead"),
            Self::TheCauldron => f.write_str("The Cauldron"),
            Self::GrimesiltDigSite => f.write_str("Grimesilt Dig Site"),
            Self::DreadmaulRock => f.write_str("Dreadmaul Rock"),
            Self::RuinsOfThaurissan => f.write_str("Ruins of Thaurissan"),
            Self::FlameCrest => f.write_str("Flame Crest"),
            Self::BlackrockStronghold => f.write_str("Blackrock Stronghold"),
            Self::ThePillarOfAsh => f.write_str("The Pillar of Ash"),
            Self::BlackrockMountain1 => f.write_str("Blackrock Mountain"),
            Self::AltarOfStorms0 => f.write_str("Altar of Storms"),
            Self::Aldrassil => f.write_str("Aldrassil"),
            Self::ShadowthreadCave => f.write_str("Shadowthread Cave"),
            Self::FelRock => f.write_str("Fel Rock"),
            Self::LakeAlAmeth => f.write_str("Lake Al'Ameth"),
            Self::StarbreezeVillage => f.write_str("Starbreeze Village"),
            Self::GnarlpineHold => f.write_str("Gnarlpine Hold"),
            Self::BanethilBarrowDen => f.write_str("Ban'ethil Barrow Den"),
            Self::TheCleft => f.write_str("The Cleft"),
            Self::TheOracleGlade => f.write_str("The Oracle Glade"),
            Self::WellspringRiver => f.write_str("Wellspring River"),
            Self::WellspringLake => f.write_str("Wellspring Lake"),
            Self::HillsbradFoothills => f.write_str("Hillsbrad Foothills"),
            Self::AzsharaCrater => f.write_str("Azshara Crater"),
            Self::DunAlgaz0 => f.write_str("Dun Algaz"),
            Self::Southshore0 => f.write_str("Southshore"),
            Self::TarrenMill0 => f.write_str("Tarren Mill"),
            Self::DurnholdeKeep0 => f.write_str("Durnholde Keep"),
            Self::UnusedStonewroughtPass => f.write_str("UNUSED Stonewrought Pass"),
            Self::TheFoothillCaverns => f.write_str("The Foothill Caverns"),
            Self::LordamereInternmentCamp => f.write_str("Lordamere Internment Camp"),
            Self::Dalaran => f.write_str("Dalaran"),
            Self::Strahnbrad => f.write_str("Strahnbrad"),
            Self::RuinsOfAlterac => f.write_str("Ruins of Alterac"),
            Self::CrushridgeHold => f.write_str("Crushridge Hold"),
            Self::SlaughterHollow => f.write_str("Slaughter Hollow"),
            Self::TheUplands => f.write_str("The Uplands"),
            Self::SouthpointTower0 => f.write_str("Southpoint Tower"),
            Self::HillsbradFields0 => f.write_str("Hillsbrad Fields"),
            Self::Hillsbrad => f.write_str("Hillsbrad"),
            Self::AzurelodeMine0 => f.write_str("Azurelode Mine"),
            Self::NethanderStead0 => f.write_str("Nethander Stead"),
            Self::DunGarok0 => f.write_str("Dun Garok"),
            Self::ThoradinsWall0 => f.write_str("Thoradin's Wall"),
            Self::EasternStrand0 => f.write_str("Eastern Strand"),
            Self::WesternStrand0 => f.write_str("Western Strand"),
            Self::SouthSeasUnused => f.write_str("South Seas UNUSED"),
            Self::JagueroIsle => f.write_str("Jaguero Isle"),
            Self::BaradinBay => f.write_str("Baradin Bay"),
            Self::MenethilBay => f.write_str("Menethil Bay"),
            Self::MistyReedStrand => f.write_str("Misty Reed Strand"),
            Self::TheSavageCoast => f.write_str("The Savage Coast"),
            Self::TheCrystalShore => f.write_str("The Crystal Shore"),
            Self::ShellBeach => f.write_str("Shell Beach"),
            Self::NorthTidesRun => f.write_str("North Tide's Run"),
            Self::SouthTidesRun => f.write_str("South Tide's Run"),
            Self::TheOverlookCliffs => f.write_str("The Overlook Cliffs"),
            Self::TheForbiddingSea0 => f.write_str("The Forbidding Sea"),
            Self::IronbeardsTomb => f.write_str("Ironbeard's Tomb"),
            Self::CrystalveinMine => f.write_str("Crystalvein Mine"),
            Self::RuinsOfAboraz => f.write_str("Ruins of Aboraz"),
            Self::JaneirosPoint => f.write_str("Janeiro's Point"),
            Self::NorthfoldManor => f.write_str("Northfold Manor"),
            Self::GoShekFarm => f.write_str("Go'Shek Farm"),
            Self::DabyriesFarmstead => f.write_str("Dabyrie's Farmstead"),
            Self::BoulderfistHall => f.write_str("Boulderfist Hall"),
            Self::WitherbarkVillage => f.write_str("Witherbark Village"),
            Self::DrywhiskerGorge => f.write_str("Drywhisker Gorge"),
            Self::RefugePointe => f.write_str("Refuge Pointe"),
            Self::Hammerfall => f.write_str("Hammerfall"),
            Self::BlackwaterShipwrecks => f.write_str("Blackwater Shipwrecks"),
            Self::OBreensCamp => f.write_str("O'Breen's Camp"),
            Self::StromgardeKeep => f.write_str("Stromgarde Keep"),
            Self::TheTowerOfArathor => f.write_str("The Tower of Arathor"),
            Self::TheSanctum => f.write_str("The Sanctum"),
            Self::FaldirsCove => f.write_str("Faldir's Cove"),
            Self::TheDrownedReef => f.write_str("The Drowned Reef"),
            Self::ThandolSpan0 => f.write_str("Thandol Span"),
            Self::Ashenvale => f.write_str("Ashenvale"),
            Self::TheGreatSea2 => f.write_str("The Great Sea"),
            Self::CircleOfEastBinding => f.write_str("Circle of East Binding"),
            Self::CircleOfWestBinding => f.write_str("Circle of West Binding"),
            Self::CircleOfInnerBinding => f.write_str("Circle of Inner Binding"),
            Self::CircleOfOuterBinding => f.write_str("Circle of Outer Binding"),
            Self::ApocryphansRest => f.write_str("Apocryphan's Rest"),
            Self::AngorFortress => f.write_str("Angor Fortress"),
            Self::LethlorRavine => f.write_str("Lethlor Ravine"),
            Self::Kargath => f.write_str("Kargath"),
            Self::CampKosh => f.write_str("Camp Kosh"),
            Self::CampBoff => f.write_str("Camp Boff"),
            Self::CampWurg => f.write_str("Camp Wurg"),
            Self::CampCagg => f.write_str("Camp Cagg"),
            Self::AgmondsEnd => f.write_str("Agmond's End"),
            Self::HammertoesDigsite => f.write_str("Hammertoe's Digsite"),
            Self::DustbelchGrotto => f.write_str("Dustbelch Grotto"),
            Self::AeriePeak => f.write_str("Aerie Peak"),
            Self::WildhammerKeep => f.write_str("Wildhammer Keep"),
            Self::QuelDanilLodge => f.write_str("Quel'Danil Lodge"),
            Self::SkulkRock => f.write_str("Skulk Rock"),
            Self::Zunwatha => f.write_str("Zun'watha"),
            Self::ShadraAlor => f.write_str("Shadra'Alor"),
            Self::JinthaAlor => f.write_str("Jintha'Alor"),
            Self::TheAltarOfZul => f.write_str("The Altar of Zul"),
            Self::Seradane => f.write_str("Seradane"),
            Self::Feralas => f.write_str("Feralas"),
            Self::BramblebladeRavine => f.write_str("Brambleblade Ravine"),
            Self::BaelModan => f.write_str("Bael Modan"),
            Self::TheVentureCoMine => f.write_str("The Venture Co. Mine"),
            Self::Felwood => f.write_str("Felwood"),
            Self::RazorHill => f.write_str("Razor Hill"),
            Self::ValleyOfTrials => f.write_str("Valley of Trials"),
            Self::TheDen => f.write_str("The Den"),
            Self::BurningBladeCoven => f.write_str("Burning Blade Coven"),
            Self::KolkarCrag => f.write_str("Kolkar Crag"),
            Self::SenjinVillage => f.write_str("Sen'jin Village"),
            Self::EchoIsles => f.write_str("Echo Isles"),
            Self::ThunderRidge => f.write_str("Thunder Ridge"),
            Self::DrygulchRavine => f.write_str("Drygulch Ravine"),
            Self::DustwindCave => f.write_str("Dustwind Cave"),
            Self::TiragardeKeep => f.write_str("Tiragarde Keep"),
            Self::ScuttleCoast => f.write_str("Scuttle Coast"),
            Self::BladefistBay => f.write_str("Bladefist Bay"),
            Self::DeadeyeShore => f.write_str("Deadeye Shore"),
            Self::SouthfuryRiver0 => f.write_str("Southfury River"),
            Self::CampTaurajo => f.write_str("Camp Taurajo"),
            Self::FarWatchPost => f.write_str("Far Watch Post"),
            Self::TheCrossroads => f.write_str("The Crossroads"),
            Self::BoulderLodeMine => f.write_str("Boulder Lode Mine"),
            Self::TheSludgeFen => f.write_str("The Sludge Fen"),
            Self::TheDryHills => f.write_str("The Dry Hills"),
            Self::DreadmistPeak => f.write_str("Dreadmist Peak"),
            Self::NorthwatchHold => f.write_str("Northwatch Hold"),
            Self::TheForgottenPools => f.write_str("The Forgotten Pools"),
            Self::LushwaterOasis => f.write_str("Lushwater Oasis"),
            Self::TheStagnantOasis => f.write_str("The Stagnant Oasis"),
            Self::FieldOfGiants => f.write_str("Field of Giants"),
            Self::TheMerchantCoast => f.write_str("The Merchant Coast"),
            Self::Ratchet => f.write_str("Ratchet"),
            Self::DarkspearStrand => f.write_str("Darkspear Strand"),
            Self::DarrowmereLakeUnused => f.write_str("Darrowmere Lake UNUSED"),
            Self::CaerDarrowUnused => f.write_str("Caer Darrow UNUSED"),
            Self::WinterhoofWaterWell => f.write_str("Winterhoof Water Well"),
            Self::ThunderhornWaterWell => f.write_str("Thunderhorn Water Well"),
            Self::WildmaneWaterWell => f.write_str("Wildmane Water Well"),
            Self::SkylineRidge => f.write_str("Skyline Ridge"),
            Self::ThousandNeedles => f.write_str("Thousand Needles"),
            Self::TheTidusStair => f.write_str("The Tidus Stair"),
            Self::ShadyRestInn => f.write_str("Shady Rest Inn"),
            Self::BaeldunDigsite => f.write_str("Bael'dun Digsite"),
            Self::Desolace => f.write_str("Desolace"),
            Self::StonetalonMountains => f.write_str("Stonetalon Mountains"),
            Self::OrgrimmarUnused => f.write_str("Orgrimmar UNUSED"),
            Self::GillijimsIsle => f.write_str("Gillijim's Isle"),
            Self::IslandOfDoctorLapidis => f.write_str("Island of Doctor Lapidis"),
            Self::RazorwindCanyon => f.write_str("Razorwind Canyon"),
            Self::BathransHaunt => f.write_str("Bathran's Haunt"),
            Self::TheRuinsOfOrdilAran => f.write_str("The Ruins of Ordil'Aran"),
            Self::MaestrasPost => f.write_str("Maestra's Post"),
            Self::TheZoramStrand => f.write_str("The Zoram Strand"),
            Self::Astranaar => f.write_str("Astranaar"),
            Self::TheShrineOfAessina => f.write_str("The Shrine of Aessina"),
            Self::FireScarShrine => f.write_str("Fire Scar Shrine"),
            Self::TheRuinsOfStardust => f.write_str("The Ruins of Stardust"),
            Self::TheHowlingVale => f.write_str("The Howling Vale"),
            Self::SilverwindRefuge => f.write_str("Silverwind Refuge"),
            Self::MystralLake => f.write_str("Mystral Lake"),
            Self::FallenSkyLake => f.write_str("Fallen Sky Lake"),
            Self::IrisLake => f.write_str("Iris Lake"),
            Self::Moonwell => f.write_str("Moonwell"),
            Self::RaynewoodRetreat => f.write_str("Raynewood Retreat"),
            Self::TheShadyNook => f.write_str("The Shady Nook"),
            Self::NightRun => f.write_str("Night Run"),
            Self::Xavian => f.write_str("Xavian"),
            Self::Satyrnaar => f.write_str("Satyrnaar"),
            Self::SplintertreePost => f.write_str("Splintertree Post"),
            Self::TheDorDanilBarrowDen => f.write_str("The Dor'Danil Barrow Den"),
            Self::FalfarrenRiver => f.write_str("Falfarren River"),
            Self::FelfireHill => f.write_str("Felfire Hill"),
            Self::DemonFallCanyon => f.write_str("Demon Fall Canyon"),
            Self::DemonFallRidge => f.write_str("Demon Fall Ridge"),
            Self::WarsongLumberCamp => f.write_str("Warsong Lumber Camp"),
            Self::BoughShadow => f.write_str("Bough Shadow"),
            Self::TheShimmeringFlats => f.write_str("The Shimmering Flats"),
            Self::Tanaris => f.write_str("Tanaris"),
            Self::LakeFalathim => f.write_str("Lake Falathim"),
            Self::Auberdine => f.write_str("Auberdine"),
            Self::RuinsOfMathystra => f.write_str("Ruins of Mathystra"),
            Self::TowerOfAlthalaxx => f.write_str("Tower of Althalaxx"),
            Self::CliffspringFalls => f.write_str("Cliffspring Falls"),
            Self::BashalAran => f.write_str("Bashal'Aran"),
            Self::AmethAran => f.write_str("Ameth'Aran"),
            Self::GroveOfTheAncients => f.write_str("Grove of the Ancients"),
            Self::TheMastersGlaive => f.write_str("The Master's Glaive"),
            Self::RemtravelsExcavation => f.write_str("Remtravel's Excavation"),
            Self::MistsEdge => f.write_str("Mist's Edge"),
            Self::TheLongWash => f.write_str("The Long Wash"),
            Self::WildbendRiver => f.write_str("Wildbend River"),
            Self::BlackwoodDen => f.write_str("Blackwood Den"),
            Self::CliffspringRiver => f.write_str("Cliffspring River"),
            Self::TheVeiledSea0 => f.write_str("The Veiled Sea"),
            Self::GoldRoad => f.write_str("Gold Road"),
            Self::ScarletWatchPost => f.write_str("Scarlet Watch Post"),
            Self::SunRockRetreat => f.write_str("Sun Rock Retreat"),
            Self::WindshearCrag => f.write_str("Windshear Crag"),
            Self::CragpoolLake => f.write_str("Cragpool Lake"),
            Self::MirkfallonLake => f.write_str("Mirkfallon Lake"),
            Self::TheCharredVale => f.write_str("The Charred Vale"),
            Self::ValleyOfTheBloodfuries => f.write_str("Valley of the Bloodfuries"),
            Self::StonetalonPeak => f.write_str("Stonetalon Peak"),
            Self::TheTalonDen => f.write_str("The Talon Den"),
            Self::GreatwoodVale => f.write_str("Greatwood Vale"),
            Self::ThunderBluffUnused => f.write_str("Thunder Bluff UNUSED"),
            Self::BraveWindMesa => f.write_str("Brave Wind Mesa"),
            Self::FireStoneMesa => f.write_str("Fire Stone Mesa"),
            Self::MantleRock => f.write_str("Mantle Rock"),
            Self::HunterRiseUnused => f.write_str("Hunter Rise UNUSED"),
            Self::SpiritRiseUnused => f.write_str("Spirit RiseUNUSED"),
            Self::ElderRiseUnused => f.write_str("Elder RiseUNUSED"),
            Self::RuinsOfJubuwal => f.write_str("Ruins of Jubuwal"),
            Self::PoolsOfArlithrien => f.write_str("Pools of Arlithrien"),
            Self::TheRustmaulDigSite => f.write_str("The Rustmaul Dig Site"),
            Self::CampEthok => f.write_str("Camp E'thok"),
            Self::SplithoofCrag => f.write_str("Splithoof Crag"),
            Self::Highperch => f.write_str("Highperch"),
            Self::TheScreechingCanyon => f.write_str("The Screeching Canyon"),
            Self::FreewindPost => f.write_str("Freewind Post"),
            Self::TheGreatLift0 => f.write_str("The Great Lift"),
            Self::GalakHold => f.write_str("Galak Hold"),
            Self::RoguefeatherDen => f.write_str("Roguefeather Den"),
            Self::TheWeatheredNook => f.write_str("The Weathered Nook"),
            Self::Thalanaar => f.write_str("Thalanaar"),
            Self::UnGoroCrater => f.write_str("Un'Goro Crater"),
            Self::RazorfenKraul0 => f.write_str("Razorfen Kraul"),
            Self::RavenHillCemetery => f.write_str("Raven Hill Cemetery"),
            Self::Moonglade => f.write_str("Moonglade"),
            Self::DeleteMe0 => f.write_str("DELETE ME"),
            Self::BrackenwallVillage => f.write_str("Brackenwall Village"),
            Self::SwamplightManor => f.write_str("Swamplight Manor"),
            Self::BloodfenBurrow => f.write_str("Bloodfen Burrow"),
            Self::DarkmistCavern => f.write_str("Darkmist Cavern"),
            Self::MogglePoint => f.write_str("Moggle Point"),
            Self::BeezilsWreck => f.write_str("Beezil's Wreck"),
            Self::WitchHill => f.write_str("Witch Hill"),
            Self::SentryPoint => f.write_str("Sentry Point"),
            Self::NorthPointTower => f.write_str("North Point Tower"),
            Self::WestPointTower => f.write_str("West Point Tower"),
            Self::LostPoint => f.write_str("Lost Point"),
            Self::Bluefen => f.write_str("Bluefen"),
            Self::StonemaulRuins => f.write_str("Stonemaul Ruins"),
            Self::TheDenOfFlame => f.write_str("The Den of Flame"),
            Self::TheDragonmurk => f.write_str("The Dragonmurk"),
            Self::Wyrmbog => f.write_str("Wyrmbog"),
            Self::OnyxiasLairUnused => f.write_str("Onyxia's Lair UNUSED"),
            Self::TheramoreIsle => f.write_str("Theramore Isle"),
            Self::FootholdCitadel => f.write_str("Foothold Citadel"),
            Self::IroncladPrison => f.write_str("Ironclad Prison"),
            Self::DustwallowBay => f.write_str("Dustwallow Bay"),
            Self::TidefuryCove => f.write_str("Tidefury Cove"),
            Self::DreadmurkShore => f.write_str("Dreadmurk Shore"),
            Self::AddlesStead => f.write_str("Addle's Stead"),
            Self::FirePlumeRidge => f.write_str("Fire Plume Ridge"),
            Self::LakkariTarPits => f.write_str("Lakkari Tar Pits"),
            Self::TerrorRun => f.write_str("Terror Run"),
            Self::TheSlitheringScar => f.write_str("The Slithering Scar"),
            Self::MarshalsRefuge => f.write_str("Marshal's Refuge"),
            Self::FungalRock => f.write_str("Fungal Rock"),
            Self::GolakkaHotSprings => f.write_str("Golakka Hot Springs"),
            Self::TheLoch => f.write_str("The Loch"),
            Self::BeggarsHaunt => f.write_str("Beggar's Haunt"),
            Self::KodoGraveyard => f.write_str("Kodo Graveyard"),
            Self::GhostWalkerPost => f.write_str("Ghost Walker Post"),
            Self::SartherisStrand => f.write_str("Sar'theris Strand"),
            Self::ThunderAxeFortress => f.write_str("Thunder Axe Fortress"),
            Self::BolgansHole => f.write_str("Bolgan's Hole"),
            Self::MannorocCoven => f.write_str("Mannoroc Coven"),
            Self::Sargeron => f.write_str("Sargeron"),
            Self::MagramVillage => f.write_str("Magram Village"),
            Self::GelkisVillage => f.write_str("Gelkis Village"),
            Self::ValleyOfSpears => f.write_str("Valley of Spears"),
            Self::NijelsPoint => f.write_str("Nijel's Point"),
            Self::KolkarVillage => f.write_str("Kolkar Village"),
            Self::Hyjal => f.write_str("Hyjal"),
            Self::Winterspring => f.write_str("Winterspring"),
            Self::BlackwolfRiver => f.write_str("Blackwolf River"),
            Self::KodoRock => f.write_str("Kodo Rock"),
            Self::HiddenPath => f.write_str("Hidden Path"),
            Self::SpiritRock => f.write_str("Spirit Rock"),
            Self::ShrineOfTheDormantFlame => f.write_str("Shrine of the Dormant Flame"),
            Self::LakeEluneara => f.write_str("Lake Elune'ara"),
            Self::TheHarborage => f.write_str("The Harborage"),
            Self::Outland => f.write_str("Outland"),
            Self::CraftsmensTerraceUnused => f.write_str("Craftsmen's Terrace UNUSED"),
            Self::TradesmensTerraceUnused => f.write_str("Tradesmen's Terrace UNUSED"),
            Self::TheTempleGardensUnused => f.write_str("The Temple Gardens UNUSED"),
            Self::TempleOfEluneUnused => f.write_str("Temple of Elune UNUSED"),
            Self::CenarionEnclaveUnused => f.write_str("Cenarion Enclave UNUSED"),
            Self::WarriorsTerraceUnused => f.write_str("Warrior's Terrace UNUSED"),
            Self::RuttheranVillage => f.write_str("Rut'theran Village"),
            Self::IronbandsCompound => f.write_str("Ironband's Compound"),
            Self::TheStockade => f.write_str("The Stockade"),
            Self::WailingCaverns => f.write_str("Wailing Caverns"),
            Self::BlackfathomDeeps0 => f.write_str("Blackfathom Deeps"),
            Self::FrayIsland => f.write_str("Fray Island"),
            Self::Gnomeregan1 => f.write_str("Gnomeregan"),
            Self::RazorfenDowns0 => f.write_str("Razorfen Downs"),
            Self::BanethilHollow => f.write_str("Ban'ethil Hollow"),
            Self::ScarletMonastery => f.write_str("Scarlet Monastery"),
            Self::JerodsLanding => f.write_str("Jerod's Landing"),
            Self::RidgepointTower => f.write_str("Ridgepoint Tower"),
            Self::TheDarkenedBank => f.write_str("The Darkened Bank"),
            Self::ColdridgePass => f.write_str("Coldridge Pass"),
            Self::ChillBreezeValley => f.write_str("Chill Breeze Valley"),
            Self::ShimmerRidge => f.write_str("Shimmer Ridge"),
            Self::AmberstillRanch => f.write_str("Amberstill Ranch"),
            Self::TheTundridHills => f.write_str("The Tundrid Hills"),
            Self::SouthGatePass0 => f.write_str("South Gate Pass"),
            Self::SouthGateOutpost => f.write_str("South Gate Outpost"),
            Self::NorthGatePass0 => f.write_str("North Gate Pass"),
            Self::NorthGateOutpost => f.write_str("North Gate Outpost"),
            Self::GatesOfIronforge => f.write_str("Gates of Ironforge"),
            Self::StillwaterPond => f.write_str("Stillwater Pond"),
            Self::NightmareVale => f.write_str("Nightmare Vale"),
            Self::VenomwebVale => f.write_str("Venomweb Vale"),
            Self::TheBulwark1 => f.write_str("The Bulwark"),
            Self::SouthfuryRiver1 => f.write_str("Southfury River"),
            Self::SouthfuryRiver2 => f.write_str("Southfury River"),
            Self::RazormaneGrounds => f.write_str("Razormane Grounds"),
            Self::SkullRock => f.write_str("Skull Rock"),
            Self::PalemaneRock => f.write_str("Palemane Rock"),
            Self::WindfuryRidge => f.write_str("Windfury Ridge"),
            Self::TheGoldenPlains => f.write_str("The Golden Plains"),
            Self::TheRollingPlains => f.write_str("The Rolling Plains"),
            Self::DunAlgaz1 => f.write_str("Dun Algaz"),
            Self::DunAlgaz2 => f.write_str("Dun Algaz"),
            Self::NorthGatePass1 => f.write_str("North Gate Pass"),
            Self::SouthGatePass1 => f.write_str("South Gate Pass"),
            Self::TwilightGrove => f.write_str("Twilight Grove"),
            Self::GmIsland => f.write_str("GM Island"),
            Self::DeleteMe1 => f.write_str("Delete ME"),
            Self::SouthfuryRiver3 => f.write_str("Southfury River"),
            Self::SouthfuryRiver4 => f.write_str("Southfury River"),
            Self::ThandolSpan1 => f.write_str("Thandol Span"),
            Self::ThandolSpan2 => f.write_str("Thandol Span"),
            Self::PurgationIsle => f.write_str("Purgation Isle"),
            Self::TheJansenStead => f.write_str("The Jansen Stead"),
            Self::TheDeadAcre => f.write_str("The Dead Acre"),
            Self::TheMolsenFarm => f.write_str("The Molsen Farm"),
            Self::StendelsPond => f.write_str("Stendel's Pond"),
            Self::TheDaggerHills => f.write_str("The Dagger Hills"),
            Self::DemontsPlace => f.write_str("Demont's Place"),
            Self::TheDustPlains => f.write_str("The Dust Plains"),
            Self::StonesplinterValley => f.write_str("Stonesplinter Valley"),
            Self::ValleyOfKings => f.write_str("Valley of Kings"),
            Self::AlgazStation => f.write_str("Algaz Station"),
            Self::BucklebreeFarm => f.write_str("Bucklebree Farm"),
            Self::TheShiningStrand => f.write_str("The Shining Strand"),
            Self::NorthTidesHollow => f.write_str("North Tide's Hollow"),
            Self::GrizzlepawRidge => f.write_str("Grizzlepaw Ridge"),
            Self::TheVerdantFields => f.write_str("The Verdant Fields"),
            Self::Gadgetzan => f.write_str("Gadgetzan"),
            Self::SteamwheedlePort => f.write_str("Steamwheedle Port"),
            Self::ZulFarrak0 => f.write_str("Zul'Farrak"),
            Self::SandsorrowWatch => f.write_str("Sandsorrow Watch"),
            Self::ThistleshrubValley => f.write_str("Thistleshrub Valley"),
            Self::TheGapingChasm => f.write_str("The Gaping Chasm"),
            Self::TheNoxiousLair => f.write_str("The Noxious Lair"),
            Self::DunemaulCompound => f.write_str("Dunemaul Compound"),
            Self::EastmoonRuins => f.write_str("Eastmoon Ruins"),
            Self::WaterspringField => f.write_str("Waterspring Field"),
            Self::ZalashjisDen => f.write_str("Zalashji's Den"),
            Self::LandsEndBeach => f.write_str("Land's End Beach"),
            Self::WavestriderBeach => f.write_str("Wavestrider Beach"),
            Self::Uldum => f.write_str("Uldum"),
            Self::ValleyOfTheWatchers => f.write_str("Valley of the Watchers"),
            Self::GunstansPost => f.write_str("Gunstan's Post"),
            Self::SouthmoonRuins => f.write_str("Southmoon Ruins"),
            Self::RendersCamp => f.write_str("Render's Camp"),
            Self::RendersValley => f.write_str("Render's Valley"),
            Self::RendersRock => f.write_str("Render's Rock"),
            Self::StonewatchTower => f.write_str("Stonewatch Tower"),
            Self::GalardellValley => f.write_str("Galardell Valley"),
            Self::LakeridgeHighway => f.write_str("Lakeridge Highway"),
            Self::ThreeCorners => f.write_str("Three Corners"),
            Self::DireforgeHill => f.write_str("Direforge Hill"),
            Self::RaptorRidge => f.write_str("Raptor Ridge"),
            Self::BlackChannelMarsh => f.write_str("Black Channel Marsh"),
            Self::TheGreenBelt0 => f.write_str("The Green Belt"),
            Self::MosshideFen => f.write_str("Mosshide Fen"),
            Self::ThelgenRock => f.write_str("Thelgen Rock"),
            Self::BluegillMarsh => f.write_str("Bluegill Marsh"),
            Self::SaltsprayGlen => f.write_str("Saltspray Glen"),
            Self::SundownMarsh => f.write_str("Sundown Marsh"),
            Self::TheGreenBelt1 => f.write_str("The Green Belt"),
            Self::AngerfangEncampment => f.write_str("Angerfang Encampment"),
            Self::GrimBatol => f.write_str("Grim Batol"),
            Self::DragonmawGates => f.write_str("Dragonmaw Gates"),
            Self::TheLostFleet => f.write_str("The Lost Fleet"),
            Self::DarrowHill0 => f.write_str("Darrow Hill"),
            Self::ThoradinsWall1 => f.write_str("Thoradin's Wall"),
            Self::WebwinderPath => f.write_str("Webwinder Path"),
            Self::TheHushedBank => f.write_str("The Hushed Bank"),
            Self::ManorMistmantle => f.write_str("Manor Mistmantle"),
            Self::CampMojache => f.write_str("Camp Mojache"),
            Self::GrimtotemCompound => f.write_str("Grimtotem Compound"),
            Self::TheWrithingDeep => f.write_str("The Writhing Deep"),
            Self::WildwindLake => f.write_str("Wildwind Lake"),
            Self::GordunniOutpost => f.write_str("Gordunni Outpost"),
            Self::MokGordun => f.write_str("Mok'Gordun"),
            Self::FeralScarVale => f.write_str("Feral Scar Vale"),
            Self::FrayfeatherHighlands => f.write_str("Frayfeather Highlands"),
            Self::IdlewindLake => f.write_str("Idlewind Lake"),
            Self::TheForgottenCoast => f.write_str("The Forgotten Coast"),
            Self::EastPillar => f.write_str("East Pillar"),
            Self::WestPillar => f.write_str("West Pillar"),
            Self::DreamBough => f.write_str("Dream Bough"),
            Self::JademirLake => f.write_str("Jademir Lake"),
            Self::Oneiros => f.write_str("Oneiros"),
            Self::RuinsOfRavenwind => f.write_str("Ruins of Ravenwind"),
            Self::RageScarHold => f.write_str("Rage Scar Hold"),
            Self::FeathermoonStronghold => f.write_str("Feathermoon Stronghold"),
            Self::RuinsOfSolarsal => f.write_str("Ruins of Solarsal"),
            Self::LowerWildsUnused => f.write_str("Lower Wilds UNUSED"),
            Self::TheTwinColossals => f.write_str("The Twin Colossals"),
            Self::SardorIsle => f.write_str("Sardor Isle"),
            Self::IsleOfDread => f.write_str("Isle of Dread"),
            Self::HighWilderness => f.write_str("High Wilderness"),
            Self::LowerWilds => f.write_str("Lower Wilds"),
            Self::SouthernBarrens => f.write_str("Southern Barrens"),
            Self::SouthernGoldRoad => f.write_str("Southern Gold Road"),
            Self::ZulFarrak1 => f.write_str("Zul'Farrak"),
            Self::UnusedAlcazIsland => f.write_str("UNUSEDAlcaz Island"),
            Self::TimbermawHold0 => f.write_str("Timbermaw Hold"),
            Self::VanndirEncampment => f.write_str("Vanndir Encampment"),
            Self::TestAzshara => f.write_str("TESTAzshara"),
            Self::LegashEncampment => f.write_str("Legash Encampment"),
            Self::ThalassianBaseCamp => f.write_str("Thalassian Base Camp"),
            Self::RuinsOfEldarath => f.write_str("Ruins of Eldarath"),
            Self::HetaerasClutch => f.write_str("Hetaera's Clutch"),
            Self::TempleOfZinMalor => f.write_str("Temple of Zin-Malor"),
            Self::BearsHead => f.write_str("Bear's Head"),
            Self::Ursolan => f.write_str("Ursolan"),
            Self::TempleOfArkkoran => f.write_str("Temple of Arkkoran"),
            Self::BayOfStorms => f.write_str("Bay of Storms"),
            Self::TheShatteredStrand => f.write_str("The Shattered Strand"),
            Self::TowerOfEldara => f.write_str("Tower of Eldara"),
            Self::JaggedReef => f.write_str("Jagged Reef"),
            Self::SouthridgeBeach => f.write_str("Southridge Beach"),
            Self::RavencrestMonument => f.write_str("Ravencrest Monument"),
            Self::ForlornRidge => f.write_str("Forlorn Ridge"),
            Self::LakeMennar => f.write_str("Lake Mennar"),
            Self::ShadowsongShrine => f.write_str("Shadowsong Shrine"),
            Self::HaldarrEncampment => f.write_str("Haldarr Encampment"),
            Self::Valormok => f.write_str("Valormok"),
            Self::TheRuinedReaches => f.write_str("The Ruined Reaches"),
            Self::TheTalondeepPath0 => f.write_str("The Talondeep Path"),
            Self::TheTalondeepPath1 => f.write_str("The Talondeep Path"),
            Self::RocktuskFarm => f.write_str("Rocktusk Farm"),
            Self::JaggedswineFarm => f.write_str("Jaggedswine Farm"),
            Self::RazorfenDowns1 => f.write_str("Razorfen Downs"),
            Self::LostRiggerCove => f.write_str("Lost Rigger Cove"),
            Self::Uldaman0 => f.write_str("Uldaman"),
            Self::LordamereLake1 => f.write_str("Lordamere Lake"),
            Self::LordamereLake2 => f.write_str("Lordamere Lake"),
            Self::GallowsCorner => f.write_str("Gallows' Corner"),
            Self::Silithus => f.write_str("Silithus"),
            Self::EmeraldForest => f.write_str("Emerald Forest"),
            Self::SunkenTemple => f.write_str("Sunken Temple"),
            Self::DreadmaulHold => f.write_str("Dreadmaul Hold"),
            Self::NethergardeKeep => f.write_str("Nethergarde Keep"),
            Self::DreadmaulPost => f.write_str("Dreadmaul Post"),
            Self::SerpentsCoil => f.write_str("Serpent's Coil"),
            Self::AltarOfStorms1 => f.write_str("Altar of Storms"),
            Self::FirewatchRidge => f.write_str("Firewatch Ridge"),
            Self::TheSlagPit => f.write_str("The Slag Pit"),
            Self::TheSeaOfCinders => f.write_str("The Sea of Cinders"),
            Self::BlackrockMountain2 => f.write_str("Blackrock Mountain"),
            Self::ThoriumPoint => f.write_str("Thorium Point"),
            Self::GarrisonArmory => f.write_str("Garrison Armory"),
            Self::TheTempleOfAtalHakkar => f.write_str("The Temple of Atal'Hakkar"),
            Self::Undercity => f.write_str("Undercity"),
            Self::Uldaman1 => f.write_str("Uldaman"),
            Self::NotUsedDeadmines => f.write_str("Not Used Deadmines"),
            Self::StormwindCity => f.write_str("Stormwind City"),
            Self::Ironforge => f.write_str("Ironforge"),
            Self::SplithoofHold => f.write_str("Splithoof Hold"),
            Self::TheCapeOfStranglethorn => f.write_str("The Cape of Stranglethorn"),
            Self::SouthernSavageCoast => f.write_str("Southern Savage Coast"),
            Self::UnusedTheDeadmines002 => f.write_str("Unused The Deadmines 002"),
            Self::UnusedIroncladCove003 => f.write_str("Unused Ironclad Cove 003"),
            Self::TheDeadmines => f.write_str("The Deadmines"),
            Self::IroncladCove => f.write_str("Ironclad Cove"),
            Self::BlackrockSpire => f.write_str("Blackrock Spire"),
            Self::BlackrockDepths => f.write_str("Blackrock Depths"),
            Self::RaptorGroundsUnused => f.write_str("Raptor Grounds UNUSED"),
            Self::GroldomFarmUnused => f.write_str("Grol'dom Farm UNUSED"),
            Self::MorshanBaseCamp => f.write_str("Mor'shan Base Camp"),
            Self::HonorsStandUnused => f.write_str("Honor's Stand UNUSED"),
            Self::BlackthornRidgeUnused => f.write_str("Blackthorn Ridge UNUSED"),
            Self::BramblescarUnused => f.write_str("Bramblescar UNUSED"),
            Self::AgamagorUnused => f.write_str("Agama'gor UNUSED"),
            Self::ValleyOfHeroes => f.write_str("Valley of Heroes"),
            Self::Orgrimmar => f.write_str("Orgrimmar"),
            Self::ThunderBluff => f.write_str("Thunder Bluff"),
            Self::ElderRise => f.write_str("Elder Rise"),
            Self::SpiritRise => f.write_str("Spirit Rise"),
            Self::HunterRise => f.write_str("Hunter Rise"),
            Self::Darnassus => f.write_str("Darnassus"),
            Self::CenarionEnclave => f.write_str("Cenarion Enclave"),
            Self::CraftsmensTerrace => f.write_str("Craftsmen's Terrace"),
            Self::WarriorsTerrace => f.write_str("Warrior's Terrace"),
            Self::TheTempleGardens => f.write_str("The Temple Gardens"),
            Self::TradesmensTerrace => f.write_str("Tradesmen's Terrace"),
            Self::GavinsNaze => f.write_str("Gavin's Naze"),
            Self::SoferasNaze => f.write_str("Sofera's Naze"),
            Self::CorrahnsDagger => f.write_str("Corrahn's Dagger"),
            Self::TheHeadland => f.write_str("The Headland"),
            Self::MistyShore => f.write_str("Misty Shore"),
            Self::DandredsFold => f.write_str("Dandred's Fold"),
            Self::GrowlessCave => f.write_str("Growless Cave"),
            Self::ChillwindPoint => f.write_str("Chillwind Point"),
            Self::RaptorGrounds => f.write_str("Raptor Grounds"),
            Self::Bramblescar => f.write_str("Bramblescar"),
            Self::ThornHill => f.write_str("Thorn Hill"),
            Self::Agamagor => f.write_str("Agama'gor"),
            Self::BlackthornRidge => f.write_str("Blackthorn Ridge"),
            Self::HonorsStand => f.write_str("Honor's Stand"),
            Self::TheMorshanRampart => f.write_str("The Mor'shan Rampart"),
            Self::GroldomFarm => f.write_str("Grol'dom Farm"),
            Self::RazorfenKraul1 => f.write_str("Razorfen Kraul"),
            Self::TheGreatLift1 => f.write_str("The Great Lift"),
            Self::MistvaleValley => f.write_str("Mistvale Valley"),
            Self::NekmaniWellspring => f.write_str("Nek'mani Wellspring"),
            Self::BloodsailCompound => f.write_str("Bloodsail Compound"),
            Self::VentureCoBaseCamp => f.write_str("Venture Co. Base Camp"),
            Self::GurubashiArena => f.write_str("Gurubashi Arena"),
            Self::SpiritDen => f.write_str("Spirit Den"),
            Self::TheCrimsonVeil => f.write_str("The Crimson Veil"),
            Self::TheRiptide => f.write_str("The Riptide"),
            Self::TheDamselsLuck => f.write_str("The Damsel's Luck"),
            Self::VentureCoOperationsCenter => f.write_str("Venture Co. Operations Center"),
            Self::DeadwoodVillage => f.write_str("Deadwood Village"),
            Self::FelpawVillage => f.write_str("Felpaw Village"),
            Self::Jaedenar => f.write_str("Jaedenar"),
            Self::BloodvenomRiver => f.write_str("Bloodvenom River"),
            Self::BloodvenomFalls => f.write_str("Bloodvenom Falls"),
            Self::ShatterScarVale => f.write_str("Shatter Scar Vale"),
            Self::IrontreeWoods => f.write_str("Irontree Woods"),
            Self::IrontreeCavern => f.write_str("Irontree Cavern"),
            Self::TimbermawHold1 => f.write_str("Timbermaw Hold"),
            Self::ShadowHold => f.write_str("Shadow Hold"),
            Self::ShrineOfTheDeceiver => f.write_str("Shrine of the Deceiver"),
            Self::IthariussCave => f.write_str("Itharius's Cave"),
            Self::Sorrowmurk => f.write_str("Sorrowmurk"),
            Self::DraenildurVillage => f.write_str("Draenil'dur Village"),
            Self::SplinterspearJunction => f.write_str("Splinterspear Junction"),
            Self::Stagalbog => f.write_str("Stagalbog"),
            Self::TheShiftingMire => f.write_str("The Shifting Mire"),
            Self::StagalbogCave => f.write_str("Stagalbog Cave"),
            Self::WitherbarkCaverns => f.write_str("Witherbark Caverns"),
            Self::ThoradinsWall2 => f.write_str("Thoradin's Wall"),
            Self::Bouldergor => f.write_str("Boulder'gor"),
            Self::ValleyOfFangs => f.write_str("Valley of Fangs"),
            Self::TheDustbowl => f.write_str("The Dustbowl"),
            Self::MirageFlats => f.write_str("Mirage Flats"),
            Self::FeatherbeardsHovel => f.write_str("Featherbeard's Hovel"),
            Self::ShindiggersCamp => f.write_str("Shindigger's Camp"),
            Self::PlaguemistRavine => f.write_str("Plaguemist Ravine"),
            Self::ValorwindLake => f.write_str("Valorwind Lake"),
            Self::Agolwatha => f.write_str("Agol'watha"),
            Self::Hiriwatha => f.write_str("Hiri'watha"),
            Self::TheCreepingRuin => f.write_str("The Creeping Ruin"),
            Self::BogensLedge => f.write_str("Bogen's Ledge"),
            Self::TheMakersTerrace => f.write_str("The Maker's Terrace"),
            Self::DustwindGulch => f.write_str("Dustwind Gulch"),
            Self::Shaolwatha => f.write_str("Shaol'watha"),
            Self::NoonshadeRuins => f.write_str("Noonshade Ruins"),
            Self::BrokenPillar => f.write_str("Broken Pillar"),
            Self::AbyssalSands => f.write_str("Abyssal Sands"),
            Self::SouthbreakShore => f.write_str("Southbreak Shore"),
            Self::CavernsOfTime0 => f.write_str("Caverns of Time"),
            Self::TheMarshlands => f.write_str("The Marshlands"),
            Self::IronstonePlateau => f.write_str("Ironstone Plateau"),
            Self::BlackcharCave => f.write_str("Blackchar Cave"),
            Self::TannerCamp => f.write_str("Tanner Camp"),
            Self::DustfireValley => f.write_str("Dustfire Valley"),
            Self::ZulGurub1 => f.write_str("Zul'Gurub"),
            Self::MistyReedPost => f.write_str("Misty Reed Post"),
            Self::BloodvenomPost => f.write_str("Bloodvenom Post"),
            Self::TalonbranchGlade => f.write_str("Talonbranch Glade"),
            Self::Stratholme0 => f.write_str("Stratholme"),
            Self::UnusedShadowfangKeep003 => f.write_str("UNUSEDShadowfang Keep 003"),
            Self::Scholomance => f.write_str("Scholomance"),
            Self::TwilightVale => f.write_str("Twilight Vale"),
            Self::TwilightShore => f.write_str("Twilight Shore"),
            Self::AlcazIsland => f.write_str("Alcaz Island"),
            Self::DarkcloudPinnacle => f.write_str("Darkcloud Pinnacle"),
            Self::DawningWoodCatacombs => f.write_str("Dawning Wood Catacombs"),
            Self::StonewatchKeep => f.write_str("Stonewatch Keep"),
            Self::Maraudon => f.write_str("Maraudon"),
            Self::StoutlagerInn => f.write_str("Stoutlager Inn"),
            Self::ThunderbrewDistillery => f.write_str("Thunderbrew Distillery"),
            Self::MenethilKeep => f.write_str("Menethil Keep"),
            Self::DeepwaterTavern => f.write_str("Deepwater Tavern"),
            Self::ShadowGrave => f.write_str("Shadow Grave"),
            Self::BrillTownHall => f.write_str("Brill Town Hall"),
            Self::GallowsEndTavern => f.write_str("Gallows' End Tavern"),
            Self::ThePoolsOfVisionUnused => f.write_str("The Pools of VisionUNUSED"),
            Self::DreadmistDen => f.write_str("Dreadmist Den"),
            Self::BaeldunKeep => f.write_str("Bael'dun Keep"),
            Self::EmberstrifesDen => f.write_str("Emberstrife's Den"),
            Self::OnyxiasLair => f.write_str("Onyxia's Lair"),
            Self::WindshearMine => f.write_str("Windshear Mine"),
            Self::RolandsDoom => f.write_str("Roland's Doom"),
            Self::BattleRing => f.write_str("Battle Ring"),
            Self::ThePoolsOfVision => f.write_str("The Pools of Vision"),
            Self::ShadowbreakRavine => f.write_str("Shadowbreak Ravine"),
            Self::BrokenSpearVillage => f.write_str("Broken Spear Village"),
            Self::WhitereachPost => f.write_str("Whitereach Post"),
            Self::Gornia => f.write_str("Gornia"),
            Self::ZanesEyeCrater => f.write_str("Zane's Eye Crater"),
            Self::MirageRaceway => f.write_str("Mirage Raceway"),
            Self::FrostsaberRock => f.write_str("Frostsaber Rock"),
            Self::TheHiddenGrove => f.write_str("The Hidden Grove"),
            Self::TimbermawPost => f.write_str("Timbermaw Post"),
            Self::WinterfallVillage => f.write_str("Winterfall Village"),
            Self::Mazthoril => f.write_str("Mazthoril"),
            Self::FrostfireHotSprings => f.write_str("Frostfire Hot Springs"),
            Self::IceThistleHills => f.write_str("Ice Thistle Hills"),
            Self::DunMandarr => f.write_str("Dun Mandarr"),
            Self::FrostwhisperGorge => f.write_str("Frostwhisper Gorge"),
            Self::OwlWingThicket => f.write_str("Owl Wing Thicket"),
            Self::LakeKelTheril => f.write_str("Lake Kel'Theril"),
            Self::TheRuinsOfKelTheril => f.write_str("The Ruins of Kel'Theril"),
            Self::StarfallVillage => f.write_str("Starfall Village"),
            Self::BanThallowBarrowDen => f.write_str("Ban'Thallow Barrow Den"),
            Self::Everlook => f.write_str("Everlook"),
            Self::DarkwhisperGorge => f.write_str("Darkwhisper Gorge"),
            Self::DeeprunTram => f.write_str("Deeprun Tram"),
            Self::TheFungalVale => f.write_str("The Fungal Vale"),
            Self::UnusedTheMarrisStead => f.write_str("UNUSEDThe Marris Stead"),
            Self::TheMarrisStead => f.write_str("The Marris Stead"),
            Self::TheUndercroft => f.write_str("The Undercroft"),
            Self::Darrowshire => f.write_str("Darrowshire"),
            Self::CrownGuardTower => f.write_str("Crown Guard Tower"),
            Self::CorinsCrossing => f.write_str("Corin's Crossing"),
            Self::ScarletBaseCamp => f.write_str("Scarlet Base Camp"),
            Self::TyrsHand => f.write_str("Tyr's Hand"),
            Self::TheScarletBasilica => f.write_str("The Scarlet Basilica"),
            Self::LightsHopeChapel => f.write_str("Light's Hope Chapel"),
            Self::BrowmanMill => f.write_str("Browman Mill"),
            Self::TheNoxiousGlade => f.write_str("The Noxious Glade"),
            Self::EastwallTower => f.write_str("Eastwall Tower"),
            Self::Northdale => f.write_str("Northdale"),
            Self::ZulMashar => f.write_str("Zul'Mashar"),
            Self::MazraAlor => f.write_str("Mazra'Alor"),
            Self::NorthpassTower => f.write_str("Northpass Tower"),
            Self::QuelLithienLodge => f.write_str("Quel'Lithien Lodge"),
            Self::Plaguewood => f.write_str("Plaguewood"),
            Self::Scourgehold => f.write_str("Scourgehold"),
            Self::Stratholme1 => f.write_str("Stratholme"),
            Self::UnusedStratholme => f.write_str("UNUSED Stratholme"),
            Self::DarrowmereLake0 => f.write_str("Darrowmere Lake"),
            Self::CaerDarrow => f.write_str("Caer Darrow"),
            Self::DarrowmereLake1 => f.write_str("Darrowmere Lake"),
            Self::CavernsOfTime1 => f.write_str("Caverns of Time"),
            Self::ThistlefurVillage => f.write_str("Thistlefur Village"),
            Self::TheQuagmire => f.write_str("The Quagmire"),
            Self::WindbreakCanyon => f.write_str("Windbreak Canyon"),
            Self::SouthSeas0 => f.write_str("South Seas"),
            Self::TheGreatSea3 => f.write_str("The Great Sea"),
            Self::TheGreatSea4 => f.write_str("The Great Sea"),
            Self::TheGreatSea5 => f.write_str("The Great Sea"),
            Self::TheGreatSea6 => f.write_str("The Great Sea"),
            Self::TheVeiledSea1 => f.write_str("The Veiled Sea"),
            Self::TheVeiledSea2 => f.write_str("The Veiled Sea"),
            Self::TheVeiledSea3 => f.write_str("The Veiled Sea"),
            Self::TheVeiledSea4 => f.write_str("The Veiled Sea"),
            Self::TheVeiledSea5 => f.write_str("The Veiled Sea"),
            Self::RazorHillBarracks => f.write_str("Razor Hill Barracks"),
            Self::SouthSeas1 => f.write_str("South Seas"),
            Self::TheGreatSea7 => f.write_str("The Great Sea"),
            Self::BloodtoothCamp => f.write_str("Bloodtooth Camp"),
            Self::ForestSong => f.write_str("Forest Song"),
            Self::GreenpawVillage => f.write_str("Greenpaw Village"),
            Self::SilverwingOutpost => f.write_str("Silverwing Outpost"),
            Self::Nighthaven => f.write_str("Nighthaven"),
            Self::ShrineOfRemulos => f.write_str("Shrine of Remulos"),
            Self::StormrageBarrowDens => f.write_str("Stormrage Barrow Dens"),
            Self::TheGreatSea8 => f.write_str("The Great Sea"),
            Self::TheGreatSea9 => f.write_str("The Great Sea"),
            Self::TheBlackMorass => f.write_str("The Black Morass"),
            Self::OldHillsbradFoothills => f.write_str("Old Hillsbrad Foothills"),
            Self::TarrenMill1 => f.write_str("Tarren Mill"),
            Self::Southshore1 => f.write_str("Southshore"),
            Self::DurnholdeKeep1 => f.write_str("Durnholde Keep"),
            Self::DunGarok1 => f.write_str("Dun Garok"),
            Self::HillsbradFields1 => f.write_str("Hillsbrad Fields"),
            Self::EasternStrand1 => f.write_str("Eastern Strand"),
            Self::NethanderStead1 => f.write_str("Nethander Stead"),
            Self::DarrowHill1 => f.write_str("Darrow Hill"),
            Self::SouthpointTower1 => f.write_str("Southpoint Tower"),
            Self::ThoradinsWall3 => f.write_str("Thoradin's Wall"),
            Self::WesternStrand1 => f.write_str("Western Strand"),
            Self::AzurelodeMine1 => f.write_str("Azurelode Mine"),
            Self::TheGreatSea10 => f.write_str("The Great Sea"),
            Self::TheGreatSea11 => f.write_str("The Great Sea"),
            Self::TheGreatSea12 => f.write_str("The Great Sea"),
            Self::TheForbiddingSea1 => f.write_str("The Forbidding Sea"),
            Self::TheForbiddingSea2 => f.write_str("The Forbidding Sea"),
            Self::TheForbiddingSea3 => f.write_str("The Forbidding Sea"),
            Self::TheForbiddingSea4 => f.write_str("The Forbidding Sea"),
            Self::TethrisAran => f.write_str("Tethris Aran"),
            Self::EthelRethor => f.write_str("Ethel Rethor"),
            Self::RanazjarIsle => f.write_str("Ranazjar Isle"),
            Self::KormeksHut => f.write_str("Kormek's Hut"),
            Self::ShadowpreyVillage => f.write_str("Shadowprey Village"),
            Self::BlackrockPass => f.write_str("Blackrock Pass"),
            Self::MorgansVigil => f.write_str("Morgan's Vigil"),
            Self::SlitherRock => f.write_str("Slither Rock"),
            Self::TerrorWingPath => f.write_str("Terror Wing Path"),
            Self::Dracodar => f.write_str("Draco'dar"),
            Self::RagefireChasm => f.write_str("Ragefire Chasm"),
            Self::NightsongWoods => f.write_str("Nightsong Woods"),
            Self::TheVeiledSea6 => f.write_str("The Veiled Sea"),
            Self::MorlosAran => f.write_str("Morlos'Aran"),
            Self::EmeraldSanctuary => f.write_str("Emerald Sanctuary"),
            Self::JadefireGlen => f.write_str("Jadefire Glen"),
            Self::RuinsOfConstellas => f.write_str("Ruins of Constellas"),
            Self::BitterReaches => f.write_str("Bitter Reaches"),
            Self::RiseOfTheDefiler => f.write_str("Rise of the Defiler"),
            Self::LarissPavilion => f.write_str("Lariss Pavilion"),
            Self::WoodpawHills => f.write_str("Woodpaw Hills"),
            Self::WoodpawDen => f.write_str("Woodpaw Den"),
            Self::VerdantisRiver => f.write_str("Verdantis River"),
            Self::RuinsOfIsildien => f.write_str("Ruins of Isildien"),
            Self::GrimtotemPost => f.write_str("Grimtotem Post"),
            Self::CampAparaje => f.write_str("Camp Aparaje"),
            Self::Malakajin => f.write_str("Malaka'jin"),
            Self::BoulderslideRavine => f.write_str("Boulderslide Ravine"),
            Self::SishirCanyon => f.write_str("Sishir Canyon"),
            Self::DireMaul0 => f.write_str("Dire Maul"),
            Self::DeadwindRavine => f.write_str("Deadwind Ravine"),
            Self::DiamondheadRiver => f.write_str("Diamondhead River"),
            Self::AridensCamp => f.write_str("Ariden's Camp"),
            Self::TheVice => f.write_str("The Vice"),
            Self::Karazhan => f.write_str("Karazhan"),
            Self::MorgansPlot => f.write_str("Morgan's Plot"),
            Self::DireMaul1 => f.write_str("Dire Maul"),
            Self::AlteracValley0 => f.write_str("Alterac Valley"),
            Self::ScrabblescrewsCamp => f.write_str("Scrabblescrew's Camp"),
            Self::JadefireRun => f.write_str("Jadefire Run"),
            Self::ThondrorilRiver0 => f.write_str("Thondroril River"),
            Self::ThondrorilRiver1 => f.write_str("Thondroril River"),
            Self::LakeMereldar => f.write_str("Lake Mereldar"),
            Self::PestilentScar => f.write_str("Pestilent Scar"),
            Self::TheInfectisScar => f.write_str("The Infectis Scar"),
            Self::BlackwoodLake => f.write_str("Blackwood Lake"),
            Self::EastwallGate => f.write_str("Eastwall Gate"),
            Self::TerrorwebTunnel => f.write_str("Terrorweb Tunnel"),
            Self::Terrordale => f.write_str("Terrordale"),
            Self::KargathiaKeep => f.write_str("Kargathia Keep"),
            Self::ValleyOfBones => f.write_str("Valley of Bones"),
            Self::BlackwingLair => f.write_str("Blackwing Lair"),
            Self::DeadmansCrossing => f.write_str("Deadman's Crossing"),
            Self::MoltenCore => f.write_str("Molten Core"),
            Self::TheScarabWall => f.write_str("The Scarab Wall"),
            Self::SouthwindVillage => f.write_str("Southwind Village"),
            Self::TwilightBaseCamp => f.write_str("Twilight Base Camp"),
            Self::TheCrystalVale => f.write_str("The Crystal Vale"),
            Self::TheScarabDais => f.write_str("The Scarab Dais"),
            Self::HiveAshi => f.write_str("Hive'Ashi"),
            Self::HiveZora => f.write_str("Hive'Zora"),
            Self::HiveRegal => f.write_str("Hive'Regal"),
            Self::ShrineOfTheFallenWarrior => f.write_str("Shrine of the Fallen Warrior"),
            Self::UnusedAlteracValley => f.write_str("UNUSED Alterac Valley"),
            Self::BlackfathomDeeps1 => f.write_str("Blackfathom Deeps"),
            Self::OnMapDungeon4 => f.write_str("***On Map Dungeon***"),
            Self::TheMastersCellar => f.write_str("The Master's Cellar"),
            Self::StonewroughtPass => f.write_str("Stonewrought Pass"),
            Self::AlteracValley1 => f.write_str("Alterac Valley"),
            Self::TheRumbleCage => f.write_str("The Rumble Cage"),
            Self::ChunkTest => f.write_str("Chunk Test"),
            Self::ZoramgarOutpost => f.write_str("Zoram'gar Outpost"),
            Self::HallOfLegends => f.write_str("Hall of Legends"),
            Self::ChampionsHall => f.write_str("Champions' Hall"),
            Self::GroshgokCompound => f.write_str("Grosh'gok Compound"),
            Self::SleepingGorge => f.write_str("Sleeping Gorge"),
            Self::IrondeepMine => f.write_str("Irondeep Mine"),
            Self::StonehearthOutpost => f.write_str("Stonehearth Outpost"),
            Self::DunBaldar => f.write_str("Dun Baldar"),
            Self::IcewingPass => f.write_str("Icewing Pass"),
            Self::FrostwolfVillage => f.write_str("Frostwolf Village"),
            Self::TowerPoint => f.write_str("Tower Point"),
            Self::ColdtoothMine => f.write_str("Coldtooth Mine"),
            Self::WinteraxHold => f.write_str("Winterax Hold"),
            Self::IcebloodGarrison => f.write_str("Iceblood Garrison"),
            Self::FrostwolfKeep => f.write_str("Frostwolf Keep"),
            Self::TorkrenFarm => f.write_str("Tor'kren Farm"),
            Self::FrostDaggerPass => f.write_str("Frost Dagger Pass"),
            Self::IronstoneCamp => f.write_str("Ironstone Camp"),
            Self::WeazelsCrater => f.write_str("Weazel's Crater"),
            Self::TahondaRuins => f.write_str("Tahonda Ruins"),
            Self::FieldOfStrife => f.write_str("Field of Strife"),
            Self::IcewingCavern => f.write_str("Icewing Cavern"),
            Self::ValorsRest => f.write_str("Valor's Rest"),
            Self::TheSwarmingPillar => f.write_str("The Swarming Pillar"),
            Self::TwilightPost => f.write_str("Twilight Post"),
            Self::TwilightOutpost => f.write_str("Twilight Outpost"),
            Self::RavagedTwilightCamp => f.write_str("Ravaged Twilight Camp"),
            Self::ShalzarusLair => f.write_str("Shalzaru's Lair"),
            Self::TalrendisPoint => f.write_str("Talrendis Point"),
            Self::RethressSanctum => f.write_str("Rethress Sanctum"),
            Self::MoonHorrorDen => f.write_str("Moon Horror Den"),
            Self::ScalebeardsCave => f.write_str("Scalebeard's Cave"),
            Self::BoulderslideCavern => f.write_str("Boulderslide Cavern"),
            Self::WarsongLaborCamp => f.write_str("Warsong Labor Camp"),
            Self::ChillwindCamp => f.write_str("Chillwind Camp"),
            Self::TheMaul => f.write_str("The Maul"),
            Self::TheMaulUnused => f.write_str("The Maul UNUSED"),
            Self::BonesOfGrakkarond => f.write_str("Bones of Grakkarond"),
            Self::WarsongGulch => f.write_str("Warsong Gulch"),
            Self::FrostwolfGraveyard => f.write_str("Frostwolf Graveyard"),
            Self::FrostwolfPass => f.write_str("Frostwolf Pass"),
            Self::DunBaldarPass => f.write_str("Dun Baldar Pass"),
            Self::IcebloodGraveyard => f.write_str("Iceblood Graveyard"),
            Self::SnowfallGraveyard => f.write_str("Snowfall Graveyard"),
            Self::StonehearthGraveyard => f.write_str("Stonehearth Graveyard"),
            Self::StormpikeGraveyard => f.write_str("Stormpike Graveyard"),
            Self::IcewingBunker => f.write_str("Icewing Bunker"),
            Self::StonehearthBunker => f.write_str("Stonehearth Bunker"),
            Self::WildpawRidge => f.write_str("Wildpaw Ridge"),
            Self::RevantuskVillage => f.write_str("Revantusk Village"),
            Self::RockOfDurotan => f.write_str("Rock of Durotan"),
            Self::SilverwingGrove => f.write_str("Silverwing Grove"),
            Self::WarsongLumberMill => f.write_str("Warsong Lumber Mill"),
            Self::SilverwingHold => f.write_str("Silverwing Hold"),
            Self::WildpawCavern => f.write_str("Wildpaw Cavern"),
            Self::TheVeiledCleft => f.write_str("The Veiled Cleft"),
            Self::YojambaIsle => f.write_str("Yojamba Isle"),
            Self::ArathiBasin => f.write_str("Arathi Basin"),
            Self::TheCoil => f.write_str("The Coil"),
            Self::AltarOfHireek => f.write_str("Altar of Hir'eek"),
            Self::Shadrazaar => f.write_str("Shadra'zaar"),
            Self::HakkariGrounds => f.write_str("Hakkari Grounds"),
            Self::NazeOfShirvallah => f.write_str("Naze of Shirvallah"),
            Self::TempleOfBethekk => f.write_str("Temple of Bethekk"),
            Self::TheBloodfirePit => f.write_str("The Bloodfire Pit"),
            Self::AltarOfTheBloodGod => f.write_str("Altar of the Blood God"),
            Self::ZanzasRise => f.write_str("Zanza's Rise"),
            Self::EdgeOfMadness => f.write_str("Edge of Madness"),
            Self::TrollbaneHall => f.write_str("Trollbane Hall"),
            Self::DefilersDen => f.write_str("Defiler's Den"),
            Self::PaglesPointe => f.write_str("Pagle's Pointe"),
            Self::Farm => f.write_str("Farm"),
            Self::Blacksmith => f.write_str("Blacksmith"),
            Self::LumberMill => f.write_str("Lumber Mill"),
            Self::GoldMine => f.write_str("Gold Mine"),
            Self::Stables => f.write_str("Stables"),
            Self::CenarionHold => f.write_str("Cenarion Hold"),
            Self::StaghelmPoint => f.write_str("Staghelm Point"),
            Self::BronzebeardEncampment => f.write_str("Bronzebeard Encampment"),
            Self::AhnQiraj => f.write_str("Ahn'Qiraj"),
            Self::RuinsOfAhnQiraj0 => f.write_str("Ruins of Ahn'Qiraj"),
            Self::TwilightsRun => f.write_str("Twilight's Run"),
            Self::OrtellsHideout => f.write_str("Ortell's Hideout"),
            Self::ScarabTerrace => f.write_str("Scarab Terrace"),
            Self::GeneralsTerrace => f.write_str("General's Terrace"),
            Self::TheReservoir => f.write_str("The Reservoir"),
            Self::TheHatchery => f.write_str("The Hatchery"),
            Self::TheComb => f.write_str("The Comb"),
            Self::WatchersTerrace => f.write_str("Watchers' Terrace"),
            Self::RuinsOfAhnQiraj1 => f.write_str("Ruins of Ahn'Qiraj"),
            Self::Naxxramas => f.write_str("Naxxramas"),
            Self::City => f.write_str("City"),
            Self::GatesOfAhnQiraj => f.write_str("Gates of Ahn'Qiraj"),
            Self::RavenholdtManor => f.write_str("Ravenholdt Manor"),
        }
    }
}

impl TryFrom<u32> for Area {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::DunMorogh),
            2 => Ok(Self::Longshore),
            3 => Ok(Self::Badlands),
            4 => Ok(Self::BlastedLands),
            7 => Ok(Self::BlackwaterCove),
            8 => Ok(Self::SwampOfSorrows),
            9 => Ok(Self::NorthshireValley),
            10 => Ok(Self::Duskwood),
            11 => Ok(Self::Wetlands),
            12 => Ok(Self::ElwynnForest),
            13 => Ok(Self::TheWorldTree),
            14 => Ok(Self::Durotar),
            15 => Ok(Self::DustwallowMarsh),
            16 => Ok(Self::Azshara),
            17 => Ok(Self::TheBarrens),
            18 => Ok(Self::CrystalLake),
            19 => Ok(Self::ZulGurub0),
            20 => Ok(Self::Moonbrook),
            21 => Ok(Self::KulTiras),
            22 => Ok(Self::ProgrammerIsle),
            23 => Ok(Self::NorthshireRiver),
            24 => Ok(Self::NorthshireAbbey),
            25 => Ok(Self::BlackrockMountain0),
            26 => Ok(Self::Lighthouse),
            28 => Ok(Self::WesternPlaguelands),
            30 => Ok(Self::Nine),
            32 => Ok(Self::TheCemetary),
            33 => Ok(Self::StranglethornVale),
            34 => Ok(Self::EchoRidgeMine),
            35 => Ok(Self::BootyBay),
            36 => Ok(Self::AlteracMountains),
            37 => Ok(Self::LakeNazferiti),
            38 => Ok(Self::LochModan),
            40 => Ok(Self::Westfall0),
            41 => Ok(Self::DeadwindPass),
            42 => Ok(Self::Darkshire),
            43 => Ok(Self::WildShore),
            44 => Ok(Self::RedridgeMountains),
            45 => Ok(Self::ArathiHighlands),
            46 => Ok(Self::BurningSteppes),
            47 => Ok(Self::TheHinterlands),
            49 => Ok(Self::DeadMansHole),
            51 => Ok(Self::SearingGorge),
            53 => Ok(Self::ThievesCamp),
            54 => Ok(Self::JasperlodeMine),
            55 => Ok(Self::ValleyOfHeroesUnused),
            56 => Ok(Self::HeroesVigil),
            57 => Ok(Self::FargodeepMine),
            59 => Ok(Self::NorthshireVineyards),
            60 => Ok(Self::ForestsEdge),
            61 => Ok(Self::ThunderFalls),
            62 => Ok(Self::BrackwellPumpkinPatch),
            63 => Ok(Self::TheStonefieldFarm),
            64 => Ok(Self::TheMaclureVineyards),
            65 => Ok(Self::OnMapDungeon0),
            66 => Ok(Self::OnMapDungeon1),
            67 => Ok(Self::OnMapDungeon2),
            68 => Ok(Self::LakeEverstill),
            69 => Ok(Self::Lakeshire),
            70 => Ok(Self::Stonewatch),
            71 => Ok(Self::StonewatchFalls),
            72 => Ok(Self::TheDarkPortal),
            73 => Ok(Self::TheTaintedScar),
            74 => Ok(Self::PoolOfTears),
            75 => Ok(Self::Stonard),
            76 => Ok(Self::FallowSanctuary),
            77 => Ok(Self::Anvilmar),
            80 => Ok(Self::StormwindMountains),
            81 => Ok(Self::JeffNeQuadrantChanged),
            82 => Ok(Self::JeffNwQuadrant),
            83 => Ok(Self::JeffSeQuadrant),
            84 => Ok(Self::JeffSwQuadrant),
            85 => Ok(Self::TirisfalGlades),
            86 => Ok(Self::StoneCairnLake),
            87 => Ok(Self::Goldshire),
            88 => Ok(Self::EastvaleLoggingCamp),
            89 => Ok(Self::MirrorLakeOrchard),
            91 => Ok(Self::TowerOfAzora),
            92 => Ok(Self::MirrorLake),
            93 => Ok(Self::VulGolOgreMound),
            94 => Ok(Self::RavenHill),
            95 => Ok(Self::RedridgeCanyons),
            96 => Ok(Self::TowerOfIlgalar),
            97 => Ok(Self::AlthersMill),
            98 => Ok(Self::RethbanCaverns),
            99 => Ok(Self::RebelCamp),
            100 => Ok(Self::NesingwarysExpedition),
            101 => Ok(Self::KurzensCompound),
            102 => Ok(Self::RuinsOfZulKunda),
            103 => Ok(Self::RuinsOfZulMamwe),
            104 => Ok(Self::TheVileReef),
            105 => Ok(Self::MoshOggOgreMound),
            106 => Ok(Self::TheStockpile),
            107 => Ok(Self::SaldeansFarm),
            108 => Ok(Self::SentinelHill),
            109 => Ok(Self::FurlbrowsPumpkinFarm),
            111 => Ok(Self::JangolodeMine),
            113 => Ok(Self::GoldCoastQuarry),
            115 => Ok(Self::WestfallLighthouse),
            116 => Ok(Self::MistyValley),
            117 => Ok(Self::GromgolBaseCamp),
            118 => Ok(Self::WhelgarsExcavationSite),
            120 => Ok(Self::WestbrookGarrison),
            121 => Ok(Self::TranquilGardensCemetery),
            122 => Ok(Self::ZuuldaiaRuins),
            123 => Ok(Self::BallalRuins),
            125 => Ok(Self::KalaiRuins),
            126 => Ok(Self::TkashiRuins),
            127 => Ok(Self::BaliamahRuins),
            128 => Ok(Self::ZiatajaiRuins),
            129 => Ok(Self::MizjahRuins),
            130 => Ok(Self::SilverpineForest),
            131 => Ok(Self::Kharanos),
            132 => Ok(Self::ColdridgeValley),
            133 => Ok(Self::Gnomeregan0),
            134 => Ok(Self::GolBolarQuarry),
            135 => Ok(Self::FrostmaneHold),
            136 => Ok(Self::TheGrizzledDen),
            137 => Ok(Self::BrewnallVillage),
            138 => Ok(Self::MistyPineRefuge),
            139 => Ok(Self::EasternPlaguelands),
            141 => Ok(Self::Teldrassil),
            142 => Ok(Self::IronbandsExcavationSite),
            143 => Ok(Self::MogroshStronghold),
            144 => Ok(Self::Thelsamar),
            145 => Ok(Self::AlgazGate),
            146 => Ok(Self::StonewroughtDam),
            147 => Ok(Self::TheFarstriderLodge),
            148 => Ok(Self::Darkshore),
            149 => Ok(Self::SilverStreamMine),
            150 => Ok(Self::MenethilHarbor),
            151 => Ok(Self::DesignerIsland),
            152 => Ok(Self::TheBulwark0),
            153 => Ok(Self::RuinsOfLordaeron),
            154 => Ok(Self::Deathknell),
            155 => Ok(Self::NightWebsHollow),
            156 => Ok(Self::SollidenFarmstead),
            157 => Ok(Self::AgamandMills),
            158 => Ok(Self::AgamandFamilyCrypt),
            159 => Ok(Self::Brill),
            160 => Ok(Self::WhisperingGardens),
            161 => Ok(Self::TerraceOfRepose),
            162 => Ok(Self::BrightwaterLake),
            163 => Ok(Self::GunthersRetreat),
            164 => Ok(Self::GarrensHaunt),
            165 => Ok(Self::BalnirFarmstead),
            166 => Ok(Self::ColdHearthManor),
            167 => Ok(Self::CrusaderOutpost),
            168 => Ok(Self::TheNorthCoast),
            169 => Ok(Self::WhisperingShore),
            170 => Ok(Self::LordamereLake0),
            172 => Ok(Self::FenrisIsle),
            173 => Ok(Self::FaolsRest),
            186 => Ok(Self::Dolanaar),
            187 => Ok(Self::DarnassusUnused),
            188 => Ok(Self::Shadowglen),
            189 => Ok(Self::SteelgrillsDepot),
            190 => Ok(Self::Hearthglen),
            192 => Ok(Self::NorthridgeLumberCamp),
            193 => Ok(Self::RuinsOfAndorhal),
            195 => Ok(Self::SchoolOfNecromancy),
            196 => Ok(Self::UthersTomb),
            197 => Ok(Self::SorrowHill),
            198 => Ok(Self::TheWeepingCave),
            199 => Ok(Self::FelstoneField),
            200 => Ok(Self::DalsonsTears),
            201 => Ok(Self::GahrronsWithering),
            202 => Ok(Self::TheWrithingHaunt),
            203 => Ok(Self::MardenholdeKeep),
            204 => Ok(Self::PyrewoodVillage),
            205 => Ok(Self::DunModr),
            206 => Ok(Self::Westfall1),
            207 => Ok(Self::TheGreatSea0),
            208 => Ok(Self::UnusedIroncladcove),
            209 => Ok(Self::ShadowfangKeep0),
            210 => Ok(Self::OnMapDungeon3),
            211 => Ok(Self::IceflowLake),
            212 => Ok(Self::HelmsBedLake),
            213 => Ok(Self::DeepElemMine),
            214 => Ok(Self::TheGreatSea1),
            215 => Ok(Self::Mulgore),
            219 => Ok(Self::AlexstonFarmstead),
            220 => Ok(Self::RedCloudMesa),
            221 => Ok(Self::CampNarache),
            222 => Ok(Self::BloodhoofVillage),
            223 => Ok(Self::StonebullLake),
            224 => Ok(Self::RavagedCaravan),
            225 => Ok(Self::RedRocks),
            226 => Ok(Self::TheSkitteringDark),
            227 => Ok(Self::ValgansField),
            228 => Ok(Self::TheSepulcher),
            229 => Ok(Self::OlsensFarthing),
            230 => Ok(Self::TheGreymaneWall),
            231 => Ok(Self::BerensPeril),
            232 => Ok(Self::TheDawningIsles),
            233 => Ok(Self::Ambermill),
            235 => Ok(Self::FenrisKeep),
            236 => Ok(Self::ShadowfangKeep1),
            237 => Ok(Self::TheDecrepitFerry),
            238 => Ok(Self::MaldensOrchard),
            239 => Ok(Self::TheIvarPatch),
            240 => Ok(Self::TheDeadField),
            241 => Ok(Self::TheRottingOrchard),
            242 => Ok(Self::BrightwoodGrove),
            243 => Ok(Self::ForlornRowe),
            244 => Ok(Self::TheWhippleEstate),
            245 => Ok(Self::TheYorgenFarmstead),
            246 => Ok(Self::TheCauldron),
            247 => Ok(Self::GrimesiltDigSite),
            249 => Ok(Self::DreadmaulRock),
            250 => Ok(Self::RuinsOfThaurissan),
            251 => Ok(Self::FlameCrest),
            252 => Ok(Self::BlackrockStronghold),
            253 => Ok(Self::ThePillarOfAsh),
            254 => Ok(Self::BlackrockMountain1),
            255 => Ok(Self::AltarOfStorms0),
            256 => Ok(Self::Aldrassil),
            257 => Ok(Self::ShadowthreadCave),
            258 => Ok(Self::FelRock),
            259 => Ok(Self::LakeAlAmeth),
            260 => Ok(Self::StarbreezeVillage),
            261 => Ok(Self::GnarlpineHold),
            262 => Ok(Self::BanethilBarrowDen),
            263 => Ok(Self::TheCleft),
            264 => Ok(Self::TheOracleGlade),
            265 => Ok(Self::WellspringRiver),
            266 => Ok(Self::WellspringLake),
            267 => Ok(Self::HillsbradFoothills),
            268 => Ok(Self::AzsharaCrater),
            269 => Ok(Self::DunAlgaz0),
            271 => Ok(Self::Southshore0),
            272 => Ok(Self::TarrenMill0),
            275 => Ok(Self::DurnholdeKeep0),
            276 => Ok(Self::UnusedStonewroughtPass),
            277 => Ok(Self::TheFoothillCaverns),
            278 => Ok(Self::LordamereInternmentCamp),
            279 => Ok(Self::Dalaran),
            280 => Ok(Self::Strahnbrad),
            281 => Ok(Self::RuinsOfAlterac),
            282 => Ok(Self::CrushridgeHold),
            283 => Ok(Self::SlaughterHollow),
            284 => Ok(Self::TheUplands),
            285 => Ok(Self::SouthpointTower0),
            286 => Ok(Self::HillsbradFields0),
            287 => Ok(Self::Hillsbrad),
            288 => Ok(Self::AzurelodeMine0),
            289 => Ok(Self::NethanderStead0),
            290 => Ok(Self::DunGarok0),
            293 => Ok(Self::ThoradinsWall0),
            294 => Ok(Self::EasternStrand0),
            295 => Ok(Self::WesternStrand0),
            296 => Ok(Self::SouthSeasUnused),
            297 => Ok(Self::JagueroIsle),
            298 => Ok(Self::BaradinBay),
            299 => Ok(Self::MenethilBay),
            300 => Ok(Self::MistyReedStrand),
            301 => Ok(Self::TheSavageCoast),
            302 => Ok(Self::TheCrystalShore),
            303 => Ok(Self::ShellBeach),
            305 => Ok(Self::NorthTidesRun),
            306 => Ok(Self::SouthTidesRun),
            307 => Ok(Self::TheOverlookCliffs),
            308 => Ok(Self::TheForbiddingSea0),
            309 => Ok(Self::IronbeardsTomb),
            310 => Ok(Self::CrystalveinMine),
            311 => Ok(Self::RuinsOfAboraz),
            312 => Ok(Self::JaneirosPoint),
            313 => Ok(Self::NorthfoldManor),
            314 => Ok(Self::GoShekFarm),
            315 => Ok(Self::DabyriesFarmstead),
            316 => Ok(Self::BoulderfistHall),
            317 => Ok(Self::WitherbarkVillage),
            318 => Ok(Self::DrywhiskerGorge),
            320 => Ok(Self::RefugePointe),
            321 => Ok(Self::Hammerfall),
            322 => Ok(Self::BlackwaterShipwrecks),
            323 => Ok(Self::OBreensCamp),
            324 => Ok(Self::StromgardeKeep),
            325 => Ok(Self::TheTowerOfArathor),
            326 => Ok(Self::TheSanctum),
            327 => Ok(Self::FaldirsCove),
            328 => Ok(Self::TheDrownedReef),
            330 => Ok(Self::ThandolSpan0),
            331 => Ok(Self::Ashenvale),
            332 => Ok(Self::TheGreatSea2),
            333 => Ok(Self::CircleOfEastBinding),
            334 => Ok(Self::CircleOfWestBinding),
            335 => Ok(Self::CircleOfInnerBinding),
            336 => Ok(Self::CircleOfOuterBinding),
            337 => Ok(Self::ApocryphansRest),
            338 => Ok(Self::AngorFortress),
            339 => Ok(Self::LethlorRavine),
            340 => Ok(Self::Kargath),
            341 => Ok(Self::CampKosh),
            342 => Ok(Self::CampBoff),
            343 => Ok(Self::CampWurg),
            344 => Ok(Self::CampCagg),
            345 => Ok(Self::AgmondsEnd),
            346 => Ok(Self::HammertoesDigsite),
            347 => Ok(Self::DustbelchGrotto),
            348 => Ok(Self::AeriePeak),
            349 => Ok(Self::WildhammerKeep),
            350 => Ok(Self::QuelDanilLodge),
            351 => Ok(Self::SkulkRock),
            352 => Ok(Self::Zunwatha),
            353 => Ok(Self::ShadraAlor),
            354 => Ok(Self::JinthaAlor),
            355 => Ok(Self::TheAltarOfZul),
            356 => Ok(Self::Seradane),
            357 => Ok(Self::Feralas),
            358 => Ok(Self::BramblebladeRavine),
            359 => Ok(Self::BaelModan),
            360 => Ok(Self::TheVentureCoMine),
            361 => Ok(Self::Felwood),
            362 => Ok(Self::RazorHill),
            363 => Ok(Self::ValleyOfTrials),
            364 => Ok(Self::TheDen),
            365 => Ok(Self::BurningBladeCoven),
            366 => Ok(Self::KolkarCrag),
            367 => Ok(Self::SenjinVillage),
            368 => Ok(Self::EchoIsles),
            369 => Ok(Self::ThunderRidge),
            370 => Ok(Self::DrygulchRavine),
            371 => Ok(Self::DustwindCave),
            372 => Ok(Self::TiragardeKeep),
            373 => Ok(Self::ScuttleCoast),
            374 => Ok(Self::BladefistBay),
            375 => Ok(Self::DeadeyeShore),
            377 => Ok(Self::SouthfuryRiver0),
            378 => Ok(Self::CampTaurajo),
            379 => Ok(Self::FarWatchPost),
            380 => Ok(Self::TheCrossroads),
            381 => Ok(Self::BoulderLodeMine),
            382 => Ok(Self::TheSludgeFen),
            383 => Ok(Self::TheDryHills),
            384 => Ok(Self::DreadmistPeak),
            385 => Ok(Self::NorthwatchHold),
            386 => Ok(Self::TheForgottenPools),
            387 => Ok(Self::LushwaterOasis),
            388 => Ok(Self::TheStagnantOasis),
            390 => Ok(Self::FieldOfGiants),
            391 => Ok(Self::TheMerchantCoast),
            392 => Ok(Self::Ratchet),
            393 => Ok(Self::DarkspearStrand),
            394 => Ok(Self::DarrowmereLakeUnused),
            395 => Ok(Self::CaerDarrowUnused),
            396 => Ok(Self::WinterhoofWaterWell),
            397 => Ok(Self::ThunderhornWaterWell),
            398 => Ok(Self::WildmaneWaterWell),
            399 => Ok(Self::SkylineRidge),
            400 => Ok(Self::ThousandNeedles),
            401 => Ok(Self::TheTidusStair),
            403 => Ok(Self::ShadyRestInn),
            404 => Ok(Self::BaeldunDigsite),
            405 => Ok(Self::Desolace),
            406 => Ok(Self::StonetalonMountains),
            407 => Ok(Self::OrgrimmarUnused),
            408 => Ok(Self::GillijimsIsle),
            409 => Ok(Self::IslandOfDoctorLapidis),
            410 => Ok(Self::RazorwindCanyon),
            411 => Ok(Self::BathransHaunt),
            412 => Ok(Self::TheRuinsOfOrdilAran),
            413 => Ok(Self::MaestrasPost),
            414 => Ok(Self::TheZoramStrand),
            415 => Ok(Self::Astranaar),
            416 => Ok(Self::TheShrineOfAessina),
            417 => Ok(Self::FireScarShrine),
            418 => Ok(Self::TheRuinsOfStardust),
            419 => Ok(Self::TheHowlingVale),
            420 => Ok(Self::SilverwindRefuge),
            421 => Ok(Self::MystralLake),
            422 => Ok(Self::FallenSkyLake),
            424 => Ok(Self::IrisLake),
            425 => Ok(Self::Moonwell),
            426 => Ok(Self::RaynewoodRetreat),
            427 => Ok(Self::TheShadyNook),
            428 => Ok(Self::NightRun),
            429 => Ok(Self::Xavian),
            430 => Ok(Self::Satyrnaar),
            431 => Ok(Self::SplintertreePost),
            432 => Ok(Self::TheDorDanilBarrowDen),
            433 => Ok(Self::FalfarrenRiver),
            434 => Ok(Self::FelfireHill),
            435 => Ok(Self::DemonFallCanyon),
            436 => Ok(Self::DemonFallRidge),
            437 => Ok(Self::WarsongLumberCamp),
            438 => Ok(Self::BoughShadow),
            439 => Ok(Self::TheShimmeringFlats),
            440 => Ok(Self::Tanaris),
            441 => Ok(Self::LakeFalathim),
            442 => Ok(Self::Auberdine),
            443 => Ok(Self::RuinsOfMathystra),
            444 => Ok(Self::TowerOfAlthalaxx),
            445 => Ok(Self::CliffspringFalls),
            446 => Ok(Self::BashalAran),
            447 => Ok(Self::AmethAran),
            448 => Ok(Self::GroveOfTheAncients),
            449 => Ok(Self::TheMastersGlaive),
            450 => Ok(Self::RemtravelsExcavation),
            452 => Ok(Self::MistsEdge),
            453 => Ok(Self::TheLongWash),
            454 => Ok(Self::WildbendRiver),
            455 => Ok(Self::BlackwoodDen),
            456 => Ok(Self::CliffspringRiver),
            457 => Ok(Self::TheVeiledSea0),
            458 => Ok(Self::GoldRoad),
            459 => Ok(Self::ScarletWatchPost),
            460 => Ok(Self::SunRockRetreat),
            461 => Ok(Self::WindshearCrag),
            463 => Ok(Self::CragpoolLake),
            464 => Ok(Self::MirkfallonLake),
            465 => Ok(Self::TheCharredVale),
            466 => Ok(Self::ValleyOfTheBloodfuries),
            467 => Ok(Self::StonetalonPeak),
            468 => Ok(Self::TheTalonDen),
            469 => Ok(Self::GreatwoodVale),
            470 => Ok(Self::ThunderBluffUnused),
            471 => Ok(Self::BraveWindMesa),
            472 => Ok(Self::FireStoneMesa),
            473 => Ok(Self::MantleRock),
            474 => Ok(Self::HunterRiseUnused),
            475 => Ok(Self::SpiritRiseUnused),
            476 => Ok(Self::ElderRiseUnused),
            477 => Ok(Self::RuinsOfJubuwal),
            478 => Ok(Self::PoolsOfArlithrien),
            479 => Ok(Self::TheRustmaulDigSite),
            480 => Ok(Self::CampEthok),
            481 => Ok(Self::SplithoofCrag),
            482 => Ok(Self::Highperch),
            483 => Ok(Self::TheScreechingCanyon),
            484 => Ok(Self::FreewindPost),
            485 => Ok(Self::TheGreatLift0),
            486 => Ok(Self::GalakHold),
            487 => Ok(Self::RoguefeatherDen),
            488 => Ok(Self::TheWeatheredNook),
            489 => Ok(Self::Thalanaar),
            490 => Ok(Self::UnGoroCrater),
            491 => Ok(Self::RazorfenKraul0),
            492 => Ok(Self::RavenHillCemetery),
            493 => Ok(Self::Moonglade),
            495 => Ok(Self::DeleteMe0),
            496 => Ok(Self::BrackenwallVillage),
            497 => Ok(Self::SwamplightManor),
            498 => Ok(Self::BloodfenBurrow),
            499 => Ok(Self::DarkmistCavern),
            500 => Ok(Self::MogglePoint),
            501 => Ok(Self::BeezilsWreck),
            502 => Ok(Self::WitchHill),
            503 => Ok(Self::SentryPoint),
            504 => Ok(Self::NorthPointTower),
            505 => Ok(Self::WestPointTower),
            506 => Ok(Self::LostPoint),
            507 => Ok(Self::Bluefen),
            508 => Ok(Self::StonemaulRuins),
            509 => Ok(Self::TheDenOfFlame),
            510 => Ok(Self::TheDragonmurk),
            511 => Ok(Self::Wyrmbog),
            512 => Ok(Self::OnyxiasLairUnused),
            513 => Ok(Self::TheramoreIsle),
            514 => Ok(Self::FootholdCitadel),
            515 => Ok(Self::IroncladPrison),
            516 => Ok(Self::DustwallowBay),
            517 => Ok(Self::TidefuryCove),
            518 => Ok(Self::DreadmurkShore),
            536 => Ok(Self::AddlesStead),
            537 => Ok(Self::FirePlumeRidge),
            538 => Ok(Self::LakkariTarPits),
            539 => Ok(Self::TerrorRun),
            540 => Ok(Self::TheSlitheringScar),
            541 => Ok(Self::MarshalsRefuge),
            542 => Ok(Self::FungalRock),
            543 => Ok(Self::GolakkaHotSprings),
            556 => Ok(Self::TheLoch),
            576 => Ok(Self::BeggarsHaunt),
            596 => Ok(Self::KodoGraveyard),
            597 => Ok(Self::GhostWalkerPost),
            598 => Ok(Self::SartherisStrand),
            599 => Ok(Self::ThunderAxeFortress),
            600 => Ok(Self::BolgansHole),
            602 => Ok(Self::MannorocCoven),
            603 => Ok(Self::Sargeron),
            604 => Ok(Self::MagramVillage),
            606 => Ok(Self::GelkisVillage),
            607 => Ok(Self::ValleyOfSpears),
            608 => Ok(Self::NijelsPoint),
            609 => Ok(Self::KolkarVillage),
            616 => Ok(Self::Hyjal),
            618 => Ok(Self::Winterspring),
            636 => Ok(Self::BlackwolfRiver),
            637 => Ok(Self::KodoRock),
            638 => Ok(Self::HiddenPath),
            639 => Ok(Self::SpiritRock),
            640 => Ok(Self::ShrineOfTheDormantFlame),
            656 => Ok(Self::LakeEluneara),
            657 => Ok(Self::TheHarborage),
            676 => Ok(Self::Outland),
            696 => Ok(Self::CraftsmensTerraceUnused),
            697 => Ok(Self::TradesmensTerraceUnused),
            698 => Ok(Self::TheTempleGardensUnused),
            699 => Ok(Self::TempleOfEluneUnused),
            700 => Ok(Self::CenarionEnclaveUnused),
            701 => Ok(Self::WarriorsTerraceUnused),
            702 => Ok(Self::RuttheranVillage),
            716 => Ok(Self::IronbandsCompound),
            717 => Ok(Self::TheStockade),
            718 => Ok(Self::WailingCaverns),
            719 => Ok(Self::BlackfathomDeeps0),
            720 => Ok(Self::FrayIsland),
            721 => Ok(Self::Gnomeregan1),
            722 => Ok(Self::RazorfenDowns0),
            736 => Ok(Self::BanethilHollow),
            796 => Ok(Self::ScarletMonastery),
            797 => Ok(Self::JerodsLanding),
            798 => Ok(Self::RidgepointTower),
            799 => Ok(Self::TheDarkenedBank),
            800 => Ok(Self::ColdridgePass),
            801 => Ok(Self::ChillBreezeValley),
            802 => Ok(Self::ShimmerRidge),
            803 => Ok(Self::AmberstillRanch),
            804 => Ok(Self::TheTundridHills),
            805 => Ok(Self::SouthGatePass0),
            806 => Ok(Self::SouthGateOutpost),
            807 => Ok(Self::NorthGatePass0),
            808 => Ok(Self::NorthGateOutpost),
            809 => Ok(Self::GatesOfIronforge),
            810 => Ok(Self::StillwaterPond),
            811 => Ok(Self::NightmareVale),
            812 => Ok(Self::VenomwebVale),
            813 => Ok(Self::TheBulwark1),
            814 => Ok(Self::SouthfuryRiver1),
            815 => Ok(Self::SouthfuryRiver2),
            816 => Ok(Self::RazormaneGrounds),
            817 => Ok(Self::SkullRock),
            818 => Ok(Self::PalemaneRock),
            819 => Ok(Self::WindfuryRidge),
            820 => Ok(Self::TheGoldenPlains),
            821 => Ok(Self::TheRollingPlains),
            836 => Ok(Self::DunAlgaz1),
            837 => Ok(Self::DunAlgaz2),
            838 => Ok(Self::NorthGatePass1),
            839 => Ok(Self::SouthGatePass1),
            856 => Ok(Self::TwilightGrove),
            876 => Ok(Self::GmIsland),
            877 => Ok(Self::DeleteMe1),
            878 => Ok(Self::SouthfuryRiver3),
            879 => Ok(Self::SouthfuryRiver4),
            880 => Ok(Self::ThandolSpan1),
            881 => Ok(Self::ThandolSpan2),
            896 => Ok(Self::PurgationIsle),
            916 => Ok(Self::TheJansenStead),
            917 => Ok(Self::TheDeadAcre),
            918 => Ok(Self::TheMolsenFarm),
            919 => Ok(Self::StendelsPond),
            920 => Ok(Self::TheDaggerHills),
            921 => Ok(Self::DemontsPlace),
            922 => Ok(Self::TheDustPlains),
            923 => Ok(Self::StonesplinterValley),
            924 => Ok(Self::ValleyOfKings),
            925 => Ok(Self::AlgazStation),
            926 => Ok(Self::BucklebreeFarm),
            927 => Ok(Self::TheShiningStrand),
            928 => Ok(Self::NorthTidesHollow),
            936 => Ok(Self::GrizzlepawRidge),
            956 => Ok(Self::TheVerdantFields),
            976 => Ok(Self::Gadgetzan),
            977 => Ok(Self::SteamwheedlePort),
            978 => Ok(Self::ZulFarrak0),
            979 => Ok(Self::SandsorrowWatch),
            980 => Ok(Self::ThistleshrubValley),
            981 => Ok(Self::TheGapingChasm),
            982 => Ok(Self::TheNoxiousLair),
            983 => Ok(Self::DunemaulCompound),
            984 => Ok(Self::EastmoonRuins),
            985 => Ok(Self::WaterspringField),
            986 => Ok(Self::ZalashjisDen),
            987 => Ok(Self::LandsEndBeach),
            988 => Ok(Self::WavestriderBeach),
            989 => Ok(Self::Uldum),
            990 => Ok(Self::ValleyOfTheWatchers),
            991 => Ok(Self::GunstansPost),
            992 => Ok(Self::SouthmoonRuins),
            996 => Ok(Self::RendersCamp),
            997 => Ok(Self::RendersValley),
            998 => Ok(Self::RendersRock),
            999 => Ok(Self::StonewatchTower),
            1000 => Ok(Self::GalardellValley),
            1001 => Ok(Self::LakeridgeHighway),
            1002 => Ok(Self::ThreeCorners),
            1016 => Ok(Self::DireforgeHill),
            1017 => Ok(Self::RaptorRidge),
            1018 => Ok(Self::BlackChannelMarsh),
            1019 => Ok(Self::TheGreenBelt0),
            1020 => Ok(Self::MosshideFen),
            1021 => Ok(Self::ThelgenRock),
            1022 => Ok(Self::BluegillMarsh),
            1023 => Ok(Self::SaltsprayGlen),
            1024 => Ok(Self::SundownMarsh),
            1025 => Ok(Self::TheGreenBelt1),
            1036 => Ok(Self::AngerfangEncampment),
            1037 => Ok(Self::GrimBatol),
            1038 => Ok(Self::DragonmawGates),
            1039 => Ok(Self::TheLostFleet),
            1056 => Ok(Self::DarrowHill0),
            1057 => Ok(Self::ThoradinsWall1),
            1076 => Ok(Self::WebwinderPath),
            1097 => Ok(Self::TheHushedBank),
            1098 => Ok(Self::ManorMistmantle),
            1099 => Ok(Self::CampMojache),
            1100 => Ok(Self::GrimtotemCompound),
            1101 => Ok(Self::TheWrithingDeep),
            1102 => Ok(Self::WildwindLake),
            1103 => Ok(Self::GordunniOutpost),
            1104 => Ok(Self::MokGordun),
            1105 => Ok(Self::FeralScarVale),
            1106 => Ok(Self::FrayfeatherHighlands),
            1107 => Ok(Self::IdlewindLake),
            1108 => Ok(Self::TheForgottenCoast),
            1109 => Ok(Self::EastPillar),
            1110 => Ok(Self::WestPillar),
            1111 => Ok(Self::DreamBough),
            1112 => Ok(Self::JademirLake),
            1113 => Ok(Self::Oneiros),
            1114 => Ok(Self::RuinsOfRavenwind),
            1115 => Ok(Self::RageScarHold),
            1116 => Ok(Self::FeathermoonStronghold),
            1117 => Ok(Self::RuinsOfSolarsal),
            1118 => Ok(Self::LowerWildsUnused),
            1119 => Ok(Self::TheTwinColossals),
            1120 => Ok(Self::SardorIsle),
            1121 => Ok(Self::IsleOfDread),
            1136 => Ok(Self::HighWilderness),
            1137 => Ok(Self::LowerWilds),
            1156 => Ok(Self::SouthernBarrens),
            1157 => Ok(Self::SouthernGoldRoad),
            1176 => Ok(Self::ZulFarrak1),
            1196 => Ok(Self::UnusedAlcazIsland),
            1216 => Ok(Self::TimbermawHold0),
            1217 => Ok(Self::VanndirEncampment),
            1218 => Ok(Self::TestAzshara),
            1219 => Ok(Self::LegashEncampment),
            1220 => Ok(Self::ThalassianBaseCamp),
            1221 => Ok(Self::RuinsOfEldarath),
            1222 => Ok(Self::HetaerasClutch),
            1223 => Ok(Self::TempleOfZinMalor),
            1224 => Ok(Self::BearsHead),
            1225 => Ok(Self::Ursolan),
            1226 => Ok(Self::TempleOfArkkoran),
            1227 => Ok(Self::BayOfStorms),
            1228 => Ok(Self::TheShatteredStrand),
            1229 => Ok(Self::TowerOfEldara),
            1230 => Ok(Self::JaggedReef),
            1231 => Ok(Self::SouthridgeBeach),
            1232 => Ok(Self::RavencrestMonument),
            1233 => Ok(Self::ForlornRidge),
            1234 => Ok(Self::LakeMennar),
            1235 => Ok(Self::ShadowsongShrine),
            1236 => Ok(Self::HaldarrEncampment),
            1237 => Ok(Self::Valormok),
            1256 => Ok(Self::TheRuinedReaches),
            1276 => Ok(Self::TheTalondeepPath0),
            1277 => Ok(Self::TheTalondeepPath1),
            1296 => Ok(Self::RocktuskFarm),
            1297 => Ok(Self::JaggedswineFarm),
            1316 => Ok(Self::RazorfenDowns1),
            1336 => Ok(Self::LostRiggerCove),
            1337 => Ok(Self::Uldaman0),
            1338 => Ok(Self::LordamereLake1),
            1339 => Ok(Self::LordamereLake2),
            1357 => Ok(Self::GallowsCorner),
            1377 => Ok(Self::Silithus),
            1397 => Ok(Self::EmeraldForest),
            1417 => Ok(Self::SunkenTemple),
            1437 => Ok(Self::DreadmaulHold),
            1438 => Ok(Self::NethergardeKeep),
            1439 => Ok(Self::DreadmaulPost),
            1440 => Ok(Self::SerpentsCoil),
            1441 => Ok(Self::AltarOfStorms1),
            1442 => Ok(Self::FirewatchRidge),
            1443 => Ok(Self::TheSlagPit),
            1444 => Ok(Self::TheSeaOfCinders),
            1445 => Ok(Self::BlackrockMountain2),
            1446 => Ok(Self::ThoriumPoint),
            1457 => Ok(Self::GarrisonArmory),
            1477 => Ok(Self::TheTempleOfAtalHakkar),
            1497 => Ok(Self::Undercity),
            1517 => Ok(Self::Uldaman1),
            1518 => Ok(Self::NotUsedDeadmines),
            1519 => Ok(Self::StormwindCity),
            1537 => Ok(Self::Ironforge),
            1557 => Ok(Self::SplithoofHold),
            1577 => Ok(Self::TheCapeOfStranglethorn),
            1578 => Ok(Self::SouthernSavageCoast),
            1579 => Ok(Self::UnusedTheDeadmines002),
            1580 => Ok(Self::UnusedIroncladCove003),
            1581 => Ok(Self::TheDeadmines),
            1582 => Ok(Self::IroncladCove),
            1583 => Ok(Self::BlackrockSpire),
            1584 => Ok(Self::BlackrockDepths),
            1597 => Ok(Self::RaptorGroundsUnused),
            1598 => Ok(Self::GroldomFarmUnused),
            1599 => Ok(Self::MorshanBaseCamp),
            1600 => Ok(Self::HonorsStandUnused),
            1601 => Ok(Self::BlackthornRidgeUnused),
            1602 => Ok(Self::BramblescarUnused),
            1603 => Ok(Self::AgamagorUnused),
            1617 => Ok(Self::ValleyOfHeroes),
            1637 => Ok(Self::Orgrimmar),
            1638 => Ok(Self::ThunderBluff),
            1639 => Ok(Self::ElderRise),
            1640 => Ok(Self::SpiritRise),
            1641 => Ok(Self::HunterRise),
            1657 => Ok(Self::Darnassus),
            1658 => Ok(Self::CenarionEnclave),
            1659 => Ok(Self::CraftsmensTerrace),
            1660 => Ok(Self::WarriorsTerrace),
            1661 => Ok(Self::TheTempleGardens),
            1662 => Ok(Self::TradesmensTerrace),
            1677 => Ok(Self::GavinsNaze),
            1678 => Ok(Self::SoferasNaze),
            1679 => Ok(Self::CorrahnsDagger),
            1680 => Ok(Self::TheHeadland),
            1681 => Ok(Self::MistyShore),
            1682 => Ok(Self::DandredsFold),
            1683 => Ok(Self::GrowlessCave),
            1684 => Ok(Self::ChillwindPoint),
            1697 => Ok(Self::RaptorGrounds),
            1698 => Ok(Self::Bramblescar),
            1699 => Ok(Self::ThornHill),
            1700 => Ok(Self::Agamagor),
            1701 => Ok(Self::BlackthornRidge),
            1702 => Ok(Self::HonorsStand),
            1703 => Ok(Self::TheMorshanRampart),
            1704 => Ok(Self::GroldomFarm),
            1717 => Ok(Self::RazorfenKraul1),
            1718 => Ok(Self::TheGreatLift1),
            1737 => Ok(Self::MistvaleValley),
            1738 => Ok(Self::NekmaniWellspring),
            1739 => Ok(Self::BloodsailCompound),
            1740 => Ok(Self::VentureCoBaseCamp),
            1741 => Ok(Self::GurubashiArena),
            1742 => Ok(Self::SpiritDen),
            1757 => Ok(Self::TheCrimsonVeil),
            1758 => Ok(Self::TheRiptide),
            1759 => Ok(Self::TheDamselsLuck),
            1760 => Ok(Self::VentureCoOperationsCenter),
            1761 => Ok(Self::DeadwoodVillage),
            1762 => Ok(Self::FelpawVillage),
            1763 => Ok(Self::Jaedenar),
            1764 => Ok(Self::BloodvenomRiver),
            1765 => Ok(Self::BloodvenomFalls),
            1766 => Ok(Self::ShatterScarVale),
            1767 => Ok(Self::IrontreeWoods),
            1768 => Ok(Self::IrontreeCavern),
            1769 => Ok(Self::TimbermawHold1),
            1770 => Ok(Self::ShadowHold),
            1771 => Ok(Self::ShrineOfTheDeceiver),
            1777 => Ok(Self::IthariussCave),
            1778 => Ok(Self::Sorrowmurk),
            1779 => Ok(Self::DraenildurVillage),
            1780 => Ok(Self::SplinterspearJunction),
            1797 => Ok(Self::Stagalbog),
            1798 => Ok(Self::TheShiftingMire),
            1817 => Ok(Self::StagalbogCave),
            1837 => Ok(Self::WitherbarkCaverns),
            1857 => Ok(Self::ThoradinsWall2),
            1858 => Ok(Self::Bouldergor),
            1877 => Ok(Self::ValleyOfFangs),
            1878 => Ok(Self::TheDustbowl),
            1879 => Ok(Self::MirageFlats),
            1880 => Ok(Self::FeatherbeardsHovel),
            1881 => Ok(Self::ShindiggersCamp),
            1882 => Ok(Self::PlaguemistRavine),
            1883 => Ok(Self::ValorwindLake),
            1884 => Ok(Self::Agolwatha),
            1885 => Ok(Self::Hiriwatha),
            1886 => Ok(Self::TheCreepingRuin),
            1887 => Ok(Self::BogensLedge),
            1897 => Ok(Self::TheMakersTerrace),
            1898 => Ok(Self::DustwindGulch),
            1917 => Ok(Self::Shaolwatha),
            1937 => Ok(Self::NoonshadeRuins),
            1938 => Ok(Self::BrokenPillar),
            1939 => Ok(Self::AbyssalSands),
            1940 => Ok(Self::SouthbreakShore),
            1941 => Ok(Self::CavernsOfTime0),
            1942 => Ok(Self::TheMarshlands),
            1943 => Ok(Self::IronstonePlateau),
            1957 => Ok(Self::BlackcharCave),
            1958 => Ok(Self::TannerCamp),
            1959 => Ok(Self::DustfireValley),
            1977 => Ok(Self::ZulGurub1),
            1978 => Ok(Self::MistyReedPost),
            1997 => Ok(Self::BloodvenomPost),
            1998 => Ok(Self::TalonbranchGlade),
            2017 => Ok(Self::Stratholme0),
            2037 => Ok(Self::UnusedShadowfangKeep003),
            2057 => Ok(Self::Scholomance),
            2077 => Ok(Self::TwilightVale),
            2078 => Ok(Self::TwilightShore),
            2079 => Ok(Self::AlcazIsland),
            2097 => Ok(Self::DarkcloudPinnacle),
            2098 => Ok(Self::DawningWoodCatacombs),
            2099 => Ok(Self::StonewatchKeep),
            2100 => Ok(Self::Maraudon),
            2101 => Ok(Self::StoutlagerInn),
            2102 => Ok(Self::ThunderbrewDistillery),
            2103 => Ok(Self::MenethilKeep),
            2104 => Ok(Self::DeepwaterTavern),
            2117 => Ok(Self::ShadowGrave),
            2118 => Ok(Self::BrillTownHall),
            2119 => Ok(Self::GallowsEndTavern),
            2137 => Ok(Self::ThePoolsOfVisionUnused),
            2138 => Ok(Self::DreadmistDen),
            2157 => Ok(Self::BaeldunKeep),
            2158 => Ok(Self::EmberstrifesDen),
            2159 => Ok(Self::OnyxiasLair),
            2160 => Ok(Self::WindshearMine),
            2161 => Ok(Self::RolandsDoom),
            2177 => Ok(Self::BattleRing),
            2197 => Ok(Self::ThePoolsOfVision),
            2198 => Ok(Self::ShadowbreakRavine),
            2217 => Ok(Self::BrokenSpearVillage),
            2237 => Ok(Self::WhitereachPost),
            2238 => Ok(Self::Gornia),
            2239 => Ok(Self::ZanesEyeCrater),
            2240 => Ok(Self::MirageRaceway),
            2241 => Ok(Self::FrostsaberRock),
            2242 => Ok(Self::TheHiddenGrove),
            2243 => Ok(Self::TimbermawPost),
            2244 => Ok(Self::WinterfallVillage),
            2245 => Ok(Self::Mazthoril),
            2246 => Ok(Self::FrostfireHotSprings),
            2247 => Ok(Self::IceThistleHills),
            2248 => Ok(Self::DunMandarr),
            2249 => Ok(Self::FrostwhisperGorge),
            2250 => Ok(Self::OwlWingThicket),
            2251 => Ok(Self::LakeKelTheril),
            2252 => Ok(Self::TheRuinsOfKelTheril),
            2253 => Ok(Self::StarfallVillage),
            2254 => Ok(Self::BanThallowBarrowDen),
            2255 => Ok(Self::Everlook),
            2256 => Ok(Self::DarkwhisperGorge),
            2257 => Ok(Self::DeeprunTram),
            2258 => Ok(Self::TheFungalVale),
            2259 => Ok(Self::UnusedTheMarrisStead),
            2260 => Ok(Self::TheMarrisStead),
            2261 => Ok(Self::TheUndercroft),
            2262 => Ok(Self::Darrowshire),
            2263 => Ok(Self::CrownGuardTower),
            2264 => Ok(Self::CorinsCrossing),
            2265 => Ok(Self::ScarletBaseCamp),
            2266 => Ok(Self::TyrsHand),
            2267 => Ok(Self::TheScarletBasilica),
            2268 => Ok(Self::LightsHopeChapel),
            2269 => Ok(Self::BrowmanMill),
            2270 => Ok(Self::TheNoxiousGlade),
            2271 => Ok(Self::EastwallTower),
            2272 => Ok(Self::Northdale),
            2273 => Ok(Self::ZulMashar),
            2274 => Ok(Self::MazraAlor),
            2275 => Ok(Self::NorthpassTower),
            2276 => Ok(Self::QuelLithienLodge),
            2277 => Ok(Self::Plaguewood),
            2278 => Ok(Self::Scourgehold),
            2279 => Ok(Self::Stratholme1),
            2280 => Ok(Self::UnusedStratholme),
            2297 => Ok(Self::DarrowmereLake0),
            2298 => Ok(Self::CaerDarrow),
            2299 => Ok(Self::DarrowmereLake1),
            2300 => Ok(Self::CavernsOfTime1),
            2301 => Ok(Self::ThistlefurVillage),
            2302 => Ok(Self::TheQuagmire),
            2303 => Ok(Self::WindbreakCanyon),
            2317 => Ok(Self::SouthSeas0),
            2318 => Ok(Self::TheGreatSea3),
            2319 => Ok(Self::TheGreatSea4),
            2320 => Ok(Self::TheGreatSea5),
            2321 => Ok(Self::TheGreatSea6),
            2322 => Ok(Self::TheVeiledSea1),
            2323 => Ok(Self::TheVeiledSea2),
            2324 => Ok(Self::TheVeiledSea3),
            2325 => Ok(Self::TheVeiledSea4),
            2326 => Ok(Self::TheVeiledSea5),
            2337 => Ok(Self::RazorHillBarracks),
            2338 => Ok(Self::SouthSeas1),
            2339 => Ok(Self::TheGreatSea7),
            2357 => Ok(Self::BloodtoothCamp),
            2358 => Ok(Self::ForestSong),
            2359 => Ok(Self::GreenpawVillage),
            2360 => Ok(Self::SilverwingOutpost),
            2361 => Ok(Self::Nighthaven),
            2362 => Ok(Self::ShrineOfRemulos),
            2363 => Ok(Self::StormrageBarrowDens),
            2364 => Ok(Self::TheGreatSea8),
            2365 => Ok(Self::TheGreatSea9),
            2366 => Ok(Self::TheBlackMorass),
            2367 => Ok(Self::OldHillsbradFoothills),
            2368 => Ok(Self::TarrenMill1),
            2369 => Ok(Self::Southshore1),
            2370 => Ok(Self::DurnholdeKeep1),
            2371 => Ok(Self::DunGarok1),
            2372 => Ok(Self::HillsbradFields1),
            2373 => Ok(Self::EasternStrand1),
            2374 => Ok(Self::NethanderStead1),
            2375 => Ok(Self::DarrowHill1),
            2376 => Ok(Self::SouthpointTower1),
            2377 => Ok(Self::ThoradinsWall3),
            2378 => Ok(Self::WesternStrand1),
            2379 => Ok(Self::AzurelodeMine1),
            2397 => Ok(Self::TheGreatSea10),
            2398 => Ok(Self::TheGreatSea11),
            2399 => Ok(Self::TheGreatSea12),
            2400 => Ok(Self::TheForbiddingSea1),
            2401 => Ok(Self::TheForbiddingSea2),
            2402 => Ok(Self::TheForbiddingSea3),
            2403 => Ok(Self::TheForbiddingSea4),
            2404 => Ok(Self::TethrisAran),
            2405 => Ok(Self::EthelRethor),
            2406 => Ok(Self::RanazjarIsle),
            2407 => Ok(Self::KormeksHut),
            2408 => Ok(Self::ShadowpreyVillage),
            2417 => Ok(Self::BlackrockPass),
            2418 => Ok(Self::MorgansVigil),
            2419 => Ok(Self::SlitherRock),
            2420 => Ok(Self::TerrorWingPath),
            2421 => Ok(Self::Dracodar),
            2437 => Ok(Self::RagefireChasm),
            2457 => Ok(Self::NightsongWoods),
            2477 => Ok(Self::TheVeiledSea6),
            2478 => Ok(Self::MorlosAran),
            2479 => Ok(Self::EmeraldSanctuary),
            2480 => Ok(Self::JadefireGlen),
            2481 => Ok(Self::RuinsOfConstellas),
            2497 => Ok(Self::BitterReaches),
            2517 => Ok(Self::RiseOfTheDefiler),
            2518 => Ok(Self::LarissPavilion),
            2519 => Ok(Self::WoodpawHills),
            2520 => Ok(Self::WoodpawDen),
            2521 => Ok(Self::VerdantisRiver),
            2522 => Ok(Self::RuinsOfIsildien),
            2537 => Ok(Self::GrimtotemPost),
            2538 => Ok(Self::CampAparaje),
            2539 => Ok(Self::Malakajin),
            2540 => Ok(Self::BoulderslideRavine),
            2541 => Ok(Self::SishirCanyon),
            2557 => Ok(Self::DireMaul0),
            2558 => Ok(Self::DeadwindRavine),
            2559 => Ok(Self::DiamondheadRiver),
            2560 => Ok(Self::AridensCamp),
            2561 => Ok(Self::TheVice),
            2562 => Ok(Self::Karazhan),
            2563 => Ok(Self::MorgansPlot),
            2577 => Ok(Self::DireMaul1),
            2597 => Ok(Self::AlteracValley0),
            2617 => Ok(Self::ScrabblescrewsCamp),
            2618 => Ok(Self::JadefireRun),
            2619 => Ok(Self::ThondrorilRiver0),
            2620 => Ok(Self::ThondrorilRiver1),
            2621 => Ok(Self::LakeMereldar),
            2622 => Ok(Self::PestilentScar),
            2623 => Ok(Self::TheInfectisScar),
            2624 => Ok(Self::BlackwoodLake),
            2625 => Ok(Self::EastwallGate),
            2626 => Ok(Self::TerrorwebTunnel),
            2627 => Ok(Self::Terrordale),
            2637 => Ok(Self::KargathiaKeep),
            2657 => Ok(Self::ValleyOfBones),
            2677 => Ok(Self::BlackwingLair),
            2697 => Ok(Self::DeadmansCrossing),
            2717 => Ok(Self::MoltenCore),
            2737 => Ok(Self::TheScarabWall),
            2738 => Ok(Self::SouthwindVillage),
            2739 => Ok(Self::TwilightBaseCamp),
            2740 => Ok(Self::TheCrystalVale),
            2741 => Ok(Self::TheScarabDais),
            2742 => Ok(Self::HiveAshi),
            2743 => Ok(Self::HiveZora),
            2744 => Ok(Self::HiveRegal),
            2757 => Ok(Self::ShrineOfTheFallenWarrior),
            2777 => Ok(Self::UnusedAlteracValley),
            2797 => Ok(Self::BlackfathomDeeps1),
            2817 => Ok(Self::OnMapDungeon4),
            2837 => Ok(Self::TheMastersCellar),
            2838 => Ok(Self::StonewroughtPass),
            2839 => Ok(Self::AlteracValley1),
            2857 => Ok(Self::TheRumbleCage),
            2877 => Ok(Self::ChunkTest),
            2897 => Ok(Self::ZoramgarOutpost),
            2917 => Ok(Self::HallOfLegends),
            2918 => Ok(Self::ChampionsHall),
            2937 => Ok(Self::GroshgokCompound),
            2938 => Ok(Self::SleepingGorge),
            2957 => Ok(Self::IrondeepMine),
            2958 => Ok(Self::StonehearthOutpost),
            2959 => Ok(Self::DunBaldar),
            2960 => Ok(Self::IcewingPass),
            2961 => Ok(Self::FrostwolfVillage),
            2962 => Ok(Self::TowerPoint),
            2963 => Ok(Self::ColdtoothMine),
            2964 => Ok(Self::WinteraxHold),
            2977 => Ok(Self::IcebloodGarrison),
            2978 => Ok(Self::FrostwolfKeep),
            2979 => Ok(Self::TorkrenFarm),
            3017 => Ok(Self::FrostDaggerPass),
            3037 => Ok(Self::IronstoneCamp),
            3038 => Ok(Self::WeazelsCrater),
            3039 => Ok(Self::TahondaRuins),
            3057 => Ok(Self::FieldOfStrife),
            3058 => Ok(Self::IcewingCavern),
            3077 => Ok(Self::ValorsRest),
            3097 => Ok(Self::TheSwarmingPillar),
            3098 => Ok(Self::TwilightPost),
            3099 => Ok(Self::TwilightOutpost),
            3100 => Ok(Self::RavagedTwilightCamp),
            3117 => Ok(Self::ShalzarusLair),
            3137 => Ok(Self::TalrendisPoint),
            3138 => Ok(Self::RethressSanctum),
            3139 => Ok(Self::MoonHorrorDen),
            3140 => Ok(Self::ScalebeardsCave),
            3157 => Ok(Self::BoulderslideCavern),
            3177 => Ok(Self::WarsongLaborCamp),
            3197 => Ok(Self::ChillwindCamp),
            3217 => Ok(Self::TheMaul),
            3237 => Ok(Self::TheMaulUnused),
            3257 => Ok(Self::BonesOfGrakkarond),
            3277 => Ok(Self::WarsongGulch),
            3297 => Ok(Self::FrostwolfGraveyard),
            3298 => Ok(Self::FrostwolfPass),
            3299 => Ok(Self::DunBaldarPass),
            3300 => Ok(Self::IcebloodGraveyard),
            3301 => Ok(Self::SnowfallGraveyard),
            3302 => Ok(Self::StonehearthGraveyard),
            3303 => Ok(Self::StormpikeGraveyard),
            3304 => Ok(Self::IcewingBunker),
            3305 => Ok(Self::StonehearthBunker),
            3306 => Ok(Self::WildpawRidge),
            3317 => Ok(Self::RevantuskVillage),
            3318 => Ok(Self::RockOfDurotan),
            3319 => Ok(Self::SilverwingGrove),
            3320 => Ok(Self::WarsongLumberMill),
            3321 => Ok(Self::SilverwingHold),
            3337 => Ok(Self::WildpawCavern),
            3338 => Ok(Self::TheVeiledCleft),
            3357 => Ok(Self::YojambaIsle),
            3358 => Ok(Self::ArathiBasin),
            3377 => Ok(Self::TheCoil),
            3378 => Ok(Self::AltarOfHireek),
            3379 => Ok(Self::Shadrazaar),
            3380 => Ok(Self::HakkariGrounds),
            3381 => Ok(Self::NazeOfShirvallah),
            3382 => Ok(Self::TempleOfBethekk),
            3383 => Ok(Self::TheBloodfirePit),
            3384 => Ok(Self::AltarOfTheBloodGod),
            3397 => Ok(Self::ZanzasRise),
            3398 => Ok(Self::EdgeOfMadness),
            3417 => Ok(Self::TrollbaneHall),
            3418 => Ok(Self::DefilersDen),
            3419 => Ok(Self::PaglesPointe),
            3420 => Ok(Self::Farm),
            3421 => Ok(Self::Blacksmith),
            3422 => Ok(Self::LumberMill),
            3423 => Ok(Self::GoldMine),
            3424 => Ok(Self::Stables),
            3425 => Ok(Self::CenarionHold),
            3426 => Ok(Self::StaghelmPoint),
            3427 => Ok(Self::BronzebeardEncampment),
            3428 => Ok(Self::AhnQiraj),
            3429 => Ok(Self::RuinsOfAhnQiraj0),
            3446 => Ok(Self::TwilightsRun),
            3447 => Ok(Self::OrtellsHideout),
            3448 => Ok(Self::ScarabTerrace),
            3449 => Ok(Self::GeneralsTerrace),
            3450 => Ok(Self::TheReservoir),
            3451 => Ok(Self::TheHatchery),
            3452 => Ok(Self::TheComb),
            3453 => Ok(Self::WatchersTerrace),
            3454 => Ok(Self::RuinsOfAhnQiraj1),
            3456 => Ok(Self::Naxxramas),
            3459 => Ok(Self::City),
            3478 => Ok(Self::GatesOfAhnQiraj),
            3486 => Ok(Self::RavenholdtManor),
            v => Err(crate::errors::EnumError::new("Area", v),)
        }
    }
}

