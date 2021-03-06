use std::convert::{TryFrom, TryInto};

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
pub enum PvpRank {
    NO_RANK,
    PARIAH,
    OUTLAW,
    EXILED,
    DISHONORED,
    RANK1,
    RANK2,
    RANK3,
    RANK4,
    RANK5,
    RANK6,
    RANK7,
    RANK8,
    RANK9,
    RANK10,
    RANK11,
    RANK12,
    RANK13,
    RANK14,
    FACTION_LEADER,
}

impl PvpRank {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NO_RANK => 0x0,
            Self::PARIAH => 0x1,
            Self::OUTLAW => 0x2,
            Self::EXILED => 0x3,
            Self::DISHONORED => 0x4,
            Self::RANK1 => 0x5,
            Self::RANK2 => 0x6,
            Self::RANK3 => 0x7,
            Self::RANK4 => 0x8,
            Self::RANK5 => 0x9,
            Self::RANK6 => 0xa,
            Self::RANK7 => 0xb,
            Self::RANK8 => 0xc,
            Self::RANK9 => 0xd,
            Self::RANK10 => 0xe,
            Self::RANK11 => 0xf,
            Self::RANK12 => 0x10,
            Self::RANK13 => 0x11,
            Self::RANK14 => 0x12,
            Self::FACTION_LEADER => 0x13,
        }
    }

}

impl Default for PvpRank {
    fn default() -> Self {
        Self::NO_RANK
    }
}

impl std::fmt::Display for PvpRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NO_RANK => f.write_str("NO_RANK"),
            Self::PARIAH => f.write_str("PARIAH"),
            Self::OUTLAW => f.write_str("OUTLAW"),
            Self::EXILED => f.write_str("EXILED"),
            Self::DISHONORED => f.write_str("DISHONORED"),
            Self::RANK1 => f.write_str("RANK1"),
            Self::RANK2 => f.write_str("RANK2"),
            Self::RANK3 => f.write_str("RANK3"),
            Self::RANK4 => f.write_str("RANK4"),
            Self::RANK5 => f.write_str("RANK5"),
            Self::RANK6 => f.write_str("RANK6"),
            Self::RANK7 => f.write_str("RANK7"),
            Self::RANK8 => f.write_str("RANK8"),
            Self::RANK9 => f.write_str("RANK9"),
            Self::RANK10 => f.write_str("RANK10"),
            Self::RANK11 => f.write_str("RANK11"),
            Self::RANK12 => f.write_str("RANK12"),
            Self::RANK13 => f.write_str("RANK13"),
            Self::RANK14 => f.write_str("RANK14"),
            Self::FACTION_LEADER => f.write_str("FACTION_LEADER"),
        }
    }
}

impl TryFrom<u8> for PvpRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NO_RANK),
            1 => Ok(Self::PARIAH),
            2 => Ok(Self::OUTLAW),
            3 => Ok(Self::EXILED),
            4 => Ok(Self::DISHONORED),
            5 => Ok(Self::RANK1),
            6 => Ok(Self::RANK2),
            7 => Ok(Self::RANK3),
            8 => Ok(Self::RANK4),
            9 => Ok(Self::RANK5),
            10 => Ok(Self::RANK6),
            11 => Ok(Self::RANK7),
            12 => Ok(Self::RANK8),
            13 => Ok(Self::RANK9),
            14 => Ok(Self::RANK10),
            15 => Ok(Self::RANK11),
            16 => Ok(Self::RANK12),
            17 => Ok(Self::RANK13),
            18 => Ok(Self::RANK14),
            19 => Ok(Self::FACTION_LEADER),
            v => Err(crate::errors::EnumError::new("PvpRank", v as u32),)
        }
    }
}

