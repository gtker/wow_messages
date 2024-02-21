/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm#L11):
/// ```text
/// flag MovementFlags : u48 {
///     NONE = 0x00000000;
///     FORWARD = 0x00000001;
///     BACKWARD = 0x00000002;
///     STRAFE_LEFT = 0x00000004;
///     STRAFE_RIGHT = 0x00000008;
///     LEFT = 0x00000010;
///     RIGHT = 0x00000020;
///     PITCH_UP = 0x00000040;
///     PITCH_DOWN = 0x00000080;
///     WALKING = 0x00000100;
///     ON_TRANSPORT = 0x00000200;
///     DISABLE_GRAVITY = 0x00000400;
///     ROOT = 0x00000800;
///     FALLING = 0x00001000;
///     FALLING_FAR = 0x00002000;
///     PENDING_STOP = 0x00004000;
///     PENDING_STRAFE_STOP = 0x00008000;
///     PENDING_FORWARD = 0x00010000;
///     PENDING_BACKWARD = 0x00020000;
///     PENDING_STRAFE_LEFT = 0x00040000;
///     PENDING_STRAFE_RIGHT = 0x00080000;
///     PENDING_ROOT = 0x00100000;
///     SWIMMING = 0x00200000;
///     ASCENDING = 0x00400000;
///     DESCENDING = 0x00800000;
///     CAN_FLY = 0x01000000;
///     FLYING = 0x02000000;
///     SPLINE_ELEVATION = 0x04000000;
///     SPLINE_ENABLED = 0x08000000;
///     WATERWALKING = 0x10000000;
///     FALLING_SLOW = 0x20000000;
///     HOVER = 0x40000000;
///     NO_STRAFE = 0x0000000100000000;
///     NO_JUMPING = 0x0000000200000000;
///     UNK3 = 0x0000000400000000;
///     FULL_SPEED_TURNING = 0x0000000800000000;
///     FULL_SPEED_PITCHING = 0x0000001000000000;
///     ALWAYS_ALLOW_PITCHING = 0x0000002000000000;
///     UNK7 = 0x0000004000000000;
///     UNK8 = 0x0000008000000000;
///     UNK9 = 0x0000010000000000;
///     UNK10 = 0x0000020000000000;
///     INTERPOLATED_MOVEMENT = 0x0000040000000000;
///     INTERPOLATED_TURNING = 0x0000080000000000;
///     INTERPOLATED_PITCHING = 0x0000100000000000;
///     UNK14 = 0x0000200000000000;
///     UNK15 = 0x0000400000000000;
///     UNK16 = 0x0000800000000000;
///     ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT = 0x0000040000000200;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct MovementFlags {
    inner: u64,
}

