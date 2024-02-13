/// Used in `SoundEntries.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/sound_type.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/sound_type.wowm#L2):
/// ```text
/// enum SoundType : u8 {
///     UNUSED = 0x00;
///     SPELLS = 0x01;
///     UI = 0x02;
///     FOOTSTEPS = 0x03;
///     WEAPON_IMPACT = 0x04;
///     WEAPON_MISS = 0x06;
///     PICK_UP_PUT_DOWN = 0x09;
///     NPC_COMBAT = 0x0A;
///     ERRORS = 0x0C;
///     OBJECTS = 0x0E;
///     DEATH = 0x10;
///     NPC_GREETINGS = 0x11;
///     TEST = 0x12;
///     ARMOUR_FOLEY = 0x13;
///     FOOTSTEPS_2 = 0x14;
///     WATER_CHARACTER = 0x15;
///     WATER_LIQUID = 0x16;
///     TRADESKILLS = 0x17;
///     DOODADS = 0x19;
///     SPELL_FIZZLE = 0x1A;
///     NPC_LOOPS = 0x1B;
///     ZONE_MUSIC = 0x1C;
///     EMOTES = 0x1D;
///     NARRATION_MUSIC = 0x1E;
///     NARRATION = 0x1F;
///     ZONE_AMBIENCE = 0x32;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SoundType {
    Unused,
    Spells,
    Ui,
    Footsteps,
    WeaponImpact,
    WeaponMiss,
    PickUpPutDown,
    NpcCombat,
    Errors,
    Objects,
    Death,
    NpcGreetings,
    Test,
    ArmourFoley,
    Footsteps2,
    WaterCharacter,
    WaterLiquid,
    Tradeskills,
    Doodads,
    SpellFizzle,
    NpcLoops,
    ZoneMusic,
    Emotes,
    NarrationMusic,
    Narration,
    ZoneAmbience,
}

impl SoundType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Unused => 0x0,
            Self::Spells => 0x1,
            Self::Ui => 0x2,
            Self::Footsteps => 0x3,
            Self::WeaponImpact => 0x4,
            Self::WeaponMiss => 0x6,
            Self::PickUpPutDown => 0x9,
            Self::NpcCombat => 0xa,
            Self::Errors => 0xc,
            Self::Objects => 0xe,
            Self::Death => 0x10,
            Self::NpcGreetings => 0x11,
            Self::Test => 0x12,
            Self::ArmourFoley => 0x13,
            Self::Footsteps2 => 0x14,
            Self::WaterCharacter => 0x15,
            Self::WaterLiquid => 0x16,
            Self::Tradeskills => 0x17,
            Self::Doodads => 0x19,
            Self::SpellFizzle => 0x1a,
            Self::NpcLoops => 0x1b,
            Self::ZoneMusic => 0x1c,
            Self::Emotes => 0x1d,
            Self::NarrationMusic => 0x1e,
            Self::Narration => 0x1f,
            Self::ZoneAmbience => 0x32,
        }
    }

    pub const fn variants() -> [Self; 26] {
        [
            Self::Unused,
            Self::Spells,
            Self::Ui,
            Self::Footsteps,
            Self::WeaponImpact,
            Self::WeaponMiss,
            Self::PickUpPutDown,
            Self::NpcCombat,
            Self::Errors,
            Self::Objects,
            Self::Death,
            Self::NpcGreetings,
            Self::Test,
            Self::ArmourFoley,
            Self::Footsteps2,
            Self::WaterCharacter,
            Self::WaterLiquid,
            Self::Tradeskills,
            Self::Doodads,
            Self::SpellFizzle,
            Self::NpcLoops,
            Self::ZoneMusic,
            Self::Emotes,
            Self::NarrationMusic,
            Self::Narration,
            Self::ZoneAmbience,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Unused),
            1 => Ok(Self::Spells),
            2 => Ok(Self::Ui),
            3 => Ok(Self::Footsteps),
            4 => Ok(Self::WeaponImpact),
            6 => Ok(Self::WeaponMiss),
            9 => Ok(Self::PickUpPutDown),
            10 => Ok(Self::NpcCombat),
            12 => Ok(Self::Errors),
            14 => Ok(Self::Objects),
            16 => Ok(Self::Death),
            17 => Ok(Self::NpcGreetings),
            18 => Ok(Self::Test),
            19 => Ok(Self::ArmourFoley),
            20 => Ok(Self::Footsteps2),
            21 => Ok(Self::WaterCharacter),
            22 => Ok(Self::WaterLiquid),
            23 => Ok(Self::Tradeskills),
            25 => Ok(Self::Doodads),
            26 => Ok(Self::SpellFizzle),
            27 => Ok(Self::NpcLoops),
            28 => Ok(Self::ZoneMusic),
            29 => Ok(Self::Emotes),
            30 => Ok(Self::NarrationMusic),
            31 => Ok(Self::Narration),
            50 => Ok(Self::ZoneAmbience),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl SoundType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Unused => "UNUSED",
            Self::Spells => "SPELLS",
            Self::Ui => "UI",
            Self::Footsteps => "FOOTSTEPS",
            Self::WeaponImpact => "WEAPON_IMPACT",
            Self::WeaponMiss => "WEAPON_MISS",
            Self::PickUpPutDown => "PICK_UP_PUT_DOWN",
            Self::NpcCombat => "NPC_COMBAT",
            Self::Errors => "ERRORS",
            Self::Objects => "OBJECTS",
            Self::Death => "DEATH",
            Self::NpcGreetings => "NPC_GREETINGS",
            Self::Test => "TEST",
            Self::ArmourFoley => "ARMOUR_FOLEY",
            Self::Footsteps2 => "FOOTSTEPS_2",
            Self::WaterCharacter => "WATER_CHARACTER",
            Self::WaterLiquid => "WATER_LIQUID",
            Self::Tradeskills => "TRADESKILLS",
            Self::Doodads => "DOODADS",
            Self::SpellFizzle => "SPELL_FIZZLE",
            Self::NpcLoops => "NPC_LOOPS",
            Self::ZoneMusic => "ZONE_MUSIC",
            Self::Emotes => "EMOTES",
            Self::NarrationMusic => "NARRATION_MUSIC",
            Self::Narration => "NARRATION",
            Self::ZoneAmbience => "ZONE_AMBIENCE",
        }
    }

}

