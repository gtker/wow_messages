use crate::Guid;
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;
use crate::shared::monster_move_type_vanilla_tbc_wrath::MonsterMoveType;
use crate::shared::spline_flag_vanilla_tbc::SplineFlag;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L11):
/// ```text
/// smsg SMSG_MONSTER_MOVE = 0x00DD {
///     PackedGuid guid;
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
///     u32 amount_of_splines;
///     Vector3d[amount_of_splines] splines;
/// }
/// ```
pub struct SMSG_MONSTER_MOVE {
    pub guid: Guid,
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: SMSG_MONSTER_MOVE_MonsterMoveType,
    pub spline_flags: SplineFlag,
    pub duration: u32,
    pub splines: Vec<Vector3d>,
}

impl crate::Message for SMSG_MONSTER_MOVE {
    const OPCODE: u32 = 0x00dd;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // spline_point: Vector3d
        self.spline_point.write_into_vec(w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int() as u8).to_le_bytes())?;

        match &self.move_type {
            SMSG_MONSTER_MOVE_MonsterMoveType::Normal => {}
            SMSG_MONSTER_MOVE_MonsterMoveType::Stop => {}
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                position,
            } => {
                // position: Vector3d
                position.write_into_vec(w)?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
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

        // amount_of_splines: u32
        w.write_all(&(self.splines.len() as u32).to_le_bytes())?;

        // splines: Vector3d[amount_of_splines]
        for i in self.splines.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(31..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00DD, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // spline_point: Vector3d
        let spline_point = Vector3d::read(r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(r)?;

        // move_type: MonsterMoveType
        let move_type: MonsterMoveType = crate::util::read_u8_le(r)?.try_into()?;

        let move_type_if = match move_type {
            MonsterMoveType::Normal => SMSG_MONSTER_MOVE_MonsterMoveType::Normal,
            MonsterMoveType::Stop => SMSG_MONSTER_MOVE_MonsterMoveType::Stop,
            MonsterMoveType::FacingSpot => {
                // position: Vector3d
                let position = Vector3d::read(r)?;

                SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                    position,
                }
            }
            MonsterMoveType::FacingTarget => {
                // target: Guid
                let target = Guid::read(r)?;

                SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                    target,
                }
            }
            MonsterMoveType::FacingAngle => {
                // angle: f32
                let angle = crate::util::read_f32_le(r)?;
                SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                    angle,
                }
            }
        };

        // spline_flags: SplineFlag
        let spline_flags = SplineFlag::new(crate::util::read_u32_le(r)?);

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        // amount_of_splines: u32
        let amount_of_splines = crate::util::read_u32_le(r)?;

        // splines: Vector3d[amount_of_splines]
        let mut splines = Vec::with_capacity(amount_of_splines as usize);
        for i in 0..amount_of_splines {
            splines.push(Vector3d::read(r)?);
        }

        Ok(Self {
            guid,
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
impl crate::vanilla::ServerMessage for SMSG_MONSTER_MOVE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MONSTER_MOVE {}

impl SMSG_MONSTER_MOVE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + self.move_type.size() // move_type: SMSG_MONSTER_MOVE_MonsterMoveType
        + 4 // spline_flags: SplineFlag
        + 4 // duration: u32
        + 4 // amount_of_splines: u32
        + self.splines.len() * 12 // splines: Vector3d[amount_of_splines]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum SMSG_MONSTER_MOVE_MonsterMoveType {
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

impl Default for SMSG_MONSTER_MOVE_MonsterMoveType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Normal
    }
}

impl SMSG_MONSTER_MOVE_MonsterMoveType {
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

impl SMSG_MONSTER_MOVE_MonsterMoveType {
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