#[cfg(feature = "print-testcase")]
impl MovementFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_forward() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FORWARD").unwrap();
            first = false;
        }
        if self.is_backward() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "BACKWARD").unwrap();
            first = false;
        }
        if self.is_strafe_left() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "STRAFE_LEFT").unwrap();
            first = false;
        }
        if self.is_strafe_right() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "STRAFE_RIGHT").unwrap();
            first = false;
        }
        if self.is_left() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "LEFT").unwrap();
            first = false;
        }
        if self.is_right() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "RIGHT").unwrap();
            first = false;
        }
        if self.is_pitch_up() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PITCH_UP").unwrap();
            first = false;
        }
        if self.is_pitch_down() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PITCH_DOWN").unwrap();
            first = false;
        }
        if self.is_walking() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "WALKING").unwrap();
            first = false;
        }
        if self.is_on_transport() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ON_TRANSPORT").unwrap();
            first = false;
        }
        if self.is_disable_gravity() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DISABLE_GRAVITY").unwrap();
            first = false;
        }
        if self.is_root() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ROOT").unwrap();
            first = false;
        }
        if self.is_falling() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FALLING").unwrap();
            first = false;
        }
        if self.is_falling_far() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FALLING_FAR").unwrap();
            first = false;
        }
        if self.is_pending_stop() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PENDING_STOP").unwrap();
            first = false;
        }
        if self.is_pending_strafe_stop() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PENDING_STRAFE_STOP").unwrap();
            first = false;
        }
        if self.is_pending_forward() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PENDING_FORWARD").unwrap();
            first = false;
        }
        if self.is_pending_backward() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PENDING_BACKWARD").unwrap();
            first = false;
        }
        if self.is_pending_strafe_left() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PENDING_STRAFE_LEFT").unwrap();
            first = false;
        }
        if self.is_pending_strafe_right() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PENDING_STRAFE_RIGHT").unwrap();
            first = false;
        }
        if self.is_pending_root() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PENDING_ROOT").unwrap();
            first = false;
        }
        if self.is_swimming() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SWIMMING").unwrap();
            first = false;
        }
        if self.is_ascending() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ASCENDING").unwrap();
            first = false;
        }
        if self.is_descending() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DESCENDING").unwrap();
            first = false;
        }
        if self.is_can_fly() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CAN_FLY").unwrap();
            first = false;
        }
        if self.is_flying() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FLYING").unwrap();
            first = false;
        }
        if self.is_spline_elevation() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SPLINE_ELEVATION").unwrap();
            first = false;
        }
        if self.is_spline_enabled() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SPLINE_ENABLED").unwrap();
            first = false;
        }
        if self.is_waterwalking() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "WATERWALKING").unwrap();
            first = false;
        }
        if self.is_falling_slow() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FALLING_SLOW").unwrap();
            first = false;
        }
        if self.is_hover() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HOVER").unwrap();
            first = false;
        }
        if self.is_no_strafe() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_STRAFE").unwrap();
            first = false;
        }
        if self.is_no_jumping() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_JUMPING").unwrap();
            first = false;
        }
        if self.is_unk3() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK3").unwrap();
            first = false;
        }
        if self.is_full_speed_turning() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FULL_SPEED_TURNING").unwrap();
            first = false;
        }
        if self.is_full_speed_pitching() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FULL_SPEED_PITCHING").unwrap();
            first = false;
        }
        if self.is_always_allow_pitching() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ALWAYS_ALLOW_PITCHING").unwrap();
            first = false;
        }
        if self.is_unk7() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK7").unwrap();
            first = false;
        }
        if self.is_unk8() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK8").unwrap();
            first = false;
        }
        if self.is_unk9() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK9").unwrap();
            first = false;
        }
        if self.is_unk10() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK10").unwrap();
            first = false;
        }
        if self.is_interpolated_movement() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INTERPOLATED_MOVEMENT").unwrap();
            first = false;
        }
        if self.is_interpolated_turning() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INTERPOLATED_TURNING").unwrap();
            first = false;
        }
        if self.is_interpolated_pitching() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INTERPOLATED_PITCHING").unwrap();
            first = false;
        }
        if self.is_unk14() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK14").unwrap();
            first = false;
        }
        if self.is_unk15() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK15").unwrap();
            first = false;
        }
        if self.is_unk16() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK16").unwrap();
            first = false;
        }
        if self.is_on_transport_and_interpolated_movement() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT").unwrap();
            first = false;
        }
        s
    }

}

impl MovementFlags {
    pub const fn new(inner: u64) -> Self {
        Self { inner }
    }

