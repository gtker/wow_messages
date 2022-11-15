use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/pet_common.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/pet_common.wowm#L18):
/// ```text
/// enum CreatureFamily : u8 {
///     NONE = 0;
///     WOLF = 1;
///     CAT = 2;
///     SPIDER = 3;
///     BEAR = 4;
///     BOAR = 5;
///     CROCOLISK = 6;
///     CARRION_BIRD = 7;
///     CRAB = 8;
///     GORILLA = 9;
///     RAPTOR = 11;
///     TALLSTRIDER = 12;
///     FELHUNTER = 15;
///     VOIDWALKER = 16;
///     SUCCUBUS = 17;
///     DOOMGUARD = 19;
///     SCORPID = 20;
///     TURTLE = 21;
///     IMP = 23;
///     BAT = 24;
///     HYENA = 25;
///     OWL = 26;
///     WIND_SERPENT = 27;
///     REMOTE_CONTROL = 28;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum CreatureFamily {
    None,
    Wolf,
    Cat,
    Spider,
    Bear,
    Boar,
    Crocolisk,
    CarrionBird,
    Crab,
    Gorilla,
    Raptor,
    Tallstrider,
    Felhunter,
    Voidwalker,
    Succubus,
    Doomguard,
    Scorpid,
    Turtle,
    Imp,
    Bat,
    Hyena,
    Owl,
    WindSerpent,
    RemoteControl,
}

impl CreatureFamily {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Wolf => 0x1,
            Self::Cat => 0x2,
            Self::Spider => 0x3,
            Self::Bear => 0x4,
            Self::Boar => 0x5,
            Self::Crocolisk => 0x6,
            Self::CarrionBird => 0x7,
            Self::Crab => 0x8,
            Self::Gorilla => 0x9,
            Self::Raptor => 0xb,
            Self::Tallstrider => 0xc,
            Self::Felhunter => 0xf,
            Self::Voidwalker => 0x10,
            Self::Succubus => 0x11,
            Self::Doomguard => 0x13,
            Self::Scorpid => 0x14,
            Self::Turtle => 0x15,
            Self::Imp => 0x17,
            Self::Bat => 0x18,
            Self::Hyena => 0x19,
            Self::Owl => 0x1a,
            Self::WindSerpent => 0x1b,
            Self::RemoteControl => 0x1c,
        }
    }

}

impl Default for CreatureFamily {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for CreatureFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Wolf => f.write_str("Wolf"),
            Self::Cat => f.write_str("Cat"),
            Self::Spider => f.write_str("Spider"),
            Self::Bear => f.write_str("Bear"),
            Self::Boar => f.write_str("Boar"),
            Self::Crocolisk => f.write_str("Crocolisk"),
            Self::CarrionBird => f.write_str("CarrionBird"),
            Self::Crab => f.write_str("Crab"),
            Self::Gorilla => f.write_str("Gorilla"),
            Self::Raptor => f.write_str("Raptor"),
            Self::Tallstrider => f.write_str("Tallstrider"),
            Self::Felhunter => f.write_str("Felhunter"),
            Self::Voidwalker => f.write_str("Voidwalker"),
            Self::Succubus => f.write_str("Succubus"),
            Self::Doomguard => f.write_str("Doomguard"),
            Self::Scorpid => f.write_str("Scorpid"),
            Self::Turtle => f.write_str("Turtle"),
            Self::Imp => f.write_str("Imp"),
            Self::Bat => f.write_str("Bat"),
            Self::Hyena => f.write_str("Hyena"),
            Self::Owl => f.write_str("Owl"),
            Self::WindSerpent => f.write_str("WindSerpent"),
            Self::RemoteControl => f.write_str("RemoteControl"),
        }
    }
}

impl TryFrom<u8> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Wolf),
            2 => Ok(Self::Cat),
            3 => Ok(Self::Spider),
            4 => Ok(Self::Bear),
            5 => Ok(Self::Boar),
            6 => Ok(Self::Crocolisk),
            7 => Ok(Self::CarrionBird),
            8 => Ok(Self::Crab),
            9 => Ok(Self::Gorilla),
            11 => Ok(Self::Raptor),
            12 => Ok(Self::Tallstrider),
            15 => Ok(Self::Felhunter),
            16 => Ok(Self::Voidwalker),
            17 => Ok(Self::Succubus),
            19 => Ok(Self::Doomguard),
            20 => Ok(Self::Scorpid),
            21 => Ok(Self::Turtle),
            23 => Ok(Self::Imp),
            24 => Ok(Self::Bat),
            25 => Ok(Self::Hyena),
            26 => Ok(Self::Owl),
            27 => Ok(Self::WindSerpent),
            28 => Ok(Self::RemoteControl),
            v => Err(crate::errors::EnumError::new("CreatureFamily", v as u32),)
        }
    }
}

