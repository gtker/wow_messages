/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common_3_3_5.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common_3_3_5.wowm#L3):
/// ```text
/// enum Emote : u32 {
///     ONESHOT_NONE = 0;
///     ONESHOT_TALK = 1;
///     ONESHOT_BOW = 2;
///     ONESHOT_WAVE = 3;
///     ONESHOT_CHEER = 4;
///     ONESHOT_EXCLAMATION = 5;
///     ONESHOT_QUESTION = 6;
///     ONESHOT_EAT = 7;
///     STATE_DANCE = 10;
///     ONESHOT_LAUGH = 11;
///     STATE_SLEEP = 12;
///     STATE_SIT = 13;
///     ONESHOT_RUDE = 14;
///     ONESHOT_ROAR = 15;
///     ONESHOT_KNEEL = 16;
///     ONESHOT_KISS = 17;
///     ONESHOT_CRY = 18;
///     ONESHOT_CHICKEN = 19;
///     ONESHOT_BEG = 20;
///     ONESHOT_APPLAUD = 21;
///     ONESHOT_SHOUT = 22;
///     ONESHOT_FLEX = 23;
///     ONESHOT_SHY = 24;
///     ONESHOT_POINT = 25;
///     STATE_STAND = 26;
///     STATE_READY_UNARMED = 27;
///     STATE_WORK_SHEATHED = 28;
///     STATE_POINT = 29;
///     STATE_NONE = 30;
///     ONESHOT_WOUND = 33;
///     ONESHOT_WOUND_CRITICAL = 34;
///     ONESHOT_ATTACK_UNARMED = 35;
///     ONESHOT_ATTACK1H = 36;
///     ONESHOT_ATTACK2HTIGHT = 37;
///     ONESHOT_ATTACK2H_LOOSE = 38;
///     ONESHOT_PARRY_UNARMED = 39;
///     ONESHOT_PARRY_SHIELD = 43;
///     ONESHOT_READY_UNARMED = 44;
///     ONESHOT_READY1H = 45;
///     ONESHOT_READY_BOW = 48;
///     ONESHOT_SPELL_PRECAST = 50;
///     ONESHOT_SPELL_CAST = 51;
///     ONESHOT_BATTLE_ROAR = 53;
///     ONESHOT_SPECIALATTACK1H = 54;
///     ONESHOT_KICK = 60;
///     ONESHOT_ATTACK_THROWN = 61;
///     STATE_STUN = 64;
///     STATE_DEAD = 65;
///     ONESHOT_SALUTE = 66;
///     STATE_KNEEL = 68;
///     STATE_USE_STANDING = 69;
///     ONESHOT_WAVE_NO_SHEATHE = 70;
///     ONESHOT_CHEER_NO_SHEATHE = 71;
///     ONESHOT_EAT_NO_SHEATHE = 92;
///     STATE_STUN_NO_SHEATHE = 93;
///     ONESHOT_DANCE = 94;
///     ONESHOT_SALUTE_NO_SHEATH = 113;
///     STATE_USE_STANDING_NO_SHEATHE = 133;
///     ONESHOT_LAUGH_NO_SHEATHE = 153;
///     STATE_WORK = 173;
///     STATE_SPELL_PRECAST = 193;
///     ONESHOT_READY_RIFLE = 213;
///     STATE_READY_RIFLE = 214;
///     STATE_WORK_MINING = 233;
///     STATE_WORK_CHOPWOOD = 234;
///     STATE_APPLAUD = 253;
///     ONESHOT_LIFTOFF = 254;
///     ONESHOT_YES = 273;
///     ONESHOT_NO = 274;
///     ONESHOT_TRAIN = 275;
///     ONESHOT_LAND = 293;
///     STATE_AT_EASE = 313;
///     STATE_READY1H = 333;
///     STATE_SPELL_KNEEL_START = 353;
///     STATE_SUBMERGED = 373;
///     ONESHOT_SUBMERGE = 374;
///     STATE_READY2H = 375;
///     STATE_READY_BOW = 376;
///     ONESHOT_MOUNT_SPECIAL = 377;
///     STATE_TALK = 378;
///     STATE_FISHING = 379;
///     ONESHOT_FISHING = 380;
///     ONESHOT_LOOT = 381;
///     STATE_WHIRLWIND = 382;
///     STATE_DROWNED = 383;
///     STATE_HOLD_BOW = 384;
///     STATE_HOLD_RIFLE = 385;
///     STATE_HOLD_THROWN = 386;
///     ONESHOT_DROWN = 387;
///     ONESHOT_STOMP = 388;
///     ONESHOT_ATTACK_OFF = 389;
///     ONESHOT_ATTACK_OFF_PIERCE = 390;
///     STATE_ROAR = 391;
///     STATE_LAUGH = 392;
///     ONESHOT_CREATURE_SPECIAL = 393;
///     ONESHOT_JUMPLANDRUN = 394;
///     ONESHOT_JUMPEND = 395;
///     ONESHOT_TALK_NO_SHEATHE = 396;
///     ONESHOT_POINT_NO_SHEATHE = 397;
///     STATE_CANNIBALIZE = 398;
///     ONESHOT_JUMPSTART = 399;
///     STATE_DANCESPECIAL = 400;
///     ONESHOT_DANCESPECIAL = 401;
///     ONESHOT_CUSTOM_SPELL_01 = 402;
///     ONESHOT_CUSTOM_SPELL_02 = 403;
///     ONESHOT_CUSTOM_SPELL_03 = 404;
///     ONESHOT_CUSTOM_SPELL_04 = 405;
///     ONESHOT_CUSTOM_SPELL_05 = 406;
///     ONESHOT_CUSTOM_SPELL_06 = 407;
///     ONESHOT_CUSTOM_SPELL_07 = 408;
///     ONESHOT_CUSTOM_SPELL_08 = 409;
///     ONESHOT_CUSTOM_SPELL_09 = 410;
///     ONESHOT_CUSTOM_SPELL_10 = 411;
///     STATE_EXCLAIM = 412;
///     STATE_DANCE_CUSTOM = 413;
///     STATE_SIT_CHAIR_MED = 415;
///     STATE_CUSTOM_SPELL_01 = 416;
///     STATE_CUSTOM_SPELL_02 = 417;
///     STATE_EAT = 418;
///     STATE_CUSTOM_SPELL_04 = 419;
///     STATE_CUSTOM_SPELL_03 = 420;
///     STATE_CUSTOM_SPELL_05 = 421;
///     STATE_SPELLEFFECT_HOLD = 422;
///     STATE_EAT_NO_SHEATHE = 423;
///     STATE_MOUNT = 424;
///     STATE_READY2HL = 425;
///     STATE_SIT_CHAIR_HIGH = 426;
///     STATE_FALL = 427;
///     STATE_LOOT = 428;
///     STATE_SUBMERGED_NEW = 429;
///     ONESHOT_COWER = 430;
///     STATE_COWER = 431;
///     ONESHOT_USE_STANDING = 432;
///     STATE_STEALTH_STAND = 433;
///     ONESHOT_OMNICAST_GHOUL = 434;
///     ONESHOT_ATTACK_BOW = 435;
///     ONESHOT_ATTACK_RIFLE = 436;
///     STATE_SWIM_IDLE = 437;
///     STATE_ATTACK_UNARMED = 438;
///     ONESHOT_SPELL_CAST_W_SOUND = 439;
///     ONESHOT_DODGE = 440;
///     ONESHOT_PARRY1H = 441;
///     ONESHOT_PARRY2H = 442;
///     ONESHOT_PARRY2HL = 443;
///     STATE_FLYFALL = 444;
///     ONESHOT_FLYDEATH = 445;
///     STATE_FLY_FALL = 446;
///     ONESHOT_FLY_SIT_GROUND_DOWN = 447;
///     ONESHOT_FLY_SIT_GROUND_UP = 448;
///     ONESHOT_EMERGE = 449;
///     ONESHOT_DRAGON_SPIT = 450;
///     STATE_SPECIAL_UNARMED = 451;
///     ONESHOT_FLYGRAB = 452;
///     STATE_FLYGRABCLOSED = 453;
///     ONESHOT_FLYGRABTHROWN = 454;
///     STATE_FLY_SIT_GROUND = 455;
///     STATE_WALK_BACKWARDS = 456;
///     ONESHOT_FLYTALK = 457;
///     ONESHOT_FLYATTACK1H = 458;
///     STATE_CUSTOM_SPELL_08 = 459;
///     ONESHOT_FLY_DRAGON_SPIT = 460;
///     STATE_SIT_CHAIR_LOW = 461;
///     ONESHOT_STUN = 462;
///     ONESHOT_SPELL_CAST_OMNI = 463;
///     STATE_READY_THROWN = 465;
///     ONESHOT_WORK_CHOPWOOD = 466;
///     ONESHOT_WORK_MINING = 467;
///     STATE_SPELL_CHANNEL_OMNI = 468;
///     STATE_SPELL_CHANNEL_DIRECTED = 469;
///     STAND_STATE_NONE = 470;
///     STATE_READYJOUST = 471;
///     STATE_STRANGULATE = 473;
///     STATE_READY_SPELL_OMNI = 474;
///     STATE_HOLD_JOUST = 475;
///     ONESHOT_CRY_JAINA = 476;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Emote {
    OneshotNone,
    OneshotTalk,
    OneshotBow,
    OneshotWave,
    OneshotCheer,
    OneshotExclamation,
    OneshotQuestion,
    OneshotEat,
    StateDance,
    OneshotLaugh,
    StateSleep,
    StateSit,
    OneshotRude,
    OneshotRoar,
    OneshotKneel,
    OneshotKiss,
    OneshotCry,
    OneshotChicken,
    OneshotBeg,
    OneshotApplaud,
    OneshotShout,
    OneshotFlex,
    OneshotShy,
    OneshotPoint,
    StateStand,
    StateReadyUnarmed,
    StateWorkSheathed,
    StatePoint,
    StateNone,
    OneshotWound,
    OneshotWoundCritical,
    OneshotAttackUnarmed,
    OneshotAttack1h,
    OneshotAttack2htight,
    OneshotAttack2hLoose,
    OneshotParryUnarmed,
    OneshotParryShield,
    OneshotReadyUnarmed,
    OneshotReady1h,
    OneshotReadyBow,
    OneshotSpellPrecast,
    OneshotSpellCast,
    OneshotBattleRoar,
    OneshotSpecialattack1h,
    OneshotKick,
    OneshotAttackThrown,
    StateStun,
    StateDead,
    OneshotSalute,
    StateKneel,
    StateUseStanding,
    OneshotWaveNoSheathe,
    OneshotCheerNoSheathe,
    OneshotEatNoSheathe,
    StateStunNoSheathe,
    OneshotDance,
    OneshotSaluteNoSheath,
    StateUseStandingNoSheathe,
    OneshotLaughNoSheathe,
    StateWork,
    StateSpellPrecast,
    OneshotReadyRifle,
    StateReadyRifle,
    StateWorkMining,
    StateWorkChopwood,
    StateApplaud,
    OneshotLiftoff,
    OneshotYes,
    OneshotNo,
    OneshotTrain,
    OneshotLand,
    StateAtEase,
    StateReady1h,
    StateSpellKneelStart,
    StateSubmerged,
    OneshotSubmerge,
    StateReady2h,
    StateReadyBow,
    OneshotMountSpecial,
    StateTalk,
    StateFishing,
    OneshotFishing,
    OneshotLoot,
    StateWhirlwind,
    StateDrowned,
    StateHoldBow,
    StateHoldRifle,
    StateHoldThrown,
    OneshotDrown,
    OneshotStomp,
    OneshotAttackOff,
    OneshotAttackOffPierce,
    StateRoar,
    StateLaugh,
    OneshotCreatureSpecial,
    OneshotJumplandrun,
    OneshotJumpend,
    OneshotTalkNoSheathe,
    OneshotPointNoSheathe,
    StateCannibalize,
    OneshotJumpstart,
    StateDancespecial,
    OneshotDancespecial,
    OneshotCustomSpell01,
    OneshotCustomSpell02,
    OneshotCustomSpell03,
    OneshotCustomSpell04,
    OneshotCustomSpell05,
    OneshotCustomSpell06,
    OneshotCustomSpell07,
    OneshotCustomSpell08,
    OneshotCustomSpell09,
    OneshotCustomSpell10,
    StateExclaim,
    StateDanceCustom,
    StateSitChairMed,
    StateCustomSpell01,
    StateCustomSpell02,
    StateEat,
    StateCustomSpell04,
    StateCustomSpell03,
    StateCustomSpell05,
    StateSpelleffectHold,
    StateEatNoSheathe,
    StateMount,
    StateReady2hl,
    StateSitChairHigh,
    StateFall,
    StateLoot,
    StateSubmergedNew,
    OneshotCower,
    StateCower,
    OneshotUseStanding,
    StateStealthStand,
    OneshotOmnicastGhoul,
    OneshotAttackBow,
    OneshotAttackRifle,
    StateSwimIdle,
    StateAttackUnarmed,
    OneshotSpellCastWSound,
    OneshotDodge,
    OneshotParry1h,
    OneshotParry2h,
    OneshotParry2hl,
    StateFlyfall,
    OneshotFlydeath,
    StateFlyFall,
    OneshotFlySitGroundDown,
    OneshotFlySitGroundUp,
    OneshotEmerge,
    OneshotDragonSpit,
    StateSpecialUnarmed,
    OneshotFlygrab,
    StateFlygrabclosed,
    OneshotFlygrabthrown,
    StateFlySitGround,
    StateWalkBackwards,
    OneshotFlytalk,
    OneshotFlyattack1h,
    StateCustomSpell08,
    OneshotFlyDragonSpit,
    StateSitChairLow,
    OneshotStun,
    OneshotSpellCastOmni,
    StateReadyThrown,
    OneshotWorkChopwood,
    OneshotWorkMining,
    StateSpellChannelOmni,
    StateSpellChannelDirected,
    StandStateNone,
    StateReadyjoust,
    StateStrangulate,
    StateReadySpellOmni,
    StateHoldJoust,
    OneshotCryJaina,
}

