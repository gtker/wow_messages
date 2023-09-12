use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MonsterMoveType, SplineFlag, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L31):
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
pub struct SMSG_MONSTER_MOVE {
    pub guid: Guid,
    /// cmangos-wotlk sets to 0
    pub unknown: u8,
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: SMSG_MONSTER_MOVE_MonsterMoveType,
    pub spline_flags: SMSG_MONSTER_MOVE_SplineFlag,
    pub duration: u32,
    pub splines: Vec<Vector3d>,
}

impl crate::private::Sealed for SMSG_MONSTER_MOVE {}
impl SMSG_MONSTER_MOVE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(31..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

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

        let move_type_if = match move_type {
            MonsterMoveType::Normal => SMSG_MONSTER_MOVE_MonsterMoveType::Normal,
            MonsterMoveType::Stop => SMSG_MONSTER_MOVE_MonsterMoveType::Stop,
            MonsterMoveType::FacingSpot => {
                // position: Vector3d
                let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

                SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                    position,
                }
            }
            MonsterMoveType::FacingTarget => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                    target,
                }
            }
            MonsterMoveType::FacingAngle => {
                // angle: f32
                let angle = crate::util::read_f32_le(&mut r)?;

                SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                    angle,
                }
            }
        };

        // spline_flags: SplineFlag
        let spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

        let spline_flags_enter_cycle = if spline_flags.is_enter_cycle() {
            // animation_id: u32
            let animation_id = crate::util::read_u32_le(&mut r)?;

            // animation_start_time: u32
            let animation_start_time = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_MONSTER_MOVE_SplineFlag_EnterCycle {
                animation_id,
                animation_start_time,
            })
        }
        else {
            None
        };

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        let spline_flags_parabolic = if spline_flags.is_parabolic() {
            // vertical_acceleration: f32
            let vertical_acceleration = crate::util::read_f32_le(&mut r)?;

            // effect_start_time: u32
            let effect_start_time = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_MONSTER_MOVE_SplineFlag_Parabolic {
                effect_start_time,
                vertical_acceleration,
            })
        }
        else {
            None
        };

        // splines: MonsterMoveSplines
        let splines = crate::util::read_monster_move_spline(&mut r)?;

        let spline_flags = SMSG_MONSTER_MOVE_SplineFlag {
            inner: spline_flags.as_int(),
            parabolic: spline_flags_parabolic,
            enter_cycle: spline_flags_enter_cycle,
        };

        Ok(Self {
            guid,
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

impl crate::Message for SMSG_MONSTER_MOVE {
    const OPCODE: u32 = 0x00dd;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MONSTER_MOVE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MONSTER_MOVE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();
        // spline_point: Vector3d
        writeln!(s, "    spline_point = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.spline_point.x.to_string().contains('.') { self.spline_point.x.to_string() } else { format!("{}.0", self.spline_point.x) }).unwrap();
        writeln!(s, "    {}", if self.spline_point.y.to_string().contains('.') { self.spline_point.y.to_string() } else { format!("{}.0", self.spline_point.y) }).unwrap();
        writeln!(s, "    {}", if self.spline_point.z.to_string().contains('.') { self.spline_point.z.to_string() } else { format!("{}.0", self.spline_point.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    spline_id = {};", self.spline_id).unwrap();
        writeln!(s, "    move_type = {};", MonsterMoveType::try_from(self.move_type.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.move_type {
            crate::wrath::SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                position,
            } => {
                // position: Vector3d
                writeln!(s, "    position = {{").unwrap();
                // Members
                writeln!(s, "    {}", if position.x.to_string().contains('.') { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                writeln!(s, "    {}", if position.y.to_string().contains('.') { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                writeln!(s, "    {}", if position.z.to_string().contains('.') { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                writeln!(s, "    }};").unwrap();
            }
            crate::wrath::SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::wrath::SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                angle,
            } => {
                writeln!(s, "    {}", if angle.to_string().contains('.') { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
            }
            _ => {}
        }

        writeln!(s, "    spline_flags = {};", SplineFlag::new(self.spline_flags.as_int()).as_test_case_value()).unwrap();
        if let Some(if_statement) = &self.spline_flags.get_enter_cycle() {
            writeln!(s, "    animation_id = {};", if_statement.animation_id).unwrap();
            writeln!(s, "    animation_start_time = {};", if_statement.animation_start_time).unwrap();
        }

        writeln!(s, "    duration = {};", self.duration).unwrap();
        if let Some(if_statement) = &self.spline_flags.get_parabolic() {
            writeln!(s, "    {}", if if_statement.vertical_acceleration.to_string().contains('.') { if_statement.vertical_acceleration.to_string() } else { format!("{}.0", if_statement.vertical_acceleration) }).unwrap();
            writeln!(s, "    effect_start_time = {};", if_statement.effect_start_time).unwrap();
        }

        panic!("unsupported type for test case printing: 'MonsterMoveSplines' for variable 'splines'");

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 221_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");
        writeln!(s, "    /* spline_point: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* spline_point: Vector3d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "move_type", "    ");
        match &self.move_type {
            crate::wrath::SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                position,
            } => {
                writeln!(s, "    /* position: Vector3d start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
                writeln!(s, "    /* position: Vector3d end */").unwrap();
            }
            crate::wrath::SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::wrath::SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                angle,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "angle", "    ");
            }
            _ => {}
        }

        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "    ");
        if let Some(if_statement) = &self.spline_flags.get_enter_cycle() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "animation_id", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "animation_start_time", "    ");
        }

        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "    ");
        if let Some(if_statement) = &self.spline_flags.get_parabolic() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "vertical_acceleration", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "effect_start_time", "    ");
        }

        panic!("unsupported type Vec<Vector3d> for variable 'splines'");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
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
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                position,
            } => {
                // position: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&position, &mut w)?;

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
            _ => {}
        }

        // spline_flags: SplineFlag
        w.write_all(&(self.spline_flags.as_int().to_le_bytes()))?;

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
        crate::util::write_monster_move_spline(self.splines.as_slice(), &mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(221, "SMSG_MONSTER_MOVE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MONSTER_MOVE {}

impl SMSG_MONSTER_MOVE {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 1 // unknown: u8
        + 12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + self.move_type.size() // move_type: SMSG_MONSTER_MOVE_MonsterMoveType
        + self.spline_flags.size() // spline_flags: SMSG_MONSTER_MOVE_SplineFlag
        + 4 // duration: u32
        + crate::util::monster_move_spline_size(self.splines.as_slice()) // splines: MonsterMoveSplines
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

impl std::fmt::Display for SMSG_MONSTER_MOVE_MonsterMoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_MONSTER_MOVE_SplineFlag {
    inner: u32,
    parabolic: Option<SMSG_MONSTER_MOVE_SplineFlag_Parabolic>,
    enter_cycle: Option<SMSG_MONSTER_MOVE_SplineFlag_EnterCycle>,
}

impl SMSG_MONSTER_MOVE_SplineFlag {
    pub const fn new(inner: u32, parabolic: Option<SMSG_MONSTER_MOVE_SplineFlag_Parabolic>,enter_cycle: Option<SMSG_MONSTER_MOVE_SplineFlag_EnterCycle>,) -> Self {
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

    pub const fn new_done() -> Self {
        Self {
            inner: SplineFlag::DONE,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_done(mut self) -> Self {
        self.inner |= SplineFlag::DONE;
        self
    }

    pub const fn get_done(&self) -> bool {
        (self.inner & SplineFlag::DONE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_done(mut self) -> Self {
        self.inner &= SplineFlag::DONE.reverse_bits();
        self
    }

    pub const fn new_falling() -> Self {
        Self {
            inner: SplineFlag::FALLING,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_falling(mut self) -> Self {
        self.inner |= SplineFlag::FALLING;
        self
    }

    pub const fn get_falling(&self) -> bool {
        (self.inner & SplineFlag::FALLING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_falling(mut self) -> Self {
        self.inner &= SplineFlag::FALLING.reverse_bits();
        self
    }

    pub const fn new_unknown3() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN3,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown3(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN3;
        self
    }

    pub const fn get_unknown3(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN3) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown3(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN3.reverse_bits();
        self
    }

    pub const fn new_unknown4() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN4,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown4(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN4;
        self
    }

    pub const fn get_unknown4(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN4) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown4(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN4.reverse_bits();
        self
    }

    pub const fn new_unknown5() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN5,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown5(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN5;
        self
    }

    pub const fn get_unknown5(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN5) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown5(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN5.reverse_bits();
        self
    }

    pub const fn new_unknown6() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN6,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown6(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN6;
        self
    }

    pub const fn get_unknown6(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN6) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown6(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN6.reverse_bits();
        self
    }

    pub const fn new_unknown7() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN7,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown7(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN7;
        self
    }

    pub const fn get_unknown7(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN7) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown7(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN7.reverse_bits();
        self
    }

    pub const fn new_unknown8() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN8,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown8(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN8;
        self
    }

    pub const fn get_unknown8(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN8) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown8(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN8.reverse_bits();
        self
    }

    pub const fn new_runmode() -> Self {
        Self {
            inner: SplineFlag::RUNMODE,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_runmode(mut self) -> Self {
        self.inner |= SplineFlag::RUNMODE;
        self
    }

    pub const fn get_runmode(&self) -> bool {
        (self.inner & SplineFlag::RUNMODE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_runmode(mut self) -> Self {
        self.inner &= SplineFlag::RUNMODE.reverse_bits();
        self
    }

    pub const fn new_flying() -> Self {
        Self {
            inner: SplineFlag::FLYING,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_flying(mut self) -> Self {
        self.inner |= SplineFlag::FLYING;
        self
    }

    pub const fn get_flying(&self) -> bool {
        (self.inner & SplineFlag::FLYING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_flying(mut self) -> Self {
        self.inner &= SplineFlag::FLYING.reverse_bits();
        self
    }

    pub const fn new_no_spline() -> Self {
        Self {
            inner: SplineFlag::NO_SPLINE,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_no_spline(mut self) -> Self {
        self.inner |= SplineFlag::NO_SPLINE;
        self
    }

    pub const fn get_no_spline(&self) -> bool {
        (self.inner & SplineFlag::NO_SPLINE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_no_spline(mut self) -> Self {
        self.inner &= SplineFlag::NO_SPLINE.reverse_bits();
        self
    }

    pub const fn new_parabolic(parabolic: SMSG_MONSTER_MOVE_SplineFlag_Parabolic) -> Self {
        Self {
            inner: SplineFlag::PARABOLIC,
            parabolic: Some(parabolic),
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_parabolic(mut self, parabolic: SMSG_MONSTER_MOVE_SplineFlag_Parabolic) -> Self {
        self.inner |= SplineFlag::PARABOLIC;
        self.parabolic = Some(parabolic);
        self
    }

    pub const fn get_parabolic(&self) -> Option<&SMSG_MONSTER_MOVE_SplineFlag_Parabolic> {
        self.parabolic.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_parabolic(mut self) -> Self {
        self.inner &= SplineFlag::PARABOLIC.reverse_bits();
        self.parabolic = None;
        self
    }

    pub const fn new_unknown13() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN13,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown13(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN13;
        self
    }

    pub const fn get_unknown13(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN13) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown13(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN13.reverse_bits();
        self
    }

    pub const fn new_unknown14() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN14,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown14(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN14;
        self
    }

    pub const fn get_unknown14(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN14) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown14(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN14.reverse_bits();
        self
    }

    pub const fn new_unknown15() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN15,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown15(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN15;
        self
    }

    pub const fn get_unknown15(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN15) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown15(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN15.reverse_bits();
        self
    }

    pub const fn new_unknown16() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN16,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown16(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN16;
        self
    }

    pub const fn get_unknown16(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN16) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown16(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN16.reverse_bits();
        self
    }

    pub const fn new_final_point() -> Self {
        Self {
            inner: SplineFlag::FINAL_POINT,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_final_point(mut self) -> Self {
        self.inner |= SplineFlag::FINAL_POINT;
        self
    }

    pub const fn get_final_point(&self) -> bool {
        (self.inner & SplineFlag::FINAL_POINT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_final_point(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_POINT.reverse_bits();
        self
    }

    pub const fn new_final_target() -> Self {
        Self {
            inner: SplineFlag::FINAL_TARGET,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_final_target(mut self) -> Self {
        self.inner |= SplineFlag::FINAL_TARGET;
        self
    }

    pub const fn get_final_target(&self) -> bool {
        (self.inner & SplineFlag::FINAL_TARGET) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_final_target(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_TARGET.reverse_bits();
        self
    }

    pub const fn new_final_angle() -> Self {
        Self {
            inner: SplineFlag::FINAL_ANGLE,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_final_angle(mut self) -> Self {
        self.inner |= SplineFlag::FINAL_ANGLE;
        self
    }

    pub const fn get_final_angle(&self) -> bool {
        (self.inner & SplineFlag::FINAL_ANGLE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_final_angle(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_ANGLE.reverse_bits();
        self
    }

    pub const fn new_unknown19() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN19,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown19(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN19;
        self
    }

    pub const fn get_unknown19(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN19) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown19(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN19.reverse_bits();
        self
    }

    pub const fn new_cyclic() -> Self {
        Self {
            inner: SplineFlag::CYCLIC,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_cyclic(mut self) -> Self {
        self.inner |= SplineFlag::CYCLIC;
        self
    }

    pub const fn get_cyclic(&self) -> bool {
        (self.inner & SplineFlag::CYCLIC) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_cyclic(mut self) -> Self {
        self.inner &= SplineFlag::CYCLIC.reverse_bits();
        self
    }

    pub const fn new_enter_cycle(enter_cycle: SMSG_MONSTER_MOVE_SplineFlag_EnterCycle) -> Self {
        Self {
            inner: SplineFlag::ENTER_CYCLE,
            parabolic: None,
            enter_cycle: Some(enter_cycle),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_enter_cycle(mut self, enter_cycle: SMSG_MONSTER_MOVE_SplineFlag_EnterCycle) -> Self {
        self.inner |= SplineFlag::ENTER_CYCLE;
        self.enter_cycle = Some(enter_cycle);
        self
    }

    pub const fn get_enter_cycle(&self) -> Option<&SMSG_MONSTER_MOVE_SplineFlag_EnterCycle> {
        self.enter_cycle.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_enter_cycle(mut self) -> Self {
        self.inner &= SplineFlag::ENTER_CYCLE.reverse_bits();
        self.enter_cycle = None;
        self
    }

    pub const fn new_frozen() -> Self {
        Self {
            inner: SplineFlag::FROZEN,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_frozen(mut self) -> Self {
        self.inner |= SplineFlag::FROZEN;
        self
    }

    pub const fn get_frozen(&self) -> bool {
        (self.inner & SplineFlag::FROZEN) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_frozen(mut self) -> Self {
        self.inner &= SplineFlag::FROZEN.reverse_bits();
        self
    }

    pub const fn new_unknown23() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN23,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown23(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN23;
        self
    }

    pub const fn get_unknown23(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN23) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown23(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN23.reverse_bits();
        self
    }

    pub const fn new_unknown24() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN24,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown24(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN24;
        self
    }

    pub const fn get_unknown24(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN24) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown24(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN24.reverse_bits();
        self
    }

    pub const fn new_unknown25() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN25,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown25(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN25;
        self
    }

    pub const fn get_unknown25(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN25) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown25(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN25.reverse_bits();
        self
    }

    pub const fn new_unknown26() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN26,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown26(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN26;
        self
    }

    pub const fn get_unknown26(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN26) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown26(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN26.reverse_bits();
        self
    }

    pub const fn new_unknown27() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN27,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown27(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN27;
        self
    }

    pub const fn get_unknown27(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN27) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown27(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN27.reverse_bits();
        self
    }

    pub const fn new_unknown28() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN28,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown28(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN28;
        self
    }

    pub const fn get_unknown28(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN28) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown28(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN28.reverse_bits();
        self
    }

    pub const fn new_unknown29() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN29,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown29(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN29;
        self
    }

    pub const fn get_unknown29(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN29) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown29(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN29.reverse_bits();
        self
    }

    pub const fn new_unknown30() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN30,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown30(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN30;
        self
    }

    pub const fn get_unknown30(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN30) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown30(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN30.reverse_bits();
        self
    }

    pub const fn new_unknown31() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN31,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown31(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN31;
        self
    }

    pub const fn get_unknown31(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN31) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown31(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN31.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_MONSTER_MOVE_SplineFlag {
    pub(crate) const fn size(&self) -> usize {
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
pub struct SMSG_MONSTER_MOVE_SplineFlag_Parabolic {
    pub effect_start_time: u32,
    pub vertical_acceleration: f32,
}

impl SMSG_MONSTER_MOVE_SplineFlag_Parabolic {
    pub(crate) const fn size(&self) -> usize {
        4 // effect_start_time: u32
        + 4 // vertical_acceleration: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_MONSTER_MOVE_SplineFlag_EnterCycle {
    pub animation_id: u32,
    pub animation_start_time: u32,
}

impl SMSG_MONSTER_MOVE_SplineFlag_EnterCycle {
    pub(crate) const fn size(&self) -> usize {
        4 // animation_id: u32
        + 4 // animation_start_time: u32
    }
}

