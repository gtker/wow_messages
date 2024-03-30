use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    MovementFlags, TransportInfo, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_2_4_3.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_2_4_3.wowm#L32):
/// ```text
/// struct MovementInfo {
///     MovementFlags flags;
///     u8 extra_flags;
///     u32 timestamp;
///     Vector3d position;
///     f32 orientation;
///     if (flags & ON_TRANSPORT) {
///         TransportInfo transport;
///     }
///     if (flags & SWIMMING) {
///         f32 pitch1;
///     }
///     else if (flags & ONTRANSPORT) {
///         f32 pitch2;
///     }
///     f32 fall_time;
///     if (flags & JUMPING) {
///         f32 z_speed;
///         f32 cos_angle;
///         f32 sin_angle;
///         f32 xy_speed;
///     }
///     if (flags & SPLINE_ELEVATION) {
///         f32 spline_elevation;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MovementInfo {
    pub flags: MovementInfo_MovementFlags,
    pub extra_flags: u8,
    pub timestamp: u32,
    pub position: Vector3d,
    pub orientation: f32,
    pub fall_time: f32,
}

impl MovementInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // flags: MovementFlags
        w.write_all(&(self.flags.as_int().to_le_bytes()))?;

        // extra_flags: u8
        w.write_all(&self.extra_flags.to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        // position: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.position, &mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        if let Some(if_statement) = &self.flags.on_transport {
            // transport: TransportInfo
            if_statement.transport.write_into_vec(&mut w)?;

        }

        if let Some(if_statement) = &self.flags.swimming {
            match if_statement {
                MovementInfo_MovementFlags_Swimming::Swimming {
                    pitch1,
                } => {
                    // pitch1: f32
                    w.write_all(&pitch1.to_le_bytes())?;

                }
                MovementInfo_MovementFlags_Swimming::Ontransport {
                    pitch2,
                } => {
                    // pitch2: f32
                    w.write_all(&pitch2.to_le_bytes())?;

                }
            }
        }

        // fall_time: f32
        w.write_all(&self.fall_time.to_le_bytes())?;

        if let Some(if_statement) = &self.flags.jumping {
            // z_speed: f32
            w.write_all(&if_statement.z_speed.to_le_bytes())?;

            // cos_angle: f32
            w.write_all(&if_statement.cos_angle.to_le_bytes())?;

            // sin_angle: f32
            w.write_all(&if_statement.sin_angle.to_le_bytes())?;

            // xy_speed: f32
            w.write_all(&if_statement.xy_speed.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.spline_elevation {
            // spline_elevation: f32
            w.write_all(&if_statement.spline_elevation.to_le_bytes())?;

        }

        Ok(())
    }
}

impl MovementInfo {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // flags: MovementFlags
        let flags = MovementFlags::new(crate::util::read_u32_le(&mut r)?);

        // extra_flags: u8
        let extra_flags = crate::util::read_u8_le(&mut r)?;

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(&mut r)?;

        // position: Vector3d
        let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        let flags_on_transport = if flags.is_on_transport() {
            // transport: TransportInfo
            let transport = TransportInfo::read(&mut r)?;

            Some(MovementInfo_MovementFlags_OnTransport {
                transport,
            })
        }
        else {
            None
        };

        let flags_swimming = if flags.is_swimming() {
            // pitch1: f32
            let pitch1 = crate::util::read_f32_le(&mut r)?;

            Some(MovementInfo_MovementFlags_Swimming::Swimming {
                pitch1,
            })
        }
        else if flags.is_ontransport() {
            // pitch2: f32
            let pitch2 = crate::util::read_f32_le(&mut r)?;

            Some(MovementInfo_MovementFlags_Swimming::Ontransport {
                pitch2,
            })
        }
        else {
            None
        };

        // fall_time: f32
        let fall_time = crate::util::read_f32_le(&mut r)?;

