use crate::Guid;
use crate::vanilla::MonsterMoveSpline;
use crate::vanilla::Vector3d;
use crate::vanilla::MonsterMoveType;
use crate::vanilla::SplineFlag;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm#L12):
/// ```text
/// struct MonsterMove {
///     Vector3d spline_point;
///     u32 spline_id;
///     MonsterMoveType move_type;
///     if (move_type == FACING_TARGET) {
///         Guid target;
///     }
///     else if (move_type == FACING_ANGLE) {
///         f32 angle;
///     }
///     else if (move_type == FACING_SPOT) {
///         Vector3d position;
///     }
///     SplineFlag spline_flags;
///     u32 duration;
///     MonsterMoveSpline splines;
/// }
/// ```
pub struct MonsterMove {
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: MonsterMove_MonsterMoveType,
    pub spline_flags: SplineFlag,
    pub duration: u32,
    pub splines: MonsterMoveSpline,
}

impl MonsterMove {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // spline_point: Vector3d
        self.spline_point.write_into_vec(w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int() as u8).to_le_bytes())?;

        match &self.move_type {
            MonsterMove_MonsterMoveType::Normal => {}
            MonsterMove_MonsterMoveType::Stop => {}
            MonsterMove_MonsterMoveType::FacingSpot {
                position,
            } => {
                // position: Vector3d
                position.write_into_vec(w)?;

            }
            MonsterMove_MonsterMoveType::FacingTarget {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MonsterMove_MonsterMoveType::FacingAngle {
                angle,
            } => {
                // angle: f32
                w.write_all(&angle.to_le_bytes())?;

            }
        }

        // spline_flags: SplineFlag
        w.write_all(&(self.spline_flags.as_int() as u32).to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // splines: MonsterMoveSpline
        self.splines.write_into_vec(w)?;

        Ok(())
    }
}

impl MonsterMove {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // spline_point: Vector3d
        let spline_point = Vector3d::read(r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(r)?;

        // move_type: MonsterMoveType
        let move_type: MonsterMoveType = crate::util::read_u8_le(r)?.try_into()?;

        let move_type_if = match move_type {
            MonsterMoveType::Normal => MonsterMove_MonsterMoveType::Normal,
            MonsterMoveType::Stop => MonsterMove_MonsterMoveType::Stop,
            MonsterMoveType::FacingSpot => {
                // position: Vector3d
                let position = Vector3d::read(r)?;

                MonsterMove_MonsterMoveType::FacingSpot {
                    position,
                }
            }
            MonsterMoveType::FacingTarget => {
                // target: Guid
                let target = Guid::read(r)?;

                MonsterMove_MonsterMoveType::FacingTarget {
                    target,
                }
            }
            MonsterMoveType::FacingAngle => {
                // angle: f32
                let angle = crate::util::read_f32_le(r)?;
                MonsterMove_MonsterMoveType::FacingAngle {
                    angle,
                }
            }
        };

        // spline_flags: SplineFlag
        let spline_flags = SplineFlag::new(crate::util::read_u32_le(r)?);

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        // splines: MonsterMoveSpline
        let splines = MonsterMoveSpline::read(r)?;

        Ok(Self {
            spline_point,
            spline_id,
            move_type: move_type_if,
            spline_flags,
            duration,
            splines,
        })
    }

}

impl MonsterMove {
    pub(crate) fn size(&self) -> usize {
        12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + self.move_type.size() // move_type: MonsterMove_MonsterMoveType
        + 4 // spline_flags: SplineFlag
        + 4 // duration: u32
        + self.splines.size() // splines: MonsterMoveSpline
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MonsterMove_MonsterMoveType {
    Normal,
    Stop,
    FacingSpot {
        position: Vector3d,
    },
    FacingTarget {
        target: Guid,
    },
    FacingAngle {
        angle: f32,
    },
}

impl Default for MonsterMove_MonsterMoveType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Normal
    }
}

impl MonsterMove_MonsterMoveType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0,
            Self::Stop => 1,
            Self::FacingSpot { .. } => 2,
            Self::FacingTarget { .. } => 3,
            Self::FacingAngle { .. } => 4,
        }
    }

}

impl MonsterMove_MonsterMoveType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Normal => {
                1
            }
            Self::Stop => {
                1
            }
            Self::FacingSpot {
                position,
            } => {
                1
                + 12 // position: Vector3d
            }
            Self::FacingTarget {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::FacingAngle {
                angle,
            } => {
                1
                + 4 // angle: f32
            }
        }
    }
}

