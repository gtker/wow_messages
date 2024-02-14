/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:51`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L51):
/// ```text
/// enum BattlegroundType : u32 {
///     NONE = 0;
///     ALTERAC_VALLEY = 1;
///     WARSONG_GULCH = 2;
///     ARATHI_BASIN = 3;
///     NAGRAND_ARENA = 4;
///     BLADES_EDGE_ARENA = 5;
///     ARENA = 6;
///     EYE_OF_THE_STORM = 7;
///     RUINS_OF_LORDAERON = 8;
///     STRAND_OF_THE_ANCIENT = 9;
///     DALARAN_SEWERS = 10;
///     RING_OF_VALOR = 11;
///     ISLE_OF_CONQUEST = 30;
///     RANDOM = 32;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BattlegroundType {
    None,
    AlteracValley,
    WarsongGulch,
    ArathiBasin,
    NagrandArena,
    BladesEdgeArena,
    Arena,
    EyeOfTheStorm,
    RuinsOfLordaeron,
    StrandOfTheAncient,
    DalaranSewers,
    RingOfValor,
    IsleOfConquest,
    Random,
}

impl BattlegroundType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0x0,
            Self::AlteracValley => 0x1,
            Self::WarsongGulch => 0x2,
            Self::ArathiBasin => 0x3,
            Self::NagrandArena => 0x4,
            Self::BladesEdgeArena => 0x5,
            Self::Arena => 0x6,
            Self::EyeOfTheStorm => 0x7,
            Self::RuinsOfLordaeron => 0x8,
            Self::StrandOfTheAncient => 0x9,
            Self::DalaranSewers => 0xa,
            Self::RingOfValor => 0xb,
            Self::IsleOfConquest => 0x1e,
            Self::Random => 0x20,
        }
    }

    pub const fn variants() -> [Self; 14] {
        [
            Self::None,
            Self::AlteracValley,
            Self::WarsongGulch,
            Self::ArathiBasin,
            Self::NagrandArena,
            Self::BladesEdgeArena,
            Self::Arena,
            Self::EyeOfTheStorm,
            Self::RuinsOfLordaeron,
            Self::StrandOfTheAncient,
            Self::DalaranSewers,
            Self::RingOfValor,
            Self::IsleOfConquest,
            Self::Random,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::AlteracValley),
            2 => Ok(Self::WarsongGulch),
            3 => Ok(Self::ArathiBasin),
            4 => Ok(Self::NagrandArena),
            5 => Ok(Self::BladesEdgeArena),
            6 => Ok(Self::Arena),
            7 => Ok(Self::EyeOfTheStorm),
            8 => Ok(Self::RuinsOfLordaeron),
            9 => Ok(Self::StrandOfTheAncient),
            10 => Ok(Self::DalaranSewers),
            11 => Ok(Self::RingOfValor),
            30 => Ok(Self::IsleOfConquest),
            32 => Ok(Self::Random),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl BattlegroundType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::AlteracValley => "ALTERAC_VALLEY",
            Self::WarsongGulch => "WARSONG_GULCH",
            Self::ArathiBasin => "ARATHI_BASIN",
            Self::NagrandArena => "NAGRAND_ARENA",
            Self::BladesEdgeArena => "BLADES_EDGE_ARENA",
            Self::Arena => "ARENA",
            Self::EyeOfTheStorm => "EYE_OF_THE_STORM",
            Self::RuinsOfLordaeron => "RUINS_OF_LORDAERON",
            Self::StrandOfTheAncient => "STRAND_OF_THE_ANCIENT",
            Self::DalaranSewers => "DALARAN_SEWERS",
            Self::RingOfValor => "RING_OF_VALOR",
            Self::IsleOfConquest => "ISLE_OF_CONQUEST",
            Self::Random => "RANDOM",
        }
    }

}

const NAME: &str = "BattlegroundType";

impl Default for BattlegroundType {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for BattlegroundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::AlteracValley => f.write_str("AlteracValley"),
            Self::WarsongGulch => f.write_str("WarsongGulch"),
            Self::ArathiBasin => f.write_str("ArathiBasin"),
            Self::NagrandArena => f.write_str("NagrandArena"),
            Self::BladesEdgeArena => f.write_str("BladesEdgeArena"),
            Self::Arena => f.write_str("Arena"),
            Self::EyeOfTheStorm => f.write_str("EyeOfTheStorm"),
            Self::RuinsOfLordaeron => f.write_str("RuinsOfLordaeron"),
            Self::StrandOfTheAncient => f.write_str("StrandOfTheAncient"),
            Self::DalaranSewers => f.write_str("DalaranSewers"),
            Self::RingOfValor => f.write_str("RingOfValor"),
            Self::IsleOfConquest => f.write_str("IsleOfConquest"),
            Self::Random => f.write_str("Random"),
        }
    }
}

impl TryFrom<u32> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u64> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i64> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

