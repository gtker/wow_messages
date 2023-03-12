use std::io::{Read, Write};

use crate::Guid;
use crate::shared::monster_move_spline_vanilla_tbc_wrath::MonsterMoveSplines;
use crate::wrath::{
    MonsterMoveType, SplineFlag, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move_transport.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move_transport.wowm#L21):
/// ```text
/// smsg SMSG_MONSTER_MOVE_TRANSPORT = 0x02AE {
///     PackedGuid guid;
///     PackedGuid transport;
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
///     SplineFlag spline_flags;
///     if (spline_flags & ENTER_CYCLE) {
///         u32 animation_id;
///         u32 animation_start_time;
///     }
///     u32 duration;
///     if (spline_flags & PARABOLIC) {
///         f32 vertical_acceleration;
///         u32 effect_start_time;
///     }
///     MonsterMoveSplines splines;
/// }
/// ```
pub struct SMSG_MONSTER_MOVE_TRANSPORT {
    pub guid: Guid,
    pub transport: Guid,
    /// cmangos-wotlk sets to 0
    ///
    pub unknown: u8,
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType,
    pub spline_flags: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag,
    pub duration: u32,
    pub splines: MonsterMoveSplines,
}

impl crate::Message for SMSG_MONSTER_MOVE_TRANSPORT {
    const OPCODE: u32 = 0x02ae;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // transport: PackedGuid
        self.transport.write_packed_guid_into_vec(&mut w)?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // spline_point: Vector3d
        self.spline_point.write_into_vec(&mut w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&u8::from(self.move_type.as_int()).to_le_bytes())?;

        match &self.move_type {
            SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::Normal => {}
            SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::Stop => {}
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
        }

        // spline_flags: SplineFlag
        w.write_all(&u32::from(self.spline_flags.as_int()).to_le_bytes())?;

        if let Some(if_statement) = &self.spline_flags.enter_cycle {
            // animation_id: u32
            w.write_all(&if_statement.animation_id.to_le_bytes())?;

            // animation_start_time: u32
            w.write_all(&if_statement.animation_start_time.to_le_bytes())?;

        }

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        if let Some(if_statement) = &self.spline_flags.parabolic {
            // vertical_acceleration: f32
            w.write_all(&if_statement.vertical_acceleration.to_le_bytes())?;

            // effect_start_time: u32
            w.write_all(&if_statement.effect_start_time.to_le_bytes())?;

        }

        // splines: MonsterMoveSplines
        self.splines.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(34..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02AE, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // transport: PackedGuid
        let transport = Guid::read_packed(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

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

        let spline_flags_ENTER_CYCLE = if spline_flags.is_ENTER_CYCLE() {
            // animation_id: u32
            let animation_id = crate::util::read_u32_le(&mut r)?;

            // animation_start_time: u32
            let animation_start_time = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle {
                animation_id,
                animation_start_time,
            })
        }
        else {
            None
        };

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        let spline_flags_PARABOLIC = if spline_flags.is_PARABOLIC() {
            // vertical_acceleration: f32
            let vertical_acceleration = crate::util::read_f32_le(&mut r)?;

            // effect_start_time: u32
            let effect_start_time = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic {
                effect_start_time,
                vertical_acceleration,
            })
        }
        else {
            None
        };

        // splines: MonsterMoveSplines
        let splines = MonsterMoveSplines::read(&mut r)?;

        let spline_flags = SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag {
            inner: spline_flags.as_int(),
            parabolic: spline_flags_PARABOLIC,
            enter_cycle: spline_flags_ENTER_CYCLE,
        };

        Ok(Self {
            guid,
            transport,
            unknown,
            spline_point,
            spline_id,
            move_type: move_type_if,
            spline_flags,
            duration,
            splines,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MONSTER_MOVE_TRANSPORT {}

impl SMSG_MONSTER_MOVE_TRANSPORT {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + self.transport.size() // transport: PackedGuid
        + 1 // unknown: u8
        + 12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + self.move_type.size() // move_type: SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType
        + self.spline_flags.size() // spline_flags: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag
        + 4 // duration: u32
        + self.splines.size() // splines: MonsterMoveSplines
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag {
    inner: u32,
    parabolic: Option<SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic>,
    enter_cycle: Option<SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle>,
}

impl SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag {
    pub const fn new(inner: u32, parabolic: Option<SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic>,enter_cycle: Option<SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle>,) -> Self {
        Self {
            inner,
            parabolic, 
            enter_cycle, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.parabolic.is_none()
        && self.enter_cycle.is_none()
    }

    pub const fn new_DONE() -> Self {
        Self {
            inner: SplineFlag::DONE,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_DONE(mut self) -> Self {
        self.inner |= SplineFlag::DONE;
        self
    }

    pub const fn get_DONE(&self) -> bool {
        (self.inner & SplineFlag::DONE) != 0
    }

    pub fn clear_DONE(mut self) -> Self {
        self.inner &= SplineFlag::DONE.reverse_bits();
        self
    }

    pub const fn new_FALLING() -> Self {
        Self {
            inner: SplineFlag::FALLING,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_FALLING(mut self) -> Self {
        self.inner |= SplineFlag::FALLING;
        self
    }

    pub const fn get_FALLING(&self) -> bool {
        (self.inner & SplineFlag::FALLING) != 0
    }

    pub fn clear_FALLING(mut self) -> Self {
        self.inner &= SplineFlag::FALLING.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN3() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN3,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN3(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN3;
        self
    }

    pub const fn get_UNKNOWN3(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN3) != 0
    }

    pub fn clear_UNKNOWN3(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN3.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN4() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN4,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN4(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN4;
        self
    }

    pub const fn get_UNKNOWN4(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN4) != 0
    }

    pub fn clear_UNKNOWN4(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN4.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN5() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN5,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN5(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN5;
        self
    }

    pub const fn get_UNKNOWN5(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN5) != 0
    }

    pub fn clear_UNKNOWN5(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN5.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN6() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN6,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN6(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN6;
        self
    }

    pub const fn get_UNKNOWN6(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN6) != 0
    }

    pub fn clear_UNKNOWN6(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN6.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN7() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN7,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN7(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN7;
        self
    }

    pub const fn get_UNKNOWN7(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN7) != 0
    }

    pub fn clear_UNKNOWN7(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN7.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN8() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN8,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN8(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN8;
        self
    }

    pub const fn get_UNKNOWN8(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN8) != 0
    }

    pub fn clear_UNKNOWN8(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN8.reverse_bits();
        self
    }

    pub const fn new_RUNMODE() -> Self {
        Self {
            inner: SplineFlag::RUNMODE,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_RUNMODE(mut self) -> Self {
        self.inner |= SplineFlag::RUNMODE;
        self
    }

    pub const fn get_RUNMODE(&self) -> bool {
        (self.inner & SplineFlag::RUNMODE) != 0
    }

    pub fn clear_RUNMODE(mut self) -> Self {
        self.inner &= SplineFlag::RUNMODE.reverse_bits();
        self
    }

    pub const fn new_FLYING() -> Self {
        Self {
            inner: SplineFlag::FLYING,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_FLYING(mut self) -> Self {
        self.inner |= SplineFlag::FLYING;
        self
    }

    pub const fn get_FLYING(&self) -> bool {
        (self.inner & SplineFlag::FLYING) != 0
    }

    pub fn clear_FLYING(mut self) -> Self {
        self.inner &= SplineFlag::FLYING.reverse_bits();
        self
    }

    pub const fn new_NO_SPLINE() -> Self {
        Self {
            inner: SplineFlag::NO_SPLINE,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_NO_SPLINE(mut self) -> Self {
        self.inner |= SplineFlag::NO_SPLINE;
        self
    }

    pub const fn get_NO_SPLINE(&self) -> bool {
        (self.inner & SplineFlag::NO_SPLINE) != 0
    }

    pub fn clear_NO_SPLINE(mut self) -> Self {
        self.inner &= SplineFlag::NO_SPLINE.reverse_bits();
        self
    }

    pub const fn new_PARABOLIC(parabolic: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic) -> Self {
        Self {
            inner: SplineFlag::PARABOLIC,
            parabolic: Some(parabolic),
            enter_cycle: None,
        }
    }

    pub fn set_PARABOLIC(mut self, parabolic: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic) -> Self {
        self.inner |= SplineFlag::PARABOLIC;
        self.parabolic = Some(parabolic);
        self
    }

    pub const fn get_PARABOLIC(&self) -> Option<&SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic> {
        self.parabolic.as_ref()
    }

    pub fn clear_PARABOLIC(mut self) -> Self {
        self.inner &= SplineFlag::PARABOLIC.reverse_bits();
        self.parabolic = None;
        self
    }

    pub const fn new_UNKNOWN13() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN13,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN13(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN13;
        self
    }

    pub const fn get_UNKNOWN13(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN13) != 0
    }

    pub fn clear_UNKNOWN13(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN13.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN14() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN14,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN14(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN14;
        self
    }

    pub const fn get_UNKNOWN14(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN14) != 0
    }

    pub fn clear_UNKNOWN14(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN14.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN15() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN15,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN15(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN15;
        self
    }

    pub const fn get_UNKNOWN15(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN15) != 0
    }

    pub fn clear_UNKNOWN15(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN15.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN16() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN16,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN16(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN16;
        self
    }

    pub const fn get_UNKNOWN16(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN16) != 0
    }

    pub fn clear_UNKNOWN16(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN16.reverse_bits();
        self
    }

    pub const fn new_FINAL_POINT() -> Self {
        Self {
            inner: SplineFlag::FINAL_POINT,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_FINAL_POINT(mut self) -> Self {
        self.inner |= SplineFlag::FINAL_POINT;
        self
    }

    pub const fn get_FINAL_POINT(&self) -> bool {
        (self.inner & SplineFlag::FINAL_POINT) != 0
    }

    pub fn clear_FINAL_POINT(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_POINT.reverse_bits();
        self
    }

    pub const fn new_FINAL_TARGET() -> Self {
        Self {
            inner: SplineFlag::FINAL_TARGET,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_FINAL_TARGET(mut self) -> Self {
        self.inner |= SplineFlag::FINAL_TARGET;
        self
    }

    pub const fn get_FINAL_TARGET(&self) -> bool {
        (self.inner & SplineFlag::FINAL_TARGET) != 0
    }

    pub fn clear_FINAL_TARGET(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_TARGET.reverse_bits();
        self
    }

    pub const fn new_FINAL_ANGLE() -> Self {
        Self {
            inner: SplineFlag::FINAL_ANGLE,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_FINAL_ANGLE(mut self) -> Self {
        self.inner |= SplineFlag::FINAL_ANGLE;
        self
    }

    pub const fn get_FINAL_ANGLE(&self) -> bool {
        (self.inner & SplineFlag::FINAL_ANGLE) != 0
    }

    pub fn clear_FINAL_ANGLE(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_ANGLE.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN19() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN19,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN19(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN19;
        self
    }

    pub const fn get_UNKNOWN19(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN19) != 0
    }

    pub fn clear_UNKNOWN19(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN19.reverse_bits();
        self
    }

    pub const fn new_CYCLIC() -> Self {
        Self {
            inner: SplineFlag::CYCLIC,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_CYCLIC(mut self) -> Self {
        self.inner |= SplineFlag::CYCLIC;
        self
    }

    pub const fn get_CYCLIC(&self) -> bool {
        (self.inner & SplineFlag::CYCLIC) != 0
    }

    pub fn clear_CYCLIC(mut self) -> Self {
        self.inner &= SplineFlag::CYCLIC.reverse_bits();
        self
    }

    pub const fn new_ENTER_CYCLE(enter_cycle: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle) -> Self {
        Self {
            inner: SplineFlag::ENTER_CYCLE,
            parabolic: None,
            enter_cycle: Some(enter_cycle),
        }
    }

    pub fn set_ENTER_CYCLE(mut self, enter_cycle: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle) -> Self {
        self.inner |= SplineFlag::ENTER_CYCLE;
        self.enter_cycle = Some(enter_cycle);
        self
    }

    pub const fn get_ENTER_CYCLE(&self) -> Option<&SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle> {
        self.enter_cycle.as_ref()
    }

    pub fn clear_ENTER_CYCLE(mut self) -> Self {
        self.inner &= SplineFlag::ENTER_CYCLE.reverse_bits();
        self.enter_cycle = None;
        self
    }

    pub const fn new_FROZEN() -> Self {
        Self {
            inner: SplineFlag::FROZEN,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_FROZEN(mut self) -> Self {
        self.inner |= SplineFlag::FROZEN;
        self
    }

    pub const fn get_FROZEN(&self) -> bool {
        (self.inner & SplineFlag::FROZEN) != 0
    }

    pub fn clear_FROZEN(mut self) -> Self {
        self.inner &= SplineFlag::FROZEN.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN23() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN23,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN23(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN23;
        self
    }

    pub const fn get_UNKNOWN23(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN23) != 0
    }

    pub fn clear_UNKNOWN23(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN23.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN24() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN24,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN24(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN24;
        self
    }

    pub const fn get_UNKNOWN24(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN24) != 0
    }

    pub fn clear_UNKNOWN24(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN24.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN25() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN25,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN25(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN25;
        self
    }

    pub const fn get_UNKNOWN25(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN25) != 0
    }

    pub fn clear_UNKNOWN25(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN25.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN26() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN26,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN26(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN26;
        self
    }

    pub const fn get_UNKNOWN26(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN26) != 0
    }

    pub fn clear_UNKNOWN26(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN26.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN27() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN27,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN27(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN27;
        self
    }

    pub const fn get_UNKNOWN27(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN27) != 0
    }

    pub fn clear_UNKNOWN27(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN27.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN28() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN28,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN28(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN28;
        self
    }

    pub const fn get_UNKNOWN28(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN28) != 0
    }

    pub fn clear_UNKNOWN28(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN28.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN29() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN29,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN29(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN29;
        self
    }

    pub const fn get_UNKNOWN29(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN29) != 0
    }

    pub fn clear_UNKNOWN29(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN29.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN30() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN30,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN30(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN30;
        self
    }

    pub const fn get_UNKNOWN30(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN30) != 0
    }

    pub fn clear_UNKNOWN30(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN30.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN31() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN31,
            parabolic: None,
            enter_cycle: None,
        }
    }

    pub fn set_UNKNOWN31(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN31;
        self
    }

    pub const fn get_UNKNOWN31(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN31) != 0
    }

    pub fn clear_UNKNOWN31(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN31.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.parabolic {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.enter_cycle {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic {
    pub effect_start_time: u32,
    pub vertical_acceleration: f32,
}

impl SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic {
    pub(crate) fn size(&self) -> usize {
        4 // effect_start_time: u32
        + 4 // vertical_acceleration: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle {
    pub animation_id: u32,
    pub animation_start_time: u32,
}

impl SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle {
    pub(crate) fn size(&self) -> usize {
        4 // animation_id: u32
        + 4 // animation_start_time: u32
    }
}