impl Emote {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::OneshotNone => 0x0,
            Self::OneshotTalk => 0x1,
            Self::OneshotBow => 0x2,
            Self::OneshotWave => 0x3,
            Self::OneshotCheer => 0x4,
            Self::OneshotExclamation => 0x5,
            Self::OneshotQuestion => 0x6,
            Self::OneshotEat => 0x7,
            Self::StateDance => 0xa,
            Self::OneshotLaugh => 0xb,
            Self::StateSleep => 0xc,
            Self::StateSit => 0xd,
            Self::OneshotRude => 0xe,
            Self::OneshotRoar => 0xf,
            Self::OneshotKneel => 0x10,
            Self::OneshotKiss => 0x11,
            Self::OneshotCry => 0x12,
            Self::OneshotChicken => 0x13,
            Self::OneshotBeg => 0x14,
            Self::OneshotApplaud => 0x15,
            Self::OneshotShout => 0x16,
            Self::OneshotFlex => 0x17,
            Self::OneshotShy => 0x18,
            Self::OneshotPoint => 0x19,
            Self::StateStand => 0x1a,
            Self::StateReadyUnarmed => 0x1b,
            Self::StateWorkSheathed => 0x1c,
            Self::StatePoint => 0x1d,
            Self::StateNone => 0x1e,
            Self::OneshotWound => 0x21,
            Self::OneshotWoundCritical => 0x22,
            Self::OneshotAttackUnarmed => 0x23,
            Self::OneshotAttack1h => 0x24,
            Self::OneshotAttack2htight => 0x25,
            Self::OneshotAttack2hLoose => 0x26,
            Self::OneshotParryUnarmed => 0x27,
            Self::OneshotParryShield => 0x2b,
            Self::OneshotReadyUnarmed => 0x2c,
            Self::OneshotReady1h => 0x2d,
            Self::OneshotReadyBow => 0x30,
            Self::OneshotSpellPrecast => 0x32,
            Self::OneshotSpellCast => 0x33,
            Self::OneshotBattleRoar => 0x35,
            Self::OneshotSpecialattack1h => 0x36,
            Self::OneshotKick => 0x3c,
            Self::OneshotAttackThrown => 0x3d,
            Self::StateStun => 0x40,
            Self::StateDead => 0x41,
            Self::OneshotSalute => 0x42,
            Self::StateKneel => 0x44,
            Self::StateUseStanding => 0x45,
            Self::OneshotWaveNoSheathe => 0x46,
            Self::OneshotCheerNoSheathe => 0x47,
            Self::OneshotEatNoSheathe => 0x5c,
            Self::StateStunNoSheathe => 0x5d,
            Self::OneshotDance => 0x5e,
            Self::OneshotSaluteNoSheath => 0x71,
            Self::StateUseStandingNoSheathe => 0x85,
            Self::OneshotLaughNoSheathe => 0x99,
            Self::StateWork => 0xad,
            Self::StateSpellPrecast => 0xc1,
            Self::OneshotReadyRifle => 0xd5,
            Self::StateReadyRifle => 0xd6,
            Self::StateWorkMining => 0xe9,
            Self::StateWorkChopwood => 0xea,
            Self::StateApplaud => 0xfd,
            Self::OneshotLiftoff => 0xfe,
            Self::OneshotYes => 0x111,
            Self::OneshotNo => 0x112,
            Self::OneshotTrain => 0x113,
            Self::OneshotLand => 0x125,
            Self::StateAtEase => 0x139,
            Self::StateReady1h => 0x14d,
            Self::StateSpellKneelStart => 0x161,
            Self::StateSubmerged => 0x175,
            Self::OneshotSubmerge => 0x176,
            Self::StateReady2h => 0x177,
            Self::StateReadyBow => 0x178,
            Self::OneshotMountSpecial => 0x179,
            Self::StateTalk => 0x17a,
            Self::StateFishing => 0x17b,
            Self::OneshotFishing => 0x17c,
            Self::OneshotLoot => 0x17d,
            Self::StateWhirlwind => 0x17e,
            Self::StateDrowned => 0x17f,
            Self::StateHoldBow => 0x180,
            Self::StateHoldRifle => 0x181,
            Self::StateHoldThrown => 0x182,
            Self::OneshotDrown => 0x183,
            Self::OneshotStomp => 0x184,
            Self::OneshotAttackOff => 0x185,
            Self::OneshotAttackOffPierce => 0x186,
            Self::StateRoar => 0x187,
            Self::StateLaugh => 0x188,
            Self::OneshotCreatureSpecial => 0x189,
            Self::OneshotJumplandrun => 0x18a,
            Self::OneshotJumpend => 0x18b,
            Self::OneshotTalkNoSheathe => 0x18c,
            Self::OneshotPointNoSheathe => 0x18d,
            Self::StateCannibalize => 0x18e,
            Self::OneshotJumpstart => 0x18f,
            Self::StateDancespecial => 0x190,
            Self::OneshotDancespecial => 0x191,
            Self::OneshotCustomSpell01 => 0x192,
            Self::OneshotCustomSpell02 => 0x193,
            Self::OneshotCustomSpell03 => 0x194,
            Self::OneshotCustomSpell04 => 0x195,
            Self::OneshotCustomSpell05 => 0x196,
            Self::OneshotCustomSpell06 => 0x197,
            Self::OneshotCustomSpell07 => 0x198,
            Self::OneshotCustomSpell08 => 0x199,
            Self::OneshotCustomSpell09 => 0x19a,
            Self::OneshotCustomSpell10 => 0x19b,
            Self::StateExclaim => 0x19c,
            Self::StateDanceCustom => 0x19d,
            Self::StateSitChairMed => 0x19f,
            Self::StateCustomSpell01 => 0x1a0,
            Self::StateCustomSpell02 => 0x1a1,
            Self::StateEat => 0x1a2,
            Self::StateCustomSpell04 => 0x1a3,
            Self::StateCustomSpell03 => 0x1a4,
            Self::StateCustomSpell05 => 0x1a5,
            Self::StateSpelleffectHold => 0x1a6,
            Self::StateEatNoSheathe => 0x1a7,
            Self::StateMount => 0x1a8,
            Self::StateReady2hl => 0x1a9,
            Self::StateSitChairHigh => 0x1aa,
            Self::StateFall => 0x1ab,
            Self::StateLoot => 0x1ac,
            Self::StateSubmergedNew => 0x1ad,
            Self::OneshotCower => 0x1ae,
            Self::StateCower => 0x1af,
            Self::OneshotUseStanding => 0x1b0,
            Self::StateStealthStand => 0x1b1,
            Self::OneshotOmnicastGhoul => 0x1b2,
            Self::OneshotAttackBow => 0x1b3,
            Self::OneshotAttackRifle => 0x1b4,
            Self::StateSwimIdle => 0x1b5,
            Self::StateAttackUnarmed => 0x1b6,
            Self::OneshotSpellCastWSound => 0x1b7,
            Self::OneshotDodge => 0x1b8,
            Self::OneshotParry1h => 0x1b9,
            Self::OneshotParry2h => 0x1ba,
            Self::OneshotParry2hl => 0x1bb,
            Self::StateFlyfall => 0x1bc,
            Self::OneshotFlydeath => 0x1bd,
            Self::StateFlyFall => 0x1be,
            Self::OneshotFlySitGroundDown => 0x1bf,
            Self::OneshotFlySitGroundUp => 0x1c0,
            Self::OneshotEmerge => 0x1c1,
            Self::OneshotDragonSpit => 0x1c2,
            Self::StateSpecialUnarmed => 0x1c3,
            Self::OneshotFlygrab => 0x1c4,
            Self::StateFlygrabclosed => 0x1c5,
            Self::OneshotFlygrabthrown => 0x1c6,
            Self::StateFlySitGround => 0x1c7,
            Self::StateWalkBackwards => 0x1c8,
            Self::OneshotFlytalk => 0x1c9,
            Self::OneshotFlyattack1h => 0x1ca,
            Self::StateCustomSpell08 => 0x1cb,
            Self::OneshotFlyDragonSpit => 0x1cc,
            Self::StateSitChairLow => 0x1cd,
            Self::OneshotStun => 0x1ce,
            Self::OneshotSpellCastOmni => 0x1cf,
            Self::StateReadyThrown => 0x1d1,
            Self::OneshotWorkChopwood => 0x1d2,
            Self::OneshotWorkMining => 0x1d3,
            Self::StateSpellChannelOmni => 0x1d4,
            Self::StateSpellChannelDirected => 0x1d5,
            Self::StandStateNone => 0x1d6,
            Self::StateReadyjoust => 0x1d7,
            Self::StateStrangulate => 0x1d9,
            Self::StateReadySpellOmni => 0x1da,
            Self::StateHoldJoust => 0x1db,
            Self::OneshotCryJaina => 0x1dc,
        }
    }

    pub const fn variants() -> [Self; 175] {
        [
            Self::OneshotNone,
            Self::OneshotTalk,
            Self::OneshotBow,
            Self::OneshotWave,
            Self::OneshotCheer,
            Self::OneshotExclamation,
            Self::OneshotQuestion,
            Self::OneshotEat,
            Self::StateDance,
            Self::OneshotLaugh,
            Self::StateSleep,
            Self::StateSit,
            Self::OneshotRude,
            Self::OneshotRoar,
            Self::OneshotKneel,
            Self::OneshotKiss,
            Self::OneshotCry,
            Self::OneshotChicken,
            Self::OneshotBeg,
            Self::OneshotApplaud,
            Self::OneshotShout,
            Self::OneshotFlex,
            Self::OneshotShy,
            Self::OneshotPoint,
            Self::StateStand,
            Self::StateReadyUnarmed,
            Self::StateWorkSheathed,
            Self::StatePoint,
            Self::StateNone,
            Self::OneshotWound,
            Self::OneshotWoundCritical,
            Self::OneshotAttackUnarmed,
            Self::OneshotAttack1h,
            Self::OneshotAttack2htight,
            Self::OneshotAttack2hLoose,
            Self::OneshotParryUnarmed,
            Self::OneshotParryShield,
            Self::OneshotReadyUnarmed,
            Self::OneshotReady1h,
            Self::OneshotReadyBow,
            Self::OneshotSpellPrecast,
            Self::OneshotSpellCast,
            Self::OneshotBattleRoar,
            Self::OneshotSpecialattack1h,
            Self::OneshotKick,
            Self::OneshotAttackThrown,
            Self::StateStun,
            Self::StateDead,
            Self::OneshotSalute,
            Self::StateKneel,
            Self::StateUseStanding,
            Self::OneshotWaveNoSheathe,
            Self::OneshotCheerNoSheathe,
            Self::OneshotEatNoSheathe,
            Self::StateStunNoSheathe,
            Self::OneshotDance,
            Self::OneshotSaluteNoSheath,
            Self::StateUseStandingNoSheathe,
            Self::OneshotLaughNoSheathe,
            Self::StateWork,
            Self::StateSpellPrecast,
            Self::OneshotReadyRifle,
            Self::StateReadyRifle,
            Self::StateWorkMining,
            Self::StateWorkChopwood,
            Self::StateApplaud,
            Self::OneshotLiftoff,
            Self::OneshotYes,
            Self::OneshotNo,
            Self::OneshotTrain,
            Self::OneshotLand,
            Self::StateAtEase,
            Self::StateReady1h,
            Self::StateSpellKneelStart,
            Self::StateSubmerged,
            Self::OneshotSubmerge,
            Self::StateReady2h,
            Self::StateReadyBow,
            Self::OneshotMountSpecial,
            Self::StateTalk,
            Self::StateFishing,
            Self::OneshotFishing,
            Self::OneshotLoot,
            Self::StateWhirlwind,
            Self::StateDrowned,
            Self::StateHoldBow,
            Self::StateHoldRifle,
            Self::StateHoldThrown,
            Self::OneshotDrown,
            Self::OneshotStomp,
            Self::OneshotAttackOff,
            Self::OneshotAttackOffPierce,
            Self::StateRoar,
            Self::StateLaugh,
            Self::OneshotCreatureSpecial,
            Self::OneshotJumplandrun,
            Self::OneshotJumpend,
            Self::OneshotTalkNoSheathe,
            Self::OneshotPointNoSheathe,
            Self::StateCannibalize,
            Self::OneshotJumpstart,
            Self::StateDancespecial,
            Self::OneshotDancespecial,
            Self::OneshotCustomSpell01,
            Self::OneshotCustomSpell02,
            Self::OneshotCustomSpell03,
            Self::OneshotCustomSpell04,
            Self::OneshotCustomSpell05,
            Self::OneshotCustomSpell06,
            Self::OneshotCustomSpell07,
            Self::OneshotCustomSpell08,
            Self::OneshotCustomSpell09,
            Self::OneshotCustomSpell10,
            Self::StateExclaim,
            Self::StateDanceCustom,
            Self::StateSitChairMed,
            Self::StateCustomSpell01,
            Self::StateCustomSpell02,
            Self::StateEat,
            Self::StateCustomSpell04,
            Self::StateCustomSpell03,
            Self::StateCustomSpell05,
            Self::StateSpelleffectHold,
            Self::StateEatNoSheathe,
            Self::StateMount,
            Self::StateReady2hl,
            Self::StateSitChairHigh,
            Self::StateFall,
            Self::StateLoot,
            Self::StateSubmergedNew,
            Self::OneshotCower,
            Self::StateCower,
            Self::OneshotUseStanding,
            Self::StateStealthStand,
            Self::OneshotOmnicastGhoul,
            Self::OneshotAttackBow,
            Self::OneshotAttackRifle,
            Self::StateSwimIdle,
            Self::StateAttackUnarmed,
            Self::OneshotSpellCastWSound,
            Self::OneshotDodge,
            Self::OneshotParry1h,
            Self::OneshotParry2h,
            Self::OneshotParry2hl,
            Self::StateFlyfall,
            Self::OneshotFlydeath,
            Self::StateFlyFall,
            Self::OneshotFlySitGroundDown,
            Self::OneshotFlySitGroundUp,
            Self::OneshotEmerge,
            Self::OneshotDragonSpit,
            Self::StateSpecialUnarmed,
            Self::OneshotFlygrab,
            Self::StateFlygrabclosed,
            Self::OneshotFlygrabthrown,
            Self::StateFlySitGround,
            Self::StateWalkBackwards,
            Self::OneshotFlytalk,
            Self::OneshotFlyattack1h,
            Self::StateCustomSpell08,
            Self::OneshotFlyDragonSpit,
            Self::StateSitChairLow,
            Self::OneshotStun,
            Self::OneshotSpellCastOmni,
            Self::StateReadyThrown,
            Self::OneshotWorkChopwood,
            Self::OneshotWorkMining,
            Self::StateSpellChannelOmni,
            Self::StateSpellChannelDirected,
            Self::StandStateNone,
            Self::StateReadyjoust,
            Self::StateStrangulate,
            Self::StateReadySpellOmni,
            Self::StateHoldJoust,
            Self::OneshotCryJaina,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::OneshotNone),
            1 => Ok(Self::OneshotTalk),
            2 => Ok(Self::OneshotBow),
            3 => Ok(Self::OneshotWave),
            4 => Ok(Self::OneshotCheer),
            5 => Ok(Self::OneshotExclamation),
            6 => Ok(Self::OneshotQuestion),
            7 => Ok(Self::OneshotEat),
            10 => Ok(Self::StateDance),
            11 => Ok(Self::OneshotLaugh),
            12 => Ok(Self::StateSleep),
            13 => Ok(Self::StateSit),
            14 => Ok(Self::OneshotRude),
            15 => Ok(Self::OneshotRoar),
            16 => Ok(Self::OneshotKneel),
            17 => Ok(Self::OneshotKiss),
            18 => Ok(Self::OneshotCry),
            19 => Ok(Self::OneshotChicken),
            20 => Ok(Self::OneshotBeg),
            21 => Ok(Self::OneshotApplaud),
            22 => Ok(Self::OneshotShout),
            23 => Ok(Self::OneshotFlex),
            24 => Ok(Self::OneshotShy),
            25 => Ok(Self::OneshotPoint),
            26 => Ok(Self::StateStand),
            27 => Ok(Self::StateReadyUnarmed),
            28 => Ok(Self::StateWorkSheathed),
            29 => Ok(Self::StatePoint),
            30 => Ok(Self::StateNone),
            33 => Ok(Self::OneshotWound),
            34 => Ok(Self::OneshotWoundCritical),
            35 => Ok(Self::OneshotAttackUnarmed),
            36 => Ok(Self::OneshotAttack1h),
            37 => Ok(Self::OneshotAttack2htight),
            38 => Ok(Self::OneshotAttack2hLoose),
            39 => Ok(Self::OneshotParryUnarmed),
            43 => Ok(Self::OneshotParryShield),
            44 => Ok(Self::OneshotReadyUnarmed),
            45 => Ok(Self::OneshotReady1h),
            48 => Ok(Self::OneshotReadyBow),
            50 => Ok(Self::OneshotSpellPrecast),
            51 => Ok(Self::OneshotSpellCast),
            53 => Ok(Self::OneshotBattleRoar),
            54 => Ok(Self::OneshotSpecialattack1h),
            60 => Ok(Self::OneshotKick),
            61 => Ok(Self::OneshotAttackThrown),
            64 => Ok(Self::StateStun),
            65 => Ok(Self::StateDead),
            66 => Ok(Self::OneshotSalute),
            68 => Ok(Self::StateKneel),
            69 => Ok(Self::StateUseStanding),
            70 => Ok(Self::OneshotWaveNoSheathe),
            71 => Ok(Self::OneshotCheerNoSheathe),
            92 => Ok(Self::OneshotEatNoSheathe),
            93 => Ok(Self::StateStunNoSheathe),
            94 => Ok(Self::OneshotDance),
            113 => Ok(Self::OneshotSaluteNoSheath),
            133 => Ok(Self::StateUseStandingNoSheathe),
            153 => Ok(Self::OneshotLaughNoSheathe),
            173 => Ok(Self::StateWork),
            193 => Ok(Self::StateSpellPrecast),
            213 => Ok(Self::OneshotReadyRifle),
            214 => Ok(Self::StateReadyRifle),
            233 => Ok(Self::StateWorkMining),
            234 => Ok(Self::StateWorkChopwood),
            253 => Ok(Self::StateApplaud),
            254 => Ok(Self::OneshotLiftoff),
            273 => Ok(Self::OneshotYes),
            274 => Ok(Self::OneshotNo),
            275 => Ok(Self::OneshotTrain),
            293 => Ok(Self::OneshotLand),
            313 => Ok(Self::StateAtEase),
            333 => Ok(Self::StateReady1h),
            353 => Ok(Self::StateSpellKneelStart),
            373 => Ok(Self::StateSubmerged),
            374 => Ok(Self::OneshotSubmerge),
            375 => Ok(Self::StateReady2h),
            376 => Ok(Self::StateReadyBow),
            377 => Ok(Self::OneshotMountSpecial),
            378 => Ok(Self::StateTalk),
            379 => Ok(Self::StateFishing),
            380 => Ok(Self::OneshotFishing),
            381 => Ok(Self::OneshotLoot),
            382 => Ok(Self::StateWhirlwind),
            383 => Ok(Self::StateDrowned),
            384 => Ok(Self::StateHoldBow),
            385 => Ok(Self::StateHoldRifle),
            386 => Ok(Self::StateHoldThrown),
            387 => Ok(Self::OneshotDrown),
            388 => Ok(Self::OneshotStomp),
            389 => Ok(Self::OneshotAttackOff),
            390 => Ok(Self::OneshotAttackOffPierce),
            391 => Ok(Self::StateRoar),
            392 => Ok(Self::StateLaugh),
            393 => Ok(Self::OneshotCreatureSpecial),
            394 => Ok(Self::OneshotJumplandrun),
            395 => Ok(Self::OneshotJumpend),
            396 => Ok(Self::OneshotTalkNoSheathe),
            397 => Ok(Self::OneshotPointNoSheathe),
            398 => Ok(Self::StateCannibalize),
            399 => Ok(Self::OneshotJumpstart),
            400 => Ok(Self::StateDancespecial),
            401 => Ok(Self::OneshotDancespecial),
            402 => Ok(Self::OneshotCustomSpell01),
            403 => Ok(Self::OneshotCustomSpell02),
            404 => Ok(Self::OneshotCustomSpell03),
            405 => Ok(Self::OneshotCustomSpell04),
            406 => Ok(Self::OneshotCustomSpell05),
            407 => Ok(Self::OneshotCustomSpell06),
            408 => Ok(Self::OneshotCustomSpell07),
            409 => Ok(Self::OneshotCustomSpell08),
            410 => Ok(Self::OneshotCustomSpell09),
            411 => Ok(Self::OneshotCustomSpell10),
            412 => Ok(Self::StateExclaim),
            413 => Ok(Self::StateDanceCustom),
            415 => Ok(Self::StateSitChairMed),
            416 => Ok(Self::StateCustomSpell01),
            417 => Ok(Self::StateCustomSpell02),
            418 => Ok(Self::StateEat),
            419 => Ok(Self::StateCustomSpell04),
            420 => Ok(Self::StateCustomSpell03),
            421 => Ok(Self::StateCustomSpell05),
            422 => Ok(Self::StateSpelleffectHold),
            423 => Ok(Self::StateEatNoSheathe),
            424 => Ok(Self::StateMount),
            425 => Ok(Self::StateReady2hl),
            426 => Ok(Self::StateSitChairHigh),
            427 => Ok(Self::StateFall),
            428 => Ok(Self::StateLoot),
            429 => Ok(Self::StateSubmergedNew),
            430 => Ok(Self::OneshotCower),
            431 => Ok(Self::StateCower),
            432 => Ok(Self::OneshotUseStanding),
            433 => Ok(Self::StateStealthStand),
            434 => Ok(Self::OneshotOmnicastGhoul),
            435 => Ok(Self::OneshotAttackBow),
            436 => Ok(Self::OneshotAttackRifle),
            437 => Ok(Self::StateSwimIdle),
            438 => Ok(Self::StateAttackUnarmed),
            439 => Ok(Self::OneshotSpellCastWSound),
            440 => Ok(Self::OneshotDodge),
            441 => Ok(Self::OneshotParry1h),
            442 => Ok(Self::OneshotParry2h),
            443 => Ok(Self::OneshotParry2hl),
            444 => Ok(Self::StateFlyfall),
            445 => Ok(Self::OneshotFlydeath),
            446 => Ok(Self::StateFlyFall),
            447 => Ok(Self::OneshotFlySitGroundDown),
            448 => Ok(Self::OneshotFlySitGroundUp),
            449 => Ok(Self::OneshotEmerge),
            450 => Ok(Self::OneshotDragonSpit),
            451 => Ok(Self::StateSpecialUnarmed),
            452 => Ok(Self::OneshotFlygrab),
            453 => Ok(Self::StateFlygrabclosed),
            454 => Ok(Self::OneshotFlygrabthrown),
            455 => Ok(Self::StateFlySitGround),
            456 => Ok(Self::StateWalkBackwards),
            457 => Ok(Self::OneshotFlytalk),
            458 => Ok(Self::OneshotFlyattack1h),
            459 => Ok(Self::StateCustomSpell08),
            460 => Ok(Self::OneshotFlyDragonSpit),
            461 => Ok(Self::StateSitChairLow),
            462 => Ok(Self::OneshotStun),
            463 => Ok(Self::OneshotSpellCastOmni),
            465 => Ok(Self::StateReadyThrown),
            466 => Ok(Self::OneshotWorkChopwood),
            467 => Ok(Self::OneshotWorkMining),
            468 => Ok(Self::StateSpellChannelOmni),
            469 => Ok(Self::StateSpellChannelDirected),
            470 => Ok(Self::StandStateNone),
            471 => Ok(Self::StateReadyjoust),
            473 => Ok(Self::StateStrangulate),
            474 => Ok(Self::StateReadySpellOmni),
            475 => Ok(Self::StateHoldJoust),
            476 => Ok(Self::OneshotCryJaina),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl Emote {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::OneshotNone => "ONESHOT_NONE",
            Self::OneshotTalk => "ONESHOT_TALK",
            Self::OneshotBow => "ONESHOT_BOW",
            Self::OneshotWave => "ONESHOT_WAVE",
            Self::OneshotCheer => "ONESHOT_CHEER",
            Self::OneshotExclamation => "ONESHOT_EXCLAMATION",
            Self::OneshotQuestion => "ONESHOT_QUESTION",
            Self::OneshotEat => "ONESHOT_EAT",
            Self::StateDance => "STATE_DANCE",
            Self::OneshotLaugh => "ONESHOT_LAUGH",
            Self::StateSleep => "STATE_SLEEP",
            Self::StateSit => "STATE_SIT",
            Self::OneshotRude => "ONESHOT_RUDE",
            Self::OneshotRoar => "ONESHOT_ROAR",
            Self::OneshotKneel => "ONESHOT_KNEEL",
            Self::OneshotKiss => "ONESHOT_KISS",
            Self::OneshotCry => "ONESHOT_CRY",
            Self::OneshotChicken => "ONESHOT_CHICKEN",
            Self::OneshotBeg => "ONESHOT_BEG",
            Self::OneshotApplaud => "ONESHOT_APPLAUD",
            Self::OneshotShout => "ONESHOT_SHOUT",
            Self::OneshotFlex => "ONESHOT_FLEX",
            Self::OneshotShy => "ONESHOT_SHY",
            Self::OneshotPoint => "ONESHOT_POINT",
            Self::StateStand => "STATE_STAND",
            Self::StateReadyUnarmed => "STATE_READY_UNARMED",
            Self::StateWorkSheathed => "STATE_WORK_SHEATHED",
            Self::StatePoint => "STATE_POINT",
            Self::StateNone => "STATE_NONE",
            Self::OneshotWound => "ONESHOT_WOUND",
            Self::OneshotWoundCritical => "ONESHOT_WOUND_CRITICAL",
            Self::OneshotAttackUnarmed => "ONESHOT_ATTACK_UNARMED",
            Self::OneshotAttack1h => "ONESHOT_ATTACK1H",
            Self::OneshotAttack2htight => "ONESHOT_ATTACK2HTIGHT",
            Self::OneshotAttack2hLoose => "ONESHOT_ATTACK2H_LOOSE",
            Self::OneshotParryUnarmed => "ONESHOT_PARRY_UNARMED",
            Self::OneshotParryShield => "ONESHOT_PARRY_SHIELD",
            Self::OneshotReadyUnarmed => "ONESHOT_READY_UNARMED",
            Self::OneshotReady1h => "ONESHOT_READY1H",
            Self::OneshotReadyBow => "ONESHOT_READY_BOW",
            Self::OneshotSpellPrecast => "ONESHOT_SPELL_PRECAST",
            Self::OneshotSpellCast => "ONESHOT_SPELL_CAST",
            Self::OneshotBattleRoar => "ONESHOT_BATTLE_ROAR",
            Self::OneshotSpecialattack1h => "ONESHOT_SPECIALATTACK1H",
            Self::OneshotKick => "ONESHOT_KICK",
            Self::OneshotAttackThrown => "ONESHOT_ATTACK_THROWN",
            Self::StateStun => "STATE_STUN",
            Self::StateDead => "STATE_DEAD",
            Self::OneshotSalute => "ONESHOT_SALUTE",
            Self::StateKneel => "STATE_KNEEL",
            Self::StateUseStanding => "STATE_USE_STANDING",
            Self::OneshotWaveNoSheathe => "ONESHOT_WAVE_NO_SHEATHE",
            Self::OneshotCheerNoSheathe => "ONESHOT_CHEER_NO_SHEATHE",
            Self::OneshotEatNoSheathe => "ONESHOT_EAT_NO_SHEATHE",
            Self::StateStunNoSheathe => "STATE_STUN_NO_SHEATHE",
            Self::OneshotDance => "ONESHOT_DANCE",
            Self::OneshotSaluteNoSheath => "ONESHOT_SALUTE_NO_SHEATH",
            Self::StateUseStandingNoSheathe => "STATE_USE_STANDING_NO_SHEATHE",
            Self::OneshotLaughNoSheathe => "ONESHOT_LAUGH_NO_SHEATHE",
            Self::StateWork => "STATE_WORK",
            Self::StateSpellPrecast => "STATE_SPELL_PRECAST",
            Self::OneshotReadyRifle => "ONESHOT_READY_RIFLE",
            Self::StateReadyRifle => "STATE_READY_RIFLE",
            Self::StateWorkMining => "STATE_WORK_MINING",
            Self::StateWorkChopwood => "STATE_WORK_CHOPWOOD",
            Self::StateApplaud => "STATE_APPLAUD",
            Self::OneshotLiftoff => "ONESHOT_LIFTOFF",
            Self::OneshotYes => "ONESHOT_YES",
            Self::OneshotNo => "ONESHOT_NO",
            Self::OneshotTrain => "ONESHOT_TRAIN",
            Self::OneshotLand => "ONESHOT_LAND",
            Self::StateAtEase => "STATE_AT_EASE",
            Self::StateReady1h => "STATE_READY1H",
            Self::StateSpellKneelStart => "STATE_SPELL_KNEEL_START",
            Self::StateSubmerged => "STATE_SUBMERGED",
            Self::OneshotSubmerge => "ONESHOT_SUBMERGE",
            Self::StateReady2h => "STATE_READY2H",
            Self::StateReadyBow => "STATE_READY_BOW",
            Self::OneshotMountSpecial => "ONESHOT_MOUNT_SPECIAL",
            Self::StateTalk => "STATE_TALK",
            Self::StateFishing => "STATE_FISHING",
            Self::OneshotFishing => "ONESHOT_FISHING",
            Self::OneshotLoot => "ONESHOT_LOOT",
            Self::StateWhirlwind => "STATE_WHIRLWIND",
            Self::StateDrowned => "STATE_DROWNED",
            Self::StateHoldBow => "STATE_HOLD_BOW",
            Self::StateHoldRifle => "STATE_HOLD_RIFLE",
            Self::StateHoldThrown => "STATE_HOLD_THROWN",
            Self::OneshotDrown => "ONESHOT_DROWN",
            Self::OneshotStomp => "ONESHOT_STOMP",
            Self::OneshotAttackOff => "ONESHOT_ATTACK_OFF",
            Self::OneshotAttackOffPierce => "ONESHOT_ATTACK_OFF_PIERCE",
            Self::StateRoar => "STATE_ROAR",
            Self::StateLaugh => "STATE_LAUGH",
            Self::OneshotCreatureSpecial => "ONESHOT_CREATURE_SPECIAL",
            Self::OneshotJumplandrun => "ONESHOT_JUMPLANDRUN",
            Self::OneshotJumpend => "ONESHOT_JUMPEND",
            Self::OneshotTalkNoSheathe => "ONESHOT_TALK_NO_SHEATHE",
            Self::OneshotPointNoSheathe => "ONESHOT_POINT_NO_SHEATHE",
            Self::StateCannibalize => "STATE_CANNIBALIZE",
            Self::OneshotJumpstart => "ONESHOT_JUMPSTART",
            Self::StateDancespecial => "STATE_DANCESPECIAL",
            Self::OneshotDancespecial => "ONESHOT_DANCESPECIAL",
            Self::OneshotCustomSpell01 => "ONESHOT_CUSTOM_SPELL_01",
            Self::OneshotCustomSpell02 => "ONESHOT_CUSTOM_SPELL_02",
            Self::OneshotCustomSpell03 => "ONESHOT_CUSTOM_SPELL_03",
            Self::OneshotCustomSpell04 => "ONESHOT_CUSTOM_SPELL_04",
            Self::OneshotCustomSpell05 => "ONESHOT_CUSTOM_SPELL_05",
            Self::OneshotCustomSpell06 => "ONESHOT_CUSTOM_SPELL_06",
            Self::OneshotCustomSpell07 => "ONESHOT_CUSTOM_SPELL_07",
            Self::OneshotCustomSpell08 => "ONESHOT_CUSTOM_SPELL_08",
            Self::OneshotCustomSpell09 => "ONESHOT_CUSTOM_SPELL_09",
            Self::OneshotCustomSpell10 => "ONESHOT_CUSTOM_SPELL_10",
            Self::StateExclaim => "STATE_EXCLAIM",
            Self::StateDanceCustom => "STATE_DANCE_CUSTOM",
            Self::StateSitChairMed => "STATE_SIT_CHAIR_MED",
            Self::StateCustomSpell01 => "STATE_CUSTOM_SPELL_01",
            Self::StateCustomSpell02 => "STATE_CUSTOM_SPELL_02",
            Self::StateEat => "STATE_EAT",
            Self::StateCustomSpell04 => "STATE_CUSTOM_SPELL_04",
            Self::StateCustomSpell03 => "STATE_CUSTOM_SPELL_03",
            Self::StateCustomSpell05 => "STATE_CUSTOM_SPELL_05",
            Self::StateSpelleffectHold => "STATE_SPELLEFFECT_HOLD",
            Self::StateEatNoSheathe => "STATE_EAT_NO_SHEATHE",
            Self::StateMount => "STATE_MOUNT",
            Self::StateReady2hl => "STATE_READY2HL",
            Self::StateSitChairHigh => "STATE_SIT_CHAIR_HIGH",
            Self::StateFall => "STATE_FALL",
            Self::StateLoot => "STATE_LOOT",
            Self::StateSubmergedNew => "STATE_SUBMERGED_NEW",
            Self::OneshotCower => "ONESHOT_COWER",
            Self::StateCower => "STATE_COWER",
            Self::OneshotUseStanding => "ONESHOT_USE_STANDING",
            Self::StateStealthStand => "STATE_STEALTH_STAND",
            Self::OneshotOmnicastGhoul => "ONESHOT_OMNICAST_GHOUL",
            Self::OneshotAttackBow => "ONESHOT_ATTACK_BOW",
            Self::OneshotAttackRifle => "ONESHOT_ATTACK_RIFLE",
            Self::StateSwimIdle => "STATE_SWIM_IDLE",
            Self::StateAttackUnarmed => "STATE_ATTACK_UNARMED",
            Self::OneshotSpellCastWSound => "ONESHOT_SPELL_CAST_W_SOUND",
            Self::OneshotDodge => "ONESHOT_DODGE",
            Self::OneshotParry1h => "ONESHOT_PARRY1H",
            Self::OneshotParry2h => "ONESHOT_PARRY2H",
            Self::OneshotParry2hl => "ONESHOT_PARRY2HL",
            Self::StateFlyfall => "STATE_FLYFALL",
            Self::OneshotFlydeath => "ONESHOT_FLYDEATH",
            Self::StateFlyFall => "STATE_FLY_FALL",
            Self::OneshotFlySitGroundDown => "ONESHOT_FLY_SIT_GROUND_DOWN",
            Self::OneshotFlySitGroundUp => "ONESHOT_FLY_SIT_GROUND_UP",
            Self::OneshotEmerge => "ONESHOT_EMERGE",
            Self::OneshotDragonSpit => "ONESHOT_DRAGON_SPIT",
            Self::StateSpecialUnarmed => "STATE_SPECIAL_UNARMED",
            Self::OneshotFlygrab => "ONESHOT_FLYGRAB",
            Self::StateFlygrabclosed => "STATE_FLYGRABCLOSED",
            Self::OneshotFlygrabthrown => "ONESHOT_FLYGRABTHROWN",
            Self::StateFlySitGround => "STATE_FLY_SIT_GROUND",
            Self::StateWalkBackwards => "STATE_WALK_BACKWARDS",
            Self::OneshotFlytalk => "ONESHOT_FLYTALK",
            Self::OneshotFlyattack1h => "ONESHOT_FLYATTACK1H",
            Self::StateCustomSpell08 => "STATE_CUSTOM_SPELL_08",
            Self::OneshotFlyDragonSpit => "ONESHOT_FLY_DRAGON_SPIT",
            Self::StateSitChairLow => "STATE_SIT_CHAIR_LOW",
            Self::OneshotStun => "ONESHOT_STUN",
            Self::OneshotSpellCastOmni => "ONESHOT_SPELL_CAST_OMNI",
            Self::StateReadyThrown => "STATE_READY_THROWN",
            Self::OneshotWorkChopwood => "ONESHOT_WORK_CHOPWOOD",
            Self::OneshotWorkMining => "ONESHOT_WORK_MINING",
            Self::StateSpellChannelOmni => "STATE_SPELL_CHANNEL_OMNI",
            Self::StateSpellChannelDirected => "STATE_SPELL_CHANNEL_DIRECTED",
            Self::StandStateNone => "STAND_STATE_NONE",
            Self::StateReadyjoust => "STATE_READYJOUST",
            Self::StateStrangulate => "STATE_STRANGULATE",
            Self::StateReadySpellOmni => "STATE_READY_SPELL_OMNI",
            Self::StateHoldJoust => "STATE_HOLD_JOUST",
            Self::OneshotCryJaina => "ONESHOT_CRY_JAINA",
        }
    }

}

