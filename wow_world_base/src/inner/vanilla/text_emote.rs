/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:207`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L207):
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
        }
    }

    pub const fn variants() -> [Self; 169] {
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
        f.write_str(match self {
            Self::Agree => "Agree",
            Self::Amaze => "Amaze",
            Self::Angry => "Angry",
            Self::Apologize => "Apologize",
            Self::Applaud => "Applaud",
            Self::Bashful => "Bashful",
            Self::Beckon => "Beckon",
            Self::Beg => "Beg",
            Self::Bite => "Bite",
            Self::Bleed => "Bleed",
            Self::Blink => "Blink",
            Self::Blush => "Blush",
            Self::Bonk => "Bonk",
            Self::Bored => "Bored",
            Self::Bounce => "Bounce",
            Self::Brb => "Brb",
            Self::Bow => "Bow",
            Self::Burp => "Burp",
            Self::Bye => "Bye",
            Self::Cackle => "Cackle",
            Self::Cheer => "Cheer",
            Self::Chicken => "Chicken",
            Self::Chuckle => "Chuckle",
            Self::Clap => "Clap",
            Self::Confused => "Confused",
            Self::Congratulate => "Congratulate",
            Self::Cough => "Cough",
            Self::Cower => "Cower",
            Self::Crack => "Crack",
            Self::Cringe => "Cringe",
            Self::Cry => "Cry",
            Self::Curious => "Curious",
            Self::Curtsey => "Curtsey",
            Self::Dance => "Dance",
            Self::Drink => "Drink",
            Self::Drool => "Drool",
            Self::Eat => "Eat",
            Self::Eye => "Eye",
            Self::Fart => "Fart",
            Self::Fidget => "Fidget",
            Self::Flex => "Flex",
            Self::Frown => "Frown",
            Self::Gasp => "Gasp",
            Self::Gaze => "Gaze",
            Self::Giggle => "Giggle",
            Self::Glare => "Glare",
            Self::Gloat => "Gloat",
            Self::Greet => "Greet",
            Self::Grin => "Grin",
            Self::Groan => "Groan",
            Self::Grovel => "Grovel",
            Self::Guffaw => "Guffaw",
            Self::Hail => "Hail",
            Self::Happy => "Happy",
            Self::Hello => "Hello",
            Self::Hug => "Hug",
            Self::Hungry => "Hungry",
            Self::Kiss => "Kiss",
            Self::Kneel => "Kneel",
            Self::Laugh => "Laugh",
            Self::Laydown => "Laydown",
            Self::Massage => "Massage",
            Self::Moan => "Moan",
            Self::Moon => "Moon",
            Self::Mourn => "Mourn",
            Self::No => "No",
            Self::Nod => "Nod",
            Self::NosePick => "NosePick",
            Self::Panic => "Panic",
            Self::Peer => "Peer",
            Self::Plead => "Plead",
            Self::Point => "Point",
            Self::Poke => "Poke",
            Self::Pray => "Pray",
            Self::Roar => "Roar",
            Self::Rofl => "Rofl",
            Self::Rude => "Rude",
            Self::Salute => "Salute",
            Self::Scratch => "Scratch",
            Self::Sexy => "Sexy",
            Self::Shake => "Shake",
            Self::Shout => "Shout",
            Self::Shrug => "Shrug",
            Self::Shy => "Shy",
            Self::Sigh => "Sigh",
            Self::Sit => "Sit",
            Self::Sleep => "Sleep",
            Self::Snarl => "Snarl",
            Self::Spit => "Spit",
            Self::Stare => "Stare",
            Self::Surprised => "Surprised",
            Self::Surrender => "Surrender",
            Self::Talk => "Talk",
            Self::TalkEx => "TalkEx",
            Self::TalkQ => "TalkQ",
            Self::Tap => "Tap",
            Self::Thank => "Thank",
            Self::Threaten => "Threaten",
            Self::Tired => "Tired",
            Self::Victory => "Victory",
            Self::Wave => "Wave",
            Self::Welcome => "Welcome",
            Self::Whine => "Whine",
            Self::Whistle => "Whistle",
            Self::Work => "Work",
            Self::Yawn => "Yawn",
            Self::Boggle => "Boggle",
            Self::Calm => "Calm",
            Self::Cold => "Cold",
            Self::Comfort => "Comfort",
            Self::Cuddle => "Cuddle",
            Self::Duck => "Duck",
            Self::Insult => "Insult",
            Self::Introduce => "Introduce",
            Self::Jk => "Jk",
            Self::Lick => "Lick",
            Self::Listen => "Listen",
            Self::Lost => "Lost",
            Self::Mock => "Mock",
            Self::Ponder => "Ponder",
            Self::Pounce => "Pounce",
            Self::Praise => "Praise",
            Self::Purr => "Purr",
            Self::Puzzle => "Puzzle",
            Self::Raise => "Raise",
            Self::Ready => "Ready",
            Self::Shimmy => "Shimmy",
            Self::Shiver => "Shiver",
            Self::Shoo => "Shoo",
            Self::Slap => "Slap",
            Self::Smirk => "Smirk",
            Self::Sniff => "Sniff",
            Self::Snub => "Snub",
            Self::Soothe => "Soothe",
            Self::Stink => "Stink",
            Self::Taunt => "Taunt",
            Self::Tease => "Tease",
            Self::Thirsty => "Thirsty",
            Self::Veto => "Veto",
            Self::Snicker => "Snicker",
            Self::Stand => "Stand",
            Self::Tickle => "Tickle",
            Self::Violin => "Violin",
            Self::Smile => "Smile",
            Self::Rasp => "Rasp",
            Self::Pity => "Pity",
            Self::Growl => "Growl",
            Self::Bark => "Bark",
            Self::Scared => "Scared",
            Self::Flop => "Flop",
            Self::Love => "Love",
            Self::Moo => "Moo",
            Self::Commend => "Commend",
            Self::Train => "Train",
            Self::Helpme => "Helpme",
            Self::Incoming => "Incoming",
            Self::Charge => "Charge",
            Self::Flee => "Flee",
            Self::AttackMyTarget => "AttackMyTarget",
            Self::Oom => "Oom",
            Self::Follow => "Follow",
            Self::Wait => "Wait",
            Self::HealMe => "HealMe",
            Self::OpenFire => "OpenFire",
            Self::Flirt => "Flirt",
            Self::Joke => "Joke",
            Self::GolfClap => "GolfClap",
            Self::Wink => "Wink",
            Self::Pat => "Pat",
        })
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
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for TextEmote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
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
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
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

