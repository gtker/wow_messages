use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{MovementFlags};
use crate::world::v1::v12::TransportInfo;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement.wowm#L40):
/// ```text
/// struct MovementInfo {
///     MovementFlags flags;
///     u32 timestamp;
///     f32 position_x;
///     f32 position_y;
///     f32 position_z;
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
    pub flags: MovementInfoMovementFlags,
    pub timestamp: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub orientation: f32,
    pub fall_time: f32,
}

impl ReadableAndWritable for MovementInfo {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // flags: MovementFlags
        let flags = MovementFlags::read(r)?;

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // orientation: f32
        let orientation = crate::util::read_f32_le(r)?;
        let flags_ON_TRANSPORT = if flags.is_ON_TRANSPORT() {
            // transport: TransportInfo
            let transport = TransportInfo::read(r)?;

            Some(MovementInfoMovementFlagsON_TRANSPORT {
                transport,
            })
        } else {
            None
        };

        let flags_SWIMMING = if flags.is_SWIMMING() {
            // pitch: f32
            let pitch = crate::util::read_f32_le(r)?;
            Some(MovementInfoMovementFlagsSWIMMING {
                pitch,
            })
        } else {
            None
        };

        // fall_time: f32
        let fall_time = crate::util::read_f32_le(r)?;
        let flags_JUMPING = if flags.is_JUMPING() {
            // z_speed: f32
            let z_speed = crate::util::read_f32_le(r)?;
            // cos_angle: f32
            let cos_angle = crate::util::read_f32_le(r)?;
            // sin_angle: f32
            let sin_angle = crate::util::read_f32_le(r)?;
            // xy_speed: f32
            let xy_speed = crate::util::read_f32_le(r)?;
            Some(MovementInfoMovementFlagsJUMPING {
                z_speed,
                cos_angle,
                sin_angle,
                xy_speed,
            })
        } else {
            None
        };

        let flags_SPLINE_ELEVATION = if flags.is_SPLINE_ELEVATION() {
            // spline_elevation: f32
            let spline_elevation = crate::util::read_f32_le(r)?;
            Some(MovementInfoMovementFlagsSPLINE_ELEVATION {
                spline_elevation,
            })
        } else {
            None
        };

        let flags = MovementInfoMovementFlags {
            inner: flags.as_u32(),
            on_transport: flags_ON_TRANSPORT,
            jumping: flags_JUMPING,
            swimming: flags_SWIMMING,
            spline_elevation: flags_SPLINE_ELEVATION,
        };

