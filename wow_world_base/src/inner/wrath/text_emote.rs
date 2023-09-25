/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common_3_3_5.wowm:181`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common_3_3_5.wowm#L181):
/// ```text
/// enum TextEmote : u32 {
///     AGREE = 1;
///     AMAZE = 2;
///     ANGRY = 3;
///     APOLOGIZE = 4;
///     APPLAUD = 5;
///     BASHFUL = 6;
///     BECKON = 7;
///     BEG = 8;
///     BITE = 9;
///     BLEED = 10;
///     BLINK = 11;
///     BLUSH = 12;
///     BONK = 13;
///     BORED = 14;
///     BOUNCE = 15;
///     BRB = 16;
///     BOW = 17;
///     BURP = 18;
///     BYE = 19;
///     CACKLE = 20;
///     CHEER = 21;
///     CHICKEN = 22;
///     CHUCKLE = 23;
///     CLAP = 24;
///     CONFUSED = 25;
///     CONGRATULATE = 26;
///     COUGH = 27;
///     COWER = 28;
///     CRACK = 29;
///     CRINGE = 30;
///     CRY = 31;
///     CURIOUS = 32;
///     CURTSEY = 33;
///     DANCE = 34;
///     DRINK = 35;
///     DROOL = 36;
///     EAT = 37;
///     EYE = 38;
///     FART = 39;
///     FIDGET = 40;
///     FLEX = 41;
///     FROWN = 42;
///     GASP = 43;
///     GAZE = 44;
///     GIGGLE = 45;
///     GLARE = 46;
///     GLOAT = 47;
///     GREET = 48;
///     GRIN = 49;
///     GROAN = 50;
///     GROVEL = 51;
///     GUFFAW = 52;
///     HAIL = 53;
///     HAPPY = 54;
///     HELLO = 55;
///     HUG = 56;
///     HUNGRY = 57;
///     KISS = 58;
///     KNEEL = 59;
///     LAUGH = 60;
///     LAYDOWN = 61;
///     MASSAGE = 62;
///     MOAN = 63;
///     MOON = 64;
///     MOURN = 65;
///     NO = 66;
///     NOD = 67;
///     NOSE_PICK = 68;
///     PANIC = 69;
///     PEER = 70;
///     PLEAD = 71;
///     POINT = 72;
///     POKE = 73;
///     PRAY = 74;
///     ROAR = 75;
///     ROFL = 76;
///     RUDE = 77;
///     SALUTE = 78;
///     SCRATCH = 79;
///     SEXY = 80;
///     SHAKE = 81;
///     SHOUT = 82;
///     SHRUG = 83;
///     SHY = 84;
///     SIGH = 85;
///     SIT = 86;
///     SLEEP = 87;
///     SNARL = 88;
///     SPIT = 89;
///     STARE = 90;
///     SURPRISED = 91;
///     SURRENDER = 92;
///     TALK = 93;
///     TALK_EX = 94;
///     TALK_Q = 95;
///     TAP = 96;
///     THANK = 97;
///     THREATEN = 98;
///     TIRED = 99;
///     VICTORY = 100;
///     WAVE = 101;
///     WELCOME = 102;
///     WHINE = 103;
///     WHISTLE = 104;
///     WORK = 105;
///     YAWN = 106;
///     BOGGLE = 107;
///     CALM = 108;
///     COLD = 109;
///     COMFORT = 110;
///     CUDDLE = 111;
///     DUCK = 112;
///     INSULT = 113;
///     INTRODUCE = 114;
///     JK = 115;
///     LICK = 116;
///     LISTEN = 117;
///     LOST = 118;
///     MOCK = 119;
///     PONDER = 120;
///     POUNCE = 121;
///     PRAISE = 122;
///     PURR = 123;
///     PUZZLE = 124;
///     RAISE = 125;
///     READY = 126;
///     SHIMMY = 127;
///     SHIVER = 128;
///     SHOO = 129;
///     SLAP = 130;
///     SMIRK = 131;
///     SNIFF = 132;
///     SNUB = 133;
///     SOOTHE = 134;
///     STINK = 135;
///     TAUNT = 136;
///     TEASE = 137;
///     THIRSTY = 138;
///     VETO = 139;
///     SNICKER = 140;
///     STAND = 141;
///     TICKLE = 142;
///     VIOLIN = 143;
///     SMILE = 163;
///     RASP = 183;
///     PITY = 203;
///     GROWL = 204;
///     BARK = 205;
///     SCARED = 223;
///     FLOP = 224;
///     LOVE = 225;
///     MOO = 226;
///     COMMEND = 243;
///     TRAIN = 264;
///     HELPME = 303;
///     INCOMING = 304;
///     CHARGE = 305;
///     FLEE = 306;
///     ATTACK_MY_TARGET = 307;
///     OOM = 323;
///     FOLLOW = 324;
///     WAIT = 325;
///     HEAL_ME = 326;
///     OPEN_FIRE = 327;
///     FLIRT = 328;
///     JOKE = 329;
///     GOLF_CLAP = 343;
///     WINK = 363;
///     PAT = 364;
///     SERIOUS = 365;
///     MOUNT_SPECIAL = 366;
///     GOOD_LUCK = 367;
///     BLAME = 368;
///     BLANK = 369;
///     BRANDISH = 370;
///     BREATH = 371;
///     DISAGREE = 372;
///     DOUBT = 373;
///     EMBARRASS = 374;
///     ENCOURAGE = 375;
///     ENEMY = 376;
///     EYE_BROW = 377;
///     TOAST = 378;
///     FAIL = 379;
///     HIGH_FIVE = 380;
///     ABSENT = 381;
///     ARM = 382;
///     AWE = 383;
///     BACKPACK = 384;
///     BAD_FEELING = 385;
///     CHALLENGE = 386;
///     CHUG = 387;
///     DING = 389;
///     FACE_PALM = 390;
///     FAINT = 391;
///     GO = 392;
///     GOING = 393;
///     GLOWER = 394;
///     HEADACHE = 395;
///     HICCUP = 396;
///     HISS = 398;
///     HOLD_HAND = 399;
///     HURRY = 401;
///     IDEA = 402;
///     JEALOUS = 403;
///     LUCK = 404;
///     MAP = 405;
///     MERCY = 406;
///     MUTTER = 407;
///     NERVOUS = 408;
///     OFFER = 409;
///     PET = 410;
///     PINCH = 411;
///     PROUD = 413;
///     PROMISE = 414;
///     PULSE = 415;
///     PUNCH = 416;
///     POUT = 417;
///     REGRET = 418;
///     REVENGE = 420;
///     ROLL_EYES = 421;
///     RUFFLE = 422;
///     SAD = 423;
///     SCOFF = 424;
///     SCOLD = 425;
///     SCOWL = 426;
///     SEARCH = 427;
///     SHAKEFIST = 428;
///     SHIFTY = 429;
///     SHUDDER = 430;
///     SIGNAL = 431;
///     SILENCE = 432;
///     SING = 433;
///     SMACK = 434;
///     SNEAK = 435;
///     SNEEZE = 436;
///     SNORT = 437;
///     SQUEAL = 438;
///     STOP_ATTACK = 439;
///     SUSPICIOUS = 440;
///     THINK = 441;
///     TRUCE = 442;
///     TWIDDLE = 443;
///     WARN = 444;
///     SNAP = 445;
///     CHARM = 446;
///     COVER_EARS = 447;
///     CROSS_ARMS = 448;
///     LOOK = 449;
///     OBJECT = 450;
///     SWEAT = 451;
///     YW = 453;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TextEmote {
    Agree,
    Amaze,
    Angry,
    Apologize,
    Applaud,
    Bashful,
    Beckon,
    Beg,
    Bite,
    Bleed,
    Blink,
    Blush,
    Bonk,
    Bored,
    Bounce,
    Brb,
    Bow,
    Burp,
    Bye,
    Cackle,
    Cheer,
    Chicken,
    Chuckle,
    Clap,
    Confused,
    Congratulate,
    Cough,
    Cower,
    Crack,
    Cringe,
    Cry,
    Curious,
    Curtsey,
    Dance,
    Drink,
    Drool,
    Eat,
    Eye,
    Fart,
    Fidget,
    Flex,
    Frown,
    Gasp,
    Gaze,
    Giggle,
    Glare,
    Gloat,
    Greet,
    Grin,
    Groan,
    Grovel,
    Guffaw,
    Hail,
    Happy,
    Hello,
    Hug,
    Hungry,
    Kiss,
    Kneel,
    Laugh,
    Laydown,
    Massage,
    Moan,
    Moon,
    Mourn,
    No,
    Nod,
    NosePick,
    Panic,
    Peer,
    Plead,
    Point,
    Poke,
    Pray,
    Roar,
    Rofl,
    Rude,
    Salute,
    Scratch,
    Sexy,
    Shake,
    Shout,
    Shrug,
    Shy,
    Sigh,
    Sit,
    Sleep,
    Snarl,
    Spit,
    Stare,
    Surprised,
    Surrender,
    Talk,
    TalkEx,
    TalkQ,
    Tap,
    Thank,
    Threaten,
    Tired,
    Victory,
    Wave,
    Welcome,
    Whine,
    Whistle,
    Work,
    Yawn,
    Boggle,
    Calm,
    Cold,
    Comfort,
    Cuddle,
    Duck,
    Insult,
    Introduce,
    Jk,
    Lick,
    Listen,
    Lost,
    Mock,
    Ponder,
    Pounce,
    Praise,
    Purr,
    Puzzle,
    Raise,
    Ready,
    Shimmy,
    Shiver,
    Shoo,
    Slap,
    Smirk,
    Sniff,
    Snub,
    Soothe,
    Stink,
    Taunt,
    Tease,
    Thirsty,
    Veto,
    Snicker,
    Stand,
    Tickle,
    Violin,
    Smile,
    Rasp,
    Pity,
    Growl,
    Bark,
    Scared,
    Flop,
    Love,
    Moo,
    Commend,
    Train,
    Helpme,
    Incoming,
    Charge,
    Flee,
    AttackMyTarget,
    Oom,
    Follow,
    Wait,
    HealMe,
    OpenFire,
    Flirt,
    Joke,
    GolfClap,
    Wink,
    Pat,
    Serious,
    MountSpecial,
    GoodLuck,
    Blame,
    Blank,
    Brandish,
    Breath,
    Disagree,
    Doubt,
    Embarrass,
    Encourage,
    Enemy,
    EyeBrow,
    Toast,
    Fail,
    HighFive,
    Absent,
    Arm,
    Awe,
    Backpack,
    BadFeeling,
    Challenge,
    Chug,
    Ding,
    FacePalm,
    Faint,
    Go,
    Going,
    Glower,
    Headache,
    Hiccup,
    Hiss,
    HoldHand,
    Hurry,
    Idea,
    Jealous,
    Luck,
    Map,
    Mercy,
    Mutter,
    Nervous,
    Offer,
    Pet,
    Pinch,
    Proud,
    Promise,
    Pulse,
    Punch,
    Pout,
    Regret,
    Revenge,
    RollEyes,
    Ruffle,
    Sad,
    Scoff,
    Scold,
    Scowl,
    Search,
    Shakefist,
    Shifty,
    Shudder,
    Signal,
    Silence,
    Sing,
    Smack,
    Sneak,
    Sneeze,
    Snort,
    Squeal,
    StopAttack,
    Suspicious,
    Think,
    Truce,
    Twiddle,
    Warn,
    Snap,
    Charm,
    CoverEars,
    CrossArms,
    Look,
    Object,
    Sweat,
    Yw,
}

