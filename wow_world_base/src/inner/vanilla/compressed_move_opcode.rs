/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm#L1):
/// ```text
/// enum CompressedMoveOpcode : u16 {
///     SMSG_MONSTER_MOVE = 0x00DD;
///     SMSG_MONSTER_MOVE_TRANSPORT = 0x02AE;
///     SMSG_SPLINE_SET_RUN_SPEED = 0x02FE;
///     SMSG_SPLINE_MOVE_UNROOT = 0x0304;
///     SMSG_SPLINE_MOVE_SET_RUN_MODE = 0x030D;
///     SMSG_SPLINE_MOVE_SET_WALK_MODE = 0x030E;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum CompressedMoveOpcode {
    SmsgMonsterMove,
    SmsgMonsterMoveTransport,
    SmsgSplineSetRunSpeed,
    SmsgSplineMoveUnroot,
    SmsgSplineMoveSetRunMode,
    SmsgSplineMoveSetWalkMode,
}

impl CompressedMoveOpcode {
    pub const fn as_int(&self) -> u16 {
        match self {
            Self::SmsgMonsterMove => 0xdd,
            Self::SmsgMonsterMoveTransport => 0x2ae,
            Self::SmsgSplineSetRunSpeed => 0x2fe,
            Self::SmsgSplineMoveUnroot => 0x304,
            Self::SmsgSplineMoveSetRunMode => 0x30d,
            Self::SmsgSplineMoveSetWalkMode => 0x30e,
        }
    }

    pub const fn variants() -> [Self; 6] {
        [
            Self::SmsgMonsterMove,
            Self::SmsgMonsterMoveTransport,
            Self::SmsgSplineSetRunSpeed,
            Self::SmsgSplineMoveUnroot,
            Self::SmsgSplineMoveSetRunMode,
            Self::SmsgSplineMoveSetWalkMode,
        ]
    }

    pub const fn from_int(value: u16) -> Result<Self, crate::errors::EnumError> {
        match value {
            221 => Ok(Self::SmsgMonsterMove),
            686 => Ok(Self::SmsgMonsterMoveTransport),
            766 => Ok(Self::SmsgSplineSetRunSpeed),
            772 => Ok(Self::SmsgSplineMoveUnroot),
            781 => Ok(Self::SmsgSplineMoveSetRunMode),
            782 => Ok(Self::SmsgSplineMoveSetWalkMode),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl CompressedMoveOpcode {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::SmsgMonsterMove => "SMSG_MONSTER_MOVE",
            Self::SmsgMonsterMoveTransport => "SMSG_MONSTER_MOVE_TRANSPORT",
            Self::SmsgSplineSetRunSpeed => "SMSG_SPLINE_SET_RUN_SPEED",
            Self::SmsgSplineMoveUnroot => "SMSG_SPLINE_MOVE_UNROOT",
            Self::SmsgSplineMoveSetRunMode => "SMSG_SPLINE_MOVE_SET_RUN_MODE",
            Self::SmsgSplineMoveSetWalkMode => "SMSG_SPLINE_MOVE_SET_WALK_MODE",
        }
    }

}

const NAME: &str = "CompressedMoveOpcode";

impl Default for CompressedMoveOpcode {
    fn default() -> Self {
        Self::SmsgMonsterMove
    }
}

impl std::fmt::Display for CompressedMoveOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SmsgMonsterMove => f.write_str("SmsgMonsterMove"),
            Self::SmsgMonsterMoveTransport => f.write_str("SmsgMonsterMoveTransport"),
            Self::SmsgSplineSetRunSpeed => f.write_str("SmsgSplineSetRunSpeed"),
            Self::SmsgSplineMoveUnroot => f.write_str("SmsgSplineMoveUnroot"),
            Self::SmsgSplineMoveSetRunMode => f.write_str("SmsgSplineMoveSetRunMode"),
            Self::SmsgSplineMoveSetWalkMode => f.write_str("SmsgSplineMoveSetWalkMode"),
        }
    }
}

impl TryFrom<u16> for CompressedMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for CompressedMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u32> for CompressedMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for CompressedMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for CompressedMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for CompressedMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i32> for CompressedMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for CompressedMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for CompressedMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u16>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

