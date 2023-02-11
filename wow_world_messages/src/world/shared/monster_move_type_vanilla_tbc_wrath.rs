/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L1):
/// ```text
/// enum MonsterMoveType : u8 {
///     NORMAL = 0;
///     STOP = 1;
///     FACING_SPOT = 2;
///     FACING_TARGET = 3;
///     FACING_ANGLE = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MonsterMoveType {
    Normal,
    Stop,
    FacingSpot,
    FacingTarget,
    FacingAngle,
}

impl MonsterMoveType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0x0,
            Self::Stop => 0x1,
            Self::FacingSpot => 0x2,
            Self::FacingTarget => 0x3,
            Self::FacingAngle => 0x4,
        }
    }

}

impl Default for MonsterMoveType {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for MonsterMoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Stop => f.write_str("Stop"),
            Self::FacingSpot => f.write_str("FacingSpot"),
            Self::FacingTarget => f.write_str("FacingTarget"),
            Self::FacingAngle => f.write_str("FacingAngle"),
        }
    }
}

impl TryFrom<u8> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Stop),
            2 => Ok(Self::FacingSpot),
            3 => Ok(Self::FacingTarget),
            4 => Ok(Self::FacingAngle),
            v => Err(crate::errors::EnumError::new("MonsterMoveType", v as u64),)
        }
    }
}

