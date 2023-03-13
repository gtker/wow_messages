use std::io::{Read, Write};

use crate::vanilla::{
    MovementFlags, TransportInfo, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement.wowm#L40):
/// ```text
/// struct MovementInfo {
///     MovementFlags flags;
///     u32 timestamp;
///     Vector3d position;
///     f32 orientation;
///     if (flags & ON_TRANSPORT) {
///         TransportInfo transport;
///     }
///     if (flags & SWIMMING) {
///         f32 pitch;
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
pub struct MovementInfo {
    pub flags: MovementInfo_MovementFlags,
    pub timestamp: u32,
    pub position: Vector3d,
    pub orientation: f32,
    pub fall_time: f32,
}

impl MovementInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // flags: MovementFlags
        w.write_all(&u32::from(self.flags.as_int()).to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        if let Some(if_statement) = &self.flags.on_transport {
            // transport: TransportInfo
            if_statement.transport.write_into_vec(&mut w)?;

        }

        if let Some(if_statement) = &self.flags.swimming {
            // pitch: f32
            w.write_all(&if_statement.pitch.to_le_bytes())?;

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
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // flags: MovementFlags
        let flags = MovementFlags::new(crate::util::read_u32_le(&mut r)?);

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(&mut r)?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        let flags_ON_TRANSPORT = if flags.is_ON_TRANSPORT() {
            // transport: TransportInfo
            let transport = TransportInfo::read(&mut r)?;

            Some(MovementInfo_MovementFlags_OnTransport {
                transport,
            })
        }
        else {
            None
        };

        let flags_SWIMMING = if flags.is_SWIMMING() {
            // pitch: f32
            let pitch = crate::util::read_f32_le(&mut r)?;

            Some(MovementInfo_MovementFlags_Swimming {
                pitch,
            })
        }
        else {
            None
        };

        // fall_time: f32
        let fall_time = crate::util::read_f32_le(&mut r)?;

        let flags_JUMPING = if flags.is_JUMPING() {
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

        let flags_SPLINE_ELEVATION = if flags.is_SPLINE_ELEVATION() {
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
            on_transport: flags_ON_TRANSPORT,
            jumping: flags_JUMPING,
            swimming: flags_SWIMMING,
            spline_elevation: flags_SPLINE_ELEVATION,
        };

        Ok(Self {
            flags,
            timestamp,
            position,
            orientation,
            fall_time,
        })
    }

}

impl MovementInfo {
    pub(crate) fn size(&self) -> usize {
        self.flags.size() // flags: MovementInfo_MovementFlags
        + 4 // timestamp: u32
        + 12 // position: Vector3d
        + 4 // orientation: f32
        + 4 // fall_time: f32
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

    pub const fn new_FORWARD() -> Self {
        Self {
            inner: MovementFlags::FORWARD,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FORWARD(mut self) -> Self {
        self.inner |= MovementFlags::FORWARD;
        self
    }

    pub const fn get_FORWARD(&self) -> bool {
        (self.inner & MovementFlags::FORWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FORWARD(mut self) -> Self {
        self.inner &= MovementFlags::FORWARD.reverse_bits();
        self
    }

    pub const fn new_BACKWARD() -> Self {
        Self {
            inner: MovementFlags::BACKWARD,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_BACKWARD(mut self) -> Self {
        self.inner |= MovementFlags::BACKWARD;
        self
    }

    pub const fn get_BACKWARD(&self) -> bool {
        (self.inner & MovementFlags::BACKWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_BACKWARD(mut self) -> Self {
        self.inner &= MovementFlags::BACKWARD.reverse_bits();
        self
    }

    pub const fn new_STRAFE_LEFT() -> Self {
        Self {
            inner: MovementFlags::STRAFE_LEFT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_STRAFE_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_LEFT;
        self
    }

    pub const fn get_STRAFE_LEFT(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_LEFT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_STRAFE_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_LEFT.reverse_bits();
        self
    }

    pub const fn new_STRAFE_RIGHT() -> Self {
        Self {
            inner: MovementFlags::STRAFE_RIGHT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_STRAFE_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_RIGHT;
        self
    }

    pub const fn get_STRAFE_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_RIGHT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_STRAFE_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_RIGHT.reverse_bits();
        self
    }

    pub const fn new_TURN_LEFT() -> Self {
        Self {
            inner: MovementFlags::TURN_LEFT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_TURN_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::TURN_LEFT;
        self
    }

    pub const fn get_TURN_LEFT(&self) -> bool {
        (self.inner & MovementFlags::TURN_LEFT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_TURN_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::TURN_LEFT.reverse_bits();
        self
    }

    pub const fn new_TURN_RIGHT() -> Self {
        Self {
            inner: MovementFlags::TURN_RIGHT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_TURN_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::TURN_RIGHT;
        self
    }

    pub const fn get_TURN_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::TURN_RIGHT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_TURN_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::TURN_RIGHT.reverse_bits();
        self
    }

    pub const fn new_PITCH_UP() -> Self {
        Self {
            inner: MovementFlags::PITCH_UP,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_PITCH_UP(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_UP;
        self
    }

    pub const fn get_PITCH_UP(&self) -> bool {
        (self.inner & MovementFlags::PITCH_UP) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_PITCH_UP(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_UP.reverse_bits();
        self
    }

    pub const fn new_PITCH_DOWN() -> Self {
        Self {
            inner: MovementFlags::PITCH_DOWN,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_PITCH_DOWN(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_DOWN;
        self
    }

    pub const fn get_PITCH_DOWN(&self) -> bool {
        (self.inner & MovementFlags::PITCH_DOWN) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_PITCH_DOWN(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_DOWN.reverse_bits();
        self
    }

    pub const fn new_WALK_MODE() -> Self {
        Self {
            inner: MovementFlags::WALK_MODE,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_WALK_MODE(mut self) -> Self {
        self.inner |= MovementFlags::WALK_MODE;
        self
    }

    pub const fn get_WALK_MODE(&self) -> bool {
        (self.inner & MovementFlags::WALK_MODE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_WALK_MODE(mut self) -> Self {
        self.inner &= MovementFlags::WALK_MODE.reverse_bits();
        self
    }

    pub const fn new_ON_TRANSPORT(on_transport: MovementInfo_MovementFlags_OnTransport) -> Self {
        Self {
            inner: MovementFlags::ON_TRANSPORT,
            on_transport: Some(on_transport),
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ON_TRANSPORT(mut self, on_transport: MovementInfo_MovementFlags_OnTransport) -> Self {
        self.inner |= MovementFlags::ON_TRANSPORT;
        self.on_transport = Some(on_transport);
        self
    }

    pub const fn get_ON_TRANSPORT(&self) -> Option<&MovementInfo_MovementFlags_OnTransport> {
        self.on_transport.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ON_TRANSPORT(mut self) -> Self {
        self.inner &= MovementFlags::ON_TRANSPORT.reverse_bits();
        self.on_transport = None;
        self
    }

    pub const fn new_LEVITATING() -> Self {
        Self {
            inner: MovementFlags::LEVITATING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_LEVITATING(mut self) -> Self {
        self.inner |= MovementFlags::LEVITATING;
        self
    }

    pub const fn get_LEVITATING(&self) -> bool {
        (self.inner & MovementFlags::LEVITATING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_LEVITATING(mut self) -> Self {
        self.inner &= MovementFlags::LEVITATING.reverse_bits();
        self
    }

    pub const fn new_FIXED_Z() -> Self {
        Self {
            inner: MovementFlags::FIXED_Z,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FIXED_Z(mut self) -> Self {
        self.inner |= MovementFlags::FIXED_Z;
        self
    }

    pub const fn get_FIXED_Z(&self) -> bool {
        (self.inner & MovementFlags::FIXED_Z) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FIXED_Z(mut self) -> Self {
        self.inner &= MovementFlags::FIXED_Z.reverse_bits();
        self
    }

    pub const fn new_ROOT() -> Self {
        Self {
            inner: MovementFlags::ROOT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ROOT(mut self) -> Self {
        self.inner |= MovementFlags::ROOT;
        self
    }

    pub const fn get_ROOT(&self) -> bool {
        (self.inner & MovementFlags::ROOT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ROOT(mut self) -> Self {
        self.inner &= MovementFlags::ROOT.reverse_bits();
        self
    }

    pub const fn new_JUMPING(jumping: MovementInfo_MovementFlags_Jumping) -> Self {
        Self {
            inner: MovementFlags::JUMPING,
            on_transport: None,
            jumping: Some(jumping),
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_JUMPING(mut self, jumping: MovementInfo_MovementFlags_Jumping) -> Self {
        self.inner |= MovementFlags::JUMPING;
        self.jumping = Some(jumping);
        self
    }

    pub const fn get_JUMPING(&self) -> Option<&MovementInfo_MovementFlags_Jumping> {
        self.jumping.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_JUMPING(mut self) -> Self {
        self.inner &= MovementFlags::JUMPING.reverse_bits();
        self.jumping = None;
        self
    }

    pub const fn new_FALLINGFAR() -> Self {
        Self {
            inner: MovementFlags::FALLINGFAR,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FALLINGFAR(mut self) -> Self {
        self.inner |= MovementFlags::FALLINGFAR;
        self
    }

    pub const fn get_FALLINGFAR(&self) -> bool {
        (self.inner & MovementFlags::FALLINGFAR) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FALLINGFAR(mut self) -> Self {
        self.inner &= MovementFlags::FALLINGFAR.reverse_bits();
        self
    }

    pub const fn new_SWIMMING(swimming: MovementInfo_MovementFlags_Swimming) -> Self {
        Self {
            inner: MovementFlags::SWIMMING,
            on_transport: None,
            jumping: None,
            swimming: Some(swimming),
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_SWIMMING(mut self, swimming: MovementInfo_MovementFlags_Swimming) -> Self {
        self.inner |= MovementFlags::SWIMMING;
        self.swimming = Some(swimming);
        self
    }

    pub const fn get_SWIMMING(&self) -> Option<&MovementInfo_MovementFlags_Swimming> {
        self.swimming.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_SWIMMING(mut self) -> Self {
        self.inner &= MovementFlags::SWIMMING.reverse_bits();
        self.swimming = None;
        self
    }

    pub const fn new_SPLINE_ENABLED() -> Self {
        Self {
            inner: MovementFlags::SPLINE_ENABLED,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_SPLINE_ENABLED(mut self) -> Self {
        self.inner |= MovementFlags::SPLINE_ENABLED;
        self
    }

    pub const fn get_SPLINE_ENABLED(&self) -> bool {
        (self.inner & MovementFlags::SPLINE_ENABLED) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_SPLINE_ENABLED(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ENABLED.reverse_bits();
        self
    }

    pub const fn new_CAN_FLY() -> Self {
        Self {
            inner: MovementFlags::CAN_FLY,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_CAN_FLY(mut self) -> Self {
        self.inner |= MovementFlags::CAN_FLY;
        self
    }

    pub const fn get_CAN_FLY(&self) -> bool {
        (self.inner & MovementFlags::CAN_FLY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_CAN_FLY(mut self) -> Self {
        self.inner &= MovementFlags::CAN_FLY.reverse_bits();
        self
    }

    pub const fn new_FLYING() -> Self {
        Self {
            inner: MovementFlags::FLYING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FLYING(mut self) -> Self {
        self.inner |= MovementFlags::FLYING;
        self
    }

    pub const fn get_FLYING(&self) -> bool {
        (self.inner & MovementFlags::FLYING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FLYING(mut self) -> Self {
        self.inner &= MovementFlags::FLYING.reverse_bits();
        self
    }

    pub const fn new_ONTRANSPORT() -> Self {
        Self {
            inner: MovementFlags::ONTRANSPORT,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ONTRANSPORT(mut self) -> Self {
        self.inner |= MovementFlags::ONTRANSPORT;
        self
    }

    pub const fn get_ONTRANSPORT(&self) -> bool {
        (self.inner & MovementFlags::ONTRANSPORT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ONTRANSPORT(mut self) -> Self {
        self.inner &= MovementFlags::ONTRANSPORT.reverse_bits();
        self
    }

    pub const fn new_SPLINE_ELEVATION(spline_elevation: MovementInfo_MovementFlags_SplineElevation) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ELEVATION,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: Some(spline_elevation),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_SPLINE_ELEVATION(mut self, spline_elevation: MovementInfo_MovementFlags_SplineElevation) -> Self {
        self.inner |= MovementFlags::SPLINE_ELEVATION;
        self.spline_elevation = Some(spline_elevation);
        self
    }

    pub const fn get_SPLINE_ELEVATION(&self) -> Option<&MovementInfo_MovementFlags_SplineElevation> {
        self.spline_elevation.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_SPLINE_ELEVATION(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ELEVATION.reverse_bits();
        self.spline_elevation = None;
        self
    }

    pub const fn new_WATERWALKING() -> Self {
        Self {
            inner: MovementFlags::WATERWALKING,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_WATERWALKING(mut self) -> Self {
        self.inner |= MovementFlags::WATERWALKING;
        self
    }

    pub const fn get_WATERWALKING(&self) -> bool {
        (self.inner & MovementFlags::WATERWALKING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_WATERWALKING(mut self) -> Self {
        self.inner &= MovementFlags::WATERWALKING.reverse_bits();
        self
    }

    pub const fn new_SAFE_FALL() -> Self {
        Self {
            inner: MovementFlags::SAFE_FALL,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_SAFE_FALL(mut self) -> Self {
        self.inner |= MovementFlags::SAFE_FALL;
        self
    }

    pub const fn get_SAFE_FALL(&self) -> bool {
        (self.inner & MovementFlags::SAFE_FALL) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_SAFE_FALL(mut self) -> Self {
        self.inner &= MovementFlags::SAFE_FALL.reverse_bits();
        self
    }

    pub const fn new_HOVER() -> Self {
        Self {
            inner: MovementFlags::HOVER,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_HOVER(mut self) -> Self {
        self.inner |= MovementFlags::HOVER;
        self
    }

    pub const fn get_HOVER(&self) -> bool {
        (self.inner & MovementFlags::HOVER) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_HOVER(mut self) -> Self {
        self.inner &= MovementFlags::HOVER.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl MovementInfo_MovementFlags {
    pub(crate) fn size(&self) -> usize {
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
    pub(crate) fn size(&self) -> usize {
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
    pub(crate) fn size(&self) -> usize {
        4 // cos_angle: f32
        + 4 // sin_angle: f32
        + 4 // xy_speed: f32
        + 4 // z_speed: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MovementInfo_MovementFlags_Swimming {
    pub pitch: f32,
}

impl MovementInfo_MovementFlags_Swimming {
    pub(crate) fn size(&self) -> usize {
        4 // pitch: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MovementInfo_MovementFlags_SplineElevation {
    pub spline_elevation: f32,
}

impl MovementInfo_MovementFlags_SplineElevation {
    pub(crate) fn size(&self) -> usize {
        4 // spline_elevation: f32
    }
}

