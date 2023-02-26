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
///     if (move_type != STOP) {
///         SplineFlag spline_flags;
///         u32 duration;
///         MonsterMoveSpline splines;
///     }
/// }
/// ```
pub struct MonsterMove {
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: MonsterMove_MonsterMoveType,
}

impl MonsterMove {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // spline_point: Vector3d
        self.spline_point.write_into_vec(w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int() as u8).to_le_bytes())?;

        match &self.move_type {
            MonsterMove_MonsterMoveType::Normal {
                duration,
                spline_flags,
                splines,
            } => {
            }
            MonsterMove_MonsterMoveType::Stop => {}
            MonsterMove_MonsterMoveType::FacingSpot {
                duration,
                position,
                spline_flags,
                splines,
            } => {
                // position: Vector3d
                position.write_into_vec(w)?;

            }
            MonsterMove_MonsterMoveType::FacingTarget {
                duration,
                spline_flags,
                splines,
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MonsterMove_MonsterMoveType::FacingAngle {
                angle,
                duration,
                spline_flags,
                splines,
            } => {
                // angle: f32
                w.write_all(&angle.to_le_bytes())?;

            }
        }

        match &self.move_type {
            MonsterMove_MonsterMoveType::Normal {
                duration,
                spline_flags,
                splines,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&(spline_flags.as_int() as u32).to_le_bytes())?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSpline
                splines.write_into_vec(w)?;

            }
            MonsterMove_MonsterMoveType::Stop => {}
            MonsterMove_MonsterMoveType::FacingSpot {
                duration,
                position,
                spline_flags,
                splines,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&(spline_flags.as_int() as u32).to_le_bytes())?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSpline
                splines.write_into_vec(w)?;

            }
            MonsterMove_MonsterMoveType::FacingTarget {
                duration,
                spline_flags,
                splines,
                target,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&(spline_flags.as_int() as u32).to_le_bytes())?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSpline
                splines.write_into_vec(w)?;

            }
            MonsterMove_MonsterMoveType::FacingAngle {
                angle,
                duration,
                spline_flags,
                splines,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&(spline_flags.as_int() as u32).to_le_bytes())?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSpline
                splines.write_into_vec(w)?;

            }
        }

        Ok(())
    }
}

impl MonsterMove {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        let mut move_type_if_target = Default::default();
        let mut move_type_if_angle = Default::default();
        let mut move_type_if_position = Default::default();
        let mut move_type_if_spline_flags = Default::default();
        let mut move_type_if_duration = Default::default();
        let mut move_type_if_splines = Default::default();

        // spline_point: Vector3d
        let spline_point = Vector3d::read(r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(r)?;

        // move_type: MonsterMoveType
        let move_type: MonsterMoveType = crate::util::read_u8_le(r)?.try_into()?;

        match move_type {
            MonsterMoveType::Normal => {
            }
            MonsterMoveType::Stop => {}
            MonsterMoveType::FacingSpot => {
                // position: Vector3d
                move_type_if_position = Vector3d::read(r)?;

            }
            MonsterMoveType::FacingTarget => {
                // target: Guid
                move_type_if_target = Guid::read(r)?;

            }
            MonsterMoveType::FacingAngle => {
                // angle: f32
                move_type_if_angle = crate::util::read_f32_le(r)?;

            }
        };

        match move_type {
            MonsterMoveType::Normal => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(r)?;

                // splines: MonsterMoveSpline
                move_type_if_splines = MonsterMoveSpline::read(r)?;

            }
            MonsterMoveType::Stop => {}
            MonsterMoveType::FacingSpot => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(r)?;

                // splines: MonsterMoveSpline
                move_type_if_splines = MonsterMoveSpline::read(r)?;

            }
            MonsterMoveType::FacingTarget => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(r)?;

                // splines: MonsterMoveSpline
                move_type_if_splines = MonsterMoveSpline::read(r)?;

            }
            MonsterMoveType::FacingAngle => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(r)?;

                // splines: MonsterMoveSpline
                move_type_if_splines = MonsterMoveSpline::read(r)?;

            }
        };

        let move_type_if = match move_type {
            MonsterMoveType::Normal => {
                MonsterMove_MonsterMoveType::Normal {
                    duration: move_type_if_duration,
                    spline_flags: move_type_if_spline_flags,
                    splines: move_type_if_splines,
                }
            }
            MonsterMoveType::Stop => {
                MonsterMove_MonsterMoveType::Stop {
                }
            }
            MonsterMoveType::FacingSpot => {
                MonsterMove_MonsterMoveType::FacingSpot {
                    duration: move_type_if_duration,
                    position: move_type_if_position,
                    spline_flags: move_type_if_spline_flags,
                    splines: move_type_if_splines,
                }
            }
            MonsterMoveType::FacingTarget => {
                MonsterMove_MonsterMoveType::FacingTarget {
                    duration: move_type_if_duration,
                    spline_flags: move_type_if_spline_flags,
                    splines: move_type_if_splines,
                    target: move_type_if_target,
                }
            }
            MonsterMoveType::FacingAngle => {
                MonsterMove_MonsterMoveType::FacingAngle {
                    angle: move_type_if_angle,
                    duration: move_type_if_duration,
                    spline_flags: move_type_if_spline_flags,
                    splines: move_type_if_splines,
                }
            }
        };

        Ok(Self {
            spline_point,
            spline_id,
            move_type: move_type_if,
        })
    }

}

impl MonsterMove {
    pub(crate) fn size(&self) -> usize {
        12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + self.move_type.size() // move_type: MonsterMove_MonsterMoveType
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MonsterMove_MonsterMoveType {
    Normal {
        duration: u32,
        spline_flags: SplineFlag,
        splines: MonsterMoveSpline,
    },
    Stop,
    FacingSpot {
        duration: u32,
        position: Vector3d,
        spline_flags: SplineFlag,
        splines: MonsterMoveSpline,
    },
    FacingTarget {
        duration: u32,
        spline_flags: SplineFlag,
        splines: MonsterMoveSpline,
        target: Guid,
    },
    FacingAngle {
        angle: f32,
        duration: u32,
        spline_flags: SplineFlag,
        splines: MonsterMoveSpline,
    },
}

impl Default for MonsterMove_MonsterMoveType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Stop
    }
}

impl MonsterMove_MonsterMoveType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Normal { .. } => 0,
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
            Self::Normal {
                duration,
                spline_flags,
                splines,
            } => {
                1
                + 4 // duration: u32
                + 4 // spline_flags: SplineFlag
                + splines.size() // splines: MonsterMoveSpline
            }
            Self::Stop => {
                1
            }
            Self::FacingSpot {
                duration,
                position,
                spline_flags,
                splines,
            } => {
                1
                + 4 // duration: u32
                + 12 // position: Vector3d
                + 4 // spline_flags: SplineFlag
                + splines.size() // splines: MonsterMoveSpline
            }
            Self::FacingTarget {
                duration,
                spline_flags,
                splines,
                target,
            } => {
                1
                + 4 // duration: u32
                + 4 // spline_flags: SplineFlag
                + splines.size() // splines: MonsterMoveSpline
                + 8 // target: Guid
            }
            Self::FacingAngle {
                angle,
                duration,
                spline_flags,
                splines,
            } => {
                1
                + 4 // angle: f32
                + 4 // duration: u32
                + 4 // spline_flags: SplineFlag
                + splines.size() // splines: MonsterMoveSpline
            }
        }
    }
}