const NAME: &str = "SoundType";

impl Default for SoundType {
    fn default() -> Self {
        Self::Unused
    }
}

impl std::fmt::Display for SoundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unused => f.write_str("Unused"),
            Self::Spells => f.write_str("Spells"),
            Self::Ui => f.write_str("Ui"),
            Self::Footsteps => f.write_str("Footsteps"),
            Self::WeaponImpact => f.write_str("WeaponImpact"),
            Self::WeaponMiss => f.write_str("WeaponMiss"),
            Self::PickUpPutDown => f.write_str("PickUpPutDown"),
            Self::NpcCombat => f.write_str("NpcCombat"),
            Self::Errors => f.write_str("Errors"),
            Self::Objects => f.write_str("Objects"),
            Self::Death => f.write_str("Death"),
            Self::NpcGreetings => f.write_str("NpcGreetings"),
            Self::Test => f.write_str("Test"),
            Self::ArmourFoley => f.write_str("ArmourFoley"),
            Self::Footsteps2 => f.write_str("Footsteps2"),
            Self::WaterCharacter => f.write_str("WaterCharacter"),
            Self::WaterLiquid => f.write_str("WaterLiquid"),
            Self::Tradeskills => f.write_str("Tradeskills"),
            Self::Doodads => f.write_str("Doodads"),
            Self::SpellFizzle => f.write_str("SpellFizzle"),
            Self::NpcLoops => f.write_str("NpcLoops"),
            Self::ZoneMusic => f.write_str("ZoneMusic"),
            Self::Emotes => f.write_str("Emotes"),
            Self::NarrationMusic => f.write_str("NarrationMusic"),
            Self::Narration => f.write_str("Narration"),
            Self::ZoneAmbience => f.write_str("ZoneAmbience"),
        }
    }
}

impl TryFrom<u8> for SoundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for SoundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SoundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SoundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SoundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for SoundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SoundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SoundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SoundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

