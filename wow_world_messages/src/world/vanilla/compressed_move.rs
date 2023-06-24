use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    CompressedMoveOpcode, MonsterMove, MonsterMoveType, SplineFlag, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm#L32):
/// ```text
/// struct CompressedMove {
///     u8 size = self.size;
///     CompressedMoveOpcode opcode;
///     PackedGuid guid;
///     if (opcode == SMSG_SPLINE_SET_RUN_SPEED) {
///         f32 speed;
///     }
///     else if (opcode == SMSG_MONSTER_MOVE) {
///         MonsterMove monster_move;
///     }
///     else if (opcode == SMSG_MONSTER_MOVE_TRANSPORT) {
///         PackedGuid transport;
///         MonsterMove monster_move_transport;
///     }
/// }
/// ```
pub struct CompressedMove {
    pub opcode: CompressedMove_CompressedMoveOpcode,
    pub guid: Guid,
}

impl CompressedMove {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // size: u8
        w.write_all(&((self.size() - 1) as u8).to_le_bytes())?;

        // opcode: CompressedMoveOpcode
        w.write_all(&(self.opcode.as_int().to_le_bytes()))?;

        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        match &self.opcode {
            CompressedMove_CompressedMoveOpcode::SmsgMonsterMove {
                monster_move,
            } => {
                // monster_move: MonsterMove
                monster_move.write_into_vec(&mut w)?;

            }
            CompressedMove_CompressedMoveOpcode::SmsgMonsterMoveTransport {
                monster_move_transport,
                transport,
            } => {
                // transport: PackedGuid
                crate::util::write_packed_guid(&transport, &mut w)?;

                // monster_move_transport: MonsterMove
                monster_move_transport.write_into_vec(&mut w)?;

            }
            CompressedMove_CompressedMoveOpcode::SmsgSplineSetRunSpeed {
                speed,
            } => {
                // speed: f32
                w.write_all(&speed.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
}

impl CompressedMove {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // size: u8
        let _size = crate::util::read_u8_le(&mut r)?;
        // size is expected to always be self.size (0)

        // opcode: CompressedMoveOpcode
        let opcode = crate::util::read_u16_le(&mut r)?.try_into()?;

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        let opcode_if = match opcode {
            CompressedMoveOpcode::SmsgMonsterMove => {
                // monster_move: MonsterMove
                let monster_move = MonsterMove::read(&mut r)?;

                CompressedMove_CompressedMoveOpcode::SmsgMonsterMove {
                    monster_move,
                }
            }
            CompressedMoveOpcode::SmsgMonsterMoveTransport => {
                // transport: PackedGuid
                let transport = crate::util::read_packed_guid(&mut r)?;

                // monster_move_transport: MonsterMove
                let monster_move_transport = MonsterMove::read(&mut r)?;

                CompressedMove_CompressedMoveOpcode::SmsgMonsterMoveTransport {
                    monster_move_transport,
                    transport,
                }
            }
            CompressedMoveOpcode::SmsgSplineSetRunSpeed => {
                // speed: f32
                let speed = crate::util::read_f32_le(&mut r)?;

                CompressedMove_CompressedMoveOpcode::SmsgSplineSetRunSpeed {
                    speed,
                }
            }
            CompressedMoveOpcode::SmsgSplineMoveUnroot => CompressedMove_CompressedMoveOpcode::SmsgSplineMoveUnroot,
            CompressedMoveOpcode::SmsgSplineMoveSetRunMode => CompressedMove_CompressedMoveOpcode::SmsgSplineMoveSetRunMode,
            CompressedMoveOpcode::SmsgSplineMoveSetWalkMode => CompressedMove_CompressedMoveOpcode::SmsgSplineMoveSetWalkMode,
        };

        Ok(Self {
            opcode: opcode_if,
            guid,
        })
    }

}

impl CompressedMove {
    pub(crate) fn size(&self) -> usize {
        1 // size: u8
        + self.opcode.size() // opcode: CompressedMove_CompressedMoveOpcode
        + crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CompressedMove_CompressedMoveOpcode {
    SmsgMonsterMove {
        monster_move: MonsterMove,
    },
    SmsgMonsterMoveTransport {
        monster_move_transport: MonsterMove,
        transport: Guid,
    },
    SmsgSplineSetRunSpeed {
        speed: f32,
    },
    SmsgSplineMoveUnroot,
    SmsgSplineMoveSetRunMode,
    SmsgSplineMoveSetWalkMode,
}

impl Default for CompressedMove_CompressedMoveOpcode {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SmsgSplineMoveUnroot
    }
}

impl CompressedMove_CompressedMoveOpcode {
    pub(crate) const fn as_int(&self) -> u16 {
        match self {
            Self::SmsgMonsterMove { .. } => 221,
            Self::SmsgMonsterMoveTransport { .. } => 686,
            Self::SmsgSplineSetRunSpeed { .. } => 766,
            Self::SmsgSplineMoveUnroot => 772,
            Self::SmsgSplineMoveSetRunMode => 781,
            Self::SmsgSplineMoveSetWalkMode => 782,
        }
    }

}

impl std::fmt::Display for CompressedMove_CompressedMoveOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SmsgMonsterMove{ .. } => f.write_str("SmsgMonsterMove"),
            Self::SmsgMonsterMoveTransport{ .. } => f.write_str("SmsgMonsterMoveTransport"),
            Self::SmsgSplineSetRunSpeed{ .. } => f.write_str("SmsgSplineSetRunSpeed"),
            Self::SmsgSplineMoveUnroot => f.write_str("SmsgSplineMoveUnroot"),
            Self::SmsgSplineMoveSetRunMode => f.write_str("SmsgSplineMoveSetRunMode"),
            Self::SmsgSplineMoveSetWalkMode => f.write_str("SmsgSplineMoveSetWalkMode"),
        }
    }
}

impl CompressedMove_CompressedMoveOpcode {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::SmsgMonsterMove {
                monster_move,
            } => {
                2
                + monster_move.size() // monster_move: MonsterMove
            }
            Self::SmsgMonsterMoveTransport {
                monster_move_transport,
                transport,
            } => {
                2
                + monster_move_transport.size() // monster_move_transport: MonsterMove
                + crate::util::packed_guid_size(&transport) // transport: PackedGuid
            }
            Self::SmsgSplineSetRunSpeed {
                ..
            } => {
                2
                + 4 // speed: f32
            }
            _ => 2,
        }
    }
}

