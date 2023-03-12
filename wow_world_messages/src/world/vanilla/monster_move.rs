use std::io::{Read, Write};

use crate::Guid;
use crate::shared::monster_move_spline_vanilla_tbc_wrath::MonsterMoveSplines;
use crate::vanilla::{
    MonsterMoveType, SplineFlag, Vector3d,
};

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
///         MonsterMoveSplines splines;
///     }
/// }
/// ```
pub struct MonsterMove {
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: MonsterMove_MonsterMoveType,
}

impl MonsterMove {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // spline_point: Vector3d
        self.spline_point.write_into_vec(&mut w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&u8::from(self.move_type.as_int()).to_le_bytes())?;

        match &self.move_type {
            MonsterMove_MonsterMoveType::Normal {
                duration,
                spline_flags,
                splines,
            } => {
            }
            MonsterMove_MonsterMoveType::FacingSpot {
                duration,
                position,
                spline_flags,
                splines,
            } => {
                // position: Vector3d
                position.write_into_vec(&mut w)?;

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
            _ => {}
        }

        match &self.move_type {
            MonsterMove_MonsterMoveType::Normal {
                duration,
                spline_flags,
                splines,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&u32::from(spline_flags.as_int()).to_le_bytes())?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSplines
                splines.write_into_vec(&mut w)?;

            }
            MonsterMove_MonsterMoveType::FacingSpot {
                duration,
                position,
                spline_flags,
                splines,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&u32::from(spline_flags.as_int()).to_le_bytes())?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSplines
                splines.write_into_vec(&mut w)?;

            }
            MonsterMove_MonsterMoveType::FacingTarget {
                duration,
                spline_flags,
                splines,
                target,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&u32::from(spline_flags.as_int()).to_le_bytes())?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSplines
                splines.write_into_vec(&mut w)?;

            }
            MonsterMove_MonsterMoveType::FacingAngle {
                angle,
                duration,
                spline_flags,
                splines,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&u32::from(spline_flags.as_int()).to_le_bytes())?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSplines
                splines.write_into_vec(&mut w)?;

            }
            _ => {}
        }

        Ok(())
    }
}

impl MonsterMove {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        let mut move_type_if_target = Default::default();
        let mut move_type_if_angle = Default::default();
        let mut move_type_if_position = Default::default();
        let mut move_type_if_spline_flags = Default::default();
        let mut move_type_if_duration = Default::default();
        let mut move_type_if_splines = Default::default();

        // spline_point: Vector3d
        let spline_point = Vector3d::read(&mut r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(&mut r)?;

        // move_type: MonsterMoveType
        let move_type: MonsterMoveType = crate::util::read_u8_le(&mut r)?.try_into()?;

        match move_type {
            MonsterMoveType::Normal => {
            }
            MonsterMoveType::Stop => {}
            MonsterMoveType::FacingSpot => {
                // position: Vector3d
                move_type_if_position = Vector3d::read(&mut r)?;

            }
            MonsterMoveType::FacingTarget => {
                // target: Guid
                move_type_if_target = Guid::read(&mut r)?;

            }
            MonsterMoveType::FacingAngle => {
                // angle: f32
                move_type_if_angle = crate::util::read_f32_le(&mut r)?;

            }
        };

        match move_type {
            MonsterMoveType::Normal => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(&mut r)?;

                // splines: MonsterMoveSplines
                move_type_if_splines = MonsterMoveSplines::read(&mut r)?;

            }
            MonsterMoveType::Stop => {}
            MonsterMoveType::FacingSpot => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(&mut r)?;

                // splines: MonsterMoveSplines
                move_type_if_splines = MonsterMoveSplines::read(&mut r)?;

            }
            MonsterMoveType::FacingTarget => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(&mut r)?;

                // splines: MonsterMoveSplines
                move_type_if_splines = MonsterMoveSplines::read(&mut r)?;

            }
            MonsterMoveType::FacingAngle => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(&mut r)?;

                // splines: MonsterMoveSplines
                move_type_if_splines = MonsterMoveSplines::read(&mut r)?;

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
        splines: MonsterMoveSplines,
    },
    Stop,
    FacingSpot {
        duration: u32,
        position: Vector3d,
        spline_flags: SplineFlag,
        splines: MonsterMoveSplines,
    },
    FacingTarget {
        duration: u32,
        spline_flags: SplineFlag,
        splines: MonsterMoveSplines,
        target: Guid,
    },
    FacingAngle {
        angle: f32,
        duration: u32,
        spline_flags: SplineFlag,
        splines: MonsterMoveSplines,
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
                + splines.size() // splines: MonsterMoveSplines
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
                + splines.size() // splines: MonsterMoveSplines
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
                + splines.size() // splines: MonsterMoveSplines
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
                + splines.size() // splines: MonsterMoveSplines
            }
        }
    }
}

