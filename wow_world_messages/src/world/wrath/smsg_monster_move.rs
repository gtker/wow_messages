use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MonsterMoveData, MonsterMoveType, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:51`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L51):
/// ```text
/// smsg SMSG_MONSTER_MOVE = 0x00DD {
///     PackedGuid guid;
///     u8 unknown;
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
///         MonsterMoveData movement;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_MONSTER_MOVE {
    pub guid: Guid,
    /// cmangos-wotlk sets to 0
    pub unknown: u8,
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: SMSG_MONSTER_MOVE_MonsterMoveType,
}

impl crate::private::Sealed for SMSG_MONSTER_MOVE {}
impl SMSG_MONSTER_MOVE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(19..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        let mut move_type_if_target = Default::default();
        let mut move_type_if_angle = Default::default();
        let mut move_type_if_position = Default::default();
        let mut move_type_if_movement = Default::default();

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // spline_point: Vector3d
        let spline_point = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(&mut r)?;

        // move_type: MonsterMoveType
        let move_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        match move_type {
            MonsterMoveType::Normal => {
            }
            MonsterMoveType::Stop => {}
            MonsterMoveType::FacingSpot => {
                // position: Vector3d
                move_type_if_position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

            }
            MonsterMoveType::FacingTarget => {
                // target: Guid
                move_type_if_target = crate::util::read_guid(&mut r)?;

            }
            MonsterMoveType::FacingAngle => {
                // angle: f32
                move_type_if_angle = crate::util::read_f32_le(&mut r)?;

            }
        };

        match move_type {
            MonsterMoveType::Normal => {
                // movement: MonsterMoveData
                move_type_if_movement = MonsterMoveData::read(&mut r)?;

            }
            MonsterMoveType::Stop => {}
            MonsterMoveType::FacingSpot => {
                // movement: MonsterMoveData
                move_type_if_movement = MonsterMoveData::read(&mut r)?;

            }
            MonsterMoveType::FacingTarget => {
                // movement: MonsterMoveData
                move_type_if_movement = MonsterMoveData::read(&mut r)?;

            }
            MonsterMoveType::FacingAngle => {
                // movement: MonsterMoveData
                move_type_if_movement = MonsterMoveData::read(&mut r)?;

            }
        };

        if !r.is_empty() {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        let move_type_if = match move_type {
            MonsterMoveType::Normal => {
                SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                    movement: move_type_if_movement,
                }
            }
            MonsterMoveType::Stop => {
                SMSG_MONSTER_MOVE_MonsterMoveType::Stop {
                }
            }
            MonsterMoveType::FacingSpot => {
                SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                    movement: move_type_if_movement,
                    position: move_type_if_position,
                }
            }
            MonsterMoveType::FacingTarget => {
                SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                    movement: move_type_if_movement,
                    target: move_type_if_target,
                }
            }
            MonsterMoveType::FacingAngle => {
                SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                    angle: move_type_if_angle,
                    movement: move_type_if_movement,
                }
            }
        };

        Ok(Self {
            guid,
            unknown,
            spline_point,
            spline_id,
            move_type: move_type_if,
        })
    }

}

impl crate::Message for SMSG_MONSTER_MOVE {
    const OPCODE: u32 = 0x00dd;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MONSTER_MOVE"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // spline_point: Vector3d
        crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.spline_point, &mut w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int().to_le_bytes()))?;

        match &self.move_type {
            SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                movement,
            } => {
            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                movement,
                position,
            } => {
                // position: Vector3d
                crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&position, &mut w)?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                movement,
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                angle,
                movement,
            } => {
                // angle: f32
                w.write_all(&angle.to_le_bytes())?;

            }
            _ => {}
        }

        match &self.move_type {
            SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                movement,
            } => {
                // movement: MonsterMoveData
                movement.write_into_vec(&mut w)?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                movement,
                position,
            } => {
                // movement: MonsterMoveData
                movement.write_into_vec(&mut w)?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                movement,
                target,
            } => {
                // movement: MonsterMoveData
                movement.write_into_vec(&mut w)?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                angle,
                movement,
            } => {
                // movement: MonsterMoveData
                movement.write_into_vec(&mut w)?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(221, "SMSG_MONSTER_MOVE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MONSTER_MOVE {}

impl SMSG_MONSTER_MOVE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 1 // unknown: u8
        + 12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + self.move_type.size() // move_type: SMSG_MONSTER_MOVE_MonsterMoveType
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[derive(Default)]
pub enum SMSG_MONSTER_MOVE_MonsterMoveType {
    Normal {
        movement: MonsterMoveData,
    },
    #[default]
    Stop,
    FacingSpot {
        movement: MonsterMoveData,
        position: Vector3d,
    },
    FacingTarget {
        movement: MonsterMoveData,
        target: Guid,
    },
    FacingAngle {
        angle: f32,
        movement: MonsterMoveData,
    },
}

impl SMSG_MONSTER_MOVE_MonsterMoveType {
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

impl std::fmt::Display for SMSG_MONSTER_MOVE_MonsterMoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal{ .. } => f.write_str("Normal"),
            Self::Stop => f.write_str("Stop"),
            Self::FacingSpot{ .. } => f.write_str("FacingSpot"),
            Self::FacingTarget{ .. } => f.write_str("FacingTarget"),
            Self::FacingAngle{ .. } => f.write_str("FacingAngle"),
        }
    }
}

