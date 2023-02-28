use crate::wrath::TransportInfo;
use crate::wrath::Vector3d;
use crate::wrath::ExtraMovementFlags;
use crate::wrath::MovementFlags;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm:97`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm#L97):
/// ```text
/// struct MovementInfo {
///     MovementFlags flags;
///     ExtraMovementFlags extra_flags;
///     u32 timestamp;
///     Vector3d position;
///     f32 orientation;
///     if (flags & ON_TRANSPORT) {
///         TransportInfo transport;
///     }
///     if (flags & SWIMMING) {
///         f32 pitch1;
///     }
///     else if (flags & FLYING) {
///         f32 pitch2;
///     }
///     f32 fall_time;
///     if (flags & FALLING) {
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
    pub extra_flags: ExtraMovementFlags,
    pub timestamp: u32,
    pub position: Vector3d,
    pub orientation: f32,
    pub fall_time: f32,
}

impl MovementInfo {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // flags: MovementFlags
        w.write_all(&u32::from(self.flags.as_int()).to_le_bytes())?;

        // extra_flags: ExtraMovementFlags
        w.write_all(&u16::from(self.extra_flags.as_int()).to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        if let Some(if_statement) = &self.flags.on_transport {
            // transport: TransportInfo
            if_statement.transport.write_into_vec(w)?;

        }

        if let Some(if_statement) = &self.flags.swimming {
            match if_statement {
                MovementInfo_MovementFlags_Swimming::Swimming {
                    pitch1,
                } => {
                    // pitch1: f32
                    w.write_all(&pitch1.to_le_bytes())?;

                }
                MovementInfo_MovementFlags_Swimming::Flying {
                    pitch2,
                } => {
                    // pitch2: f32
                    w.write_all(&pitch2.to_le_bytes())?;

                }
            }
        }

        // fall_time: f32
        w.write_all(&self.fall_time.to_le_bytes())?;

        if let Some(if_statement) = &self.flags.falling {
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
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // flags: MovementFlags
        let flags = MovementFlags::new(crate::util::read_u32_le(r)?);

        // extra_flags: ExtraMovementFlags
        let extra_flags = ExtraMovementFlags::new(crate::util::read_u16_le(r)?);

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(r)?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(r)?;

        let flags_ON_TRANSPORT = if flags.is_ON_TRANSPORT() {
            // transport: TransportInfo
            let transport = TransportInfo::read(r)?;

            Some(MovementInfo_MovementFlags_OnTransport {
                transport,
            })
        }
        else {
            None
        };

        let flags_SWIMMING = if flags.is_SWIMMING() {
            // pitch1: f32
            let pitch1 = crate::util::read_f32_le(r)?;

            Some(MovementInfo_MovementFlags_Swimming::Swimming {
                pitch1,
            })
        }
        else if flags.is_FLYING() {
            // pitch2: f32
            let pitch2 = crate::util::read_f32_le(r)?;

            Some(MovementInfo_MovementFlags_Swimming::Flying {
                pitch2,
            })
        }
        else {
            None
        };

        // fall_time: f32
        let fall_time = crate::util::read_f32_le(r)?;

        let flags_FALLING = if flags.is_FALLING() {
            // z_speed: f32
            let z_speed = crate::util::read_f32_le(r)?;

            // cos_angle: f32
            let cos_angle = crate::util::read_f32_le(r)?;

            // sin_angle: f32
            let sin_angle = crate::util::read_f32_le(r)?;

            // xy_speed: f32
            let xy_speed = crate::util::read_f32_le(r)?;

            Some(MovementInfo_MovementFlags_Falling {
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
            let spline_elevation = crate::util::read_f32_le(r)?;

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
            falling: flags_FALLING,
            swimming: flags_SWIMMING,
            spline_elevation: flags_SPLINE_ELEVATION,
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
    pub(crate) fn size(&self) -> usize {
        self.flags.size() // flags: MovementInfo_MovementFlags
        + 2 // extra_flags: ExtraMovementFlags
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
    Flying {
        pitch2: f32,
    },
}

impl MovementInfo_MovementFlags_Swimming {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Swimming { .. } => 2097152,
            Self::Flying { .. } => 33554432,
        }
    }

}

impl MovementInfo_MovementFlags_Swimming {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Swimming {
                pitch1,
            } => {
                // Not an actual enum sent over the wire
                4 // pitch1: f32
            }
            Self::Flying {
                pitch2,
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
    falling: Option<MovementInfo_MovementFlags_Falling>,
    swimming: Option<MovementInfo_MovementFlags_Swimming>,
    spline_elevation: Option<MovementInfo_MovementFlags_SplineElevation>,
}

impl MovementInfo_MovementFlags {
    pub const fn new(inner: u32, on_transport: Option<MovementInfo_MovementFlags_OnTransport>,falling: Option<MovementInfo_MovementFlags_Falling>,swimming: Option<MovementInfo_MovementFlags_Swimming>,spline_elevation: Option<MovementInfo_MovementFlags_SplineElevation>,) -> Self {
        Self {
            inner,
            on_transport, 
            falling, 
            swimming, 
            spline_elevation, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.on_transport.is_none()
        && self.falling.is_none()
        && self.swimming.is_none()
        && self.spline_elevation.is_none()
    }

    pub const fn new_FORWARD() -> Self {
        Self {
            inner: MovementFlags::FORWARD,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_FORWARD(mut self) -> Self {
        self.inner |= MovementFlags::FORWARD;
        self
    }

    pub const fn get_FORWARD(&self) -> bool {
        (self.inner & MovementFlags::FORWARD) != 0
    }

    pub fn clear_FORWARD(mut self) -> Self {
        self.inner &= MovementFlags::FORWARD.reverse_bits();
        self
    }

    pub const fn new_BACKWARD() -> Self {
        Self {
            inner: MovementFlags::BACKWARD,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_BACKWARD(mut self) -> Self {
        self.inner |= MovementFlags::BACKWARD;
        self
    }

    pub const fn get_BACKWARD(&self) -> bool {
        (self.inner & MovementFlags::BACKWARD) != 0
    }

    pub fn clear_BACKWARD(mut self) -> Self {
        self.inner &= MovementFlags::BACKWARD.reverse_bits();
        self
    }

    pub const fn new_STRAFE_LEFT() -> Self {
        Self {
            inner: MovementFlags::STRAFE_LEFT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_STRAFE_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_LEFT;
        self
    }

    pub const fn get_STRAFE_LEFT(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_LEFT) != 0
    }

    pub fn clear_STRAFE_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_LEFT.reverse_bits();
        self
    }

    pub const fn new_STRAFE_RIGHT() -> Self {
        Self {
            inner: MovementFlags::STRAFE_RIGHT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_STRAFE_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_RIGHT;
        self
    }

    pub const fn get_STRAFE_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_RIGHT) != 0
    }

    pub fn clear_STRAFE_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_RIGHT.reverse_bits();
        self
    }

    pub const fn new_LEFT() -> Self {
        Self {
            inner: MovementFlags::LEFT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::LEFT;
        self
    }

    pub const fn get_LEFT(&self) -> bool {
        (self.inner & MovementFlags::LEFT) != 0
    }

    pub fn clear_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::LEFT.reverse_bits();
        self
    }

    pub const fn new_RIGHT() -> Self {
        Self {
            inner: MovementFlags::RIGHT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::RIGHT;
        self
    }

    pub const fn get_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::RIGHT) != 0
    }

    pub fn clear_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::RIGHT.reverse_bits();
        self
    }

    pub const fn new_PITCH_UP() -> Self {
        Self {
            inner: MovementFlags::PITCH_UP,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_PITCH_UP(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_UP;
        self
    }

    pub const fn get_PITCH_UP(&self) -> bool {
        (self.inner & MovementFlags::PITCH_UP) != 0
    }

    pub fn clear_PITCH_UP(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_UP.reverse_bits();
        self
    }

    pub const fn new_PITCH_DOWN() -> Self {
        Self {
            inner: MovementFlags::PITCH_DOWN,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_PITCH_DOWN(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_DOWN;
        self
    }

    pub const fn get_PITCH_DOWN(&self) -> bool {
        (self.inner & MovementFlags::PITCH_DOWN) != 0
    }

    pub fn clear_PITCH_DOWN(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_DOWN.reverse_bits();
        self
    }

    pub const fn new_WALKING() -> Self {
        Self {
            inner: MovementFlags::WALKING,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_WALKING(mut self) -> Self {
        self.inner |= MovementFlags::WALKING;
        self
    }

    pub const fn get_WALKING(&self) -> bool {
        (self.inner & MovementFlags::WALKING) != 0
    }

    pub fn clear_WALKING(mut self) -> Self {
        self.inner &= MovementFlags::WALKING.reverse_bits();
        self
    }

    pub const fn new_ON_TRANSPORT(on_transport: MovementInfo_MovementFlags_OnTransport) -> Self {
        Self {
            inner: MovementFlags::ON_TRANSPORT,
            on_transport: Some(on_transport),
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_ON_TRANSPORT(mut self, on_transport: MovementInfo_MovementFlags_OnTransport) -> Self {
        self.inner |= MovementFlags::ON_TRANSPORT;
        self.on_transport = Some(on_transport);
        self
    }

    pub const fn get_ON_TRANSPORT(&self) -> Option<&MovementInfo_MovementFlags_OnTransport> {
        self.on_transport.as_ref()
    }

    pub fn clear_ON_TRANSPORT(mut self) -> Self {
        self.inner &= MovementFlags::ON_TRANSPORT.reverse_bits();
        self.on_transport = None;
        self
    }

    pub const fn new_DISABLE_GRAVITY() -> Self {
        Self {
            inner: MovementFlags::DISABLE_GRAVITY,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_DISABLE_GRAVITY(mut self) -> Self {
        self.inner |= MovementFlags::DISABLE_GRAVITY;
        self
    }

    pub const fn get_DISABLE_GRAVITY(&self) -> bool {
        (self.inner & MovementFlags::DISABLE_GRAVITY) != 0
    }

    pub fn clear_DISABLE_GRAVITY(mut self) -> Self {
        self.inner &= MovementFlags::DISABLE_GRAVITY.reverse_bits();
        self
    }

    pub const fn new_ROOT() -> Self {
        Self {
            inner: MovementFlags::ROOT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_ROOT(mut self) -> Self {
        self.inner |= MovementFlags::ROOT;
        self
    }

    pub const fn get_ROOT(&self) -> bool {
        (self.inner & MovementFlags::ROOT) != 0
    }

    pub fn clear_ROOT(mut self) -> Self {
        self.inner &= MovementFlags::ROOT.reverse_bits();
        self
    }

    pub const fn new_FALLING(falling: MovementInfo_MovementFlags_Falling) -> Self {
        Self {
            inner: MovementFlags::FALLING,
            on_transport: None,
            falling: Some(falling),
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_FALLING(mut self, falling: MovementInfo_MovementFlags_Falling) -> Self {
        self.inner |= MovementFlags::FALLING;
        self.falling = Some(falling);
        self
    }

    pub const fn get_FALLING(&self) -> Option<&MovementInfo_MovementFlags_Falling> {
        self.falling.as_ref()
    }

    pub fn clear_FALLING(mut self) -> Self {
        self.inner &= MovementFlags::FALLING.reverse_bits();
        self.falling = None;
        self
    }

    pub const fn new_FALLING_FAR() -> Self {
        Self {
            inner: MovementFlags::FALLING_FAR,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_FALLING_FAR(mut self) -> Self {
        self.inner |= MovementFlags::FALLING_FAR;
        self
    }

    pub const fn get_FALLING_FAR(&self) -> bool {
        (self.inner & MovementFlags::FALLING_FAR) != 0
    }

    pub fn clear_FALLING_FAR(mut self) -> Self {
        self.inner &= MovementFlags::FALLING_FAR.reverse_bits();
        self
    }

    pub const fn new_PENDING_STOP() -> Self {
        Self {
            inner: MovementFlags::PENDING_STOP,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_PENDING_STOP(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STOP;
        self
    }

    pub const fn get_PENDING_STOP(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STOP) != 0
    }

    pub fn clear_PENDING_STOP(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STOP.reverse_bits();
        self
    }

    pub const fn new_PENDING_STRAFE_STOP() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_STOP,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_PENDING_STRAFE_STOP(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_STOP;
        self
    }

    pub const fn get_PENDING_STRAFE_STOP(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_STOP) != 0
    }

    pub fn clear_PENDING_STRAFE_STOP(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_STOP.reverse_bits();
        self
    }

    pub const fn new_PENDING_FORWARD() -> Self {
        Self {
            inner: MovementFlags::PENDING_FORWARD,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_PENDING_FORWARD(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_FORWARD;
        self
    }

    pub const fn get_PENDING_FORWARD(&self) -> bool {
        (self.inner & MovementFlags::PENDING_FORWARD) != 0
    }

    pub fn clear_PENDING_FORWARD(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_FORWARD.reverse_bits();
        self
    }

    pub const fn new_PENDING_BACKWARD() -> Self {
        Self {
            inner: MovementFlags::PENDING_BACKWARD,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_PENDING_BACKWARD(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_BACKWARD;
        self
    }

    pub const fn get_PENDING_BACKWARD(&self) -> bool {
        (self.inner & MovementFlags::PENDING_BACKWARD) != 0
    }

    pub fn clear_PENDING_BACKWARD(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_BACKWARD.reverse_bits();
        self
    }

    pub const fn new_PENDING_STRAFE_LEFT() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_LEFT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_PENDING_STRAFE_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_LEFT;
        self
    }

    pub const fn get_PENDING_STRAFE_LEFT(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_LEFT) != 0
    }

    pub fn clear_PENDING_STRAFE_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_LEFT.reverse_bits();
        self
    }

    pub const fn new_PENDING_STRAFE_RIGHT() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_RIGHT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_PENDING_STRAFE_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_RIGHT;
        self
    }

    pub const fn get_PENDING_STRAFE_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_RIGHT) != 0
    }

    pub fn clear_PENDING_STRAFE_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_RIGHT.reverse_bits();
        self
    }

    pub const fn new_PENDING_ROOT() -> Self {
        Self {
            inner: MovementFlags::PENDING_ROOT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_PENDING_ROOT(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_ROOT;
        self
    }

    pub const fn get_PENDING_ROOT(&self) -> bool {
        (self.inner & MovementFlags::PENDING_ROOT) != 0
    }

    pub fn clear_PENDING_ROOT(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_ROOT.reverse_bits();
        self
    }

    pub const fn new_SWIMMING(swimming: MovementInfo_MovementFlags_Swimming) -> Self {
        Self {
            inner: swimming.as_int(),
            on_transport: None,
            falling: None,
            swimming: Some(swimming),
            spline_elevation: None,
        }
    }

    pub fn set_SWIMMING(mut self, swimming: MovementInfo_MovementFlags_Swimming) -> Self {
        self.inner |= swimming.as_int();
        self.swimming = Some(swimming);
        self
    }

    pub const fn get_SWIMMING(&self) -> Option<&MovementInfo_MovementFlags_Swimming> {
        self.swimming.as_ref()
    }

    pub fn clear_SWIMMING(mut self) -> Self {
        self.inner &= MovementFlags::SWIMMING.reverse_bits();
        self.swimming = None;
        self
    }

    pub const fn new_ASCENDING() -> Self {
        Self {
            inner: MovementFlags::ASCENDING,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_ASCENDING(mut self) -> Self {
        self.inner |= MovementFlags::ASCENDING;
        self
    }

    pub const fn get_ASCENDING(&self) -> bool {
        (self.inner & MovementFlags::ASCENDING) != 0
    }

    pub fn clear_ASCENDING(mut self) -> Self {
        self.inner &= MovementFlags::ASCENDING.reverse_bits();
        self
    }

    pub const fn new_DESCENDING() -> Self {
        Self {
            inner: MovementFlags::DESCENDING,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_DESCENDING(mut self) -> Self {
        self.inner |= MovementFlags::DESCENDING;
        self
    }

    pub const fn get_DESCENDING(&self) -> bool {
        (self.inner & MovementFlags::DESCENDING) != 0
    }

    pub fn clear_DESCENDING(mut self) -> Self {
        self.inner &= MovementFlags::DESCENDING.reverse_bits();
        self
    }

    pub const fn new_CAN_FLY() -> Self {
        Self {
            inner: MovementFlags::CAN_FLY,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_CAN_FLY(mut self) -> Self {
        self.inner |= MovementFlags::CAN_FLY;
        self
    }

    pub const fn get_CAN_FLY(&self) -> bool {
        (self.inner & MovementFlags::CAN_FLY) != 0
    }

    pub fn clear_CAN_FLY(mut self) -> Self {
        self.inner &= MovementFlags::CAN_FLY.reverse_bits();
        self
    }

    pub const fn new_SPLINE_ELEVATION(spline_elevation: MovementInfo_MovementFlags_SplineElevation) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ELEVATION,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: Some(spline_elevation),
        }
    }

    pub fn set_SPLINE_ELEVATION(mut self, spline_elevation: MovementInfo_MovementFlags_SplineElevation) -> Self {
        self.inner |= MovementFlags::SPLINE_ELEVATION;
        self.spline_elevation = Some(spline_elevation);
        self
    }

    pub const fn get_SPLINE_ELEVATION(&self) -> Option<&MovementInfo_MovementFlags_SplineElevation> {
        self.spline_elevation.as_ref()
    }

    pub fn clear_SPLINE_ELEVATION(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ELEVATION.reverse_bits();
        self.spline_elevation = None;
        self
    }

    pub const fn new_SPLINE_ENABLED() -> Self {
        Self {
            inner: MovementFlags::SPLINE_ENABLED,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_SPLINE_ENABLED(mut self) -> Self {
        self.inner |= MovementFlags::SPLINE_ENABLED;
        self
    }

    pub const fn get_SPLINE_ENABLED(&self) -> bool {
        (self.inner & MovementFlags::SPLINE_ENABLED) != 0
    }

    pub fn clear_SPLINE_ENABLED(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ENABLED.reverse_bits();
        self
    }

    pub const fn new_WATERWALKING() -> Self {
        Self {
            inner: MovementFlags::WATERWALKING,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_WATERWALKING(mut self) -> Self {
        self.inner |= MovementFlags::WATERWALKING;
        self
    }

    pub const fn get_WATERWALKING(&self) -> bool {
        (self.inner & MovementFlags::WATERWALKING) != 0
    }

    pub fn clear_WATERWALKING(mut self) -> Self {
        self.inner &= MovementFlags::WATERWALKING.reverse_bits();
        self
    }

    pub const fn new_FALLING_SLOW() -> Self {
        Self {
            inner: MovementFlags::FALLING_SLOW,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_FALLING_SLOW(mut self) -> Self {
        self.inner |= MovementFlags::FALLING_SLOW;
        self
    }

    pub const fn get_FALLING_SLOW(&self) -> bool {
        (self.inner & MovementFlags::FALLING_SLOW) != 0
    }

    pub fn clear_FALLING_SLOW(mut self) -> Self {
        self.inner &= MovementFlags::FALLING_SLOW.reverse_bits();
        self
    }

    pub const fn new_HOVER() -> Self {
        Self {
            inner: MovementFlags::HOVER,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_HOVER(mut self) -> Self {
        self.inner |= MovementFlags::HOVER;
        self
    }

    pub const fn get_HOVER(&self) -> bool {
        (self.inner & MovementFlags::HOVER) != 0
    }

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
            if let Some(s) = &self.falling {
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
pub struct MovementInfo_MovementFlags_Falling {
    pub cos_angle: f32,
    pub sin_angle: f32,
    pub xy_speed: f32,
    pub z_speed: f32,
}

impl MovementInfo_MovementFlags_Falling {
    pub(crate) fn size(&self) -> usize {
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
    pub(crate) fn size(&self) -> usize {
        4 // spline_elevation: f32
    }
}