impl TextEmote {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Agree => 0x1,
            Self::Amaze => 0x2,
            Self::Angry => 0x3,
            Self::Apologize => 0x4,
            Self::Applaud => 0x5,
            Self::Bashful => 0x6,
            Self::Beckon => 0x7,
            Self::Beg => 0x8,
            Self::Bite => 0x9,
            Self::Bleed => 0xa,
            Self::Blink => 0xb,
            Self::Blush => 0xc,
            Self::Bonk => 0xd,
            Self::Bored => 0xe,
            Self::Bounce => 0xf,
            Self::Brb => 0x10,
            Self::Bow => 0x11,
            Self::Burp => 0x12,
            Self::Bye => 0x13,
            Self::Cackle => 0x14,
            Self::Cheer => 0x15,
            Self::Chicken => 0x16,
            Self::Chuckle => 0x17,
            Self::Clap => 0x18,
            Self::Confused => 0x19,
            Self::Congratulate => 0x1a,
            Self::Cough => 0x1b,
            Self::Cower => 0x1c,
            Self::Crack => 0x1d,
            Self::Cringe => 0x1e,
            Self::Cry => 0x1f,
            Self::Curious => 0x20,
            Self::Curtsey => 0x21,
            Self::Dance => 0x22,
            Self::Drink => 0x23,
            Self::Drool => 0x24,
            Self::Eat => 0x25,
            Self::Eye => 0x26,
            Self::Fart => 0x27,
            Self::Fidget => 0x28,
            Self::Flex => 0x29,
            Self::Frown => 0x2a,
            Self::Gasp => 0x2b,
            Self::Gaze => 0x2c,
            Self::Giggle => 0x2d,
            Self::Glare => 0x2e,
            Self::Gloat => 0x2f,
            Self::Greet => 0x30,
            Self::Grin => 0x31,
            Self::Groan => 0x32,
            Self::Grovel => 0x33,
            Self::Guffaw => 0x34,
            Self::Hail => 0x35,
            Self::Happy => 0x36,
            Self::Hello => 0x37,
            Self::Hug => 0x38,
            Self::Hungry => 0x39,
            Self::Kiss => 0x3a,
            Self::Kneel => 0x3b,
            Self::Laugh => 0x3c,
            Self::Laydown => 0x3d,
            Self::Massage => 0x3e,
            Self::Moan => 0x3f,
            Self::Moon => 0x40,
            Self::Mourn => 0x41,
            Self::No => 0x42,
            Self::Nod => 0x43,
            Self::NosePick => 0x44,
            Self::Panic => 0x45,
            Self::Peer => 0x46,
            Self::Plead => 0x47,
            Self::Point => 0x48,
            Self::Poke => 0x49,
            Self::Pray => 0x4a,
            Self::Roar => 0x4b,
            Self::Rofl => 0x4c,
            Self::Rude => 0x4d,
            Self::Salute => 0x4e,
            Self::Scratch => 0x4f,
            Self::Sexy => 0x50,
            Self::Shake => 0x51,
            Self::Shout => 0x52,
            Self::Shrug => 0x53,
            Self::Shy => 0x54,
            Self::Sigh => 0x55,
            Self::Sit => 0x56,
            Self::Sleep => 0x57,
            Self::Snarl => 0x58,
            Self::Spit => 0x59,
            Self::Stare => 0x5a,
            Self::Surprised => 0x5b,
            Self::Surrender => 0x5c,
            Self::Talk => 0x5d,
            Self::TalkEx => 0x5e,
            Self::TalkQ => 0x5f,
            Self::Tap => 0x60,
            Self::Thank => 0x61,
            Self::Threaten => 0x62,
            Self::Tired => 0x63,
            Self::Victory => 0x64,
            Self::Wave => 0x65,
            Self::Welcome => 0x66,
            Self::Whine => 0x67,
            Self::Whistle => 0x68,
            Self::Work => 0x69,
            Self::Yawn => 0x6a,
            Self::Boggle => 0x6b,
            Self::Calm => 0x6c,
            Self::Cold => 0x6d,
            Self::Comfort => 0x6e,
            Self::Cuddle => 0x6f,
            Self::Duck => 0x70,
            Self::Insult => 0x71,
            Self::Introduce => 0x72,
            Self::Jk => 0x73,
            Self::Lick => 0x74,
            Self::Listen => 0x75,
            Self::Lost => 0x76,
            Self::Mock => 0x77,
            Self::Ponder => 0x78,
            Self::Pounce => 0x79,
            Self::Praise => 0x7a,
            Self::Purr => 0x7b,
            Self::Puzzle => 0x7c,
            Self::Raise => 0x7d,
            Self::Ready => 0x7e,
            Self::Shimmy => 0x7f,
            Self::Shiver => 0x80,
            Self::Shoo => 0x81,
            Self::Slap => 0x82,
            Self::Smirk => 0x83,
            Self::Sniff => 0x84,
            Self::Snub => 0x85,
            Self::Soothe => 0x86,
            Self::Stink => 0x87,
            Self::Taunt => 0x88,
            Self::Tease => 0x89,
            Self::Thirsty => 0x8a,
            Self::Veto => 0x8b,
            Self::Snicker => 0x8c,
            Self::Stand => 0x8d,
            Self::Tickle => 0x8e,
            Self::Violin => 0x8f,
            Self::Smile => 0xa3,
            Self::Rasp => 0xb7,
            Self::Pity => 0xcb,
            Self::Growl => 0xcc,
            Self::Bark => 0xcd,
            Self::Scared => 0xdf,
            Self::Flop => 0xe0,
            Self::Love => 0xe1,
            Self::Moo => 0xe2,
            Self::Commend => 0xf3,
            Self::Train => 0x108,
            Self::Helpme => 0x12f,
            Self::Incoming => 0x130,
            Self::Charge => 0x131,
            Self::Flee => 0x132,
            Self::AttackMyTarget => 0x133,
            Self::Oom => 0x143,
            Self::Follow => 0x144,
            Self::Wait => 0x145,
            Self::HealMe => 0x146,
            Self::OpenFire => 0x147,
            Self::Flirt => 0x148,
            Self::Joke => 0x149,
            Self::GolfClap => 0x157,
            Self::Wink => 0x16b,
            Self::Pat => 0x16c,
            Self::Serious => 0x16d,
            Self::MountSpecial => 0x16e,
            Self::GoodLuck => 0x16f,
            Self::Blame => 0x170,
            Self::Blank => 0x171,
            Self::Brandish => 0x172,
            Self::Breath => 0x173,
            Self::Disagree => 0x174,
            Self::Doubt => 0x175,
            Self::Embarrass => 0x176,
            Self::Encourage => 0x177,
            Self::Enemy => 0x178,
            Self::EyeBrow => 0x179,
            Self::Toast => 0x17a,
            Self::Fail => 0x17b,
            Self::HighFive => 0x17c,
            Self::Absent => 0x17d,
            Self::Arm => 0x17e,
            Self::Awe => 0x17f,
            Self::Backpack => 0x180,
            Self::BadFeeling => 0x181,
            Self::Challenge => 0x182,
            Self::Chug => 0x183,
            Self::Ding => 0x185,
            Self::FacePalm => 0x186,
            Self::Faint => 0x187,
            Self::Go => 0x188,
            Self::Going => 0x189,
            Self::Glower => 0x18a,
            Self::Headache => 0x18b,
            Self::Hiccup => 0x18c,
            Self::Hiss => 0x18e,
            Self::HoldHand => 0x18f,
            Self::Hurry => 0x191,
            Self::Idea => 0x192,
            Self::Jealous => 0x193,
            Self::Luck => 0x194,
            Self::Map => 0x195,
            Self::Mercy => 0x196,
            Self::Mutter => 0x197,
            Self::Nervous => 0x198,
            Self::Offer => 0x199,
            Self::Pet => 0x19a,
            Self::Pinch => 0x19b,
            Self::Proud => 0x19d,
            Self::Promise => 0x19e,
            Self::Pulse => 0x19f,
            Self::Punch => 0x1a0,
            Self::Pout => 0x1a1,
            Self::Regret => 0x1a2,
            Self::Revenge => 0x1a4,
            Self::RollEyes => 0x1a5,
            Self::Ruffle => 0x1a6,
            Self::Sad => 0x1a7,
            Self::Scoff => 0x1a8,
            Self::Scold => 0x1a9,
            Self::Scowl => 0x1aa,
            Self::Search => 0x1ab,
            Self::Shakefist => 0x1ac,
            Self::Shifty => 0x1ad,
            Self::Shudder => 0x1ae,
            Self::Signal => 0x1af,
            Self::Silence => 0x1b0,
            Self::Sing => 0x1b1,
            Self::Smack => 0x1b2,
            Self::Sneak => 0x1b3,
            Self::Sneeze => 0x1b4,
            Self::Snort => 0x1b5,
            Self::Squeal => 0x1b6,
            Self::StopAttack => 0x1b7,
            Self::Suspicious => 0x1b8,
            Self::Think => 0x1b9,
            Self::Truce => 0x1ba,
            Self::Twiddle => 0x1bb,
            Self::Warn => 0x1bc,
            Self::Snap => 0x1bd,
            Self::Charm => 0x1be,
            Self::CoverEars => 0x1bf,
            Self::CrossArms => 0x1c0,
            Self::Look => 0x1c1,
            Self::Object => 0x1c2,
            Self::Sweat => 0x1c3,
            Self::Yw => 0x1c5,
        }
    }

    pub const fn variants() -> [Self; 252] {
        [
            Self::Agree,
            Self::Amaze,
            Self::Angry,
            Self::Apologize,
            Self::Applaud,
            Self::Bashful,
            Self::Beckon,
            Self::Beg,
            Self::Bite,
            Self::Bleed,
            Self::Blink,
            Self::Blush,
            Self::Bonk,
            Self::Bored,
            Self::Bounce,
            Self::Brb,
            Self::Bow,
            Self::Burp,
            Self::Bye,
            Self::Cackle,
            Self::Cheer,
            Self::Chicken,
            Self::Chuckle,
            Self::Clap,
            Self::Confused,
            Self::Congratulate,
            Self::Cough,
            Self::Cower,
            Self::Crack,
            Self::Cringe,
            Self::Cry,
            Self::Curious,
            Self::Curtsey,
            Self::Dance,
            Self::Drink,
            Self::Drool,
            Self::Eat,
            Self::Eye,
            Self::Fart,
            Self::Fidget,
            Self::Flex,
            Self::Frown,
            Self::Gasp,
            Self::Gaze,
            Self::Giggle,
            Self::Glare,
            Self::Gloat,
            Self::Greet,
            Self::Grin,
            Self::Groan,
            Self::Grovel,
            Self::Guffaw,
            Self::Hail,
            Self::Happy,
            Self::Hello,
            Self::Hug,
            Self::Hungry,
            Self::Kiss,
            Self::Kneel,
            Self::Laugh,
            Self::Laydown,
            Self::Massage,
            Self::Moan,
            Self::Moon,
            Self::Mourn,
            Self::No,
            Self::Nod,
            Self::NosePick,
            Self::Panic,
            Self::Peer,
            Self::Plead,
            Self::Point,
            Self::Poke,
            Self::Pray,
            Self::Roar,
            Self::Rofl,
            Self::Rude,
            Self::Salute,
            Self::Scratch,
            Self::Sexy,
            Self::Shake,
            Self::Shout,
            Self::Shrug,
            Self::Shy,
            Self::Sigh,
            Self::Sit,
            Self::Sleep,
            Self::Snarl,
            Self::Spit,
            Self::Stare,
            Self::Surprised,
            Self::Surrender,
            Self::Talk,
            Self::TalkEx,
            Self::TalkQ,
            Self::Tap,
            Self::Thank,
            Self::Threaten,
            Self::Tired,
            Self::Victory,
            Self::Wave,
            Self::Welcome,
            Self::Whine,
            Self::Whistle,
            Self::Work,
            Self::Yawn,
            Self::Boggle,
            Self::Calm,
            Self::Cold,
            Self::Comfort,
            Self::Cuddle,
            Self::Duck,
            Self::Insult,
            Self::Introduce,
            Self::Jk,
            Self::Lick,
            Self::Listen,
            Self::Lost,
            Self::Mock,
            Self::Ponder,
            Self::Pounce,
            Self::Praise,
            Self::Purr,
            Self::Puzzle,
            Self::Raise,
            Self::Ready,
            Self::Shimmy,
            Self::Shiver,
            Self::Shoo,
            Self::Slap,
            Self::Smirk,
            Self::Sniff,
            Self::Snub,
            Self::Soothe,
            Self::Stink,
            Self::Taunt,
            Self::Tease,
            Self::Thirsty,
            Self::Veto,
            Self::Snicker,
            Self::Stand,
            Self::Tickle,
            Self::Violin,
            Self::Smile,
            Self::Rasp,
            Self::Pity,
            Self::Growl,
            Self::Bark,
            Self::Scared,
            Self::Flop,
            Self::Love,
            Self::Moo,
            Self::Commend,
            Self::Train,
            Self::Helpme,
            Self::Incoming,
            Self::Charge,
            Self::Flee,
            Self::AttackMyTarget,
            Self::Oom,
            Self::Follow,
            Self::Wait,
            Self::HealMe,
            Self::OpenFire,
            Self::Flirt,
            Self::Joke,
            Self::GolfClap,
            Self::Wink,
            Self::Pat,
            Self::Serious,
            Self::MountSpecial,
            Self::GoodLuck,
            Self::Blame,
            Self::Blank,
            Self::Brandish,
            Self::Breath,
            Self::Disagree,
            Self::Doubt,
            Self::Embarrass,
            Self::Encourage,
            Self::Enemy,
            Self::EyeBrow,
            Self::Toast,
            Self::Fail,
            Self::HighFive,
            Self::Absent,
            Self::Arm,
            Self::Awe,
            Self::Backpack,
            Self::BadFeeling,
            Self::Challenge,
            Self::Chug,
            Self::Ding,
            Self::FacePalm,
            Self::Faint,
            Self::Go,
            Self::Going,
            Self::Glower,
            Self::Headache,
            Self::Hiccup,
            Self::Hiss,
            Self::HoldHand,
            Self::Hurry,
            Self::Idea,
            Self::Jealous,
            Self::Luck,
            Self::Map,
            Self::Mercy,
            Self::Mutter,
            Self::Nervous,
            Self::Offer,
            Self::Pet,
            Self::Pinch,
            Self::Proud,
            Self::Promise,
            Self::Pulse,
            Self::Punch,
            Self::Pout,
            Self::Regret,
            Self::Revenge,
            Self::RollEyes,
            Self::Ruffle,
            Self::Sad,
            Self::Scoff,
            Self::Scold,
            Self::Scowl,
            Self::Search,
            Self::Shakefist,
            Self::Shifty,
            Self::Shudder,
            Self::Signal,
            Self::Silence,
            Self::Sing,
            Self::Smack,
            Self::Sneak,
            Self::Sneeze,
            Self::Snort,
            Self::Squeal,
            Self::StopAttack,
            Self::Suspicious,
            Self::Think,
            Self::Truce,
            Self::Twiddle,
            Self::Warn,
            Self::Snap,
            Self::Charm,
            Self::CoverEars,
            Self::CrossArms,
            Self::Look,
            Self::Object,
            Self::Sweat,
            Self::Yw,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            1 => Ok(Self::Agree),
            2 => Ok(Self::Amaze),
            3 => Ok(Self::Angry),
            4 => Ok(Self::Apologize),
            5 => Ok(Self::Applaud),
            6 => Ok(Self::Bashful),
            7 => Ok(Self::Beckon),
            8 => Ok(Self::Beg),
            9 => Ok(Self::Bite),
            10 => Ok(Self::Bleed),
            11 => Ok(Self::Blink),
            12 => Ok(Self::Blush),
            13 => Ok(Self::Bonk),
            14 => Ok(Self::Bored),
            15 => Ok(Self::Bounce),
            16 => Ok(Self::Brb),
            17 => Ok(Self::Bow),
            18 => Ok(Self::Burp),
            19 => Ok(Self::Bye),
            20 => Ok(Self::Cackle),
            21 => Ok(Self::Cheer),
            22 => Ok(Self::Chicken),
            23 => Ok(Self::Chuckle),
            24 => Ok(Self::Clap),
            25 => Ok(Self::Confused),
            26 => Ok(Self::Congratulate),
            27 => Ok(Self::Cough),
            28 => Ok(Self::Cower),
            29 => Ok(Self::Crack),
            30 => Ok(Self::Cringe),
            31 => Ok(Self::Cry),
            32 => Ok(Self::Curious),
            33 => Ok(Self::Curtsey),
            34 => Ok(Self::Dance),
            35 => Ok(Self::Drink),
            36 => Ok(Self::Drool),
            37 => Ok(Self::Eat),
            38 => Ok(Self::Eye),
            39 => Ok(Self::Fart),
            40 => Ok(Self::Fidget),
            41 => Ok(Self::Flex),
            42 => Ok(Self::Frown),
            43 => Ok(Self::Gasp),
            44 => Ok(Self::Gaze),
            45 => Ok(Self::Giggle),
            46 => Ok(Self::Glare),
            47 => Ok(Self::Gloat),
            48 => Ok(Self::Greet),
            49 => Ok(Self::Grin),
            50 => Ok(Self::Groan),
            51 => Ok(Self::Grovel),
            52 => Ok(Self::Guffaw),
            53 => Ok(Self::Hail),
            54 => Ok(Self::Happy),
            55 => Ok(Self::Hello),
            56 => Ok(Self::Hug),
            57 => Ok(Self::Hungry),
            58 => Ok(Self::Kiss),
            59 => Ok(Self::Kneel),
            60 => Ok(Self::Laugh),
            61 => Ok(Self::Laydown),
            62 => Ok(Self::Massage),
            63 => Ok(Self::Moan),
            64 => Ok(Self::Moon),
            65 => Ok(Self::Mourn),
            66 => Ok(Self::No),
            67 => Ok(Self::Nod),
            68 => Ok(Self::NosePick),
            69 => Ok(Self::Panic),
            70 => Ok(Self::Peer),
            71 => Ok(Self::Plead),
            72 => Ok(Self::Point),
            73 => Ok(Self::Poke),
            74 => Ok(Self::Pray),
            75 => Ok(Self::Roar),
            76 => Ok(Self::Rofl),
            77 => Ok(Self::Rude),
            78 => Ok(Self::Salute),
            79 => Ok(Self::Scratch),
            80 => Ok(Self::Sexy),
            81 => Ok(Self::Shake),
            82 => Ok(Self::Shout),
            83 => Ok(Self::Shrug),
            84 => Ok(Self::Shy),
            85 => Ok(Self::Sigh),
            86 => Ok(Self::Sit),
            87 => Ok(Self::Sleep),
            88 => Ok(Self::Snarl),
            89 => Ok(Self::Spit),
            90 => Ok(Self::Stare),
            91 => Ok(Self::Surprised),
            92 => Ok(Self::Surrender),
            93 => Ok(Self::Talk),
            94 => Ok(Self::TalkEx),
            95 => Ok(Self::TalkQ),
            96 => Ok(Self::Tap),
            97 => Ok(Self::Thank),
            98 => Ok(Self::Threaten),
            99 => Ok(Self::Tired),
            100 => Ok(Self::Victory),
            101 => Ok(Self::Wave),
            102 => Ok(Self::Welcome),
            103 => Ok(Self::Whine),
            104 => Ok(Self::Whistle),
            105 => Ok(Self::Work),
            106 => Ok(Self::Yawn),
            107 => Ok(Self::Boggle),
            108 => Ok(Self::Calm),
            109 => Ok(Self::Cold),
            110 => Ok(Self::Comfort),
            111 => Ok(Self::Cuddle),
            112 => Ok(Self::Duck),
            113 => Ok(Self::Insult),
            114 => Ok(Self::Introduce),
            115 => Ok(Self::Jk),
            116 => Ok(Self::Lick),
            117 => Ok(Self::Listen),
            118 => Ok(Self::Lost),
            119 => Ok(Self::Mock),
            120 => Ok(Self::Ponder),
            121 => Ok(Self::Pounce),
            122 => Ok(Self::Praise),
            123 => Ok(Self::Purr),
            124 => Ok(Self::Puzzle),
            125 => Ok(Self::Raise),
            126 => Ok(Self::Ready),
            127 => Ok(Self::Shimmy),
            128 => Ok(Self::Shiver),
            129 => Ok(Self::Shoo),
            130 => Ok(Self::Slap),
            131 => Ok(Self::Smirk),
            132 => Ok(Self::Sniff),
            133 => Ok(Self::Snub),
            134 => Ok(Self::Soothe),
            135 => Ok(Self::Stink),
            136 => Ok(Self::Taunt),
            137 => Ok(Self::Tease),
            138 => Ok(Self::Thirsty),
            139 => Ok(Self::Veto),
            140 => Ok(Self::Snicker),
            141 => Ok(Self::Stand),
            142 => Ok(Self::Tickle),
            143 => Ok(Self::Violin),
            163 => Ok(Self::Smile),
            183 => Ok(Self::Rasp),
            203 => Ok(Self::Pity),
            204 => Ok(Self::Growl),
            205 => Ok(Self::Bark),
            223 => Ok(Self::Scared),
            224 => Ok(Self::Flop),
            225 => Ok(Self::Love),
            226 => Ok(Self::Moo),
            243 => Ok(Self::Commend),
            264 => Ok(Self::Train),
            303 => Ok(Self::Helpme),
            304 => Ok(Self::Incoming),
            305 => Ok(Self::Charge),
            306 => Ok(Self::Flee),
            307 => Ok(Self::AttackMyTarget),
            323 => Ok(Self::Oom),
            324 => Ok(Self::Follow),
            325 => Ok(Self::Wait),
            326 => Ok(Self::HealMe),
            327 => Ok(Self::OpenFire),
            328 => Ok(Self::Flirt),
            329 => Ok(Self::Joke),
            343 => Ok(Self::GolfClap),
            363 => Ok(Self::Wink),
            364 => Ok(Self::Pat),
            365 => Ok(Self::Serious),
            366 => Ok(Self::MountSpecial),
            367 => Ok(Self::GoodLuck),
            368 => Ok(Self::Blame),
            369 => Ok(Self::Blank),
            370 => Ok(Self::Brandish),
            371 => Ok(Self::Breath),
            372 => Ok(Self::Disagree),
            373 => Ok(Self::Doubt),
            374 => Ok(Self::Embarrass),
            375 => Ok(Self::Encourage),
            376 => Ok(Self::Enemy),
            377 => Ok(Self::EyeBrow),
            378 => Ok(Self::Toast),
            379 => Ok(Self::Fail),
            380 => Ok(Self::HighFive),
            381 => Ok(Self::Absent),
            382 => Ok(Self::Arm),
            383 => Ok(Self::Awe),
            384 => Ok(Self::Backpack),
            385 => Ok(Self::BadFeeling),
            386 => Ok(Self::Challenge),
            387 => Ok(Self::Chug),
            389 => Ok(Self::Ding),
            390 => Ok(Self::FacePalm),
            391 => Ok(Self::Faint),
            392 => Ok(Self::Go),
            393 => Ok(Self::Going),
            394 => Ok(Self::Glower),
            395 => Ok(Self::Headache),
            396 => Ok(Self::Hiccup),
            398 => Ok(Self::Hiss),
            399 => Ok(Self::HoldHand),
            401 => Ok(Self::Hurry),
            402 => Ok(Self::Idea),
            403 => Ok(Self::Jealous),
            404 => Ok(Self::Luck),
            405 => Ok(Self::Map),
            406 => Ok(Self::Mercy),
            407 => Ok(Self::Mutter),
            408 => Ok(Self::Nervous),
            409 => Ok(Self::Offer),
            410 => Ok(Self::Pet),
            411 => Ok(Self::Pinch),
            413 => Ok(Self::Proud),
            414 => Ok(Self::Promise),
            415 => Ok(Self::Pulse),
            416 => Ok(Self::Punch),
            417 => Ok(Self::Pout),
            418 => Ok(Self::Regret),
            420 => Ok(Self::Revenge),
            421 => Ok(Self::RollEyes),
            422 => Ok(Self::Ruffle),
            423 => Ok(Self::Sad),
            424 => Ok(Self::Scoff),
            425 => Ok(Self::Scold),
            426 => Ok(Self::Scowl),
            427 => Ok(Self::Search),
            428 => Ok(Self::Shakefist),
            429 => Ok(Self::Shifty),
            430 => Ok(Self::Shudder),
            431 => Ok(Self::Signal),
            432 => Ok(Self::Silence),
            433 => Ok(Self::Sing),
            434 => Ok(Self::Smack),
            435 => Ok(Self::Sneak),
            436 => Ok(Self::Sneeze),
            437 => Ok(Self::Snort),
            438 => Ok(Self::Squeal),
            439 => Ok(Self::StopAttack),
            440 => Ok(Self::Suspicious),
            441 => Ok(Self::Think),
            442 => Ok(Self::Truce),
            443 => Ok(Self::Twiddle),
            444 => Ok(Self::Warn),
            445 => Ok(Self::Snap),
            446 => Ok(Self::Charm),
            447 => Ok(Self::CoverEars),
            448 => Ok(Self::CrossArms),
            449 => Ok(Self::Look),
            450 => Ok(Self::Object),
            451 => Ok(Self::Sweat),
            453 => Ok(Self::Yw),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl TextEmote {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Agree => "AGREE",
            Self::Amaze => "AMAZE",
            Self::Angry => "ANGRY",
            Self::Apologize => "APOLOGIZE",
            Self::Applaud => "APPLAUD",
            Self::Bashful => "BASHFUL",
            Self::Beckon => "BECKON",
            Self::Beg => "BEG",
            Self::Bite => "BITE",
            Self::Bleed => "BLEED",
            Self::Blink => "BLINK",
            Self::Blush => "BLUSH",
            Self::Bonk => "BONK",
            Self::Bored => "BORED",
            Self::Bounce => "BOUNCE",
            Self::Brb => "BRB",
            Self::Bow => "BOW",
            Self::Burp => "BURP",
            Self::Bye => "BYE",
            Self::Cackle => "CACKLE",
            Self::Cheer => "CHEER",
            Self::Chicken => "CHICKEN",
            Self::Chuckle => "CHUCKLE",
            Self::Clap => "CLAP",
            Self::Confused => "CONFUSED",
            Self::Congratulate => "CONGRATULATE",
            Self::Cough => "COUGH",
            Self::Cower => "COWER",
            Self::Crack => "CRACK",
            Self::Cringe => "CRINGE",
            Self::Cry => "CRY",
            Self::Curious => "CURIOUS",
            Self::Curtsey => "CURTSEY",
            Self::Dance => "DANCE",
            Self::Drink => "DRINK",
            Self::Drool => "DROOL",
            Self::Eat => "EAT",
            Self::Eye => "EYE",
            Self::Fart => "FART",
            Self::Fidget => "FIDGET",
            Self::Flex => "FLEX",
            Self::Frown => "FROWN",
            Self::Gasp => "GASP",
            Self::Gaze => "GAZE",
            Self::Giggle => "GIGGLE",
            Self::Glare => "GLARE",
            Self::Gloat => "GLOAT",
            Self::Greet => "GREET",
            Self::Grin => "GRIN",
            Self::Groan => "GROAN",
            Self::Grovel => "GROVEL",
            Self::Guffaw => "GUFFAW",
            Self::Hail => "HAIL",
            Self::Happy => "HAPPY",
            Self::Hello => "HELLO",
            Self::Hug => "HUG",
            Self::Hungry => "HUNGRY",
            Self::Kiss => "KISS",
            Self::Kneel => "KNEEL",
            Self::Laugh => "LAUGH",
            Self::Laydown => "LAYDOWN",
            Self::Massage => "MASSAGE",
            Self::Moan => "MOAN",
            Self::Moon => "MOON",
            Self::Mourn => "MOURN",
            Self::No => "NO",
            Self::Nod => "NOD",
            Self::NosePick => "NOSE_PICK",
            Self::Panic => "PANIC",
            Self::Peer => "PEER",
            Self::Plead => "PLEAD",
            Self::Point => "POINT",
            Self::Poke => "POKE",
            Self::Pray => "PRAY",
            Self::Roar => "ROAR",
            Self::Rofl => "ROFL",
            Self::Rude => "RUDE",
            Self::Salute => "SALUTE",
            Self::Scratch => "SCRATCH",
            Self::Sexy => "SEXY",
            Self::Shake => "SHAKE",
            Self::Shout => "SHOUT",
            Self::Shrug => "SHRUG",
            Self::Shy => "SHY",
            Self::Sigh => "SIGH",
            Self::Sit => "SIT",
            Self::Sleep => "SLEEP",
            Self::Snarl => "SNARL",
            Self::Spit => "SPIT",
            Self::Stare => "STARE",
            Self::Surprised => "SURPRISED",
            Self::Surrender => "SURRENDER",
            Self::Talk => "TALK",
            Self::TalkEx => "TALK_EX",
            Self::TalkQ => "TALK_Q",
            Self::Tap => "TAP",
            Self::Thank => "THANK",
            Self::Threaten => "THREATEN",
            Self::Tired => "TIRED",
            Self::Victory => "VICTORY",
            Self::Wave => "WAVE",
            Self::Welcome => "WELCOME",
            Self::Whine => "WHINE",
            Self::Whistle => "WHISTLE",
            Self::Work => "WORK",
            Self::Yawn => "YAWN",
            Self::Boggle => "BOGGLE",
            Self::Calm => "CALM",
            Self::Cold => "COLD",
            Self::Comfort => "COMFORT",
            Self::Cuddle => "CUDDLE",
            Self::Duck => "DUCK",
            Self::Insult => "INSULT",
            Self::Introduce => "INTRODUCE",
            Self::Jk => "JK",
            Self::Lick => "LICK",
            Self::Listen => "LISTEN",
            Self::Lost => "LOST",
            Self::Mock => "MOCK",
            Self::Ponder => "PONDER",
            Self::Pounce => "POUNCE",
            Self::Praise => "PRAISE",
            Self::Purr => "PURR",
            Self::Puzzle => "PUZZLE",
            Self::Raise => "RAISE",
            Self::Ready => "READY",
            Self::Shimmy => "SHIMMY",
            Self::Shiver => "SHIVER",
            Self::Shoo => "SHOO",
            Self::Slap => "SLAP",
            Self::Smirk => "SMIRK",
            Self::Sniff => "SNIFF",
            Self::Snub => "SNUB",
            Self::Soothe => "SOOTHE",
            Self::Stink => "STINK",
            Self::Taunt => "TAUNT",
            Self::Tease => "TEASE",
            Self::Thirsty => "THIRSTY",
            Self::Veto => "VETO",
            Self::Snicker => "SNICKER",
            Self::Stand => "STAND",
            Self::Tickle => "TICKLE",
            Self::Violin => "VIOLIN",
            Self::Smile => "SMILE",
            Self::Rasp => "RASP",
            Self::Pity => "PITY",
            Self::Growl => "GROWL",
            Self::Bark => "BARK",
            Self::Scared => "SCARED",
            Self::Flop => "FLOP",
            Self::Love => "LOVE",
            Self::Moo => "MOO",
            Self::Commend => "COMMEND",
            Self::Train => "TRAIN",
            Self::Helpme => "HELPME",
            Self::Incoming => "INCOMING",
            Self::Charge => "CHARGE",
            Self::Flee => "FLEE",
            Self::AttackMyTarget => "ATTACK_MY_TARGET",
            Self::Oom => "OOM",
            Self::Follow => "FOLLOW",
            Self::Wait => "WAIT",
            Self::HealMe => "HEAL_ME",
            Self::OpenFire => "OPEN_FIRE",
            Self::Flirt => "FLIRT",
            Self::Joke => "JOKE",
            Self::GolfClap => "GOLF_CLAP",
            Self::Wink => "WINK",
            Self::Pat => "PAT",
            Self::Serious => "SERIOUS",
            Self::MountSpecial => "MOUNT_SPECIAL",
            Self::GoodLuck => "GOOD_LUCK",
            Self::Blame => "BLAME",
            Self::Blank => "BLANK",
            Self::Brandish => "BRANDISH",
            Self::Breath => "BREATH",
            Self::Disagree => "DISAGREE",
            Self::Doubt => "DOUBT",
            Self::Embarrass => "EMBARRASS",
            Self::Encourage => "ENCOURAGE",
            Self::Enemy => "ENEMY",
            Self::EyeBrow => "EYE_BROW",
            Self::Toast => "TOAST",
            Self::Fail => "FAIL",
            Self::HighFive => "HIGH_FIVE",
            Self::Absent => "ABSENT",
            Self::Arm => "ARM",
            Self::Awe => "AWE",
            Self::Backpack => "BACKPACK",
            Self::BadFeeling => "BAD_FEELING",
            Self::Challenge => "CHALLENGE",
            Self::Chug => "CHUG",
            Self::Ding => "DING",
            Self::FacePalm => "FACE_PALM",
            Self::Faint => "FAINT",
            Self::Go => "GO",
            Self::Going => "GOING",
            Self::Glower => "GLOWER",
            Self::Headache => "HEADACHE",
            Self::Hiccup => "HICCUP",
            Self::Hiss => "HISS",
            Self::HoldHand => "HOLD_HAND",
            Self::Hurry => "HURRY",
            Self::Idea => "IDEA",
            Self::Jealous => "JEALOUS",
            Self::Luck => "LUCK",
            Self::Map => "MAP",
            Self::Mercy => "MERCY",
            Self::Mutter => "MUTTER",
            Self::Nervous => "NERVOUS",
            Self::Offer => "OFFER",
            Self::Pet => "PET",
            Self::Pinch => "PINCH",
            Self::Proud => "PROUD",
            Self::Promise => "PROMISE",
            Self::Pulse => "PULSE",
            Self::Punch => "PUNCH",
            Self::Pout => "POUT",
            Self::Regret => "REGRET",
            Self::Revenge => "REVENGE",
            Self::RollEyes => "ROLL_EYES",
            Self::Ruffle => "RUFFLE",
            Self::Sad => "SAD",
            Self::Scoff => "SCOFF",
            Self::Scold => "SCOLD",
            Self::Scowl => "SCOWL",
            Self::Search => "SEARCH",
            Self::Shakefist => "SHAKEFIST",
            Self::Shifty => "SHIFTY",
            Self::Shudder => "SHUDDER",
            Self::Signal => "SIGNAL",
            Self::Silence => "SILENCE",
            Self::Sing => "SING",
            Self::Smack => "SMACK",
            Self::Sneak => "SNEAK",
            Self::Sneeze => "SNEEZE",
            Self::Snort => "SNORT",
            Self::Squeal => "SQUEAL",
            Self::StopAttack => "STOP_ATTACK",
            Self::Suspicious => "SUSPICIOUS",
            Self::Think => "THINK",
            Self::Truce => "TRUCE",
            Self::Twiddle => "TWIDDLE",
            Self::Warn => "WARN",
            Self::Snap => "SNAP",
            Self::Charm => "CHARM",
            Self::CoverEars => "COVER_EARS",
            Self::CrossArms => "CROSS_ARMS",
            Self::Look => "LOOK",
            Self::Object => "OBJECT",
            Self::Sweat => "SWEAT",
            Self::Yw => "YW",
        }
    }

}

