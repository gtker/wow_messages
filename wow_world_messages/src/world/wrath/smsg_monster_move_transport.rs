use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MonsterMoveType, SplineFlag, Vector3d,
};

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
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_MONSTER_MOVE_TRANSPORT {
    pub guid: Guid,
    pub transport: Guid,
    /// cmangos-wotlk sets to 0
    pub unknown: u8,
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType,
    pub spline_flags: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag,
    pub duration: u32,
    pub splines: Vec<Vector3d>,
}

impl crate::private::Sealed for SMSG_MONSTER_MOVE_TRANSPORT {}
impl SMSG_MONSTER_MOVE_TRANSPORT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(32..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // transport: PackedGuid
        let transport = crate::util::read_packed_guid(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // spline_point: Vector3d
        let spline_point = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(&mut r)?;

        // move_type: MonsterMoveType
        let move_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        let move_type_if = match move_type {
            MonsterMoveType::Normal => SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::Normal,
            MonsterMoveType::Stop => SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::Stop,
            MonsterMoveType::FacingSpot => {
                // position: Vector3d
                let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

                SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingSpot {
                    position,
                }
            }
            MonsterMoveType::FacingTarget => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

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

        let spline_flags_enter_cycle = if spline_flags.is_enter_cycle() {
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

        let spline_flags_parabolic = if spline_flags.is_parabolic() {
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
        let splines = crate::util::read_monster_move_spline(&mut r)?;

        let spline_flags = SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag {
            inner: spline_flags.as_int(),
            parabolic: spline_flags_parabolic,
            enter_cycle: spline_flags_enter_cycle,
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

impl crate::Message for SMSG_MONSTER_MOVE_TRANSPORT {
    const OPCODE: u32 = 0x02ae;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MONSTER_MOVE_TRANSPORT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MONSTER_MOVE_TRANSPORT {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    transport = {};", self.transport.guid()).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();
        // spline_point: Vector3d
        writeln!(s, "    spline_point = {{").unwrap();
        // Members
        writeln!(s, "        x = {};", if self.spline_point.x.to_string().contains('.') { self.spline_point.x.to_string() } else { format!("{}.0", self.spline_point.x) }).unwrap();
        writeln!(s, "        y = {};", if self.spline_point.y.to_string().contains('.') { self.spline_point.y.to_string() } else { format!("{}.0", self.spline_point.y) }).unwrap();
        writeln!(s, "        z = {};", if self.spline_point.z.to_string().contains('.') { self.spline_point.z.to_string() } else { format!("{}.0", self.spline_point.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    spline_id = {};", self.spline_id).unwrap();
        writeln!(s, "    move_type = {};", MonsterMoveType::try_from(self.move_type.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.move_type {
            crate::wrath::SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingSpot {
                position,
            } => {
                // position: Vector3d
                writeln!(s, "    position = {{").unwrap();
                // Members
                writeln!(s, "        x = {};", if position.x.to_string().contains('.') { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                writeln!(s, "        y = {};", if position.y.to_string().contains('.') { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                writeln!(s, "        z = {};", if position.z.to_string().contains('.') { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                writeln!(s, "    }};").unwrap();
            }
            crate::wrath::SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingTarget {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::wrath::SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingAngle {
                angle,
            } => {
                writeln!(s, "    angle = {};", if angle.to_string().contains('.') { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
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
            writeln!(s, "    vertical_acceleration = {};", if if_statement.vertical_acceleration.to_string().contains('.') { if_statement.vertical_acceleration.to_string() } else { format!("{}.0", if_statement.vertical_acceleration) }).unwrap();
            writeln!(s, "    effect_start_time = {};", if_statement.effect_start_time).unwrap();
        }

        writeln!(s, "    splines = [").unwrap();
        for v in self.splines.as_slice() {
            writeln!(s, "        {{").unwrap();
            writeln!(s, "            x = {};", if v.x.to_string().contains('.') { v.x.to_string() } else { format!("{}.0", v.x) }).unwrap();
            writeln!(s, "            y = {};", if v.y.to_string().contains('.') { v.y.to_string() } else { format!("{}.0", v.y) }).unwrap();
            writeln!(s, "            z = {};", if v.z.to_string().contains('.') { v.z.to_string() } else { format!("{}.0", v.z) }).unwrap();
            writeln!(s, "        }},").unwrap();
        }

        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 686_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.transport), "transport", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");
        writeln!(s, "    /* spline_point: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* spline_point: Vector3d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "move_type", "    ");
        match &self.move_type {
            crate::wrath::SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingSpot {
                position,
            } => {
                writeln!(s, "    /* position: Vector3d start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
                writeln!(s, "    /* position: Vector3d end */").unwrap();
            }
            crate::wrath::SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingTarget {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::wrath::SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingAngle {
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

        crate::util::write_bytes(&mut s, &mut bytes, 4, "splines_length", "    ");
        if !self.splines.is_empty() {
            writeln!(s, "    /* splines: MonsterMoveSplines start */").unwrap();
            let mut v = self.splines.iter();
            let _ = v.next().unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "splines_x", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "splines_y", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "splines_z", "    ");
            for v in v {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "splines_packed", "    ");
            }

            writeln!(s, "    /* splines: MonsterMoveSplines end */").unwrap();
        }


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

        // transport: PackedGuid
        crate::util::write_packed_guid(&self.transport, &mut w)?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // spline_point: Vector3d
        crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.spline_point, &mut w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int().to_le_bytes()))?;

        match &self.move_type {
            SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType::FacingSpot {
                position,
            } => {
                // position: Vector3d
                crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&position, &mut w)?;

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
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(686, "SMSG_MONSTER_MOVE_TRANSPORT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MONSTER_MOVE_TRANSPORT {}

impl SMSG_MONSTER_MOVE_TRANSPORT {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + crate::util::packed_guid_size(&self.transport) // transport: PackedGuid
        + 1 // unknown: u8
        + 12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + self.move_type.size() // move_type: SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType
        + self.spline_flags.size() // spline_flags: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag
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

impl std::fmt::Display for SMSG_MONSTER_MOVE_TRANSPORT_MonsterMoveType {
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

    pub const fn new_parabolic(parabolic: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic) -> Self {
        Self {
            inner: SplineFlag::PARABOLIC,
            parabolic: Some(parabolic),
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_parabolic(mut self, parabolic: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic) -> Self {
        self.inner |= SplineFlag::PARABOLIC;
        self.parabolic = Some(parabolic);
        self
    }

    pub const fn get_parabolic(&self) -> Option<&SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_Parabolic> {
        self.parabolic.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_parabolic(mut self) -> Self {
        self.inner &= SplineFlag::PARABOLIC.reverse_bits();
        self.parabolic = None;
        self
    }

    pub const fn new_walk_mode() -> Self {
        Self {
            inner: SplineFlag::WALK_MODE,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_walk_mode(mut self) -> Self {
        self.inner |= SplineFlag::WALK_MODE;
        self
    }

    pub const fn get_walk_mode(&self) -> bool {
        (self.inner & SplineFlag::WALK_MODE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_walk_mode(mut self) -> Self {
        self.inner &= SplineFlag::WALK_MODE.reverse_bits();
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

    pub const fn new_orientation_fixed() -> Self {
        Self {
            inner: SplineFlag::ORIENTATION_FIXED,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_orientation_fixed(mut self) -> Self {
        self.inner |= SplineFlag::ORIENTATION_FIXED;
        self
    }

    pub const fn get_orientation_fixed(&self) -> bool {
        (self.inner & SplineFlag::ORIENTATION_FIXED) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_orientation_fixed(mut self) -> Self {
        self.inner &= SplineFlag::ORIENTATION_FIXED.reverse_bits();
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

    pub const fn new_catmullrom() -> Self {
        Self {
            inner: SplineFlag::CATMULLROM,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_catmullrom(mut self) -> Self {
        self.inner |= SplineFlag::CATMULLROM;
        self
    }

    pub const fn get_catmullrom(&self) -> bool {
        (self.inner & SplineFlag::CATMULLROM) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_catmullrom(mut self) -> Self {
        self.inner &= SplineFlag::CATMULLROM.reverse_bits();
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

    pub const fn new_enter_cycle(enter_cycle: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle) -> Self {
        Self {
            inner: SplineFlag::ENTER_CYCLE,
            parabolic: None,
            enter_cycle: Some(enter_cycle),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_enter_cycle(mut self, enter_cycle: SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle) -> Self {
        self.inner |= SplineFlag::ENTER_CYCLE;
        self.enter_cycle = Some(enter_cycle);
        self
    }

    pub const fn get_enter_cycle(&self) -> Option<&SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle> {
        self.enter_cycle.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_enter_cycle(mut self) -> Self {
        self.inner &= SplineFlag::ENTER_CYCLE.reverse_bits();
        self.enter_cycle = None;
        self
    }

    pub const fn new_animation() -> Self {
        Self {
            inner: SplineFlag::ANIMATION,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_animation(mut self) -> Self {
        self.inner |= SplineFlag::ANIMATION;
        self
    }

    pub const fn get_animation(&self) -> bool {
        (self.inner & SplineFlag::ANIMATION) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_animation(mut self) -> Self {
        self.inner &= SplineFlag::ANIMATION.reverse_bits();
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

    pub const fn new_transport_enter() -> Self {
        Self {
            inner: SplineFlag::TRANSPORT_ENTER,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_transport_enter(mut self) -> Self {
        self.inner |= SplineFlag::TRANSPORT_ENTER;
        self
    }

    pub const fn get_transport_enter(&self) -> bool {
        (self.inner & SplineFlag::TRANSPORT_ENTER) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_transport_enter(mut self) -> Self {
        self.inner &= SplineFlag::TRANSPORT_ENTER.reverse_bits();
        self
    }

    pub const fn new_transport_exit() -> Self {
        Self {
            inner: SplineFlag::TRANSPORT_EXIT,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_transport_exit(mut self) -> Self {
        self.inner |= SplineFlag::TRANSPORT_EXIT;
        self
    }

    pub const fn get_transport_exit(&self) -> bool {
        (self.inner & SplineFlag::TRANSPORT_EXIT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_transport_exit(mut self) -> Self {
        self.inner &= SplineFlag::TRANSPORT_EXIT.reverse_bits();
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

    pub const fn new_orientation_inversed() -> Self {
        Self {
            inner: SplineFlag::ORIENTATION_INVERSED,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_orientation_inversed(mut self) -> Self {
        self.inner |= SplineFlag::ORIENTATION_INVERSED;
        self
    }

    pub const fn get_orientation_inversed(&self) -> bool {
        (self.inner & SplineFlag::ORIENTATION_INVERSED) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_orientation_inversed(mut self) -> Self {
        self.inner &= SplineFlag::ORIENTATION_INVERSED.reverse_bits();
        self
    }

    pub const fn new_unknown10() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN10,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown10(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN10;
        self
    }

    pub const fn get_unknown10(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN10) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown10(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN10.reverse_bits();
        self
    }

    pub const fn new_unknown11() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN11,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown11(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN11;
        self
    }

    pub const fn get_unknown11(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN11) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown11(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN11.reverse_bits();
        self
    }

    pub const fn new_unknown12() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN12,
            parabolic: None,
            enter_cycle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown12(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN12;
        self
    }

    pub const fn get_unknown12(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN12) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown12(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN12.reverse_bits();
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

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag {
    pub(crate) const fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.parabolic {
                8
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.enter_cycle {
                8
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_MONSTER_MOVE_TRANSPORT_SplineFlag_EnterCycle {
    pub animation_id: u32,
    pub animation_start_time: u32,
}

