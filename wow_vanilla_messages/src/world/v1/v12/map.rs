use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Map {
    EASTERN_KINGDOMS,
    KALIMDOR,
    TESTING,
    SCOTT_TEST,
    CASH_TEST,
    ALTERAC_VALLEY,
    SHADOWFANG_KEEP,
    STORMWIND_STOCKADE,
    STORMWIND_PRISON,
    DEADMINES,
    AZSHARA_CRATER,
    COLLINS_TEST,
    WAILING_CAVERNS,
    MONASTERY,
    RAZORFEN_KRAUL,
    BLACKFATHOM_DEEPS,
    ULDAMAN,
    GNOMERAGON,
    SUNKEN_TEMPLE,
    RAZORFEN_DOWNS,
    EMERALD_DREAM,
    SCARLET_MONASTERY,
    ZUL_FARRAK,
    BLACKROCK_SPIRE,
    BLACKROCK_DEPTHS,
    ONYXIAS_LAIR,
    CAVERNS_OF_TIME,
    SCHOLOMANCE,
    ZUL_GURUB,
    STRATHOLME,
    MAURADON,
    DEEPRUN_TRAM,
    RAGEFIRE_CHASM,
    MOLTEN_CORE,
    DIRE_MAUL,
    ALLIANCE_PVP_BARRACKS,
    HORDE_PVP_BARRACKS,
    DEVELOPMENT_LAND,
    BLACKWING_LAIR,
    WARSONG_GULCH,
    RUINS_OF_AHN_QIRAJ,
    ARATHI_BASIN,
    AHN_QIRAJ_TEMPLE,
    NAXXRAMAS,
}

impl ReadableAndWritable for Map {
    type Error = MapError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl Map {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MapError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MapError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MapError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::EASTERN_KINGDOMS => 0x0,
            Self::KALIMDOR => 0x1,
            Self::TESTING => 0xd,
            Self::SCOTT_TEST => 0x19,
            Self::CASH_TEST => 0x1d,
            Self::ALTERAC_VALLEY => 0x1e,
            Self::SHADOWFANG_KEEP => 0x21,
            Self::STORMWIND_STOCKADE => 0x22,
            Self::STORMWIND_PRISON => 0x23,
            Self::DEADMINES => 0x24,
            Self::AZSHARA_CRATER => 0x25,
            Self::COLLINS_TEST => 0x2a,
            Self::WAILING_CAVERNS => 0x2b,
            Self::MONASTERY => 0x2c,
            Self::RAZORFEN_KRAUL => 0x2f,
            Self::BLACKFATHOM_DEEPS => 0x30,
            Self::ULDAMAN => 0x46,
            Self::GNOMERAGON => 0x5a,
            Self::SUNKEN_TEMPLE => 0x6d,
            Self::RAZORFEN_DOWNS => 0x81,
            Self::EMERALD_DREAM => 0xa9,
            Self::SCARLET_MONASTERY => 0xbd,
            Self::ZUL_FARRAK => 0xd1,
            Self::BLACKROCK_SPIRE => 0xe5,
            Self::BLACKROCK_DEPTHS => 0xe6,
            Self::ONYXIAS_LAIR => 0xf9,
            Self::CAVERNS_OF_TIME => 0x10d,
            Self::SCHOLOMANCE => 0x121,
            Self::ZUL_GURUB => 0x135,
            Self::STRATHOLME => 0x149,
            Self::MAURADON => 0x15d,
            Self::DEEPRUN_TRAM => 0x171,
            Self::RAGEFIRE_CHASM => 0x185,
            Self::MOLTEN_CORE => 0x199,
            Self::DIRE_MAUL => 0x1ad,
            Self::ALLIANCE_PVP_BARRACKS => 0x1c1,
            Self::HORDE_PVP_BARRACKS => 0x1c2,
            Self::DEVELOPMENT_LAND => 0x1c3,
            Self::BLACKWING_LAIR => 0x1d5,
            Self::WARSONG_GULCH => 0x1e9,
            Self::RUINS_OF_AHN_QIRAJ => 0x1fd,
            Self::ARATHI_BASIN => 0x211,
            Self::AHN_QIRAJ_TEMPLE => 0x213,
            Self::NAXXRAMAS => 0x215,
        }
    }

    pub const fn new() -> Self {
        Self::EASTERN_KINGDOMS
    }

}

