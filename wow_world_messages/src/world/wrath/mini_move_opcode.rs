use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_multiple_moves.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_multiple_moves.wowm#L1):
/// ```text
/// enum MiniMoveOpcode : u16 {
///     SMSG_FORCE_MOVE_ROOT = 0xE8;
///     SMSG_MOVE_FEATHER_FALL = 0xF2;
///     SMSG_MOVE_WATER_WALK = 0xDE;
///     SMSG_MOVE_SET_HOVER = 0xF4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MiniMoveOpcode {
    SmsgForceMoveRoot,
    SmsgMoveFeatherFall,
    SmsgMoveWaterWalk,
    SmsgMoveSetHover,
}

impl MiniMoveOpcode {
    pub(crate) const fn as_int(&self) -> u16 {
        match self {
            Self::SmsgForceMoveRoot => 0xe8,
            Self::SmsgMoveFeatherFall => 0xf2,
            Self::SmsgMoveWaterWalk => 0xde,
            Self::SmsgMoveSetHover => 0xf4,
        }
    }

}

impl Default for MiniMoveOpcode {
    fn default() -> Self {
        Self::SmsgForceMoveRoot
    }
}

impl std::fmt::Display for MiniMoveOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SmsgForceMoveRoot => f.write_str("SmsgForceMoveRoot"),
            Self::SmsgMoveFeatherFall => f.write_str("SmsgMoveFeatherFall"),
            Self::SmsgMoveWaterWalk => f.write_str("SmsgMoveWaterWalk"),
            Self::SmsgMoveSetHover => f.write_str("SmsgMoveSetHover"),
        }
    }
}

impl TryFrom<u16> for MiniMoveOpcode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        match value {
            232 => Ok(Self::SmsgForceMoveRoot),
            242 => Ok(Self::SmsgMoveFeatherFall),
            222 => Ok(Self::SmsgMoveWaterWalk),
            244 => Ok(Self::SmsgMoveSetHover),
            v => Err(crate::errors::EnumError::new("MiniMoveOpcode", v as u64),)
        }
    }
}

