/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/pet_common.wowm:48`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/pet_common.wowm#L48):
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
///     FELGUARD = 29;
///     DRAGONHAWK = 30;
///     RAVAGER = 31;
///     WARP_STALKER = 32;
///     SPOREBAT = 33;
///     NETHER_RAY = 34;
///     SERPENT = 35;
///     SEA_LION = 36;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
    Felguard,
    Dragonhawk,
    Ravager,
    WarpStalker,
    Sporebat,
    NetherRay,
    Serpent,
    SeaLion,
}

impl CreatureFamily {
    pub const fn as_int(&self) -> u8 {
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
            Self::Felguard => 0x1d,
            Self::Dragonhawk => 0x1e,
            Self::Ravager => 0x1f,
            Self::WarpStalker => 0x20,
            Self::Sporebat => 0x21,
            Self::NetherRay => 0x22,
            Self::Serpent => 0x23,
            Self::SeaLion => 0x24,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl CreatureFamily {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Wolf => "WOLF",
            Self::Cat => "CAT",
            Self::Spider => "SPIDER",
            Self::Bear => "BEAR",
            Self::Boar => "BOAR",
            Self::Crocolisk => "CROCOLISK",
            Self::CarrionBird => "CARRION_BIRD",
            Self::Crab => "CRAB",
            Self::Gorilla => "GORILLA",
            Self::Raptor => "RAPTOR",
            Self::Tallstrider => "TALLSTRIDER",
            Self::Felhunter => "FELHUNTER",
            Self::Voidwalker => "VOIDWALKER",
            Self::Succubus => "SUCCUBUS",
            Self::Doomguard => "DOOMGUARD",
            Self::Scorpid => "SCORPID",
            Self::Turtle => "TURTLE",
            Self::Imp => "IMP",
            Self::Bat => "BAT",
            Self::Hyena => "HYENA",
            Self::Owl => "OWL",
            Self::WindSerpent => "WIND_SERPENT",
            Self::RemoteControl => "REMOTE_CONTROL",
            Self::Felguard => "FELGUARD",
            Self::Dragonhawk => "DRAGONHAWK",
            Self::Ravager => "RAVAGER",
            Self::WarpStalker => "WARP_STALKER",
            Self::Sporebat => "SPOREBAT",
            Self::NetherRay => "NETHER_RAY",
            Self::Serpent => "SERPENT",
            Self::SeaLion => "SEA_LION",
        }
    }

}

const NAME: &str = "CreatureFamily";

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
            Self::Felguard => f.write_str("Felguard"),
            Self::Dragonhawk => f.write_str("Dragonhawk"),
            Self::Ravager => f.write_str("Ravager"),
            Self::WarpStalker => f.write_str("WarpStalker"),
            Self::Sporebat => f.write_str("Sporebat"),
            Self::NetherRay => f.write_str("NetherRay"),
            Self::Serpent => f.write_str("Serpent"),
            Self::SeaLion => f.write_str("SeaLion"),
        }
    }
}

impl TryFrom<u8> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
            29 => Ok(Self::Felguard),
            30 => Ok(Self::Dragonhawk),
            31 => Ok(Self::Ravager),
            32 => Ok(Self::WarpStalker),
            33 => Ok(Self::Sporebat),
            34 => Ok(Self::NetherRay),
            35 => Ok(Self::Serpent),
            36 => Ok(Self::SeaLion),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for CreatureFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