    pub const NONE: u64 = 0x00;
    pub const FORWARD: u64 = 0x01;
    pub const BACKWARD: u64 = 0x02;
    pub const STRAFE_LEFT: u64 = 0x04;
    pub const STRAFE_RIGHT: u64 = 0x08;
    pub const LEFT: u64 = 0x10;
    pub const RIGHT: u64 = 0x20;
    pub const PITCH_UP: u64 = 0x40;
    pub const PITCH_DOWN: u64 = 0x80;
    pub const WALKING: u64 = 0x100;
    pub const ON_TRANSPORT: u64 = 0x200;
    pub const DISABLE_GRAVITY: u64 = 0x400;
    pub const ROOT: u64 = 0x800;
    pub const FALLING: u64 = 0x1000;
    pub const FALLING_FAR: u64 = 0x2000;
    pub const PENDING_STOP: u64 = 0x4000;
    pub const PENDING_STRAFE_STOP: u64 = 0x8000;
    pub const PENDING_FORWARD: u64 = 0x10000;
    pub const PENDING_BACKWARD: u64 = 0x20000;
    pub const PENDING_STRAFE_LEFT: u64 = 0x40000;
    pub const PENDING_STRAFE_RIGHT: u64 = 0x80000;
    pub const PENDING_ROOT: u64 = 0x100000;
    pub const SWIMMING: u64 = 0x200000;
    pub const ASCENDING: u64 = 0x400000;
    pub const DESCENDING: u64 = 0x800000;
    pub const CAN_FLY: u64 = 0x1000000;
    pub const FLYING: u64 = 0x2000000;
    pub const SPLINE_ELEVATION: u64 = 0x4000000;
    pub const SPLINE_ENABLED: u64 = 0x8000000;
    pub const WATERWALKING: u64 = 0x10000000;
    pub const FALLING_SLOW: u64 = 0x20000000;
    pub const HOVER: u64 = 0x40000000;
    pub const NO_STRAFE: u64 = 0x100000000;
    pub const NO_JUMPING: u64 = 0x200000000;
    pub const UNK3: u64 = 0x400000000;
    pub const FULL_SPEED_TURNING: u64 = 0x800000000;
    pub const FULL_SPEED_PITCHING: u64 = 0x1000000000;
    pub const ALWAYS_ALLOW_PITCHING: u64 = 0x2000000000;
    pub const UNK7: u64 = 0x4000000000;
    pub const UNK8: u64 = 0x8000000000;
    pub const UNK9: u64 = 0x10000000000;
    pub const UNK10: u64 = 0x20000000000;
    pub const INTERPOLATED_MOVEMENT: u64 = 0x40000000000;
    pub const INTERPOLATED_TURNING: u64 = 0x80000000000;
    pub const INTERPOLATED_PITCHING: u64 = 0x100000000000;
    pub const UNK14: u64 = 0x200000000000;
    pub const UNK15: u64 = 0x400000000000;
    pub const UNK16: u64 = 0x800000000000;
    pub const ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT: u64 = 0x40000000200;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::FORWARD
                | Self::BACKWARD
                | Self::STRAFE_LEFT
                | Self::STRAFE_RIGHT
                | Self::LEFT
                | Self::RIGHT
                | Self::PITCH_UP
                | Self::PITCH_DOWN
                | Self::WALKING
                | Self::ON_TRANSPORT
                | Self::DISABLE_GRAVITY
                | Self::ROOT
                | Self::FALLING
                | Self::FALLING_FAR
                | Self::PENDING_STOP
                | Self::PENDING_STRAFE_STOP
                | Self::PENDING_FORWARD
                | Self::PENDING_BACKWARD
                | Self::PENDING_STRAFE_LEFT
                | Self::PENDING_STRAFE_RIGHT
                | Self::PENDING_ROOT
                | Self::SWIMMING
                | Self::ASCENDING
                | Self::DESCENDING
                | Self::CAN_FLY
                | Self::FLYING
                | Self::SPLINE_ELEVATION
                | Self::SPLINE_ENABLED
                | Self::WATERWALKING
                | Self::FALLING_SLOW
                | Self::HOVER
                | Self::NO_STRAFE
                | Self::NO_JUMPING
                | Self::UNK3
                | Self::FULL_SPEED_TURNING
                | Self::FULL_SPEED_PITCHING
                | Self::ALWAYS_ALLOW_PITCHING
                | Self::UNK7
                | Self::UNK8
                | Self::UNK9
                | Self::UNK10
                | Self::INTERPOLATED_MOVEMENT
                | Self::INTERPOLATED_TURNING
                | Self::INTERPOLATED_PITCHING
                | Self::UNK14
                | Self::UNK15
                | Self::UNK16
                | Self::ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT
        }
    }

    pub const fn is_forward(&self) -> bool {
        (self.inner & Self::FORWARD) != 0
    }

    pub const fn new_forward() -> Self {
        Self { inner: Self::FORWARD }
    }

    pub fn set_forward(&mut self) -> Self {
        self.inner |= Self::FORWARD;
        *self
    }

    pub fn clear_forward(&mut self) -> Self {
        self.inner &= Self::FORWARD.reverse_bits();
        *self
    }

    pub const fn is_backward(&self) -> bool {
        (self.inner & Self::BACKWARD) != 0
    }

    pub const fn new_backward() -> Self {
        Self { inner: Self::BACKWARD }
    }

    pub fn set_backward(&mut self) -> Self {
        self.inner |= Self::BACKWARD;
        *self
    }

    pub fn clear_backward(&mut self) -> Self {
        self.inner &= Self::BACKWARD.reverse_bits();
        *self
    }

    pub const fn is_strafe_left(&self) -> bool {
        (self.inner & Self::STRAFE_LEFT) != 0
    }

    pub const fn new_strafe_left() -> Self {
        Self { inner: Self::STRAFE_LEFT }
    }

    pub fn set_strafe_left(&mut self) -> Self {
        self.inner |= Self::STRAFE_LEFT;
        *self
    }

    pub fn clear_strafe_left(&mut self) -> Self {
        self.inner &= Self::STRAFE_LEFT.reverse_bits();
        *self
    }

    pub const fn is_strafe_right(&self) -> bool {
        (self.inner & Self::STRAFE_RIGHT) != 0
    }

    pub const fn new_strafe_right() -> Self {
        Self { inner: Self::STRAFE_RIGHT }
    }

    pub fn set_strafe_right(&mut self) -> Self {
        self.inner |= Self::STRAFE_RIGHT;
        *self
    }

    pub fn clear_strafe_right(&mut self) -> Self {
        self.inner &= Self::STRAFE_RIGHT.reverse_bits();
        *self
    }

    pub const fn is_left(&self) -> bool {
        (self.inner & Self::LEFT) != 0
    }

    pub const fn new_left() -> Self {
        Self { inner: Self::LEFT }
    }

    pub fn set_left(&mut self) -> Self {
        self.inner |= Self::LEFT;
        *self
    }

    pub fn clear_left(&mut self) -> Self {
        self.inner &= Self::LEFT.reverse_bits();
        *self
    }

    pub const fn is_right(&self) -> bool {
        (self.inner & Self::RIGHT) != 0
    }

    pub const fn new_right() -> Self {
        Self { inner: Self::RIGHT }
    }

    pub fn set_right(&mut self) -> Self {
        self.inner |= Self::RIGHT;
        *self
    }

    pub fn clear_right(&mut self) -> Self {
        self.inner &= Self::RIGHT.reverse_bits();
        *self
    }

    pub const fn is_pitch_up(&self) -> bool {
        (self.inner & Self::PITCH_UP) != 0
    }

    pub const fn new_pitch_up() -> Self {
        Self { inner: Self::PITCH_UP }
    }

    pub fn set_pitch_up(&mut self) -> Self {
        self.inner |= Self::PITCH_UP;
        *self
    }

    pub fn clear_pitch_up(&mut self) -> Self {
        self.inner &= Self::PITCH_UP.reverse_bits();
        *self
    }

    pub const fn is_pitch_down(&self) -> bool {
        (self.inner & Self::PITCH_DOWN) != 0
    }

    pub const fn new_pitch_down() -> Self {
        Self { inner: Self::PITCH_DOWN }
    }

    pub fn set_pitch_down(&mut self) -> Self {
        self.inner |= Self::PITCH_DOWN;
        *self
    }

    pub fn clear_pitch_down(&mut self) -> Self {
        self.inner &= Self::PITCH_DOWN.reverse_bits();
        *self
    }

    pub const fn is_walking(&self) -> bool {
        (self.inner & Self::WALKING) != 0
    }

    /// Walking
    pub const fn new_walking() -> Self {
        Self { inner: Self::WALKING }
    }

    pub fn set_walking(&mut self) -> Self {
        self.inner |= Self::WALKING;
        *self
    }

    pub fn clear_walking(&mut self) -> Self {
        self.inner &= Self::WALKING.reverse_bits();
        *self
    }

    pub const fn is_on_transport(&self) -> bool {
        (self.inner & Self::ON_TRANSPORT) != 0
    }

    /// `AzerothCore`: Used for flying on some creatures
    pub const fn new_on_transport() -> Self {
        Self { inner: Self::ON_TRANSPORT }
    }

    pub fn set_on_transport(&mut self) -> Self {
        self.inner |= Self::ON_TRANSPORT;
        *self
    }

    pub fn clear_on_transport(&mut self) -> Self {
        self.inner &= Self::ON_TRANSPORT.reverse_bits();
        *self
    }

    pub const fn is_disable_gravity(&self) -> bool {
        (self.inner & Self::DISABLE_GRAVITY) != 0
    }

    /// `AzerothCore`: Former `MOVEMENTFLAG_LEVITATING`. This is used when walking is not possible.
    pub const fn new_disable_gravity() -> Self {
        Self { inner: Self::DISABLE_GRAVITY }
    }

    pub fn set_disable_gravity(&mut self) -> Self {
        self.inner |= Self::DISABLE_GRAVITY;
        *self
    }

    pub fn clear_disable_gravity(&mut self) -> Self {
        self.inner &= Self::DISABLE_GRAVITY.reverse_bits();
        *self
    }

    pub const fn is_root(&self) -> bool {
        (self.inner & Self::ROOT) != 0
    }

    /// `AzerothCore`: Must not be set along with `MOVEMENTFLAG_MASK_MOVING`
    pub const fn new_root() -> Self {
        Self { inner: Self::ROOT }
    }

    pub fn set_root(&mut self) -> Self {
        self.inner |= Self::ROOT;
        *self
    }

    pub fn clear_root(&mut self) -> Self {
        self.inner &= Self::ROOT.reverse_bits();
        *self
    }

    pub const fn is_falling(&self) -> bool {
        (self.inner & Self::FALLING) != 0
    }

    /// `AzerothCore`: damage dealt on that type of falling
    pub const fn new_falling() -> Self {
        Self { inner: Self::FALLING }
    }

    pub fn set_falling(&mut self) -> Self {
        self.inner |= Self::FALLING;
        *self
    }

    pub fn clear_falling(&mut self) -> Self {
        self.inner &= Self::FALLING.reverse_bits();
        *self
    }

    pub const fn is_falling_far(&self) -> bool {
        (self.inner & Self::FALLING_FAR) != 0
    }

    pub const fn new_falling_far() -> Self {
        Self { inner: Self::FALLING_FAR }
    }

    pub fn set_falling_far(&mut self) -> Self {
        self.inner |= Self::FALLING_FAR;
        *self
    }

    pub fn clear_falling_far(&mut self) -> Self {
        self.inner &= Self::FALLING_FAR.reverse_bits();
        *self
    }

    pub const fn is_pending_stop(&self) -> bool {
        (self.inner & Self::PENDING_STOP) != 0
    }

    pub const fn new_pending_stop() -> Self {
        Self { inner: Self::PENDING_STOP }
    }

    pub fn set_pending_stop(&mut self) -> Self {
        self.inner |= Self::PENDING_STOP;
        *self
    }

    pub fn clear_pending_stop(&mut self) -> Self {
        self.inner &= Self::PENDING_STOP.reverse_bits();
        *self
    }

    pub const fn is_pending_strafe_stop(&self) -> bool {
        (self.inner & Self::PENDING_STRAFE_STOP) != 0
    }

    pub const fn new_pending_strafe_stop() -> Self {
        Self { inner: Self::PENDING_STRAFE_STOP }
    }

    pub fn set_pending_strafe_stop(&mut self) -> Self {
        self.inner |= Self::PENDING_STRAFE_STOP;
        *self
    }

    pub fn clear_pending_strafe_stop(&mut self) -> Self {
        self.inner &= Self::PENDING_STRAFE_STOP.reverse_bits();
        *self
    }

    pub const fn is_pending_forward(&self) -> bool {
        (self.inner & Self::PENDING_FORWARD) != 0
    }

    pub const fn new_pending_forward() -> Self {
        Self { inner: Self::PENDING_FORWARD }
    }

    pub fn set_pending_forward(&mut self) -> Self {
        self.inner |= Self::PENDING_FORWARD;
        *self
    }

    pub fn clear_pending_forward(&mut self) -> Self {
        self.inner &= Self::PENDING_FORWARD.reverse_bits();
        *self
    }

    pub const fn is_pending_backward(&self) -> bool {
        (self.inner & Self::PENDING_BACKWARD) != 0
    }

    pub const fn new_pending_backward() -> Self {
        Self { inner: Self::PENDING_BACKWARD }
    }

    pub fn set_pending_backward(&mut self) -> Self {
        self.inner |= Self::PENDING_BACKWARD;
        *self
    }

    pub fn clear_pending_backward(&mut self) -> Self {
        self.inner &= Self::PENDING_BACKWARD.reverse_bits();
        *self
    }

    pub const fn is_pending_strafe_left(&self) -> bool {
        (self.inner & Self::PENDING_STRAFE_LEFT) != 0
    }

    pub const fn new_pending_strafe_left() -> Self {
        Self { inner: Self::PENDING_STRAFE_LEFT }
    }

    pub fn set_pending_strafe_left(&mut self) -> Self {
        self.inner |= Self::PENDING_STRAFE_LEFT;
        *self
    }

    pub fn clear_pending_strafe_left(&mut self) -> Self {
        self.inner &= Self::PENDING_STRAFE_LEFT.reverse_bits();
        *self
    }

    pub const fn is_pending_strafe_right(&self) -> bool {
        (self.inner & Self::PENDING_STRAFE_RIGHT) != 0
    }

    pub const fn new_pending_strafe_right() -> Self {
        Self { inner: Self::PENDING_STRAFE_RIGHT }
    }

    pub fn set_pending_strafe_right(&mut self) -> Self {
        self.inner |= Self::PENDING_STRAFE_RIGHT;
        *self
    }

    pub fn clear_pending_strafe_right(&mut self) -> Self {
        self.inner &= Self::PENDING_STRAFE_RIGHT.reverse_bits();
        *self
    }

    pub const fn is_pending_root(&self) -> bool {
        (self.inner & Self::PENDING_ROOT) != 0
    }

    pub const fn new_pending_root() -> Self {
        Self { inner: Self::PENDING_ROOT }
    }

    pub fn set_pending_root(&mut self) -> Self {
        self.inner |= Self::PENDING_ROOT;
        *self
    }

    pub fn clear_pending_root(&mut self) -> Self {
        self.inner &= Self::PENDING_ROOT.reverse_bits();
        *self
    }

    pub const fn is_swimming(&self) -> bool {
        (self.inner & Self::SWIMMING) != 0
    }

    /// `AzerothCore`: appears with fly flag also
    pub const fn new_swimming() -> Self {
        Self { inner: Self::SWIMMING }
    }

    pub fn set_swimming(&mut self) -> Self {
        self.inner |= Self::SWIMMING;
        *self
    }

    pub fn clear_swimming(&mut self) -> Self {
        self.inner &= Self::SWIMMING.reverse_bits();
        *self
    }

    pub const fn is_ascending(&self) -> bool {
        (self.inner & Self::ASCENDING) != 0
    }

    /// `AzerothCore`: press 'space' when flying
    pub const fn new_ascending() -> Self {
        Self { inner: Self::ASCENDING }
    }

    pub fn set_ascending(&mut self) -> Self {
        self.inner |= Self::ASCENDING;
        *self
    }

    pub fn clear_ascending(&mut self) -> Self {
        self.inner &= Self::ASCENDING.reverse_bits();
        *self
    }

    pub const fn is_descending(&self) -> bool {
        (self.inner & Self::DESCENDING) != 0
    }

    pub const fn new_descending() -> Self {
        Self { inner: Self::DESCENDING }
    }

    pub fn set_descending(&mut self) -> Self {
        self.inner |= Self::DESCENDING;
        *self
    }

    pub fn clear_descending(&mut self) -> Self {
        self.inner &= Self::DESCENDING.reverse_bits();
        *self
    }

    pub const fn is_can_fly(&self) -> bool {
        (self.inner & Self::CAN_FLY) != 0
    }

    /// `AzerothCore`: Appears when unit can fly AND also walk
    pub const fn new_can_fly() -> Self {
        Self { inner: Self::CAN_FLY }
    }

    pub fn set_can_fly(&mut self) -> Self {
        self.inner |= Self::CAN_FLY;
        *self
    }

    pub fn clear_can_fly(&mut self) -> Self {
        self.inner &= Self::CAN_FLY.reverse_bits();
        *self
    }

    pub const fn is_flying(&self) -> bool {
        (self.inner & Self::FLYING) != 0
    }

    /// `AzerothCore`: unit is actually flying. pretty sure this is only used for players. creatures use `disable_gravity`
    pub const fn new_flying() -> Self {
        Self { inner: Self::FLYING }
    }

    pub fn set_flying(&mut self) -> Self {
        self.inner |= Self::FLYING;
        *self
    }

    pub fn clear_flying(&mut self) -> Self {
        self.inner &= Self::FLYING.reverse_bits();
        *self
    }

    pub const fn is_spline_elevation(&self) -> bool {
        (self.inner & Self::SPLINE_ELEVATION) != 0
    }

    /// `AzerothCore`: used for flight paths
    pub const fn new_spline_elevation() -> Self {
        Self { inner: Self::SPLINE_ELEVATION }
    }

    pub fn set_spline_elevation(&mut self) -> Self {
        self.inner |= Self::SPLINE_ELEVATION;
        *self
    }

    pub fn clear_spline_elevation(&mut self) -> Self {
        self.inner &= Self::SPLINE_ELEVATION.reverse_bits();
        *self
    }

    pub const fn is_spline_enabled(&self) -> bool {
        (self.inner & Self::SPLINE_ENABLED) != 0
    }

    /// `AzerothCore`: used for flight paths
    pub const fn new_spline_enabled() -> Self {
        Self { inner: Self::SPLINE_ENABLED }
    }

    pub fn set_spline_enabled(&mut self) -> Self {
        self.inner |= Self::SPLINE_ENABLED;
        *self
    }

    pub fn clear_spline_enabled(&mut self) -> Self {
        self.inner &= Self::SPLINE_ENABLED.reverse_bits();
        *self
    }

    pub const fn is_waterwalking(&self) -> bool {
        (self.inner & Self::WATERWALKING) != 0
    }

    /// `AzerothCore`: prevent unit from falling through water
    pub const fn new_waterwalking() -> Self {
        Self { inner: Self::WATERWALKING }
    }

    pub fn set_waterwalking(&mut self) -> Self {
        self.inner |= Self::WATERWALKING;
        *self
    }

    pub fn clear_waterwalking(&mut self) -> Self {
        self.inner &= Self::WATERWALKING.reverse_bits();
        *self
    }

    pub const fn is_falling_slow(&self) -> bool {
        (self.inner & Self::FALLING_SLOW) != 0
    }

    /// `AzerothCore`: active rogue safe fall spell (passive)
    pub const fn new_falling_slow() -> Self {
        Self { inner: Self::FALLING_SLOW }
    }

    pub fn set_falling_slow(&mut self) -> Self {
        self.inner |= Self::FALLING_SLOW;
        *self
    }

    pub fn clear_falling_slow(&mut self) -> Self {
        self.inner &= Self::FALLING_SLOW.reverse_bits();
        *self
    }

    pub const fn is_hover(&self) -> bool {
        (self.inner & Self::HOVER) != 0
    }

    /// `AzerothCore`: hover, cannot jump
    pub const fn new_hover() -> Self {
        Self { inner: Self::HOVER }
    }

    pub fn set_hover(&mut self) -> Self {
        self.inner |= Self::HOVER;
        *self
    }

    pub fn clear_hover(&mut self) -> Self {
        self.inner &= Self::HOVER.reverse_bits();
        *self
    }

    pub const fn is_no_strafe(&self) -> bool {
        (self.inner & Self::NO_STRAFE) != 0
    }

    pub const fn new_no_strafe() -> Self {
        Self { inner: Self::NO_STRAFE }
    }

    pub fn set_no_strafe(&mut self) -> Self {
        self.inner |= Self::NO_STRAFE;
        *self
    }

    pub fn clear_no_strafe(&mut self) -> Self {
        self.inner &= Self::NO_STRAFE.reverse_bits();
        *self
    }

    pub const fn is_no_jumping(&self) -> bool {
        (self.inner & Self::NO_JUMPING) != 0
    }

    pub const fn new_no_jumping() -> Self {
        Self { inner: Self::NO_JUMPING }
    }

    pub fn set_no_jumping(&mut self) -> Self {
        self.inner |= Self::NO_JUMPING;
        *self
    }

    pub fn clear_no_jumping(&mut self) -> Self {
        self.inner &= Self::NO_JUMPING.reverse_bits();
        *self
    }

    pub const fn is_unk3(&self) -> bool {
        (self.inner & Self::UNK3) != 0
    }

    /// `AzerothCore`: Overrides various clientside checks
    pub const fn new_unk3() -> Self {
        Self { inner: Self::UNK3 }
    }

    pub fn set_unk3(&mut self) -> Self {
        self.inner |= Self::UNK3;
        *self
    }

    pub fn clear_unk3(&mut self) -> Self {
        self.inner &= Self::UNK3.reverse_bits();
        *self
    }

    pub const fn is_full_speed_turning(&self) -> bool {
        (self.inner & Self::FULL_SPEED_TURNING) != 0
    }

    pub const fn new_full_speed_turning() -> Self {
        Self { inner: Self::FULL_SPEED_TURNING }
    }

    pub fn set_full_speed_turning(&mut self) -> Self {
        self.inner |= Self::FULL_SPEED_TURNING;
        *self
    }

    pub fn clear_full_speed_turning(&mut self) -> Self {
        self.inner &= Self::FULL_SPEED_TURNING.reverse_bits();
        *self
    }

    pub const fn is_full_speed_pitching(&self) -> bool {
        (self.inner & Self::FULL_SPEED_PITCHING) != 0
    }

    pub const fn new_full_speed_pitching() -> Self {
        Self { inner: Self::FULL_SPEED_PITCHING }
    }

    pub fn set_full_speed_pitching(&mut self) -> Self {
        self.inner |= Self::FULL_SPEED_PITCHING;
        *self
    }

    pub fn clear_full_speed_pitching(&mut self) -> Self {
        self.inner &= Self::FULL_SPEED_PITCHING.reverse_bits();
        *self
    }

    pub const fn is_always_allow_pitching(&self) -> bool {
        (self.inner & Self::ALWAYS_ALLOW_PITCHING) != 0
    }

    pub const fn new_always_allow_pitching() -> Self {
        Self { inner: Self::ALWAYS_ALLOW_PITCHING }
    }

    pub fn set_always_allow_pitching(&mut self) -> Self {
        self.inner |= Self::ALWAYS_ALLOW_PITCHING;
        *self
    }

    pub fn clear_always_allow_pitching(&mut self) -> Self {
        self.inner &= Self::ALWAYS_ALLOW_PITCHING.reverse_bits();
        *self
    }

    pub const fn is_unk7(&self) -> bool {
        (self.inner & Self::UNK7) != 0
    }

    pub const fn new_unk7() -> Self {
        Self { inner: Self::UNK7 }
    }

    pub fn set_unk7(&mut self) -> Self {
        self.inner |= Self::UNK7;
        *self
    }

    pub fn clear_unk7(&mut self) -> Self {
        self.inner &= Self::UNK7.reverse_bits();
        *self
    }

    pub const fn is_unk8(&self) -> bool {
        (self.inner & Self::UNK8) != 0
    }

    pub const fn new_unk8() -> Self {
        Self { inner: Self::UNK8 }
    }

    pub fn set_unk8(&mut self) -> Self {
        self.inner |= Self::UNK8;
        *self
    }

    pub fn clear_unk8(&mut self) -> Self {
        self.inner &= Self::UNK8.reverse_bits();
        *self
    }

    pub const fn is_unk9(&self) -> bool {
        (self.inner & Self::UNK9) != 0
    }

    pub const fn new_unk9() -> Self {
        Self { inner: Self::UNK9 }
    }

    pub fn set_unk9(&mut self) -> Self {
        self.inner |= Self::UNK9;
        *self
    }

    pub fn clear_unk9(&mut self) -> Self {
        self.inner &= Self::UNK9.reverse_bits();
        *self
    }

    pub const fn is_unk10(&self) -> bool {
        (self.inner & Self::UNK10) != 0
    }

    pub const fn new_unk10() -> Self {
        Self { inner: Self::UNK10 }
    }

    pub fn set_unk10(&mut self) -> Self {
        self.inner |= Self::UNK10;
        *self
    }

    pub fn clear_unk10(&mut self) -> Self {
        self.inner &= Self::UNK10.reverse_bits();
        *self
    }

    pub const fn is_interpolated_movement(&self) -> bool {
        (self.inner & Self::INTERPOLATED_MOVEMENT) != 0
    }

    pub const fn new_interpolated_movement() -> Self {
        Self { inner: Self::INTERPOLATED_MOVEMENT }
    }

    pub fn set_interpolated_movement(&mut self) -> Self {
        self.inner |= Self::INTERPOLATED_MOVEMENT;
        *self
    }

    pub fn clear_interpolated_movement(&mut self) -> Self {
        self.inner &= Self::INTERPOLATED_MOVEMENT.reverse_bits();
        *self
    }

    pub const fn is_interpolated_turning(&self) -> bool {
        (self.inner & Self::INTERPOLATED_TURNING) != 0
    }

    pub const fn new_interpolated_turning() -> Self {
        Self { inner: Self::INTERPOLATED_TURNING }
    }

    pub fn set_interpolated_turning(&mut self) -> Self {
        self.inner |= Self::INTERPOLATED_TURNING;
        *self
    }

    pub fn clear_interpolated_turning(&mut self) -> Self {
        self.inner &= Self::INTERPOLATED_TURNING.reverse_bits();
        *self
    }

    pub const fn is_interpolated_pitching(&self) -> bool {
        (self.inner & Self::INTERPOLATED_PITCHING) != 0
    }

    pub const fn new_interpolated_pitching() -> Self {
        Self { inner: Self::INTERPOLATED_PITCHING }
    }

    pub fn set_interpolated_pitching(&mut self) -> Self {
        self.inner |= Self::INTERPOLATED_PITCHING;
        *self
    }

    pub fn clear_interpolated_pitching(&mut self) -> Self {
        self.inner &= Self::INTERPOLATED_PITCHING.reverse_bits();
        *self
    }

    pub const fn is_unk14(&self) -> bool {
        (self.inner & Self::UNK14) != 0
    }

    pub const fn new_unk14() -> Self {
        Self { inner: Self::UNK14 }
    }

    pub fn set_unk14(&mut self) -> Self {
        self.inner |= Self::UNK14;
        *self
    }

    pub fn clear_unk14(&mut self) -> Self {
        self.inner &= Self::UNK14.reverse_bits();
        *self
    }

    pub const fn is_unk15(&self) -> bool {
        (self.inner & Self::UNK15) != 0
    }

    pub const fn new_unk15() -> Self {
        Self { inner: Self::UNK15 }
    }

    pub fn set_unk15(&mut self) -> Self {
        self.inner |= Self::UNK15;
        *self
    }

    pub fn clear_unk15(&mut self) -> Self {
        self.inner &= Self::UNK15.reverse_bits();
        *self
    }

    pub const fn is_unk16(&self) -> bool {
        (self.inner & Self::UNK16) != 0
    }

    pub const fn new_unk16() -> Self {
        Self { inner: Self::UNK16 }
    }

    pub fn set_unk16(&mut self) -> Self {
        self.inner |= Self::UNK16;
        *self
    }

    pub fn clear_unk16(&mut self) -> Self {
        self.inner &= Self::UNK16.reverse_bits();
        *self
    }

    pub const fn is_on_transport_and_interpolated_movement(&self) -> bool {
        (self.inner & Self::ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT) != 0
    }

    pub const fn new_on_transport_and_interpolated_movement() -> Self {
        Self { inner: Self::ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT }
    }

    pub fn set_on_transport_and_interpolated_movement(&mut self) -> Self {
        self.inner |= Self::ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT;
        *self
    }

    pub fn clear_on_transport_and_interpolated_movement(&mut self) -> Self {
        self.inner &= Self::ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u64 {
        self.inner
    }

}

impl std::fmt::UpperHex for MovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for MovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for MovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for MovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for MovementFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for MovementFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for MovementFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for MovementFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for MovementFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for MovementFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u64> for MovementFlags {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<u8> for MovementFlags {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for MovementFlags {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl From<u32> for MovementFlags {
    fn from(value: u32) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<i8> for MovementFlags {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl TryFrom<i16> for MovementFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl TryFrom<i32> for MovementFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl TryFrom<i64> for MovementFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u64>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for MovementFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u64>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