impl SMSG_MONSTER_MOVE_MonsterMoveType {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Normal {
                movement,
            } => {
                1
                + movement.size() // movement: MonsterMoveData
            }
            Self::FacingSpot {
                movement,
                ..
            } => {
                1
                + movement.size() // movement: MonsterMoveData
                + 12 // position: Vector3d
            }
            Self::FacingTarget {
                movement,
                ..
            } => {
                1
                + movement.size() // movement: MonsterMoveData
                + 8 // target: Guid
            }
            Self::FacingAngle {
                movement,
                ..
            } => {
                1
                + 4 // angle: f32
                + movement.size() // movement: MonsterMoveData
            }
            _ => 1,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_MONSTER_MOVE;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 23] = [ 0x00, 0x15, 0xDD, 0x00, 0x00, 0x5A, 0x00, 0x00, 0x80,
         0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x40, 0x40, 0x44, 0x33, 0x22,
         0x11, 0x01, ];

    pub(crate) fn expected0() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0x0),
            unknown: 0x5A,
            spline_point: Vector3d {
                x: 1_f32,
                y: 2_f32,
                z: 3_f32,
            },
            spline_id: 0x11223344,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Stop,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 72.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 72.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 72.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 51] = [ 0x00, 0x31, 0xDD, 0x00, 0x00, 0x5A, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x33, 0x22,
         0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x03, 0x02, 0x01, 0x02, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00,
         0x40, 0x40, 0xFF, 0x37, 0x00, 0x80, ];

    pub(crate) fn expected1() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0x0),
            unknown: 0x5A,
            spline_point: Vector3d {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32,
            },
            spline_id: 0x11223344,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                movement: MonsterMoveData {
                    spline_flags: MonsterMoveData_SplineFlag::empty()
                        ,
                    duration: 0x1020304,
                    splines: vec![
                        Vector3d {
                            x: 1.0,
                            y: 2.0,
                            z: 3.0,
                        },
                        Vector3d {
                            x: -0.25,
                            y: 1.5,
                            z: -128.0,
                        },
                    ],
                },
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 96.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 96.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 96.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 47] = [ 0x00, 0x2D, 0xDD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x88, 0x77, 0x66, 0x55, 0x01, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, ];

    pub(crate) fn expected2() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0x0),
            unknown: 0x0,
            spline_point: Vector3d {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32,
            },
            spline_id: 0x0,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                movement: MonsterMoveData {
                    spline_flags: MonsterMoveData_SplineFlag::empty()
                        .set_enter_cycle()
                        ,
                    duration: 0x55667788,
                    splines: vec![
                        Vector3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    ],
                },
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 141.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 141.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 141.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    const RAW3: [u8; 52] = [ 0x00, 0x32, 0xDD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x7A, 0x44, 0x33, 0x22, 0x11, 0x88,
         0x77, 0x66, 0x55, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected3() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0x0),
            unknown: 0x0,
            spline_point: Vector3d {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32,
            },
            spline_id: 0x0,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                movement: MonsterMoveData {
                    spline_flags: MonsterMoveData_SplineFlag::empty()
                        .set_animation(MonsterMoveData_SplineFlag_Animation {
                            animation_id: 0x7A,
                            animation_start_time: 0x11223344,
                        })
                        ,
                    duration: 0x55667788,
                    splines: vec![
                        Vector3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    ],
                },
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 178.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW3)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 178.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 178.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    const RAW4: [u8; 55] = [ 0x00, 0x35, 0xDD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x88, 0x77, 0x66, 0x55, 0x00, 0x00,
         0xC0, 0x3F, 0x44, 0x33, 0x22, 0x11, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x80, 0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x40, 0x40, ];

    pub(crate) fn expected4() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0x0),
            unknown: 0x0,
            spline_point: Vector3d {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32,
            },
            spline_id: 0x0,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                movement: MonsterMoveData {
                    spline_flags: MonsterMoveData_SplineFlag::empty()
                        .set_parabolic(MonsterMoveData_SplineFlag_Parabolic {
                            effect_start_time: 0x11223344,
                            vertical_acceleration: 1.5_f32,
                        })
                        ,
                    duration: 0x55667788,
                    splines: vec![
                        Vector3d {
                            x: 1.0,
                            y: 2.0,
                            z: 3.0,
                        },
                    ],
                },
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 219.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move4() {
        let expected = expected4();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW4)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 219.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move4() {
        let expected = expected4();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 219.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move4() {
        let expected = expected4();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

    const RAW5: [u8; 59] = [ 0x00, 0x39, 0xDD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x04, 0x03, 0x02, 0x01, 0x02, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00,
         0x40, 0x40, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0xA8, 0xC0, 0x00, 0x00,
         0xC4, 0x40, ];

    pub(crate) fn expected5() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0x0),
            unknown: 0x0,
            spline_point: Vector3d {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32,
            },
            spline_id: 0x0,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                movement: MonsterMoveData {
                    spline_flags: MonsterMoveData_SplineFlag::empty()
                        .set_flying()
                        ,
                    duration: 0x1020304,
                    splines: vec![
                        Vector3d {
                            x: 1.0,
                            y: 2.0,
                            z: 3.0,
                        },
                        Vector3d {
                            x: 4.5,
                            y: -5.25,
                            z: 6.125,
                        },
                    ],
                },
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 262.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move5() {
        let expected = expected5();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW5)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW5);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 262.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move5() {
        let expected = expected5();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW5)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW5);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 262.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move5() {
        let expected = expected5();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW5)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW5);
    }

    const RAW6: [u8; 59] = [ 0x00, 0x39, 0xDD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x20, 0x04, 0x00, 0x04, 0x03, 0x02, 0x01, 0x02, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00,
         0x40, 0x40, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0xA8, 0xC0, 0x00, 0x00,
         0xC4, 0x40, ];

    pub(crate) fn expected6() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0x0),
            unknown: 0x0,
            spline_point: Vector3d {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32,
            },
            spline_id: 0x0,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                movement: MonsterMoveData {
                    spline_flags: MonsterMoveData_SplineFlag::empty()
                        .set_flying()
                        .set_catmullrom()
                        ,
                    duration: 0x1020304,
                    splines: vec![
                        Vector3d {
                            x: 1.0,
                            y: 2.0,
                            z: 3.0,
                        },
                        Vector3d {
                            x: 4.5,
                            y: -5.25,
                            z: 6.125,
                        },
                    ],
                },
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 305.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move6() {
        let expected = expected6();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW6)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW6.len());

        let mut dest = Vec::with_capacity(RAW6.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW6);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 305.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move6() {
        let expected = expected6();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW6)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW6.len());

        let mut dest = Vec::with_capacity(RAW6.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW6);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 305.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move6() {
        let expected = expected6();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW6)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW6.len());

        let mut dest = Vec::with_capacity(RAW6.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW6);
    }

    const RAW7: [u8; 59] = [ 0x00, 0x39, 0xDD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x04, 0x03, 0x02, 0x01, 0x02, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00,
         0x40, 0x40, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0xA8, 0xC0, 0x00, 0x00,
         0xC4, 0x40, ];

    pub(crate) fn expected7() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0x0),
            unknown: 0x0,
            spline_point: Vector3d {
                x: 0_f32,
                y: 0_f32,
                z: 0_f32,
            },
            spline_id: 0x0,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                movement: MonsterMoveData {
                    spline_flags: MonsterMoveData_SplineFlag::empty()
                        .set_catmullrom()
                        ,
                    duration: 0x1020304,
                    splines: vec![
                        Vector3d {
                            x: 1.0,
                            y: 2.0,
                            z: 3.0,
                        },
                        Vector3d {
                            x: 4.5,
                            y: -5.25,
                            z: 6.125,
                        },
                    ],
                },
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 348.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move7() {
        let expected = expected7();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW7)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW7.len());

        let mut dest = Vec::with_capacity(RAW7.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW7);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 348.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move7() {
        let expected = expected7();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW7)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW7.len());

        let mut dest = Vec::with_capacity(RAW7.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW7);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 348.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move7() {
        let expected = expected7();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW7)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW7.len());

        let mut dest = Vec::with_capacity(RAW7.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW7);
    }

}