        let flags_jumping = if flags.is_jumping() {
            // z_speed: f32
            let z_speed = crate::util::read_f32_le(&mut r)?;

            // cos_angle: f32
            let cos_angle = crate::util::read_f32_le(&mut r)?;

            // sin_angle: f32
            let sin_angle = crate::util::read_f32_le(&mut r)?;

            // xy_speed: f32
            let xy_speed = crate::util::read_f32_le(&mut r)?;

            Some(MovementInfo_MovementFlags_Jumping {
                cos_angle,
                sin_angle,
                xy_speed,
                z_speed,
            })
        }
        else {
            None
        };

        let flags_spline_elevation = if flags.is_spline_elevation() {
            // spline_elevation: f32
            let spline_elevation = crate::util::read_f32_le(&mut r)?;

            Some(MovementInfo_MovementFlags_SplineElevation {
                spline_elevation,
            })
        }
        else {
            None
        };

        let flags = MovementInfo_MovementFlags {
            inner: flags.as_int(),
            on_transport: flags_on_transport,
            jumping: flags_jumping,
            swimming: flags_swimming,
            spline_elevation: flags_spline_elevation,
        };

        Ok(Self {
            flags,
            extra_flags,
            timestamp,
            position,
            orientation,
            fall_time,
        })
    }

}

impl MovementInfo {
    pub(crate) const fn size(&self) -> usize {
        self.flags.size() // flags: MovementInfo_MovementFlags
        + 1 // extra_flags: u8
        + 4 // timestamp: u32
        + 12 // position: Vector3d
        + 4 // orientation: f32
        + 4 // fall_time: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MovementInfo_MovementFlags_Swimming {
    Swimming {
        pitch1: f32,
    },
    Ontransport {
        pitch2: f32,
    },
}

impl MovementInfo_MovementFlags_Swimming {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Swimming { .. } => 2097152,
            Self::Ontransport { .. } => 33554432,
        }
    }

}

impl std::fmt::Display for MovementInfo_MovementFlags_Swimming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Swimming{ .. } => f.write_str("Swimming"),
            Self::Ontransport{ .. } => f.write_str("Ontransport"),
        }
    }
}