impl ConstantSized for Map {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for Map {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::EASTERN_KINGDOMS
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EASTERN_KINGDOMS => f.write_str("Eastern Kingdoms"),
            Self::KALIMDOR => f.write_str("Kalimdor"),
            Self::TESTING => f.write_str("Testing"),
            Self::SCOTT_TEST => f.write_str("Scott Test"),
            Self::CASH_TEST => f.write_str("CashTest"),
            Self::ALTERAC_VALLEY => f.write_str("Alterac Valley"),
            Self::SHADOWFANG_KEEP => f.write_str("Shadowfang Keep"),
            Self::STORMWIND_STOCKADE => f.write_str("Stormwind Stockade"),
            Self::STORMWIND_PRISON => f.write_str("Stormwind Prison"),
            Self::DEADMINES => f.write_str("Deadmines"),
            Self::AZSHARA_CRATER => f.write_str("Azshara Crater"),
            Self::COLLINS_TEST => f.write_str("Collin's Test"),
            Self::WAILING_CAVERNS => f.write_str("Wailing Caverns"),
            Self::MONASTERY => f.write_str("Monastery"),
            Self::RAZORFEN_KRAUL => f.write_str("Razorfen Kraul"),
            Self::BLACKFATHOM_DEEPS => f.write_str("Blackfathom Deeps"),
            Self::ULDAMAN => f.write_str("Uldaman"),
            Self::GNOMERAGON => f.write_str("Gnomeragon"),
            Self::SUNKEN_TEMPLE => f.write_str("SunkenTemple"),
            Self::RAZORFEN_DOWNS => f.write_str("Razorfen Downs"),
            Self::EMERALD_DREAM => f.write_str("Emerald Dream"),
            Self::SCARLET_MONASTERY => f.write_str("Scarlet Monastery"),
            Self::ZUL_FARRAK => f.write_str("Zul'Farrak"),
            Self::BLACKROCK_SPIRE => f.write_str("Blackrock Spire"),
            Self::BLACKROCK_DEPTHS => f.write_str("Blackrock Depths"),
            Self::ONYXIAS_LAIR => f.write_str("Onyxia's Lair"),
            Self::CAVERNS_OF_TIME => f.write_str("Caverns of Time"),
            Self::SCHOLOMANCE => f.write_str("Scholomance"),
            Self::ZUL_GURUB => f.write_str("Zul'Gurub"),
            Self::STRATHOLME => f.write_str("Stratholme"),
            Self::MAURADON => f.write_str("Mauradon"),
            Self::DEEPRUN_TRAM => f.write_str("Deeprun Tram"),
            Self::RAGEFIRE_CHASM => f.write_str("Ragefire Chasm"),
            Self::MOLTEN_CORE => f.write_str("Molten Core"),
            Self::DIRE_MAUL => f.write_str("Dire Maul"),
            Self::ALLIANCE_PVP_BARRACKS => f.write_str("Alliance PVP Barracks"),
            Self::HORDE_PVP_BARRACKS => f.write_str("Horde PVP Barracks"),
            Self::DEVELOPMENT_LAND => f.write_str("Development Land"),
            Self::BLACKWING_LAIR => f.write_str("Blackwing Lair"),
            Self::WARSONG_GULCH => f.write_str("Warsong Gulch"),
            Self::RUINS_OF_AHN_QIRAJ => f.write_str("Ruins of Ahn'Qiraj"),
            Self::ARATHI_BASIN => f.write_str("Arathi Basin"),
            Self::AHN_QIRAJ_TEMPLE => f.write_str("Ahn'Qiraj Temple"),
            Self::NAXXRAMAS => f.write_str("Naxxramas"),
        }
    }
}

impl TryFrom<u32> for Map {
    type Error = TryFromMapError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EASTERN_KINGDOMS),
            1 => Ok(Self::KALIMDOR),
            13 => Ok(Self::TESTING),
            25 => Ok(Self::SCOTT_TEST),
            29 => Ok(Self::CASH_TEST),
            30 => Ok(Self::ALTERAC_VALLEY),
            33 => Ok(Self::SHADOWFANG_KEEP),
            34 => Ok(Self::STORMWIND_STOCKADE),
            35 => Ok(Self::STORMWIND_PRISON),
            36 => Ok(Self::DEADMINES),
            37 => Ok(Self::AZSHARA_CRATER),
            42 => Ok(Self::COLLINS_TEST),
            43 => Ok(Self::WAILING_CAVERNS),
            44 => Ok(Self::MONASTERY),
            47 => Ok(Self::RAZORFEN_KRAUL),
            48 => Ok(Self::BLACKFATHOM_DEEPS),
            70 => Ok(Self::ULDAMAN),
            90 => Ok(Self::GNOMERAGON),
            109 => Ok(Self::SUNKEN_TEMPLE),
            129 => Ok(Self::RAZORFEN_DOWNS),
            169 => Ok(Self::EMERALD_DREAM),
            189 => Ok(Self::SCARLET_MONASTERY),
            209 => Ok(Self::ZUL_FARRAK),
            229 => Ok(Self::BLACKROCK_SPIRE),
            230 => Ok(Self::BLACKROCK_DEPTHS),
            249 => Ok(Self::ONYXIAS_LAIR),
            269 => Ok(Self::CAVERNS_OF_TIME),
            289 => Ok(Self::SCHOLOMANCE),
            309 => Ok(Self::ZUL_GURUB),
            329 => Ok(Self::STRATHOLME),
            349 => Ok(Self::MAURADON),
            369 => Ok(Self::DEEPRUN_TRAM),
            389 => Ok(Self::RAGEFIRE_CHASM),
            409 => Ok(Self::MOLTEN_CORE),
            429 => Ok(Self::DIRE_MAUL),
            449 => Ok(Self::ALLIANCE_PVP_BARRACKS),
            450 => Ok(Self::HORDE_PVP_BARRACKS),
            451 => Ok(Self::DEVELOPMENT_LAND),
            469 => Ok(Self::BLACKWING_LAIR),
            489 => Ok(Self::WARSONG_GULCH),
            509 => Ok(Self::RUINS_OF_AHN_QIRAJ),
            529 => Ok(Self::ARATHI_BASIN),
            531 => Ok(Self::AHN_QIRAJ_TEMPLE),
            533 => Ok(Self::NAXXRAMAS),
            _ => Err(TryFromMapError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromMapError {
    value: u32,
}

impl TryFromMapError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum MapError {
    Read(std::io::Error),
    TryFrom(TryFromMapError),
}

impl std::error::Error for MapError {}
impl std::fmt::Display for TryFromMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'Map': '{}'", self.value))
    }
}

impl std::fmt::Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for MapError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromMapError> for MapError {
    fn from(value: TryFromMapError) -> Self {
        Self::TryFrom(value)
    }
}

