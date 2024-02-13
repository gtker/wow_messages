/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:85`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L85):
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
///     STATE_READYUNARMED = 27;
///     STATE_WORK_SHEATHED = 28;
///     STATE_POINT = 29;
///     STATE_NONE = 30;
///     ONESHOT_WOUND = 33;
///     ONESHOT_WOUNDCRITICAL = 34;
///     ONESHOT_ATTACKUNARMED = 35;
///     ONESHOT_ATTACK1H = 36;
///     ONESHOT_ATTACK2HTIGHT = 37;
///     ONESHOT_ATTACK2HLOOSE = 38;
///     ONESHOT_PARRYUNARMED = 39;
///     ONESHOT_PARRYSHIELD = 43;
///     ONESHOT_READYUNARMED = 44;
///     ONESHOT_READY1H = 45;
///     ONESHOT_READYBOW = 48;
///     ONESHOT_SPELLPRECAST = 50;
///     ONESHOT_SPELLCAST = 51;
///     ONESHOT_BATTLEROAR = 53;
///     ONESHOT_SPECIALATTACK1H = 54;
///     ONESHOT_KICK = 60;
///     ONESHOT_ATTACKTHROWN = 61;
///     STATE_STUN = 64;
///     STATE_DEAD = 65;
///     ONESHOT_SALUTE = 66;
///     STATE_KNEEL = 68;
///     STATE_USESTANDING = 69;
///     ONESHOT_WAVE_NOSHEATHE = 70;
///     ONESHOT_CHEER_NOSHEATHE = 71;
///     ONESHOT_EAT_NOSHEATHE = 92;
///     STATE_STUN_NOSHEATHE = 93;
///     ONESHOT_DANCE = 94;
///     ONESHOT_SALUTE_NOSHEATH = 113;
///     STATE_USESTANDING_NOSHEATHE = 133;
///     ONESHOT_LAUGH_NOSHEATHE = 153;
///     STATE_WORK = 173;
///     STATE_SPELLPRECAST = 193;
///     ONESHOT_READYRIFLE = 213;
///     STATE_READYRIFLE = 214;
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
///     STATE_SPELLKNEELSTART = 353;
///     STATE_SUBMERGED = 373;
///     ONESHOT_SUBMERGE = 374;
///     STATE_READY2H = 375;
///     STATE_READYBOW = 376;
///     ONESHOT_MOUNTSPECIAL = 377;
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
///     ONESHOT_ATTACKOFF = 389;
///     ONESHOT_ATTACKOFFPIERCE = 390;
///     STATE_ROAR = 391;
///     STATE_LAUGH = 392;
///     ONESHOT_CREATURE_SPECIAL = 393;
///     ONESHOT_JUMPLANDRUN = 394;
///     ONESHOT_JUMPEND = 395;
///     ONESHOT_TALK_NOSHEATHE = 396;
///     ONESHOT_POINT_NOSHEATHE = 397;
///     STATE_CANNIBALIZE = 398;
///     ONESHOT_JUMPSTART = 399;
///     STATE_DANCESPECIAL = 400;
///     ONESHOT_DANCESPECIAL = 401;
///     ONESHOT_CUSTOMSPELL01 = 402;
///     ONESHOT_CUSTOMSPELL02 = 403;
///     ONESHOT_CUSTOMSPELL03 = 404;
///     ONESHOT_CUSTOMSPELL04 = 405;
///     ONESHOT_CUSTOMSPELL05 = 406;
///     ONESHOT_CUSTOMSPELL06 = 407;
///     ONESHOT_CUSTOMSPELL07 = 408;
///     ONESHOT_CUSTOMSPELL08 = 409;
///     ONESHOT_CUSTOMSPELL09 = 410;
///     ONESHOT_CUSTOMSPELL10 = 411;
///     STATE_EXCLAIM = 412;
///     STATE_SIT_CHAIR_MED = 415;
///     STATE_SPELLEFFECT_HOLD = 422;
///     STATE_EAT_NO_SHEATHE = 423;
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
    StateReadyunarmed,
    StateWorkSheathed,
    StatePoint,
    StateNone,
    OneshotWound,
    OneshotWoundcritical,
    OneshotAttackunarmed,
    OneshotAttack1h,
    OneshotAttack2htight,
    OneshotAttack2hloose,
    OneshotParryunarmed,
    OneshotParryshield,
    OneshotReadyunarmed,
    OneshotReady1h,
    OneshotReadybow,
    OneshotSpellprecast,
    OneshotSpellcast,
    OneshotBattleroar,
    OneshotSpecialattack1h,
    OneshotKick,
    OneshotAttackthrown,
    StateStun,
    StateDead,
    OneshotSalute,
    StateKneel,
    StateUsestanding,
    OneshotWaveNosheathe,
    OneshotCheerNosheathe,
    OneshotEatNosheathe,
    StateStunNosheathe,
    OneshotDance,
    OneshotSaluteNosheath,
    StateUsestandingNosheathe,
    OneshotLaughNosheathe,
    StateWork,
    StateSpellprecast,
    OneshotReadyrifle,
    StateReadyrifle,
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
    StateSpellkneelstart,
    StateSubmerged,
    OneshotSubmerge,
    StateReady2h,
    StateReadybow,
    OneshotMountspecial,
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
    OneshotAttackoff,
    OneshotAttackoffpierce,
    StateRoar,
    StateLaugh,
    OneshotCreatureSpecial,
    OneshotJumplandrun,
    OneshotJumpend,
    OneshotTalkNosheathe,
    OneshotPointNosheathe,
    StateCannibalize,
    OneshotJumpstart,
    StateDancespecial,
    OneshotDancespecial,
    OneshotCustomspell01,
    OneshotCustomspell02,
    OneshotCustomspell03,
    OneshotCustomspell04,
    OneshotCustomspell05,
    OneshotCustomspell06,
    OneshotCustomspell07,
    OneshotCustomspell08,
    OneshotCustomspell09,
    OneshotCustomspell10,
    StateExclaim,
    StateSitChairMed,
    StateSpelleffectHold,
    StateEatNoSheathe,
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
            Self::StateReadyunarmed => 0x1b,
            Self::StateWorkSheathed => 0x1c,
            Self::StatePoint => 0x1d,
            Self::StateNone => 0x1e,
            Self::OneshotWound => 0x21,
            Self::OneshotWoundcritical => 0x22,
            Self::OneshotAttackunarmed => 0x23,
            Self::OneshotAttack1h => 0x24,
            Self::OneshotAttack2htight => 0x25,
            Self::OneshotAttack2hloose => 0x26,
            Self::OneshotParryunarmed => 0x27,
            Self::OneshotParryshield => 0x2b,
            Self::OneshotReadyunarmed => 0x2c,
            Self::OneshotReady1h => 0x2d,
            Self::OneshotReadybow => 0x30,
            Self::OneshotSpellprecast => 0x32,
            Self::OneshotSpellcast => 0x33,
            Self::OneshotBattleroar => 0x35,
            Self::OneshotSpecialattack1h => 0x36,
            Self::OneshotKick => 0x3c,
            Self::OneshotAttackthrown => 0x3d,
            Self::StateStun => 0x40,
            Self::StateDead => 0x41,
            Self::OneshotSalute => 0x42,
            Self::StateKneel => 0x44,
            Self::StateUsestanding => 0x45,
            Self::OneshotWaveNosheathe => 0x46,
            Self::OneshotCheerNosheathe => 0x47,
            Self::OneshotEatNosheathe => 0x5c,
            Self::StateStunNosheathe => 0x5d,
            Self::OneshotDance => 0x5e,
            Self::OneshotSaluteNosheath => 0x71,
            Self::StateUsestandingNosheathe => 0x85,
            Self::OneshotLaughNosheathe => 0x99,
            Self::StateWork => 0xad,
            Self::StateSpellprecast => 0xc1,
            Self::OneshotReadyrifle => 0xd5,
            Self::StateReadyrifle => 0xd6,
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
            Self::StateSpellkneelstart => 0x161,
            Self::StateSubmerged => 0x175,
            Self::OneshotSubmerge => 0x176,
            Self::StateReady2h => 0x177,
            Self::StateReadybow => 0x178,
            Self::OneshotMountspecial => 0x179,
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
            Self::OneshotAttackoff => 0x185,
            Self::OneshotAttackoffpierce => 0x186,
            Self::StateRoar => 0x187,
            Self::StateLaugh => 0x188,
            Self::OneshotCreatureSpecial => 0x189,
            Self::OneshotJumplandrun => 0x18a,
            Self::OneshotJumpend => 0x18b,
            Self::OneshotTalkNosheathe => 0x18c,
            Self::OneshotPointNosheathe => 0x18d,
            Self::StateCannibalize => 0x18e,
            Self::OneshotJumpstart => 0x18f,
            Self::StateDancespecial => 0x190,
            Self::OneshotDancespecial => 0x191,
            Self::OneshotCustomspell01 => 0x192,
            Self::OneshotCustomspell02 => 0x193,
            Self::OneshotCustomspell03 => 0x194,
            Self::OneshotCustomspell04 => 0x195,
            Self::OneshotCustomspell05 => 0x196,
            Self::OneshotCustomspell06 => 0x197,
            Self::OneshotCustomspell07 => 0x198,
            Self::OneshotCustomspell08 => 0x199,
            Self::OneshotCustomspell09 => 0x19a,
            Self::OneshotCustomspell10 => 0x19b,
            Self::StateExclaim => 0x19c,
            Self::StateSitChairMed => 0x19f,
            Self::StateSpelleffectHold => 0x1a6,
            Self::StateEatNoSheathe => 0x1a7,
        }
    }

    pub const fn variants() -> [Self; 117] {
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
            Self::StateReadyunarmed,
            Self::StateWorkSheathed,
            Self::StatePoint,
            Self::StateNone,
            Self::OneshotWound,
            Self::OneshotWoundcritical,
            Self::OneshotAttackunarmed,
            Self::OneshotAttack1h,
            Self::OneshotAttack2htight,
            Self::OneshotAttack2hloose,
            Self::OneshotParryunarmed,
            Self::OneshotParryshield,
            Self::OneshotReadyunarmed,
            Self::OneshotReady1h,
            Self::OneshotReadybow,
            Self::OneshotSpellprecast,
            Self::OneshotSpellcast,
            Self::OneshotBattleroar,
            Self::OneshotSpecialattack1h,
            Self::OneshotKick,
            Self::OneshotAttackthrown,
            Self::StateStun,
            Self::StateDead,
            Self::OneshotSalute,
            Self::StateKneel,
            Self::StateUsestanding,
            Self::OneshotWaveNosheathe,
            Self::OneshotCheerNosheathe,
            Self::OneshotEatNosheathe,
            Self::StateStunNosheathe,
            Self::OneshotDance,
            Self::OneshotSaluteNosheath,
            Self::StateUsestandingNosheathe,
            Self::OneshotLaughNosheathe,
            Self::StateWork,
            Self::StateSpellprecast,
            Self::OneshotReadyrifle,
            Self::StateReadyrifle,
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
            Self::StateSpellkneelstart,
            Self::StateSubmerged,
            Self::OneshotSubmerge,
            Self::StateReady2h,
            Self::StateReadybow,
            Self::OneshotMountspecial,
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
            Self::OneshotAttackoff,
            Self::OneshotAttackoffpierce,
            Self::StateRoar,
            Self::StateLaugh,
            Self::OneshotCreatureSpecial,
            Self::OneshotJumplandrun,
            Self::OneshotJumpend,
            Self::OneshotTalkNosheathe,
            Self::OneshotPointNosheathe,
            Self::StateCannibalize,
            Self::OneshotJumpstart,
            Self::StateDancespecial,
            Self::OneshotDancespecial,
            Self::OneshotCustomspell01,
            Self::OneshotCustomspell02,
            Self::OneshotCustomspell03,
            Self::OneshotCustomspell04,
            Self::OneshotCustomspell05,
            Self::OneshotCustomspell06,
            Self::OneshotCustomspell07,
            Self::OneshotCustomspell08,
            Self::OneshotCustomspell09,
            Self::OneshotCustomspell10,
            Self::StateExclaim,
            Self::StateSitChairMed,
            Self::StateSpelleffectHold,
            Self::StateEatNoSheathe,
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
            27 => Ok(Self::StateReadyunarmed),
            28 => Ok(Self::StateWorkSheathed),
            29 => Ok(Self::StatePoint),
            30 => Ok(Self::StateNone),
            33 => Ok(Self::OneshotWound),
            34 => Ok(Self::OneshotWoundcritical),
            35 => Ok(Self::OneshotAttackunarmed),
            36 => Ok(Self::OneshotAttack1h),
            37 => Ok(Self::OneshotAttack2htight),
            38 => Ok(Self::OneshotAttack2hloose),
            39 => Ok(Self::OneshotParryunarmed),
            43 => Ok(Self::OneshotParryshield),
            44 => Ok(Self::OneshotReadyunarmed),
            45 => Ok(Self::OneshotReady1h),
            48 => Ok(Self::OneshotReadybow),
            50 => Ok(Self::OneshotSpellprecast),
            51 => Ok(Self::OneshotSpellcast),
            53 => Ok(Self::OneshotBattleroar),
            54 => Ok(Self::OneshotSpecialattack1h),
            60 => Ok(Self::OneshotKick),
            61 => Ok(Self::OneshotAttackthrown),
            64 => Ok(Self::StateStun),
            65 => Ok(Self::StateDead),
            66 => Ok(Self::OneshotSalute),
            68 => Ok(Self::StateKneel),
            69 => Ok(Self::StateUsestanding),
            70 => Ok(Self::OneshotWaveNosheathe),
            71 => Ok(Self::OneshotCheerNosheathe),
            92 => Ok(Self::OneshotEatNosheathe),
            93 => Ok(Self::StateStunNosheathe),
            94 => Ok(Self::OneshotDance),
            113 => Ok(Self::OneshotSaluteNosheath),
            133 => Ok(Self::StateUsestandingNosheathe),
            153 => Ok(Self::OneshotLaughNosheathe),
            173 => Ok(Self::StateWork),
            193 => Ok(Self::StateSpellprecast),
            213 => Ok(Self::OneshotReadyrifle),
            214 => Ok(Self::StateReadyrifle),
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
            353 => Ok(Self::StateSpellkneelstart),
            373 => Ok(Self::StateSubmerged),
            374 => Ok(Self::OneshotSubmerge),
            375 => Ok(Self::StateReady2h),
            376 => Ok(Self::StateReadybow),
            377 => Ok(Self::OneshotMountspecial),
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
            389 => Ok(Self::OneshotAttackoff),
            390 => Ok(Self::OneshotAttackoffpierce),
            391 => Ok(Self::StateRoar),
            392 => Ok(Self::StateLaugh),
            393 => Ok(Self::OneshotCreatureSpecial),
            394 => Ok(Self::OneshotJumplandrun),
            395 => Ok(Self::OneshotJumpend),
            396 => Ok(Self::OneshotTalkNosheathe),
            397 => Ok(Self::OneshotPointNosheathe),
            398 => Ok(Self::StateCannibalize),
            399 => Ok(Self::OneshotJumpstart),
            400 => Ok(Self::StateDancespecial),
            401 => Ok(Self::OneshotDancespecial),
            402 => Ok(Self::OneshotCustomspell01),
            403 => Ok(Self::OneshotCustomspell02),
            404 => Ok(Self::OneshotCustomspell03),
            405 => Ok(Self::OneshotCustomspell04),
            406 => Ok(Self::OneshotCustomspell05),
            407 => Ok(Self::OneshotCustomspell06),
            408 => Ok(Self::OneshotCustomspell07),
            409 => Ok(Self::OneshotCustomspell08),
            410 => Ok(Self::OneshotCustomspell09),
            411 => Ok(Self::OneshotCustomspell10),
            412 => Ok(Self::StateExclaim),
            415 => Ok(Self::StateSitChairMed),
            422 => Ok(Self::StateSpelleffectHold),
            423 => Ok(Self::StateEatNoSheathe),
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
            Self::StateReadyunarmed => "STATE_READYUNARMED",
            Self::StateWorkSheathed => "STATE_WORK_SHEATHED",
            Self::StatePoint => "STATE_POINT",
            Self::StateNone => "STATE_NONE",
            Self::OneshotWound => "ONESHOT_WOUND",
            Self::OneshotWoundcritical => "ONESHOT_WOUNDCRITICAL",
            Self::OneshotAttackunarmed => "ONESHOT_ATTACKUNARMED",
            Self::OneshotAttack1h => "ONESHOT_ATTACK1H",
            Self::OneshotAttack2htight => "ONESHOT_ATTACK2HTIGHT",
            Self::OneshotAttack2hloose => "ONESHOT_ATTACK2HLOOSE",
            Self::OneshotParryunarmed => "ONESHOT_PARRYUNARMED",
            Self::OneshotParryshield => "ONESHOT_PARRYSHIELD",
            Self::OneshotReadyunarmed => "ONESHOT_READYUNARMED",
            Self::OneshotReady1h => "ONESHOT_READY1H",
            Self::OneshotReadybow => "ONESHOT_READYBOW",
            Self::OneshotSpellprecast => "ONESHOT_SPELLPRECAST",
            Self::OneshotSpellcast => "ONESHOT_SPELLCAST",
            Self::OneshotBattleroar => "ONESHOT_BATTLEROAR",
            Self::OneshotSpecialattack1h => "ONESHOT_SPECIALATTACK1H",
            Self::OneshotKick => "ONESHOT_KICK",
            Self::OneshotAttackthrown => "ONESHOT_ATTACKTHROWN",
            Self::StateStun => "STATE_STUN",
            Self::StateDead => "STATE_DEAD",
            Self::OneshotSalute => "ONESHOT_SALUTE",
            Self::StateKneel => "STATE_KNEEL",
            Self::StateUsestanding => "STATE_USESTANDING",
            Self::OneshotWaveNosheathe => "ONESHOT_WAVE_NOSHEATHE",
            Self::OneshotCheerNosheathe => "ONESHOT_CHEER_NOSHEATHE",
            Self::OneshotEatNosheathe => "ONESHOT_EAT_NOSHEATHE",
            Self::StateStunNosheathe => "STATE_STUN_NOSHEATHE",
            Self::OneshotDance => "ONESHOT_DANCE",
            Self::OneshotSaluteNosheath => "ONESHOT_SALUTE_NOSHEATH",
            Self::StateUsestandingNosheathe => "STATE_USESTANDING_NOSHEATHE",
            Self::OneshotLaughNosheathe => "ONESHOT_LAUGH_NOSHEATHE",
            Self::StateWork => "STATE_WORK",
            Self::StateSpellprecast => "STATE_SPELLPRECAST",
            Self::OneshotReadyrifle => "ONESHOT_READYRIFLE",
            Self::StateReadyrifle => "STATE_READYRIFLE",
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
            Self::StateSpellkneelstart => "STATE_SPELLKNEELSTART",
            Self::StateSubmerged => "STATE_SUBMERGED",
            Self::OneshotSubmerge => "ONESHOT_SUBMERGE",
            Self::StateReady2h => "STATE_READY2H",
            Self::StateReadybow => "STATE_READYBOW",
            Self::OneshotMountspecial => "ONESHOT_MOUNTSPECIAL",
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
            Self::OneshotAttackoff => "ONESHOT_ATTACKOFF",
            Self::OneshotAttackoffpierce => "ONESHOT_ATTACKOFFPIERCE",
            Self::StateRoar => "STATE_ROAR",
            Self::StateLaugh => "STATE_LAUGH",
            Self::OneshotCreatureSpecial => "ONESHOT_CREATURE_SPECIAL",
            Self::OneshotJumplandrun => "ONESHOT_JUMPLANDRUN",
            Self::OneshotJumpend => "ONESHOT_JUMPEND",
            Self::OneshotTalkNosheathe => "ONESHOT_TALK_NOSHEATHE",
            Self::OneshotPointNosheathe => "ONESHOT_POINT_NOSHEATHE",
            Self::StateCannibalize => "STATE_CANNIBALIZE",
            Self::OneshotJumpstart => "ONESHOT_JUMPSTART",
            Self::StateDancespecial => "STATE_DANCESPECIAL",
            Self::OneshotDancespecial => "ONESHOT_DANCESPECIAL",
            Self::OneshotCustomspell01 => "ONESHOT_CUSTOMSPELL01",
            Self::OneshotCustomspell02 => "ONESHOT_CUSTOMSPELL02",
            Self::OneshotCustomspell03 => "ONESHOT_CUSTOMSPELL03",
            Self::OneshotCustomspell04 => "ONESHOT_CUSTOMSPELL04",
            Self::OneshotCustomspell05 => "ONESHOT_CUSTOMSPELL05",
            Self::OneshotCustomspell06 => "ONESHOT_CUSTOMSPELL06",
            Self::OneshotCustomspell07 => "ONESHOT_CUSTOMSPELL07",
            Self::OneshotCustomspell08 => "ONESHOT_CUSTOMSPELL08",
            Self::OneshotCustomspell09 => "ONESHOT_CUSTOMSPELL09",
            Self::OneshotCustomspell10 => "ONESHOT_CUSTOMSPELL10",
            Self::StateExclaim => "STATE_EXCLAIM",
            Self::StateSitChairMed => "STATE_SIT_CHAIR_MED",
            Self::StateSpelleffectHold => "STATE_SPELLEFFECT_HOLD",
            Self::StateEatNoSheathe => "STATE_EAT_NO_SHEATHE",
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
            Self::StateReadyunarmed => f.write_str("StateReadyunarmed"),
            Self::StateWorkSheathed => f.write_str("StateWorkSheathed"),
            Self::StatePoint => f.write_str("StatePoint"),
            Self::StateNone => f.write_str("StateNone"),
            Self::OneshotWound => f.write_str("OneshotWound"),
            Self::OneshotWoundcritical => f.write_str("OneshotWoundcritical"),
            Self::OneshotAttackunarmed => f.write_str("OneshotAttackunarmed"),
            Self::OneshotAttack1h => f.write_str("OneshotAttack1h"),
            Self::OneshotAttack2htight => f.write_str("OneshotAttack2htight"),
            Self::OneshotAttack2hloose => f.write_str("OneshotAttack2hloose"),
            Self::OneshotParryunarmed => f.write_str("OneshotParryunarmed"),
            Self::OneshotParryshield => f.write_str("OneshotParryshield"),
            Self::OneshotReadyunarmed => f.write_str("OneshotReadyunarmed"),
            Self::OneshotReady1h => f.write_str("OneshotReady1h"),
            Self::OneshotReadybow => f.write_str("OneshotReadybow"),
            Self::OneshotSpellprecast => f.write_str("OneshotSpellprecast"),
            Self::OneshotSpellcast => f.write_str("OneshotSpellcast"),
            Self::OneshotBattleroar => f.write_str("OneshotBattleroar"),
            Self::OneshotSpecialattack1h => f.write_str("OneshotSpecialattack1h"),
            Self::OneshotKick => f.write_str("OneshotKick"),
            Self::OneshotAttackthrown => f.write_str("OneshotAttackthrown"),
            Self::StateStun => f.write_str("StateStun"),
            Self::StateDead => f.write_str("StateDead"),
            Self::OneshotSalute => f.write_str("OneshotSalute"),
            Self::StateKneel => f.write_str("StateKneel"),
            Self::StateUsestanding => f.write_str("StateUsestanding"),
            Self::OneshotWaveNosheathe => f.write_str("OneshotWaveNosheathe"),
            Self::OneshotCheerNosheathe => f.write_str("OneshotCheerNosheathe"),
            Self::OneshotEatNosheathe => f.write_str("OneshotEatNosheathe"),
            Self::StateStunNosheathe => f.write_str("StateStunNosheathe"),
            Self::OneshotDance => f.write_str("OneshotDance"),
            Self::OneshotSaluteNosheath => f.write_str("OneshotSaluteNosheath"),
            Self::StateUsestandingNosheathe => f.write_str("StateUsestandingNosheathe"),
            Self::OneshotLaughNosheathe => f.write_str("OneshotLaughNosheathe"),
            Self::StateWork => f.write_str("StateWork"),
            Self::StateSpellprecast => f.write_str("StateSpellprecast"),
            Self::OneshotReadyrifle => f.write_str("OneshotReadyrifle"),
            Self::StateReadyrifle => f.write_str("StateReadyrifle"),
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
            Self::StateSpellkneelstart => f.write_str("StateSpellkneelstart"),
            Self::StateSubmerged => f.write_str("StateSubmerged"),
            Self::OneshotSubmerge => f.write_str("OneshotSubmerge"),
            Self::StateReady2h => f.write_str("StateReady2h"),
            Self::StateReadybow => f.write_str("StateReadybow"),
            Self::OneshotMountspecial => f.write_str("OneshotMountspecial"),
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
            Self::OneshotAttackoff => f.write_str("OneshotAttackoff"),
            Self::OneshotAttackoffpierce => f.write_str("OneshotAttackoffpierce"),
            Self::StateRoar => f.write_str("StateRoar"),
            Self::StateLaugh => f.write_str("StateLaugh"),
            Self::OneshotCreatureSpecial => f.write_str("OneshotCreatureSpecial"),
            Self::OneshotJumplandrun => f.write_str("OneshotJumplandrun"),
            Self::OneshotJumpend => f.write_str("OneshotJumpend"),
            Self::OneshotTalkNosheathe => f.write_str("OneshotTalkNosheathe"),
            Self::OneshotPointNosheathe => f.write_str("OneshotPointNosheathe"),
            Self::StateCannibalize => f.write_str("StateCannibalize"),
            Self::OneshotJumpstart => f.write_str("OneshotJumpstart"),
            Self::StateDancespecial => f.write_str("StateDancespecial"),
            Self::OneshotDancespecial => f.write_str("OneshotDancespecial"),
            Self::OneshotCustomspell01 => f.write_str("OneshotCustomspell01"),
            Self::OneshotCustomspell02 => f.write_str("OneshotCustomspell02"),
            Self::OneshotCustomspell03 => f.write_str("OneshotCustomspell03"),
            Self::OneshotCustomspell04 => f.write_str("OneshotCustomspell04"),
            Self::OneshotCustomspell05 => f.write_str("OneshotCustomspell05"),
            Self::OneshotCustomspell06 => f.write_str("OneshotCustomspell06"),
            Self::OneshotCustomspell07 => f.write_str("OneshotCustomspell07"),
            Self::OneshotCustomspell08 => f.write_str("OneshotCustomspell08"),
            Self::OneshotCustomspell09 => f.write_str("OneshotCustomspell09"),
            Self::OneshotCustomspell10 => f.write_str("OneshotCustomspell10"),
            Self::StateExclaim => f.write_str("StateExclaim"),
            Self::StateSitChairMed => f.write_str("StateSitChairMed"),
            Self::StateSpelleffectHold => f.write_str("StateSpelleffectHold"),
            Self::StateEatNoSheathe => f.write_str("StateEatNoSheathe"),
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

