/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/pvp_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/pvp_common.wowm#L3):
/// ```text
/// enum PvpRank : u8 {
///     NO_RANK = 0;
///     PARIAH = 1;
///     OUTLAW = 2;
///     EXILED = 3;
///     DISHONORED = 4;
///     RANK1 = 5;
///     RANK2 = 6;
///     RANK3 = 7;
///     RANK4 = 8;
///     RANK5 = 9;
///     RANK6 = 10;
///     RANK7 = 11;
///     RANK8 = 12;
///     RANK9 = 13;
///     RANK10 = 14;
///     RANK11 = 15;
///     RANK12 = 16;
///     RANK13 = 17;
///     RANK14 = 18;
///     FACTION_LEADER = 19;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PvpRank {
    NoRank,
    Pariah,
    Outlaw,
    Exiled,
    Dishonored,
    /// Alliance name: Private
    /// Horde name: Scout
    Rank1,
    /// Alliance name: Corporal
    /// Horde name: Grunt
    Rank2,
    /// Alliance name: Sergeant
    /// Horde name: Sergeant
    Rank3,
    /// Alliance name: Master Sergeant
    /// Horde name: Senior Sergeatn
    Rank4,
    /// Alliance name: Sergeant Major
    /// Horde name: First Sergeant
    Rank5,
    /// Alliance name: Knight
    /// Horde name: Stone Guard
    Rank6,
    /// Alliance name: Knight Lieutenant
    /// Horde name: Blood Guard
    Rank7,
    /// Alliance name: Knight Captain
    /// Horde name: Legionnare
    Rank8,
    /// Alliance name: Kngith Champion
    /// Horde name: Centurion
    Rank9,
    /// Alliance name: Liuetenant Commander
    /// Horde name: Champion
    Rank10,
    /// Alliance name: Commander
    /// Horde name: Lieutenant General
    Rank11,
    /// Alliance name: Marshal
    /// Horde name: General
    Rank12,
    /// Alliance name: Field Marshal
    /// Horde name: Warlord
    Rank13,
    /// Alliance name: Grand Marshal
    /// Horde name: High Warlord
    Rank14,
    FactionLeader,
}

impl PvpRank {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NoRank => 0x0,
            Self::Pariah => 0x1,
            Self::Outlaw => 0x2,
            Self::Exiled => 0x3,
            Self::Dishonored => 0x4,
            Self::Rank1 => 0x5,
            Self::Rank2 => 0x6,
            Self::Rank3 => 0x7,
            Self::Rank4 => 0x8,
            Self::Rank5 => 0x9,
            Self::Rank6 => 0xa,
            Self::Rank7 => 0xb,
            Self::Rank8 => 0xc,
            Self::Rank9 => 0xd,
            Self::Rank10 => 0xe,
            Self::Rank11 => 0xf,
            Self::Rank12 => 0x10,
            Self::Rank13 => 0x11,
            Self::Rank14 => 0x12,
            Self::FactionLeader => 0x13,
        }
    }

    pub const fn variants() -> [Self; 20] {
        [
            Self::NoRank,
            Self::Pariah,
            Self::Outlaw,
            Self::Exiled,
            Self::Dishonored,
            Self::Rank1,
            Self::Rank2,
            Self::Rank3,
            Self::Rank4,
            Self::Rank5,
            Self::Rank6,
            Self::Rank7,
            Self::Rank8,
            Self::Rank9,
            Self::Rank10,
            Self::Rank11,
            Self::Rank12,
            Self::Rank13,
            Self::Rank14,
            Self::FactionLeader,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::NoRank),
            1 => Ok(Self::Pariah),
            2 => Ok(Self::Outlaw),
            3 => Ok(Self::Exiled),
            4 => Ok(Self::Dishonored),
            5 => Ok(Self::Rank1),
            6 => Ok(Self::Rank2),
            7 => Ok(Self::Rank3),
            8 => Ok(Self::Rank4),
            9 => Ok(Self::Rank5),
            10 => Ok(Self::Rank6),
            11 => Ok(Self::Rank7),
            12 => Ok(Self::Rank8),
            13 => Ok(Self::Rank9),
            14 => Ok(Self::Rank10),
            15 => Ok(Self::Rank11),
            16 => Ok(Self::Rank12),
            17 => Ok(Self::Rank13),
            18 => Ok(Self::Rank14),
            19 => Ok(Self::FactionLeader),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl PvpRank {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NoRank => "NO_RANK",
            Self::Pariah => "PARIAH",
            Self::Outlaw => "OUTLAW",
            Self::Exiled => "EXILED",
            Self::Dishonored => "DISHONORED",
            Self::Rank1 => "RANK1",
            Self::Rank2 => "RANK2",
            Self::Rank3 => "RANK3",
            Self::Rank4 => "RANK4",
            Self::Rank5 => "RANK5",
            Self::Rank6 => "RANK6",
            Self::Rank7 => "RANK7",
            Self::Rank8 => "RANK8",
            Self::Rank9 => "RANK9",
            Self::Rank10 => "RANK10",
            Self::Rank11 => "RANK11",
            Self::Rank12 => "RANK12",
            Self::Rank13 => "RANK13",
            Self::Rank14 => "RANK14",
            Self::FactionLeader => "FACTION_LEADER",
        }
    }

}

const NAME: &str = "PvpRank";

impl Default for PvpRank {
    fn default() -> Self {
        Self::NoRank
    }
}

impl std::fmt::Display for PvpRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoRank => f.write_str("NoRank"),
            Self::Pariah => f.write_str("Pariah"),
            Self::Outlaw => f.write_str("Outlaw"),
            Self::Exiled => f.write_str("Exiled"),
            Self::Dishonored => f.write_str("Dishonored"),
            Self::Rank1 => f.write_str("Rank1"),
            Self::Rank2 => f.write_str("Rank2"),
            Self::Rank3 => f.write_str("Rank3"),
            Self::Rank4 => f.write_str("Rank4"),
            Self::Rank5 => f.write_str("Rank5"),
            Self::Rank6 => f.write_str("Rank6"),
            Self::Rank7 => f.write_str("Rank7"),
            Self::Rank8 => f.write_str("Rank8"),
            Self::Rank9 => f.write_str("Rank9"),
            Self::Rank10 => f.write_str("Rank10"),
            Self::Rank11 => f.write_str("Rank11"),
            Self::Rank12 => f.write_str("Rank12"),
            Self::Rank13 => f.write_str("Rank13"),
            Self::Rank14 => f.write_str("Rank14"),
            Self::FactionLeader => f.write_str("FactionLeader"),
        }
    }
}

impl TryFrom<u8> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