        Ok(Self {
            flags,
            timestamp,
            position_x,
            position_y,
            position_z,
            orientation,
            fall_time,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // flags: MovementFlags
        self.flags.write(w)?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        if let Some(s) = &self.flags.on_transport {
            s.write(w)?;
        }

        if let Some(s) = &self.flags.swimming {
            s.write(w)?;
        }

        // fall_time: f32
        w.write_all(&self.fall_time.to_le_bytes())?;

        if let Some(s) = &self.flags.jumping {
            s.write(w)?;
        }

        if let Some(s) = &self.flags.spline_elevation {
            s.write(w)?;
        }

        Ok(())
    }

}

impl VariableSized for MovementInfo {
    fn size(&self) -> usize {
        self.flags.size() // flags: MovementFlags and subfields
        + 4 // timestamp: u32
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // orientation: f32
        + 4 // fall_time: f32
    }
}

impl MaximumPossibleSized for MovementInfo {
    fn maximum_possible_size() -> usize {
        MovementFlags::maximum_possible_size() // flags: MovementFlags
        + 4 // timestamp: u32
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // orientation: f32
        + 4 // fall_time: f32
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct MovementInfoMovementFlags {
    inner: u32,
    on_transport: Option<MovementInfoMovementFlagsON_TRANSPORT>,
    jumping: Option<MovementInfoMovementFlagsJUMPING>,
    swimming: Option<MovementInfoMovementFlagsSWIMMING>,
    spline_elevation: Option<MovementInfoMovementFlagsSPLINE_ELEVATION>,
}

impl From<&MovementInfoMovementFlags> for MovementFlags {
    fn from(e: &MovementInfoMovementFlags) -> Self {
        Self::new(e.inner)
    }
}

impl MovementInfoMovementFlags {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MovementFlags = self.into();
        a.write(w)?;
        Ok(())
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

    pub const fn new_NONE() -> Self {
        Self {
            inner: MovementFlags::NONE,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_NONE(&mut self) -> Self {
        self.inner |= MovementFlags::NONE;
        self.clone()
    }

    pub fn clear_NONE(&mut self) -> Self {
        self.inner &= MovementFlags::NONE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_FORWARD(&mut self) -> Self {
        self.inner |= MovementFlags::FORWARD;
        self.clone()
    }

    pub fn clear_FORWARD(&mut self) -> Self {
        self.inner &= MovementFlags::FORWARD.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_BACKWARD(&mut self) -> Self {
        self.inner |= MovementFlags::BACKWARD;
        self.clone()
    }

    pub fn clear_BACKWARD(&mut self) -> Self {
        self.inner &= MovementFlags::BACKWARD.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_STRAFE_LEFT(&mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_LEFT;
        self.clone()
    }

    pub fn clear_STRAFE_LEFT(&mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_LEFT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_STRAFE_RIGHT(&mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_RIGHT;
        self.clone()
    }

    pub fn clear_STRAFE_RIGHT(&mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_RIGHT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_TURN_LEFT(&mut self) -> Self {
        self.inner |= MovementFlags::TURN_LEFT;
        self.clone()
    }

    pub fn clear_TURN_LEFT(&mut self) -> Self {
        self.inner &= MovementFlags::TURN_LEFT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_TURN_RIGHT(&mut self) -> Self {
        self.inner |= MovementFlags::TURN_RIGHT;
        self.clone()
    }

    pub fn clear_TURN_RIGHT(&mut self) -> Self {
        self.inner &= MovementFlags::TURN_RIGHT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_PITCH_UP(&mut self) -> Self {
        self.inner |= MovementFlags::PITCH_UP;
        self.clone()
    }

    pub fn clear_PITCH_UP(&mut self) -> Self {
        self.inner &= MovementFlags::PITCH_UP.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_PITCH_DOWN(&mut self) -> Self {
        self.inner |= MovementFlags::PITCH_DOWN;
        self.clone()
    }

    pub fn clear_PITCH_DOWN(&mut self) -> Self {
        self.inner &= MovementFlags::PITCH_DOWN.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_WALK_MODE(&mut self) -> Self {
        self.inner |= MovementFlags::WALK_MODE;
        self.clone()
    }

    pub fn clear_WALK_MODE(&mut self) -> Self {
        self.inner &= MovementFlags::WALK_MODE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_ON_TRANSPORT(on_transport: MovementInfoMovementFlagsON_TRANSPORT) -> Self {
        Self {
            inner: MovementFlags::ON_TRANSPORT,
            on_transport: Some(on_transport),
            jumping: None,
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_ON_TRANSPORT(&mut self, on_transport: MovementInfoMovementFlagsON_TRANSPORT) -> Self {
        self.inner |= MovementFlags::ON_TRANSPORT;
        self.on_transport = Some(on_transport);
        self.clone()
    }

    pub fn clear_ON_TRANSPORT(&mut self) -> Self {
        self.inner &= MovementFlags::ON_TRANSPORT.reverse_bits();
        self.on_transport = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_LEVITATING(&mut self) -> Self {
        self.inner |= MovementFlags::LEVITATING;
        self.clone()
    }

    pub fn clear_LEVITATING(&mut self) -> Self {
        self.inner &= MovementFlags::LEVITATING.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_FIXED_Z(&mut self) -> Self {
        self.inner |= MovementFlags::FIXED_Z;
        self.clone()
    }

    pub fn clear_FIXED_Z(&mut self) -> Self {
        self.inner &= MovementFlags::FIXED_Z.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_ROOT(&mut self) -> Self {
        self.inner |= MovementFlags::ROOT;
        self.clone()
    }

    pub fn clear_ROOT(&mut self) -> Self {
        self.inner &= MovementFlags::ROOT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_JUMPING(jumping: MovementInfoMovementFlagsJUMPING) -> Self {
        Self {
            inner: MovementFlags::JUMPING,
            on_transport: None,
            jumping: Some(jumping),
            swimming: None,
            spline_elevation: None,
        }
    }

    pub fn set_JUMPING(&mut self, jumping: MovementInfoMovementFlagsJUMPING) -> Self {
        self.inner |= MovementFlags::JUMPING;
        self.jumping = Some(jumping);
        self.clone()
    }

    pub fn clear_JUMPING(&mut self) -> Self {
        self.inner &= MovementFlags::JUMPING.reverse_bits();
        self.jumping = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_FALLINGFAR(&mut self) -> Self {
        self.inner |= MovementFlags::FALLINGFAR;
        self.clone()
    }

    pub fn clear_FALLINGFAR(&mut self) -> Self {
        self.inner &= MovementFlags::FALLINGFAR.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_SWIMMING(swimming: MovementInfoMovementFlagsSWIMMING) -> Self {
        Self {
            inner: MovementFlags::SWIMMING,
            on_transport: None,
            jumping: None,
            swimming: Some(swimming),
            spline_elevation: None,
        }
    }

    pub fn set_SWIMMING(&mut self, swimming: MovementInfoMovementFlagsSWIMMING) -> Self {
        self.inner |= MovementFlags::SWIMMING;
        self.swimming = Some(swimming);
        self.clone()
    }

    pub fn clear_SWIMMING(&mut self) -> Self {
        self.inner &= MovementFlags::SWIMMING.reverse_bits();
        self.swimming = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_SPLINE_ENABLED(&mut self) -> Self {
        self.inner |= MovementFlags::SPLINE_ENABLED;
        self.clone()
    }

    pub fn clear_SPLINE_ENABLED(&mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ENABLED.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_CAN_FLY(&mut self) -> Self {
        self.inner |= MovementFlags::CAN_FLY;
        self.clone()
    }

    pub fn clear_CAN_FLY(&mut self) -> Self {
        self.inner &= MovementFlags::CAN_FLY.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_FLYING(&mut self) -> Self {
        self.inner |= MovementFlags::FLYING;
        self.clone()
    }

    pub fn clear_FLYING(&mut self) -> Self {
        self.inner &= MovementFlags::FLYING.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_ONTRANSPORT(&mut self) -> Self {
        self.inner |= MovementFlags::ONTRANSPORT;
        self.clone()
    }

    pub fn clear_ONTRANSPORT(&mut self) -> Self {
        self.inner &= MovementFlags::ONTRANSPORT.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_SPLINE_ELEVATION(spline_elevation: MovementInfoMovementFlagsSPLINE_ELEVATION) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ELEVATION,
            on_transport: None,
            jumping: None,
            swimming: None,
            spline_elevation: Some(spline_elevation),
        }
    }

    pub fn set_SPLINE_ELEVATION(&mut self, spline_elevation: MovementInfoMovementFlagsSPLINE_ELEVATION) -> Self {
        self.inner |= MovementFlags::SPLINE_ELEVATION;
        self.spline_elevation = Some(spline_elevation);
        self.clone()
    }

    pub fn clear_SPLINE_ELEVATION(&mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ELEVATION.reverse_bits();
        self.spline_elevation = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_WATERWALKING(&mut self) -> Self {
        self.inner |= MovementFlags::WATERWALKING;
        self.clone()
    }

    pub fn clear_WATERWALKING(&mut self) -> Self {
        self.inner &= MovementFlags::WATERWALKING.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_SAFE_FALL(&mut self) -> Self {
        self.inner |= MovementFlags::SAFE_FALL;
        self.clone()
    }

    pub fn clear_SAFE_FALL(&mut self) -> Self {
        self.inner &= MovementFlags::SAFE_FALL.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn set_HOVER(&mut self) -> Self {
        self.inner |= MovementFlags::HOVER;
        self.clone()
    }

    pub fn clear_HOVER(&mut self) -> Self {
        self.inner &= MovementFlags::HOVER.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for MovementInfoMovementFlags {
    fn size(&self) -> usize {
        4 // inner: MovementFlags (u32)
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

impl MaximumPossibleSized for MovementInfoMovementFlags {
    fn maximum_possible_size() -> usize {
        4 // inner: MovementFlags (u32)
        + MovementInfoMovementFlagsON_TRANSPORT::maximum_possible_size() // ON_TRANSPORT enumerator
        + MovementInfoMovementFlagsJUMPING::maximum_possible_size() // JUMPING enumerator
        + MovementInfoMovementFlagsSWIMMING::maximum_possible_size() // SWIMMING enumerator
        + MovementInfoMovementFlagsSPLINE_ELEVATION::maximum_possible_size() // SPLINE_ELEVATION enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementInfoMovementFlagsON_TRANSPORT {
    pub transport: TransportInfo,
}

impl VariableSized for MovementInfoMovementFlagsON_TRANSPORT {
    fn size(&self) -> usize {
        self.transport.size() // transport: TransportInfo
    }
}

impl MaximumPossibleSized for MovementInfoMovementFlagsON_TRANSPORT {
    fn maximum_possible_size() -> usize {
        TransportInfo::maximum_possible_size() // transport: TransportInfo
    }
}

impl MovementInfoMovementFlagsON_TRANSPORT {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.transport.write(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementInfoMovementFlagsJUMPING {
    pub z_speed: f32,
    pub cos_angle: f32,
    pub sin_angle: f32,
    pub xy_speed: f32,
}

impl VariableSized for MovementInfoMovementFlagsJUMPING {
    fn size(&self) -> usize {
        4 // z_speed: f32
        + 4 // cos_angle: f32
        + 4 // sin_angle: f32
        + 4 // xy_speed: f32
    }
}

impl MaximumPossibleSized for MovementInfoMovementFlagsJUMPING {
    fn maximum_possible_size() -> usize {
        4 // z_speed: f32
        + 4 // cos_angle: f32
        + 4 // sin_angle: f32
        + 4 // xy_speed: f32
    }
}

impl MovementInfoMovementFlagsJUMPING {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.z_speed.to_le_bytes())?;

        w.write_all(&self.cos_angle.to_le_bytes())?;

        w.write_all(&self.sin_angle.to_le_bytes())?;

        w.write_all(&self.xy_speed.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementInfoMovementFlagsSWIMMING {
    pub pitch: f32,
}

impl VariableSized for MovementInfoMovementFlagsSWIMMING {
    fn size(&self) -> usize {
        4 // pitch: f32
    }
}

impl MaximumPossibleSized for MovementInfoMovementFlagsSWIMMING {
    fn maximum_possible_size() -> usize {
        4 // pitch: f32
    }
}

impl MovementInfoMovementFlagsSWIMMING {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.pitch.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MovementInfoMovementFlagsSPLINE_ELEVATION {
    pub spline_elevation: f32,
}

impl VariableSized for MovementInfoMovementFlagsSPLINE_ELEVATION {
    fn size(&self) -> usize {
        4 // spline_elevation: f32
    }
}

impl MaximumPossibleSized for MovementInfoMovementFlagsSPLINE_ELEVATION {
    fn maximum_possible_size() -> usize {
        4 // spline_elevation: f32
    }
}

impl MovementInfoMovementFlagsSPLINE_ELEVATION {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.spline_elevation.to_le_bytes())?;

        Ok(())
    }
}