const NAME: &str = "Emote";

impl Default for Emote {
    fn default() -> Self {
        Self::OneshotNone
    }
}

impl std::fmt::Display for Emote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneshotNone => f.write_str("OneshotNone"),
            Self::OneshotTalk => f.write_str("OneshotTalk"),
            Self::OneshotBow => f.write_str("OneshotBow"),
            Self::OneshotWave => f.write_str("OneshotWave"),
            Self::OneshotCheer => f.write_str("OneshotCheer"),
            Self::OneshotExclamation => f.write_str("OneshotExclamation"),
            Self::OneshotQuestion => f.write_str("OneshotQuestion"),
            Self::OneshotEat => f.write_str("OneshotEat"),
            Self::StateDance => f.write_str("StateDance"),
            Self::OneshotLaugh => f.write_str("OneshotLaugh"),
            Self::StateSleep => f.write_str("StateSleep"),
            Self::StateSit => f.write_str("StateSit"),
            Self::OneshotRude => f.write_str("OneshotRude"),
            Self::OneshotRoar => f.write_str("OneshotRoar"),
            Self::OneshotKneel => f.write_str("OneshotKneel"),
            Self::OneshotKiss => f.write_str("OneshotKiss"),
            Self::OneshotCry => f.write_str("OneshotCry"),
            Self::OneshotChicken => f.write_str("OneshotChicken"),
            Self::OneshotBeg => f.write_str("OneshotBeg"),
            Self::OneshotApplaud => f.write_str("OneshotApplaud"),
            Self::OneshotShout => f.write_str("OneshotShout"),
            Self::OneshotFlex => f.write_str("OneshotFlex"),
            Self::OneshotShy => f.write_str("OneshotShy"),
            Self::OneshotPoint => f.write_str("OneshotPoint"),
            Self::StateStand => f.write_str("StateStand"),
            Self::StateReadyUnarmed => f.write_str("StateReadyUnarmed"),
            Self::StateWorkSheathed => f.write_str("StateWorkSheathed"),
            Self::StatePoint => f.write_str("StatePoint"),
            Self::StateNone => f.write_str("StateNone"),
            Self::OneshotWound => f.write_str("OneshotWound"),
            Self::OneshotWoundCritical => f.write_str("OneshotWoundCritical"),
            Self::OneshotAttackUnarmed => f.write_str("OneshotAttackUnarmed"),
            Self::OneshotAttack1h => f.write_str("OneshotAttack1h"),
            Self::OneshotAttack2htight => f.write_str("OneshotAttack2htight"),
            Self::OneshotAttack2hLoose => f.write_str("OneshotAttack2hLoose"),
            Self::OneshotParryUnarmed => f.write_str("OneshotParryUnarmed"),
            Self::OneshotParryShield => f.write_str("OneshotParryShield"),
            Self::OneshotReadyUnarmed => f.write_str("OneshotReadyUnarmed"),
            Self::OneshotReady1h => f.write_str("OneshotReady1h"),
            Self::OneshotReadyBow => f.write_str("OneshotReadyBow"),
            Self::OneshotSpellPrecast => f.write_str("OneshotSpellPrecast"),
            Self::OneshotSpellCast => f.write_str("OneshotSpellCast"),
            Self::OneshotBattleRoar => f.write_str("OneshotBattleRoar"),
            Self::OneshotSpecialattack1h => f.write_str("OneshotSpecialattack1h"),
            Self::OneshotKick => f.write_str("OneshotKick"),
            Self::OneshotAttackThrown => f.write_str("OneshotAttackThrown"),
            Self::StateStun => f.write_str("StateStun"),
            Self::StateDead => f.write_str("StateDead"),
            Self::OneshotSalute => f.write_str("OneshotSalute"),
            Self::StateKneel => f.write_str("StateKneel"),
            Self::StateUseStanding => f.write_str("StateUseStanding"),
            Self::OneshotWaveNoSheathe => f.write_str("OneshotWaveNoSheathe"),
            Self::OneshotCheerNoSheathe => f.write_str("OneshotCheerNoSheathe"),
            Self::OneshotEatNoSheathe => f.write_str("OneshotEatNoSheathe"),
            Self::StateStunNoSheathe => f.write_str("StateStunNoSheathe"),
            Self::OneshotDance => f.write_str("OneshotDance"),
            Self::OneshotSaluteNoSheath => f.write_str("OneshotSaluteNoSheath"),
            Self::StateUseStandingNoSheathe => f.write_str("StateUseStandingNoSheathe"),
            Self::OneshotLaughNoSheathe => f.write_str("OneshotLaughNoSheathe"),
            Self::StateWork => f.write_str("StateWork"),
            Self::StateSpellPrecast => f.write_str("StateSpellPrecast"),
            Self::OneshotReadyRifle => f.write_str("OneshotReadyRifle"),
            Self::StateReadyRifle => f.write_str("StateReadyRifle"),
            Self::StateWorkMining => f.write_str("StateWorkMining"),
            Self::StateWorkChopwood => f.write_str("StateWorkChopwood"),
            Self::StateApplaud => f.write_str("StateApplaud"),
            Self::OneshotLiftoff => f.write_str("OneshotLiftoff"),
            Self::OneshotYes => f.write_str("OneshotYes"),
            Self::OneshotNo => f.write_str("OneshotNo"),
            Self::OneshotTrain => f.write_str("OneshotTrain"),
            Self::OneshotLand => f.write_str("OneshotLand"),
            Self::StateAtEase => f.write_str("StateAtEase"),
            Self::StateReady1h => f.write_str("StateReady1h"),
            Self::StateSpellKneelStart => f.write_str("StateSpellKneelStart"),
            Self::StateSubmerged => f.write_str("StateSubmerged"),
            Self::OneshotSubmerge => f.write_str("OneshotSubmerge"),
            Self::StateReady2h => f.write_str("StateReady2h"),
            Self::StateReadyBow => f.write_str("StateReadyBow"),
            Self::OneshotMountSpecial => f.write_str("OneshotMountSpecial"),
            Self::StateTalk => f.write_str("StateTalk"),
            Self::StateFishing => f.write_str("StateFishing"),
            Self::OneshotFishing => f.write_str("OneshotFishing"),
            Self::OneshotLoot => f.write_str("OneshotLoot"),
            Self::StateWhirlwind => f.write_str("StateWhirlwind"),
            Self::StateDrowned => f.write_str("StateDrowned"),
            Self::StateHoldBow => f.write_str("StateHoldBow"),
            Self::StateHoldRifle => f.write_str("StateHoldRifle"),
            Self::StateHoldThrown => f.write_str("StateHoldThrown"),
            Self::OneshotDrown => f.write_str("OneshotDrown"),
            Self::OneshotStomp => f.write_str("OneshotStomp"),
            Self::OneshotAttackOff => f.write_str("OneshotAttackOff"),
            Self::OneshotAttackOffPierce => f.write_str("OneshotAttackOffPierce"),
            Self::StateRoar => f.write_str("StateRoar"),
            Self::StateLaugh => f.write_str("StateLaugh"),
            Self::OneshotCreatureSpecial => f.write_str("OneshotCreatureSpecial"),
            Self::OneshotJumplandrun => f.write_str("OneshotJumplandrun"),
            Self::OneshotJumpend => f.write_str("OneshotJumpend"),
            Self::OneshotTalkNoSheathe => f.write_str("OneshotTalkNoSheathe"),
            Self::OneshotPointNoSheathe => f.write_str("OneshotPointNoSheathe"),
            Self::StateCannibalize => f.write_str("StateCannibalize"),
            Self::OneshotJumpstart => f.write_str("OneshotJumpstart"),
            Self::StateDancespecial => f.write_str("StateDancespecial"),
            Self::OneshotDancespecial => f.write_str("OneshotDancespecial"),
            Self::OneshotCustomSpell01 => f.write_str("OneshotCustomSpell01"),
            Self::OneshotCustomSpell02 => f.write_str("OneshotCustomSpell02"),
            Self::OneshotCustomSpell03 => f.write_str("OneshotCustomSpell03"),
            Self::OneshotCustomSpell04 => f.write_str("OneshotCustomSpell04"),
            Self::OneshotCustomSpell05 => f.write_str("OneshotCustomSpell05"),
            Self::OneshotCustomSpell06 => f.write_str("OneshotCustomSpell06"),
            Self::OneshotCustomSpell07 => f.write_str("OneshotCustomSpell07"),
            Self::OneshotCustomSpell08 => f.write_str("OneshotCustomSpell08"),
            Self::OneshotCustomSpell09 => f.write_str("OneshotCustomSpell09"),
            Self::OneshotCustomSpell10 => f.write_str("OneshotCustomSpell10"),
            Self::StateExclaim => f.write_str("StateExclaim"),
            Self::StateDanceCustom => f.write_str("StateDanceCustom"),
            Self::StateSitChairMed => f.write_str("StateSitChairMed"),
            Self::StateCustomSpell01 => f.write_str("StateCustomSpell01"),
            Self::StateCustomSpell02 => f.write_str("StateCustomSpell02"),
            Self::StateEat => f.write_str("StateEat"),
            Self::StateCustomSpell04 => f.write_str("StateCustomSpell04"),
            Self::StateCustomSpell03 => f.write_str("StateCustomSpell03"),
            Self::StateCustomSpell05 => f.write_str("StateCustomSpell05"),
            Self::StateSpelleffectHold => f.write_str("StateSpelleffectHold"),
            Self::StateEatNoSheathe => f.write_str("StateEatNoSheathe"),
            Self::StateMount => f.write_str("StateMount"),
            Self::StateReady2hl => f.write_str("StateReady2hl"),
            Self::StateSitChairHigh => f.write_str("StateSitChairHigh"),
            Self::StateFall => f.write_str("StateFall"),
            Self::StateLoot => f.write_str("StateLoot"),
            Self::StateSubmergedNew => f.write_str("StateSubmergedNew"),
            Self::OneshotCower => f.write_str("OneshotCower"),
            Self::StateCower => f.write_str("StateCower"),
            Self::OneshotUseStanding => f.write_str("OneshotUseStanding"),
            Self::StateStealthStand => f.write_str("StateStealthStand"),
            Self::OneshotOmnicastGhoul => f.write_str("OneshotOmnicastGhoul"),
            Self::OneshotAttackBow => f.write_str("OneshotAttackBow"),
            Self::OneshotAttackRifle => f.write_str("OneshotAttackRifle"),
            Self::StateSwimIdle => f.write_str("StateSwimIdle"),
            Self::StateAttackUnarmed => f.write_str("StateAttackUnarmed"),
            Self::OneshotSpellCastWSound => f.write_str("OneshotSpellCastWSound"),
            Self::OneshotDodge => f.write_str("OneshotDodge"),
            Self::OneshotParry1h => f.write_str("OneshotParry1h"),
            Self::OneshotParry2h => f.write_str("OneshotParry2h"),
            Self::OneshotParry2hl => f.write_str("OneshotParry2hl"),
            Self::StateFlyfall => f.write_str("StateFlyfall"),
            Self::OneshotFlydeath => f.write_str("OneshotFlydeath"),
            Self::StateFlyFall => f.write_str("StateFlyFall"),
            Self::OneshotFlySitGroundDown => f.write_str("OneshotFlySitGroundDown"),
            Self::OneshotFlySitGroundUp => f.write_str("OneshotFlySitGroundUp"),
            Self::OneshotEmerge => f.write_str("OneshotEmerge"),
            Self::OneshotDragonSpit => f.write_str("OneshotDragonSpit"),
            Self::StateSpecialUnarmed => f.write_str("StateSpecialUnarmed"),
            Self::OneshotFlygrab => f.write_str("OneshotFlygrab"),
            Self::StateFlygrabclosed => f.write_str("StateFlygrabclosed"),
            Self::OneshotFlygrabthrown => f.write_str("OneshotFlygrabthrown"),
            Self::StateFlySitGround => f.write_str("StateFlySitGround"),
            Self::StateWalkBackwards => f.write_str("StateWalkBackwards"),
            Self::OneshotFlytalk => f.write_str("OneshotFlytalk"),
            Self::OneshotFlyattack1h => f.write_str("OneshotFlyattack1h"),
            Self::StateCustomSpell08 => f.write_str("StateCustomSpell08"),
            Self::OneshotFlyDragonSpit => f.write_str("OneshotFlyDragonSpit"),
            Self::StateSitChairLow => f.write_str("StateSitChairLow"),
            Self::OneshotStun => f.write_str("OneshotStun"),
            Self::OneshotSpellCastOmni => f.write_str("OneshotSpellCastOmni"),
            Self::StateReadyThrown => f.write_str("StateReadyThrown"),
            Self::OneshotWorkChopwood => f.write_str("OneshotWorkChopwood"),
            Self::OneshotWorkMining => f.write_str("OneshotWorkMining"),
            Self::StateSpellChannelOmni => f.write_str("StateSpellChannelOmni"),
            Self::StateSpellChannelDirected => f.write_str("StateSpellChannelDirected"),
            Self::StandStateNone => f.write_str("StandStateNone"),
            Self::StateReadyjoust => f.write_str("StateReadyjoust"),
            Self::StateStrangulate => f.write_str("StateStrangulate"),
            Self::StateReadySpellOmni => f.write_str("StateReadySpellOmni"),
            Self::StateHoldJoust => f.write_str("StateHoldJoust"),
            Self::OneshotCryJaina => f.write_str("OneshotCryJaina"),
        }
    }
}

impl TryFrom<u32> for Emote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for Emote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for Emote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u64> for Emote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for Emote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for Emote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for Emote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i64> for Emote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for Emote {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

