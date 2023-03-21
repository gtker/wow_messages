use std::io::{Read, Write};

use crate::wrath::{
    MovementFlags, TransportInfo, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm:94`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm#L94):
/// ```text
/// struct MovementInfo {
///     MovementFlags flags;
///     u32 timestamp;
///     Vector3d position;
///     f32 orientation;
///     if (flags & ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT) {
///         TransportInfo transport_info;
///         u32 transport_time;
///     }
///     else if (flags & ON_TRANSPORT) {
///         TransportInfo transport;
///     }
///     if (flags & SWIMMING) {
///         f32 pitch1;
///     }
///     else if (flags & FLYING) {
///         f32 pitch2;
///     }
///     else if (flags & ALWAYS_ALLOW_PITCHING) {
///         f32 pitch3;
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
    pub timestamp: u32,
    pub position: Vector3d,
    pub orientation: f32,
    pub fall_time: f32,
}

impl MovementInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // flags: MovementFlags
        w.write_all(&(self.flags.as_int() as u32).to_le_bytes())?;
        w.write_all(&((self.flags.as_int() >> 32) as u16).to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        if let Some(if_statement) = &self.flags.on_transport_and_interpolated_movement {
            match if_statement {
                MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                    transport_info,
                    transport_time,
                } => {
                    // transport_info: TransportInfo
                    transport_info.write_into_vec(&mut w)?;

                    // transport_time: u32
                    w.write_all(&transport_time.to_le_bytes())?;

                }
                MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                    transport,
                } => {
                    // transport: TransportInfo
                    transport.write_into_vec(&mut w)?;

                }
            }
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
                MovementInfo_MovementFlags_Swimming::AlwaysAllowPitching {
                    pitch3,
                } => {
                    // pitch3: f32
                    w.write_all(&pitch3.to_le_bytes())?;

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
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // flags: MovementFlags
        let flags: MovementFlags = {
            let a = crate::util::read_u32_le(&mut r)?;
            let b = crate::util::read_u16_le(&mut r)?;
            MovementFlags::new((a as u64) | ((b as u64) << 32))
        };

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(&mut r)?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        let flags_ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT = if flags.is_ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT() {
            // transport_info: TransportInfo
            let transport_info = TransportInfo::read(&mut r)?;

            // transport_time: u32
            let transport_time = crate::util::read_u32_le(&mut r)?;

            Some(MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                transport_info,
                transport_time,
            })
        }
        else if flags.is_ON_TRANSPORT() {
            // transport: TransportInfo
            let transport = TransportInfo::read(&mut r)?;

            Some(MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                transport,
            })
        }
        else {
            None
        };

        let flags_SWIMMING = if flags.is_SWIMMING() {
            // pitch1: f32
            let pitch1 = crate::util::read_f32_le(&mut r)?;

            Some(MovementInfo_MovementFlags_Swimming::Swimming {
                pitch1,
            })
        }
        else if flags.is_FLYING() {
            // pitch2: f32
            let pitch2 = crate::util::read_f32_le(&mut r)?;

            Some(MovementInfo_MovementFlags_Swimming::Flying {
                pitch2,
            })
        }
        else if flags.is_ALWAYS_ALLOW_PITCHING() {
            // pitch3: f32
            let pitch3 = crate::util::read_f32_le(&mut r)?;

            Some(MovementInfo_MovementFlags_Swimming::AlwaysAllowPitching {
                pitch3,
            })
        }
        else {
            None
        };

        // fall_time: f32
        let fall_time = crate::util::read_f32_le(&mut r)?;

        let flags_FALLING = if flags.is_FALLING() {
            // z_speed: f32
            let z_speed = crate::util::read_f32_le(&mut r)?;

            // cos_angle: f32
            let cos_angle = crate::util::read_f32_le(&mut r)?;

            // sin_angle: f32
            let sin_angle = crate::util::read_f32_le(&mut r)?;

            // xy_speed: f32
            let xy_speed = crate::util::read_f32_le(&mut r)?;

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
            falling: flags_FALLING,
            swimming: flags_SWIMMING,
            spline_elevation: flags_SPLINE_ELEVATION,
            on_transport_and_interpolated_movement: flags_ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT,
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
    pub(crate) const fn size(&self) -> usize {
        self.flags.size() // flags: MovementInfo_MovementFlags
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
    AlwaysAllowPitching {
        pitch3: f32,
    },
}

impl MovementInfo_MovementFlags_Swimming {
    pub(crate) const fn as_int(&self) -> u64 {
        match self {
            Self::Swimming { .. } => 2097152,
            Self::Flying { .. } => 33554432,
            Self::AlwaysAllowPitching { .. } => 137438953472,
        }
    }

}

impl MovementInfo_MovementFlags_Swimming {
    pub(crate) const fn size(&self) -> usize {
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
            Self::AlwaysAllowPitching {
                pitch3,
            } => {
                // Not an actual enum sent over the wire
                4 // pitch3: f32
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement {
    OnTransportAndInterpolatedMovement {
        transport_info: TransportInfo,
        transport_time: u32,
    },
    OnTransport {
        transport: TransportInfo,
    },
}

impl MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement {
    pub(crate) const fn as_int(&self) -> u64 {
        match self {
            Self::OnTransportAndInterpolatedMovement { .. } => 4398046511616,
            Self::OnTransport { .. } => 512,
        }
    }

}

impl MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::OnTransportAndInterpolatedMovement {
                transport_info,
                transport_time,
            } => {
                // Not an actual enum sent over the wire
                transport_info.size() // transport_info: TransportInfo
                + 4 // transport_time: u32
            }
            Self::OnTransport {
                transport,
            } => {
                // Not an actual enum sent over the wire
                transport.size() // transport: TransportInfo
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MovementInfo_MovementFlags {
    inner: u64,
    falling: Option<MovementInfo_MovementFlags_Falling>,
    swimming: Option<MovementInfo_MovementFlags_Swimming>,
    spline_elevation: Option<MovementInfo_MovementFlags_SplineElevation>,
    on_transport_and_interpolated_movement: Option<MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement>,
}

impl MovementInfo_MovementFlags {
    pub const fn new(inner: u64, falling: Option<MovementInfo_MovementFlags_Falling>,swimming: Option<MovementInfo_MovementFlags_Swimming>,spline_elevation: Option<MovementInfo_MovementFlags_SplineElevation>,on_transport_and_interpolated_movement: Option<MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement>,) -> Self {
        Self {
            inner,
            falling, 
            swimming, 
            spline_elevation, 
            on_transport_and_interpolated_movement, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.falling.is_none()
        && self.swimming.is_none()
        && self.spline_elevation.is_none()
        && self.on_transport_and_interpolated_movement.is_none()
    }

    pub const fn new_FORWARD() -> Self {
        Self {
            inner: MovementFlags::FORWARD,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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

    pub const fn new_LEFT() -> Self {
        Self {
            inner: MovementFlags::LEFT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::LEFT;
        self
    }

    pub const fn get_LEFT(&self) -> bool {
        (self.inner & MovementFlags::LEFT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::LEFT.reverse_bits();
        self
    }

    pub const fn new_RIGHT() -> Self {
        Self {
            inner: MovementFlags::RIGHT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::RIGHT;
        self
    }

    pub const fn get_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::RIGHT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::RIGHT.reverse_bits();
        self
    }

    pub const fn new_PITCH_UP() -> Self {
        Self {
            inner: MovementFlags::PITCH_UP,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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

    pub const fn new_WALKING() -> Self {
        Self {
            inner: MovementFlags::WALKING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_WALKING(mut self) -> Self {
        self.inner |= MovementFlags::WALKING;
        self
    }

    pub const fn get_WALKING(&self) -> bool {
        (self.inner & MovementFlags::WALKING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_WALKING(mut self) -> Self {
        self.inner &= MovementFlags::WALKING.reverse_bits();
        self
    }

    pub const fn new_DISABLE_GRAVITY() -> Self {
        Self {
            inner: MovementFlags::DISABLE_GRAVITY,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_DISABLE_GRAVITY(mut self) -> Self {
        self.inner |= MovementFlags::DISABLE_GRAVITY;
        self
    }

    pub const fn get_DISABLE_GRAVITY(&self) -> bool {
        (self.inner & MovementFlags::DISABLE_GRAVITY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_DISABLE_GRAVITY(mut self) -> Self {
        self.inner &= MovementFlags::DISABLE_GRAVITY.reverse_bits();
        self
    }

    pub const fn new_ROOT() -> Self {
        Self {
            inner: MovementFlags::ROOT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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

    pub const fn new_FALLING(falling: MovementInfo_MovementFlags_Falling) -> Self {
        Self {
            inner: MovementFlags::FALLING,
            falling: Some(falling),
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FALLING(mut self, falling: MovementInfo_MovementFlags_Falling) -> Self {
        self.inner |= MovementFlags::FALLING;
        self.falling = Some(falling);
        self
    }

    pub const fn get_FALLING(&self) -> Option<&MovementInfo_MovementFlags_Falling> {
        self.falling.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FALLING(mut self) -> Self {
        self.inner &= MovementFlags::FALLING.reverse_bits();
        self.falling = None;
        self
    }

    pub const fn new_FALLING_FAR() -> Self {
        Self {
            inner: MovementFlags::FALLING_FAR,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FALLING_FAR(mut self) -> Self {
        self.inner |= MovementFlags::FALLING_FAR;
        self
    }

    pub const fn get_FALLING_FAR(&self) -> bool {
        (self.inner & MovementFlags::FALLING_FAR) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FALLING_FAR(mut self) -> Self {
        self.inner &= MovementFlags::FALLING_FAR.reverse_bits();
        self
    }

    pub const fn new_PENDING_STOP() -> Self {
        Self {
            inner: MovementFlags::PENDING_STOP,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_PENDING_STOP(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STOP;
        self
    }

    pub const fn get_PENDING_STOP(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STOP) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_PENDING_STOP(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STOP.reverse_bits();
        self
    }

    pub const fn new_PENDING_STRAFE_STOP() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_STOP,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_PENDING_STRAFE_STOP(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_STOP;
        self
    }

    pub const fn get_PENDING_STRAFE_STOP(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_STOP) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_PENDING_STRAFE_STOP(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_STOP.reverse_bits();
        self
    }

    pub const fn new_PENDING_FORWARD() -> Self {
        Self {
            inner: MovementFlags::PENDING_FORWARD,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_PENDING_FORWARD(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_FORWARD;
        self
    }

    pub const fn get_PENDING_FORWARD(&self) -> bool {
        (self.inner & MovementFlags::PENDING_FORWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_PENDING_FORWARD(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_FORWARD.reverse_bits();
        self
    }

    pub const fn new_PENDING_BACKWARD() -> Self {
        Self {
            inner: MovementFlags::PENDING_BACKWARD,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_PENDING_BACKWARD(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_BACKWARD;
        self
    }

    pub const fn get_PENDING_BACKWARD(&self) -> bool {
        (self.inner & MovementFlags::PENDING_BACKWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_PENDING_BACKWARD(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_BACKWARD.reverse_bits();
        self
    }

    pub const fn new_PENDING_STRAFE_LEFT() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_LEFT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_PENDING_STRAFE_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_LEFT;
        self
    }

    pub const fn get_PENDING_STRAFE_LEFT(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_LEFT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_PENDING_STRAFE_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_LEFT.reverse_bits();
        self
    }

    pub const fn new_PENDING_STRAFE_RIGHT() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_RIGHT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_PENDING_STRAFE_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_RIGHT;
        self
    }

    pub const fn get_PENDING_STRAFE_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_RIGHT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_PENDING_STRAFE_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_RIGHT.reverse_bits();
        self
    }

    pub const fn new_PENDING_ROOT() -> Self {
        Self {
            inner: MovementFlags::PENDING_ROOT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_PENDING_ROOT(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_ROOT;
        self
    }

    pub const fn get_PENDING_ROOT(&self) -> bool {
        (self.inner & MovementFlags::PENDING_ROOT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_PENDING_ROOT(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_ROOT.reverse_bits();
        self
    }

    pub const fn new_SWIMMING(swimming: MovementInfo_MovementFlags_Swimming) -> Self {
        Self {
            inner: swimming.as_int(),
            falling: None,
            swimming: Some(swimming),
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_SWIMMING(mut self, swimming: MovementInfo_MovementFlags_Swimming) -> Self {
        self.inner |= swimming.as_int();
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

    pub const fn new_ASCENDING() -> Self {
        Self {
            inner: MovementFlags::ASCENDING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ASCENDING(mut self) -> Self {
        self.inner |= MovementFlags::ASCENDING;
        self
    }

    pub const fn get_ASCENDING(&self) -> bool {
        (self.inner & MovementFlags::ASCENDING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ASCENDING(mut self) -> Self {
        self.inner &= MovementFlags::ASCENDING.reverse_bits();
        self
    }

    pub const fn new_DESCENDING() -> Self {
        Self {
            inner: MovementFlags::DESCENDING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_DESCENDING(mut self) -> Self {
        self.inner |= MovementFlags::DESCENDING;
        self
    }

    pub const fn get_DESCENDING(&self) -> bool {
        (self.inner & MovementFlags::DESCENDING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_DESCENDING(mut self) -> Self {
        self.inner &= MovementFlags::DESCENDING.reverse_bits();
        self
    }

    pub const fn new_CAN_FLY() -> Self {
        Self {
            inner: MovementFlags::CAN_FLY,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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

    pub const fn new_SPLINE_ELEVATION(spline_elevation: MovementInfo_MovementFlags_SplineElevation) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ELEVATION,
            falling: None,
            swimming: None,
            spline_elevation: Some(spline_elevation),
            on_transport_and_interpolated_movement: None,
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

    pub const fn new_SPLINE_ENABLED() -> Self {
        Self {
            inner: MovementFlags::SPLINE_ENABLED,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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

    pub const fn new_WATERWALKING() -> Self {
        Self {
            inner: MovementFlags::WATERWALKING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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

    pub const fn new_FALLING_SLOW() -> Self {
        Self {
            inner: MovementFlags::FALLING_SLOW,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FALLING_SLOW(mut self) -> Self {
        self.inner |= MovementFlags::FALLING_SLOW;
        self
    }

    pub const fn get_FALLING_SLOW(&self) -> bool {
        (self.inner & MovementFlags::FALLING_SLOW) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FALLING_SLOW(mut self) -> Self {
        self.inner &= MovementFlags::FALLING_SLOW.reverse_bits();
        self
    }

    pub const fn new_HOVER() -> Self {
        Self {
            inner: MovementFlags::HOVER,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
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

    pub const fn new_NO_STRAFE() -> Self {
        Self {
            inner: MovementFlags::NO_STRAFE,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_NO_STRAFE(mut self) -> Self {
        self.inner |= MovementFlags::NO_STRAFE;
        self
    }

    pub const fn get_NO_STRAFE(&self) -> bool {
        (self.inner & MovementFlags::NO_STRAFE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_NO_STRAFE(mut self) -> Self {
        self.inner &= MovementFlags::NO_STRAFE.reverse_bits();
        self
    }

    pub const fn new_NO_JUMPING() -> Self {
        Self {
            inner: MovementFlags::NO_JUMPING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_NO_JUMPING(mut self) -> Self {
        self.inner |= MovementFlags::NO_JUMPING;
        self
    }

    pub const fn get_NO_JUMPING(&self) -> bool {
        (self.inner & MovementFlags::NO_JUMPING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_NO_JUMPING(mut self) -> Self {
        self.inner &= MovementFlags::NO_JUMPING.reverse_bits();
        self
    }

    pub const fn new_UNK3() -> Self {
        Self {
            inner: MovementFlags::UNK3,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_UNK3(mut self) -> Self {
        self.inner |= MovementFlags::UNK3;
        self
    }

    pub const fn get_UNK3(&self) -> bool {
        (self.inner & MovementFlags::UNK3) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_UNK3(mut self) -> Self {
        self.inner &= MovementFlags::UNK3.reverse_bits();
        self
    }

    pub const fn new_FULL_SPEED_TURNING() -> Self {
        Self {
            inner: MovementFlags::FULL_SPEED_TURNING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FULL_SPEED_TURNING(mut self) -> Self {
        self.inner |= MovementFlags::FULL_SPEED_TURNING;
        self
    }

    pub const fn get_FULL_SPEED_TURNING(&self) -> bool {
        (self.inner & MovementFlags::FULL_SPEED_TURNING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FULL_SPEED_TURNING(mut self) -> Self {
        self.inner &= MovementFlags::FULL_SPEED_TURNING.reverse_bits();
        self
    }

    pub const fn new_FULL_SPEED_PITCHING() -> Self {
        Self {
            inner: MovementFlags::FULL_SPEED_PITCHING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FULL_SPEED_PITCHING(mut self) -> Self {
        self.inner |= MovementFlags::FULL_SPEED_PITCHING;
        self
    }

    pub const fn get_FULL_SPEED_PITCHING(&self) -> bool {
        (self.inner & MovementFlags::FULL_SPEED_PITCHING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FULL_SPEED_PITCHING(mut self) -> Self {
        self.inner &= MovementFlags::FULL_SPEED_PITCHING.reverse_bits();
        self
    }

    pub const fn new_UNK7() -> Self {
        Self {
            inner: MovementFlags::UNK7,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_UNK7(mut self) -> Self {
        self.inner |= MovementFlags::UNK7;
        self
    }

    pub const fn get_UNK7(&self) -> bool {
        (self.inner & MovementFlags::UNK7) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_UNK7(mut self) -> Self {
        self.inner &= MovementFlags::UNK7.reverse_bits();
        self
    }

    pub const fn new_UNK8() -> Self {
        Self {
            inner: MovementFlags::UNK8,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_UNK8(mut self) -> Self {
        self.inner |= MovementFlags::UNK8;
        self
    }

    pub const fn get_UNK8(&self) -> bool {
        (self.inner & MovementFlags::UNK8) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_UNK8(mut self) -> Self {
        self.inner &= MovementFlags::UNK8.reverse_bits();
        self
    }

    pub const fn new_UNK9() -> Self {
        Self {
            inner: MovementFlags::UNK9,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_UNK9(mut self) -> Self {
        self.inner |= MovementFlags::UNK9;
        self
    }

    pub const fn get_UNK9(&self) -> bool {
        (self.inner & MovementFlags::UNK9) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_UNK9(mut self) -> Self {
        self.inner &= MovementFlags::UNK9.reverse_bits();
        self
    }

    pub const fn new_UNK10() -> Self {
        Self {
            inner: MovementFlags::UNK10,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_UNK10(mut self) -> Self {
        self.inner |= MovementFlags::UNK10;
        self
    }

    pub const fn get_UNK10(&self) -> bool {
        (self.inner & MovementFlags::UNK10) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_UNK10(mut self) -> Self {
        self.inner &= MovementFlags::UNK10.reverse_bits();
        self
    }

    pub const fn new_INTERPOLATED_MOVEMENT() -> Self {
        Self {
            inner: MovementFlags::INTERPOLATED_MOVEMENT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_INTERPOLATED_MOVEMENT(mut self) -> Self {
        self.inner |= MovementFlags::INTERPOLATED_MOVEMENT;
        self
    }

    pub const fn get_INTERPOLATED_MOVEMENT(&self) -> bool {
        (self.inner & MovementFlags::INTERPOLATED_MOVEMENT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_INTERPOLATED_MOVEMENT(mut self) -> Self {
        self.inner &= MovementFlags::INTERPOLATED_MOVEMENT.reverse_bits();
        self
    }

    pub const fn new_INTERPOLATED_TURNING() -> Self {
        Self {
            inner: MovementFlags::INTERPOLATED_TURNING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_INTERPOLATED_TURNING(mut self) -> Self {
        self.inner |= MovementFlags::INTERPOLATED_TURNING;
        self
    }

    pub const fn get_INTERPOLATED_TURNING(&self) -> bool {
        (self.inner & MovementFlags::INTERPOLATED_TURNING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_INTERPOLATED_TURNING(mut self) -> Self {
        self.inner &= MovementFlags::INTERPOLATED_TURNING.reverse_bits();
        self
    }

    pub const fn new_INTERPOLATED_PITCHING() -> Self {
        Self {
            inner: MovementFlags::INTERPOLATED_PITCHING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_INTERPOLATED_PITCHING(mut self) -> Self {
        self.inner |= MovementFlags::INTERPOLATED_PITCHING;
        self
    }

    pub const fn get_INTERPOLATED_PITCHING(&self) -> bool {
        (self.inner & MovementFlags::INTERPOLATED_PITCHING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_INTERPOLATED_PITCHING(mut self) -> Self {
        self.inner &= MovementFlags::INTERPOLATED_PITCHING.reverse_bits();
        self
    }

    pub const fn new_UNK14() -> Self {
        Self {
            inner: MovementFlags::UNK14,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_UNK14(mut self) -> Self {
        self.inner |= MovementFlags::UNK14;
        self
    }

    pub const fn get_UNK14(&self) -> bool {
        (self.inner & MovementFlags::UNK14) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_UNK14(mut self) -> Self {
        self.inner &= MovementFlags::UNK14.reverse_bits();
        self
    }

    pub const fn new_UNK15() -> Self {
        Self {
            inner: MovementFlags::UNK15,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_UNK15(mut self) -> Self {
        self.inner |= MovementFlags::UNK15;
        self
    }

    pub const fn get_UNK15(&self) -> bool {
        (self.inner & MovementFlags::UNK15) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_UNK15(mut self) -> Self {
        self.inner &= MovementFlags::UNK15.reverse_bits();
        self
    }

    pub const fn new_UNK16() -> Self {
        Self {
            inner: MovementFlags::UNK16,
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_UNK16(mut self) -> Self {
        self.inner |= MovementFlags::UNK16;
        self
    }

    pub const fn get_UNK16(&self) -> bool {
        (self.inner & MovementFlags::UNK16) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_UNK16(mut self) -> Self {
        self.inner &= MovementFlags::UNK16.reverse_bits();
        self
    }

    pub const fn new_ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT(on_transport_and_interpolated_movement: MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement) -> Self {
        Self {
            inner: on_transport_and_interpolated_movement.as_int(),
            falling: None,
            swimming: None,
            spline_elevation: None,
            on_transport_and_interpolated_movement: Some(on_transport_and_interpolated_movement),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT(mut self, on_transport_and_interpolated_movement: MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement) -> Self {
        self.inner |= on_transport_and_interpolated_movement.as_int();
        self.on_transport_and_interpolated_movement = Some(on_transport_and_interpolated_movement);
        self
    }

    pub const fn get_ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT(&self) -> Option<&MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement> {
        self.on_transport_and_interpolated_movement.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT(mut self) -> Self {
        self.inner &= MovementFlags::ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT.reverse_bits();
        self.on_transport_and_interpolated_movement = None;
        self
    }

    pub(crate) const fn as_int(&self) -> u64 {
        self.inner
    }

}
impl MovementInfo_MovementFlags {
    pub(crate) const fn size(&self) -> usize {
        6 // inner
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
        + {
            if let Some(s) = &self.on_transport_and_interpolated_movement {
                s.size()
            } else {
                0
            }
        }
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