const NAME: &str = "TextEmote";

impl Default for TextEmote {
    fn default() -> Self {
        Self::Agree
    }
}

impl std::fmt::Display for TextEmote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Agree => f.write_str("Agree"),
            Self::Amaze => f.write_str("Amaze"),
            Self::Angry => f.write_str("Angry"),
            Self::Apologize => f.write_str("Apologize"),
            Self::Applaud => f.write_str("Applaud"),
            Self::Bashful => f.write_str("Bashful"),
            Self::Beckon => f.write_str("Beckon"),
            Self::Beg => f.write_str("Beg"),
            Self::Bite => f.write_str("Bite"),
            Self::Bleed => f.write_str("Bleed"),
            Self::Blink => f.write_str("Blink"),
            Self::Blush => f.write_str("Blush"),
            Self::Bonk => f.write_str("Bonk"),
            Self::Bored => f.write_str("Bored"),
            Self::Bounce => f.write_str("Bounce"),
            Self::Brb => f.write_str("Brb"),
            Self::Bow => f.write_str("Bow"),
            Self::Burp => f.write_str("Burp"),
            Self::Bye => f.write_str("Bye"),
            Self::Cackle => f.write_str("Cackle"),
            Self::Cheer => f.write_str("Cheer"),
            Self::Chicken => f.write_str("Chicken"),
            Self::Chuckle => f.write_str("Chuckle"),
            Self::Clap => f.write_str("Clap"),
            Self::Confused => f.write_str("Confused"),
            Self::Congratulate => f.write_str("Congratulate"),
            Self::Cough => f.write_str("Cough"),
            Self::Cower => f.write_str("Cower"),
            Self::Crack => f.write_str("Crack"),
            Self::Cringe => f.write_str("Cringe"),
            Self::Cry => f.write_str("Cry"),
            Self::Curious => f.write_str("Curious"),
            Self::Curtsey => f.write_str("Curtsey"),
            Self::Dance => f.write_str("Dance"),
            Self::Drink => f.write_str("Drink"),
            Self::Drool => f.write_str("Drool"),
            Self::Eat => f.write_str("Eat"),
            Self::Eye => f.write_str("Eye"),
            Self::Fart => f.write_str("Fart"),
            Self::Fidget => f.write_str("Fidget"),
            Self::Flex => f.write_str("Flex"),
            Self::Frown => f.write_str("Frown"),
            Self::Gasp => f.write_str("Gasp"),
            Self::Gaze => f.write_str("Gaze"),
            Self::Giggle => f.write_str("Giggle"),
            Self::Glare => f.write_str("Glare"),
            Self::Gloat => f.write_str("Gloat"),
            Self::Greet => f.write_str("Greet"),
            Self::Grin => f.write_str("Grin"),
            Self::Groan => f.write_str("Groan"),
            Self::Grovel => f.write_str("Grovel"),
            Self::Guffaw => f.write_str("Guffaw"),
            Self::Hail => f.write_str("Hail"),
            Self::Happy => f.write_str("Happy"),
            Self::Hello => f.write_str("Hello"),
            Self::Hug => f.write_str("Hug"),
            Self::Hungry => f.write_str("Hungry"),
            Self::Kiss => f.write_str("Kiss"),
            Self::Kneel => f.write_str("Kneel"),
            Self::Laugh => f.write_str("Laugh"),
            Self::Laydown => f.write_str("Laydown"),
            Self::Massage => f.write_str("Massage"),
            Self::Moan => f.write_str("Moan"),
            Self::Moon => f.write_str("Moon"),
            Self::Mourn => f.write_str("Mourn"),
            Self::No => f.write_str("No"),
            Self::Nod => f.write_str("Nod"),
            Self::NosePick => f.write_str("NosePick"),
            Self::Panic => f.write_str("Panic"),
            Self::Peer => f.write_str("Peer"),
            Self::Plead => f.write_str("Plead"),
            Self::Point => f.write_str("Point"),
            Self::Poke => f.write_str("Poke"),
            Self::Pray => f.write_str("Pray"),
            Self::Roar => f.write_str("Roar"),
            Self::Rofl => f.write_str("Rofl"),
            Self::Rude => f.write_str("Rude"),
            Self::Salute => f.write_str("Salute"),
            Self::Scratch => f.write_str("Scratch"),
            Self::Sexy => f.write_str("Sexy"),
            Self::Shake => f.write_str("Shake"),
            Self::Shout => f.write_str("Shout"),
            Self::Shrug => f.write_str("Shrug"),
            Self::Shy => f.write_str("Shy"),
            Self::Sigh => f.write_str("Sigh"),
            Self::Sit => f.write_str("Sit"),
            Self::Sleep => f.write_str("Sleep"),
            Self::Snarl => f.write_str("Snarl"),
            Self::Spit => f.write_str("Spit"),
            Self::Stare => f.write_str("Stare"),
            Self::Surprised => f.write_str("Surprised"),
            Self::Surrender => f.write_str("Surrender"),
            Self::Talk => f.write_str("Talk"),
            Self::TalkEx => f.write_str("TalkEx"),
            Self::TalkQ => f.write_str("TalkQ"),
            Self::Tap => f.write_str("Tap"),
            Self::Thank => f.write_str("Thank"),
            Self::Threaten => f.write_str("Threaten"),
            Self::Tired => f.write_str("Tired"),
            Self::Victory => f.write_str("Victory"),
            Self::Wave => f.write_str("Wave"),
            Self::Welcome => f.write_str("Welcome"),
            Self::Whine => f.write_str("Whine"),
            Self::Whistle => f.write_str("Whistle"),
            Self::Work => f.write_str("Work"),
            Self::Yawn => f.write_str("Yawn"),
            Self::Boggle => f.write_str("Boggle"),
            Self::Calm => f.write_str("Calm"),
            Self::Cold => f.write_str("Cold"),
            Self::Comfort => f.write_str("Comfort"),
            Self::Cuddle => f.write_str("Cuddle"),
            Self::Duck => f.write_str("Duck"),
            Self::Insult => f.write_str("Insult"),
            Self::Introduce => f.write_str("Introduce"),
            Self::Jk => f.write_str("Jk"),
            Self::Lick => f.write_str("Lick"),
            Self::Listen => f.write_str("Listen"),
            Self::Lost => f.write_str("Lost"),
            Self::Mock => f.write_str("Mock"),
            Self::Ponder => f.write_str("Ponder"),
            Self::Pounce => f.write_str("Pounce"),
            Self::Praise => f.write_str("Praise"),
            Self::Purr => f.write_str("Purr"),
            Self::Puzzle => f.write_str("Puzzle"),
            Self::Raise => f.write_str("Raise"),
            Self::Ready => f.write_str("Ready"),
            Self::Shimmy => f.write_str("Shimmy"),
            Self::Shiver => f.write_str("Shiver"),
            Self::Shoo => f.write_str("Shoo"),
            Self::Slap => f.write_str("Slap"),
            Self::Smirk => f.write_str("Smirk"),
            Self::Sniff => f.write_str("Sniff"),
            Self::Snub => f.write_str("Snub"),
            Self::Soothe => f.write_str("Soothe"),
            Self::Stink => f.write_str("Stink"),
            Self::Taunt => f.write_str("Taunt"),
            Self::Tease => f.write_str("Tease"),
            Self::Thirsty => f.write_str("Thirsty"),
            Self::Veto => f.write_str("Veto"),
            Self::Snicker => f.write_str("Snicker"),
            Self::Stand => f.write_str("Stand"),
            Self::Tickle => f.write_str("Tickle"),
            Self::Violin => f.write_str("Violin"),
            Self::Smile => f.write_str("Smile"),
            Self::Rasp => f.write_str("Rasp"),
            Self::Pity => f.write_str("Pity"),
            Self::Growl => f.write_str("Growl"),
            Self::Bark => f.write_str("Bark"),
            Self::Scared => f.write_str("Scared"),
            Self::Flop => f.write_str("Flop"),
            Self::Love => f.write_str("Love"),
            Self::Moo => f.write_str("Moo"),
            Self::Commend => f.write_str("Commend"),
            Self::Train => f.write_str("Train"),
            Self::Helpme => f.write_str("Helpme"),
            Self::Incoming => f.write_str("Incoming"),
            Self::Charge => f.write_str("Charge"),
            Self::Flee => f.write_str("Flee"),
            Self::AttackMyTarget => f.write_str("AttackMyTarget"),
            Self::Oom => f.write_str("Oom"),
            Self::Follow => f.write_str("Follow"),
            Self::Wait => f.write_str("Wait"),
            Self::HealMe => f.write_str("HealMe"),
            Self::OpenFire => f.write_str("OpenFire"),
            Self::Flirt => f.write_str("Flirt"),
            Self::Joke => f.write_str("Joke"),
            Self::GolfClap => f.write_str("GolfClap"),
            Self::Wink => f.write_str("Wink"),
            Self::Pat => f.write_str("Pat"),
            Self::Serious => f.write_str("Serious"),
            Self::MountSpecial => f.write_str("MountSpecial"),
            Self::GoodLuck => f.write_str("GoodLuck"),
            Self::Blame => f.write_str("Blame"),
            Self::Blank => f.write_str("Blank"),
            Self::Brandish => f.write_str("Brandish"),
            Self::Breath => f.write_str("Breath"),
            Self::Disagree => f.write_str("Disagree"),
            Self::Doubt => f.write_str("Doubt"),
            Self::Embarrass => f.write_str("Embarrass"),
            Self::Encourage => f.write_str("Encourage"),
            Self::Enemy => f.write_str("Enemy"),
            Self::EyeBrow => f.write_str("EyeBrow"),
            Self::Toast => f.write_str("Toast"),
            Self::Fail => f.write_str("Fail"),
            Self::HighFive => f.write_str("HighFive"),
            Self::Absent => f.write_str("Absent"),
            Self::Arm => f.write_str("Arm"),
            Self::Awe => f.write_str("Awe"),
            Self::Backpack => f.write_str("Backpack"),
            Self::BadFeeling => f.write_str("BadFeeling"),
            Self::Challenge => f.write_str("Challenge"),
            Self::Chug => f.write_str("Chug"),
            Self::Ding => f.write_str("Ding"),
            Self::FacePalm => f.write_str("FacePalm"),
            Self::Faint => f.write_str("Faint"),
            Self::Go => f.write_str("Go"),
            Self::Going => f.write_str("Going"),
            Self::Glower => f.write_str("Glower"),
            Self::Headache => f.write_str("Headache"),
            Self::Hiccup => f.write_str("Hiccup"),
            Self::Hiss => f.write_str("Hiss"),
            Self::HoldHand => f.write_str("HoldHand"),
            Self::Hurry => f.write_str("Hurry"),
            Self::Idea => f.write_str("Idea"),
            Self::Jealous => f.write_str("Jealous"),
            Self::Luck => f.write_str("Luck"),
            Self::Map => f.write_str("Map"),
            Self::Mercy => f.write_str("Mercy"),
            Self::Mutter => f.write_str("Mutter"),
            Self::Nervous => f.write_str("Nervous"),
            Self::Offer => f.write_str("Offer"),
            Self::Pet => f.write_str("Pet"),
            Self::Pinch => f.write_str("Pinch"),
            Self::Proud => f.write_str("Proud"),
            Self::Promise => f.write_str("Promise"),
            Self::Pulse => f.write_str("Pulse"),
            Self::Punch => f.write_str("Punch"),
            Self::Pout => f.write_str("Pout"),
            Self::Regret => f.write_str("Regret"),
            Self::Revenge => f.write_str("Revenge"),
            Self::RollEyes => f.write_str("RollEyes"),
            Self::Ruffle => f.write_str("Ruffle"),
            Self::Sad => f.write_str("Sad"),
            Self::Scoff => f.write_str("Scoff"),
            Self::Scold => f.write_str("Scold"),
            Self::Scowl => f.write_str("Scowl"),
            Self::Search => f.write_str("Search"),
            Self::Shakefist => f.write_str("Shakefist"),
            Self::Shifty => f.write_str("Shifty"),
            Self::Shudder => f.write_str("Shudder"),
            Self::Signal => f.write_str("Signal"),
            Self::Silence => f.write_str("Silence"),
            Self::Sing => f.write_str("Sing"),
            Self::Smack => f.write_str("Smack"),
            Self::Sneak => f.write_str("Sneak"),
            Self::Sneeze => f.write_str("Sneeze"),
            Self::Snort => f.write_str("Snort"),
            Self::Squeal => f.write_str("Squeal"),
            Self::StopAttack => f.write_str("StopAttack"),
            Self::Suspicious => f.write_str("Suspicious"),
            Self::Think => f.write_str("Think"),
            Self::Truce => f.write_str("Truce"),
            Self::Twiddle => f.write_str("Twiddle"),
            Self::Warn => f.write_str("Warn"),
            Self::Snap => f.write_str("Snap"),
            Self::Charm => f.write_str("Charm"),
            Self::CoverEars => f.write_str("CoverEars"),
            Self::CrossArms => f.write_str("CrossArms"),
            Self::Look => f.write_str("Look"),
            Self::Object => f.write_str("Object"),
            Self::Sweat => f.write_str("Sweat"),
            Self::Yw => f.write_str("Yw"),
        }
    }
}

impl TryFrom<u32> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