impl MovementInfo_MovementFlags_Swimming {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Swimming {
                ..
            } => {
                // Not an actual enum sent over the wire
                4 // pitch1: f32
            }
            Self::Ontransport {
                ..
            } => {
                // Not an actual enum sent over the wire
                4 // pitch2: f32
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MovementInfo_MovementFlags {
    inner: u32,
    on_transport: Option<MovementInfo_MovementFlags_OnTransport>,
    jumping: Option<MovementInfo_MovementFlags_Jumping>,
    swimming: Option<MovementInfo_MovementFlags_Swimming>,
    spline_elevation: Option<MovementInfo_MovementFlags_SplineElevation>,
}

impl MovementInfo_MovementFlags {
    pub const fn new(inner: u32, on_transport: Option<MovementInfo_MovementFlags_OnTransport>,jumping: Option<MovementInfo_MovementFlags_Jumping>,swimming: Option<MovementInfo_MovementFlags_Swimming>,spline_elevation: Option<MovementInfo_MovementFlags_SplineElevation>,) -> Self {
        Self {
            inner,
            on_transport, 
            jumping, 
            swimming, 
            spline_elevation, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.on_transport.is_none()
        && self.jumping.is_none()
        && self.swimming.is_none()
        && self.spline_elevation.is_none()
    }

    pub const fn new_forward() -> Self {
        Self {
            inner: MovementFlags::FORWARD,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_forward(mut self) -> Self {
        self.inner |= MovementFlags::FORWARD;
        self
    }

    pub const fn get_forward(&self) -> bool {
        (self.inner & MovementFlags::FORWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_forward(mut self) -> Self {
        self.inner &= MovementFlags::FORWARD.reverse_bits();
        self
    }

    pub const fn new_backward() -> Self {
        Self {
            inner: MovementFlags::BACKWARD,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_backward(mut self) -> Self {
        self.inner |= MovementFlags::BACKWARD;
        self
    }

    pub const fn get_backward(&self) -> bool {
        (self.inner & MovementFlags::BACKWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_backward(mut self) -> Self {
        self.inner &= MovementFlags::BACKWARD.reverse_bits();
        self
    }

    pub const fn new_strafe_left() -> Self {
        Self {
            inner: MovementFlags::STRAFE_LEFT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_strafe_left(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_LEFT;
        self
    }

    pub const fn get_strafe_left(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_LEFT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_strafe_left(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_LEFT.reverse_bits();
        self
    }

    pub const fn new_strafe_right() -> Self {
        Self {
            inner: MovementFlags::STRAFE_RIGHT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_strafe_right(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_RIGHT;
        self
    }

    pub const fn get_strafe_right(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_RIGHT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_strafe_right(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_RIGHT.reverse_bits();
        self
    }

    pub const fn new_turn_left() -> Self {
        Self {
            inner: MovementFlags::TURN_LEFT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_turn_left(mut self) -> Self {
        self.inner |= MovementFlags::TURN_LEFT;
        self
    }

    pub const fn get_turn_left(&self) -> bool {
        (self.inner & MovementFlags::TURN_LEFT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_turn_left(mut self) -> Self {
        self.inner &= MovementFlags::TURN_LEFT.reverse_bits();
        self
    }

    pub const fn new_turn_right() -> Self {
        Self {
            inner: MovementFlags::TURN_RIGHT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_turn_right(mut self) -> Self {
        self.inner |= MovementFlags::TURN_RIGHT;
        self
    }

    pub const fn get_turn_right(&self) -> bool {
        (self.inner & MovementFlags::TURN_RIGHT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_turn_right(mut self) -> Self {
        self.inner &= MovementFlags::TURN_RIGHT.reverse_bits();
        self
    }

    pub const fn new_pitch_up() -> Self {
        Self {
            inner: MovementFlags::PITCH_UP,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pitch_up(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_UP;
        self
    }

    pub const fn get_pitch_up(&self) -> bool {
        (self.inner & MovementFlags::PITCH_UP) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pitch_up(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_UP.reverse_bits();
        self
    }

    pub const fn new_pitch_down() -> Self {
        Self {
            inner: MovementFlags::PITCH_DOWN,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pitch_down(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_DOWN;
        self
    }

    pub const fn get_pitch_down(&self) -> bool {
        (self.inner & MovementFlags::PITCH_DOWN) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pitch_down(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_DOWN.reverse_bits();
        self
    }

    pub const fn new_walk_mode() -> Self {
        Self {
            inner: MovementFlags::WALK_MODE,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_walk_mode(mut self) -> Self {
        self.inner |= MovementFlags::WALK_MODE;
        self
    }

    pub const fn get_walk_mode(&self) -> bool {
        (self.inner & MovementFlags::WALK_MODE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_walk_mode(mut self) -> Self {
        self.inner &= MovementFlags::WALK_MODE.reverse_bits();
        self
    }

    pub const fn new_on_transport(on_transport: MovementInfo_MovementFlags_OnTransport) -> Self {
        Self {
            inner: MovementFlags::ON_TRANSPORT,
            on_transport: Some(on_transport),
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_on_transport(mut self, on_transport: MovementInfo_MovementFlags_OnTransport) -> Self {
        self.inner |= MovementFlags::ON_TRANSPORT;
        self.on_transport = Some(on_transport);
        self
    }

    pub const fn get_on_transport(&self) -> Option<&MovementInfo_MovementFlags_OnTransport> {
        self.on_transport.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_on_transport(mut self) -> Self {
        self.inner &= MovementFlags::ON_TRANSPORT.reverse_bits();
        self.on_transport = None;
        self
    }

    pub const fn new_levitating() -> Self {
        Self {
            inner: MovementFlags::LEVITATING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_levitating(mut self) -> Self {
        self.inner |= MovementFlags::LEVITATING;
        self
    }

    pub const fn get_levitating(&self) -> bool {
        (self.inner & MovementFlags::LEVITATING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_levitating(mut self) -> Self {
        self.inner &= MovementFlags::LEVITATING.reverse_bits();
        self
    }

    pub const fn new_fixed_z() -> Self {
        Self {
            inner: MovementFlags::FIXED_Z,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_fixed_z(mut self) -> Self {
        self.inner |= MovementFlags::FIXED_Z;
        self
    }

    pub const fn get_fixed_z(&self) -> bool {
        (self.inner & MovementFlags::FIXED_Z) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_fixed_z(mut self) -> Self {
        self.inner &= MovementFlags::FIXED_Z.reverse_bits();
        self
    }

    pub const fn new_root() -> Self {
        Self {
            inner: MovementFlags::ROOT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_root(mut self) -> Self {
        self.inner |= MovementFlags::ROOT;
        self
    }

    pub const fn get_root(&self) -> bool {
        (self.inner & MovementFlags::ROOT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_root(mut self) -> Self {
        self.inner &= MovementFlags::ROOT.reverse_bits();
        self
    }

    pub const fn new_jumping(jumping: MovementInfo_MovementFlags_Jumping) -> Self {
        Self {
            inner: MovementFlags::JUMPING,
            on_transport: None,
            jumping: Some(jumping),
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_jumping(mut self, jumping: MovementInfo_MovementFlags_Jumping) -> Self {
        self.inner |= MovementFlags::JUMPING;
        self.jumping = Some(jumping);
        self
    }

    pub const fn get_jumping(&self) -> Option<&MovementInfo_MovementFlags_Jumping> {
        self.jumping.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_jumping(mut self) -> Self {
        self.inner &= MovementFlags::JUMPING.reverse_bits();
        self.jumping = None;
        self
    }

    pub const fn new_fallingfar() -> Self {
        Self {
            inner: MovementFlags::FALLINGFAR,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_fallingfar(mut self) -> Self {
        self.inner |= MovementFlags::FALLINGFAR;
        self
    }

    pub const fn get_fallingfar(&self) -> bool {
        (self.inner & MovementFlags::FALLINGFAR) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_fallingfar(mut self) -> Self {
        self.inner &= MovementFlags::FALLINGFAR.reverse_bits();
        self
    }

    pub const fn new_swimming(swimming: MovementInfo_MovementFlags_Swimming) -> Self {
        Self {
            inner: swimming.as_int(),
            on_transport: None,
            jumping: None,
            swimming: Some(swimming),
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_swimming(mut self, swimming: MovementInfo_MovementFlags_Swimming) -> Self {
        self.inner |= swimming.as_int();
        self.swimming = Some(swimming);
        self
    }

    pub const fn get_swimming(&self) -> Option<&MovementInfo_MovementFlags_Swimming> {
        self.swimming.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_swimming(mut self) -> Self {
        self.inner &= MovementFlags::SWIMMING.reverse_bits();
        self.swimming = None;
        self
    }

    pub const fn new_ascending() -> Self {
        Self {
            inner: MovementFlags::ASCENDING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ascending(mut self) -> Self {
        self.inner |= MovementFlags::ASCENDING;
        self
    }

    pub const fn get_ascending(&self) -> bool {
        (self.inner & MovementFlags::ASCENDING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ascending(mut self) -> Self {
        self.inner &= MovementFlags::ASCENDING.reverse_bits();
        self
    }

    pub const fn new_can_fly() -> Self {
        Self {
            inner: MovementFlags::CAN_FLY,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_can_fly(mut self) -> Self {
        self.inner |= MovementFlags::CAN_FLY;
        self
    }

    pub const fn get_can_fly(&self) -> bool {
        (self.inner & MovementFlags::CAN_FLY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_can_fly(mut self) -> Self {
        self.inner &= MovementFlags::CAN_FLY.reverse_bits();
        self
    }

    pub const fn new_flying() -> Self {
        Self {
            inner: MovementFlags::FLYING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_flying(mut self) -> Self {
        self.inner |= MovementFlags::FLYING;
        self
    }

    pub const fn get_flying(&self) -> bool {
        (self.inner & MovementFlags::FLYING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_flying(mut self) -> Self {
        self.inner &= MovementFlags::FLYING.reverse_bits();
        self
    }

    pub const fn new_spline_elevation(spline_elevation: MovementInfo_MovementFlags_SplineElevation) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ELEVATION,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: Some(spline_elevation),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_spline_elevation(mut self, spline_elevation: MovementInfo_MovementFlags_SplineElevation) -> Self {
        self.inner |= MovementFlags::SPLINE_ELEVATION;
        self.spline_elevation = Some(spline_elevation);
        self
    }

    pub const fn get_spline_elevation(&self) -> Option<&MovementInfo_MovementFlags_SplineElevation> {
        self.spline_elevation.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_spline_elevation(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ELEVATION.reverse_bits();
        self.spline_elevation = None;
        self
    }

    pub const fn new_spline_enabled() -> Self {
        Self {
            inner: MovementFlags::SPLINE_ENABLED,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_spline_enabled(mut self) -> Self {
        self.inner |= MovementFlags::SPLINE_ENABLED;
        self
    }

    pub const fn get_spline_enabled(&self) -> bool {
        (self.inner & MovementFlags::SPLINE_ENABLED) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_spline_enabled(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ENABLED.reverse_bits();
        self
    }

    pub const fn new_waterwalking() -> Self {
        Self {
            inner: MovementFlags::WATERWALKING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_waterwalking(mut self) -> Self {
        self.inner |= MovementFlags::WATERWALKING;
        self
    }

    pub const fn get_waterwalking(&self) -> bool {
        (self.inner & MovementFlags::WATERWALKING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_waterwalking(mut self) -> Self {
        self.inner &= MovementFlags::WATERWALKING.reverse_bits();
        self
    }

    pub const fn new_safe_fall() -> Self {
        Self {
            inner: MovementFlags::SAFE_FALL,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_safe_fall(mut self) -> Self {
        self.inner |= MovementFlags::SAFE_FALL;
        self
    }

    pub const fn get_safe_fall(&self) -> bool {
        (self.inner & MovementFlags::SAFE_FALL) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_safe_fall(mut self) -> Self {
        self.inner &= MovementFlags::SAFE_FALL.reverse_bits();
        self
    }

    pub const fn new_hover() -> Self {
        Self {
            inner: MovementFlags::HOVER,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_hover(mut self) -> Self {
        self.inner |= MovementFlags::HOVER;
        self
    }

    pub const fn get_hover(&self) -> bool {
        (self.inner & MovementFlags::HOVER) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_hover(mut self) -> Self {
        self.inner &= MovementFlags::HOVER.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl MovementInfo_MovementFlags {
    pub(crate) const fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.on_transport {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.jumping {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.swimming {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.spline_elevation {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MovementInfo_MovementFlags_OnTransport {
    pub transport: TransportInfo,
}

impl MovementInfo_MovementFlags_OnTransport {
    pub(crate) const fn size(&self) -> usize {
        self.transport.size() // transport: TransportInfo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MovementInfo_MovementFlags_Jumping {
    pub cos_angle: f32,
    pub sin_angle: f32,
    pub xy_speed: f32,
    pub z_speed: f32,
}

impl MovementInfo_MovementFlags_Jumping {
    pub(crate) const fn size(&self) -> usize {
        4 // cos_angle: f32
        + 4 // sin_angle: f32
        + 4 // xy_speed: f32
        + 4 // z_speed: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MovementInfo_MovementFlags_SplineElevation {
    pub spline_elevation: f32,
}

impl MovementInfo_MovementFlags_SplineElevation {
    pub(crate) const fn size(&self) -> usize {
        4 // spline_elevation: f32
    }
}

