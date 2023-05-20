use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::monster_move_type_vanilla_tbc_wrath::MonsterMoveType;
use wow_world_base::shared::spline_flag_vanilla_tbc::SplineFlag;
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move_transport.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move_transport.wowm#L1):
/// ```text
/// smsg SMSG_MONSTER_MOVE_TRANSPORT = 0x02AE {
///     PackedGuid guid;
///     PackedGuid transport;
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
///     MonsterMoveSplines splines;
/// }
/// ```
pub struct SMSG_MONSTER_MOVE_TRANSPORT {
    pub guid: Guid,
    pub transport: Guid,
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType,
    pub spline_flags: SplineFlag,
    pub duration: u32,
    pub splines: Vec<Vector3d>,
}

impl crate::private::Sealed for SMSG_MONSTER_MOVE_TRANSPORT {}
impl crate::Message for SMSG_MONSTER_MOVE_TRANSPORT {
    const OPCODE: u32 = 0x02ae;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // transport: PackedGuid
        self.transport.write_packed_guid_into_vec(&mut w)?;

        // spline_point: Vector3d
        self.spline_point.write_into_vec(&mut w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int().to_le_bytes()))?;

        match &self.move_type {
            SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingSpot {
                position,
            } => {
                // position: Vector3d
                position.write_into_vec(&mut w)?;

            }
            SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingTarget {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingAngle {
                angle,
            } => {
                // angle: f32
                w.write_all(&angle.to_le_bytes())?;

            }
            _ => {}
        }

        // spline_flags: SplineFlag
        w.write_all(&(self.spline_flags.as_int().to_le_bytes()))?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // splines: MonsterMoveSplines
        crate::util::write_monster_move_spline(self.splines.as_slice(), &mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(33..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02AE, size: body_size });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // transport: PackedGuid
        let transport = Guid::read_packed(&mut r)?;

        // spline_point: Vector3d
        let spline_point = Vector3d::read(&mut r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(&mut r)?;

        // move_type: MonsterMoveType
        let move_type: MonsterMoveType = crate::util::read_u8_le(&mut r)?.try_into()?;

        let move_type_if = match move_type {
            MonsterMoveType::Normal => SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::Normal,
            MonsterMoveType::Stop => SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::Stop,
            MonsterMoveType::FacingSpot => {
                // position: Vector3d
                let position = Vector3d::read(&mut r)?;

                SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingSpot {
                    position,
                }
            }
            MonsterMoveType::FacingTarget => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingTarget {
                    target,
                }
            }
            MonsterMoveType::FacingAngle => {
                // angle: f32
                let angle = crate::util::read_f32_le(&mut r)?;

                SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingAngle {
                    angle,
                }
            }
        };

        // spline_flags: SplineFlag
        let spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        // splines: MonsterMoveSplines
        let splines = crate::util::read_monster_move_spline(&mut r)?;

        Ok(Self {
            guid,
            transport,
            spline_point,
            spline_id,
            move_type: move_type_if,
            spline_flags,
            duration,
            splines,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MONSTER_MOVE_TRANSPORT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MONSTER_MOVE_TRANSPORT {}

impl SMSG_MONSTER_MOVE_TRANSPORT {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + self.transport.size() // transport: PackedGuid
        + 12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + self.move_type.size() // move_type: SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType
        + 4 // spline_flags: SplineFlag
        + 4 // duration: u32
        + crate::util::monster_move_spline_size(self.splines.as_slice()) // splines: MonsterMoveSplines
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType {
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

impl Default for SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Normal
    }
}

impl SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType {
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

impl SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::FacingSpot {
                ..
            } => {
                1
                + 12 // position: Vector3d
            }
            Self::FacingTarget {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            Self::FacingAngle {
                ..
            } => {
                1
                + 4 // angle: f32
            }
            _ => 1,
        }
    }
}

