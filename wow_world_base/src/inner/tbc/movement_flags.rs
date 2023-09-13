/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_2_4_3.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_2_4_3.wowm#L3):
/// ```text
/// flag MovementFlags : u32 {
///     NONE = 0x00000000;
///     FORWARD = 0x00000001;
///     BACKWARD = 0x00000002;
///     STRAFE_LEFT = 0x00000004;
///     STRAFE_RIGHT = 0x00000008;
///     TURN_LEFT = 0x00000010;
///     TURN_RIGHT = 0x00000020;
///     PITCH_UP = 0x00000040;
///     PITCH_DOWN = 0x00000080;
///     WALK_MODE = 0x00000100;
///     ON_TRANSPORT = 0x00000200;
///     LEVITATING = 0x00000400;
///     FIXED_Z = 0x00000800;
///     ROOT = 0x00001000;
///     JUMPING = 0x00002000;
///     FALLINGFAR = 0x00004000;
///     SWIMMING = 0x00200000;
///     ASCENDING = 0x00400000;
///     CAN_FLY = 0x00800000;
///     FLYING = 0x01000000;
///     ONTRANSPORT = 0x02000000;
///     SPLINE_ELEVATION = 0x04000000;
///     SPLINE_ENABLED = 0x08000000;
///     WATERWALKING = 0x10000000;
///     SAFE_FALL = 0x20000000;
///     HOVER = 0x40000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MovementFlags {
    inner: u32,
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
        if self.is_turn_left() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TURN_LEFT").unwrap();
            first = false;
        }
        if self.is_turn_right() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TURN_RIGHT").unwrap();
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
        if self.is_walk_mode() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "WALK_MODE").unwrap();
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
        if self.is_levitating() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "LEVITATING").unwrap();
            first = false;
        }
        if self.is_fixed_z() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FIXED_Z").unwrap();
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
        if self.is_jumping() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "JUMPING").unwrap();
            first = false;
        }
        if self.is_fallingfar() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FALLINGFAR").unwrap();
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
        if self.is_ontransport() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ONTRANSPORT").unwrap();
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
        if self.is_safe_fall() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SAFE_FALL").unwrap();
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
        s
    }

}

impl MovementFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const FORWARD: u32 = 0x01;
    pub const BACKWARD: u32 = 0x02;
    pub const STRAFE_LEFT: u32 = 0x04;
    pub const STRAFE_RIGHT: u32 = 0x08;
    pub const TURN_LEFT: u32 = 0x10;
    pub const TURN_RIGHT: u32 = 0x20;
    pub const PITCH_UP: u32 = 0x40;
    pub const PITCH_DOWN: u32 = 0x80;
    pub const WALK_MODE: u32 = 0x100;
    pub const ON_TRANSPORT: u32 = 0x200;
    pub const LEVITATING: u32 = 0x400;
    pub const FIXED_Z: u32 = 0x800;
    pub const ROOT: u32 = 0x1000;
    pub const JUMPING: u32 = 0x2000;
    pub const FALLINGFAR: u32 = 0x4000;
    pub const SWIMMING: u32 = 0x200000;
    pub const ASCENDING: u32 = 0x400000;
    pub const CAN_FLY: u32 = 0x800000;
    pub const FLYING: u32 = 0x1000000;
    pub const ONTRANSPORT: u32 = 0x2000000;
    pub const SPLINE_ELEVATION: u32 = 0x4000000;
    pub const SPLINE_ENABLED: u32 = 0x8000000;
    pub const WATERWALKING: u32 = 0x10000000;
    pub const SAFE_FALL: u32 = 0x20000000;
    pub const HOVER: u32 = 0x40000000;

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
                | Self::TURN_LEFT
                | Self::TURN_RIGHT
                | Self::PITCH_UP
                | Self::PITCH_DOWN
                | Self::WALK_MODE
                | Self::ON_TRANSPORT
                | Self::LEVITATING
                | Self::FIXED_Z
                | Self::ROOT
                | Self::JUMPING
                | Self::FALLINGFAR
                | Self::SWIMMING
                | Self::ASCENDING
                | Self::CAN_FLY
                | Self::FLYING
                | Self::ONTRANSPORT
                | Self::SPLINE_ELEVATION
                | Self::SPLINE_ENABLED
                | Self::WATERWALKING
                | Self::SAFE_FALL
                | Self::HOVER
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

    pub const fn is_turn_left(&self) -> bool {
        (self.inner & Self::TURN_LEFT) != 0
    }

    pub const fn new_turn_left() -> Self {
        Self { inner: Self::TURN_LEFT }
    }

    pub fn set_turn_left(&mut self) -> Self {
        self.inner |= Self::TURN_LEFT;
        *self
    }

    pub fn clear_turn_left(&mut self) -> Self {
        self.inner &= Self::TURN_LEFT.reverse_bits();
        *self
    }

    pub const fn is_turn_right(&self) -> bool {
        (self.inner & Self::TURN_RIGHT) != 0
    }

    pub const fn new_turn_right() -> Self {
        Self { inner: Self::TURN_RIGHT }
    }

    pub fn set_turn_right(&mut self) -> Self {
        self.inner |= Self::TURN_RIGHT;
        *self
    }

    pub fn clear_turn_right(&mut self) -> Self {
        self.inner &= Self::TURN_RIGHT.reverse_bits();
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

    pub const fn is_walk_mode(&self) -> bool {
        (self.inner & Self::WALK_MODE) != 0
    }

    pub const fn new_walk_mode() -> Self {
        Self { inner: Self::WALK_MODE }
    }

    pub fn set_walk_mode(&mut self) -> Self {
        self.inner |= Self::WALK_MODE;
        *self
    }

    pub fn clear_walk_mode(&mut self) -> Self {
        self.inner &= Self::WALK_MODE.reverse_bits();
        *self
    }

    pub const fn is_on_transport(&self) -> bool {
        (self.inner & Self::ON_TRANSPORT) != 0
    }

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

    pub const fn is_levitating(&self) -> bool {
        (self.inner & Self::LEVITATING) != 0
    }

    pub const fn new_levitating() -> Self {
        Self { inner: Self::LEVITATING }
    }

    pub fn set_levitating(&mut self) -> Self {
        self.inner |= Self::LEVITATING;
        *self
    }

    pub fn clear_levitating(&mut self) -> Self {
        self.inner &= Self::LEVITATING.reverse_bits();
        *self
    }

    pub const fn is_fixed_z(&self) -> bool {
        (self.inner & Self::FIXED_Z) != 0
    }

    pub const fn new_fixed_z() -> Self {
        Self { inner: Self::FIXED_Z }
    }

    pub fn set_fixed_z(&mut self) -> Self {
        self.inner |= Self::FIXED_Z;
        *self
    }

    pub fn clear_fixed_z(&mut self) -> Self {
        self.inner &= Self::FIXED_Z.reverse_bits();
        *self
    }

    pub const fn is_root(&self) -> bool {
        (self.inner & Self::ROOT) != 0
    }

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

    pub const fn is_jumping(&self) -> bool {
        (self.inner & Self::JUMPING) != 0
    }

    pub const fn new_jumping() -> Self {
        Self { inner: Self::JUMPING }
    }

    pub fn set_jumping(&mut self) -> Self {
        self.inner |= Self::JUMPING;
        *self
    }

    pub fn clear_jumping(&mut self) -> Self {
        self.inner &= Self::JUMPING.reverse_bits();
        *self
    }

    pub const fn is_fallingfar(&self) -> bool {
        (self.inner & Self::FALLINGFAR) != 0
    }

    pub const fn new_fallingfar() -> Self {
        Self { inner: Self::FALLINGFAR }
    }

    pub fn set_fallingfar(&mut self) -> Self {
        self.inner |= Self::FALLINGFAR;
        *self
    }

    pub fn clear_fallingfar(&mut self) -> Self {
        self.inner &= Self::FALLINGFAR.reverse_bits();
        *self
    }

    pub const fn is_swimming(&self) -> bool {
        (self.inner & Self::SWIMMING) != 0
    }

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

    pub const fn is_can_fly(&self) -> bool {
        (self.inner & Self::CAN_FLY) != 0
    }

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

    pub const fn is_ontransport(&self) -> bool {
        (self.inner & Self::ONTRANSPORT) != 0
    }

    pub const fn new_ontransport() -> Self {
        Self { inner: Self::ONTRANSPORT }
    }

    pub fn set_ontransport(&mut self) -> Self {
        self.inner |= Self::ONTRANSPORT;
        *self
    }

    pub fn clear_ontransport(&mut self) -> Self {
        self.inner &= Self::ONTRANSPORT.reverse_bits();
        *self
    }

    pub const fn is_spline_elevation(&self) -> bool {
        (self.inner & Self::SPLINE_ELEVATION) != 0
    }

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

    pub const fn is_safe_fall(&self) -> bool {
        (self.inner & Self::SAFE_FALL) != 0
    }

    pub const fn new_safe_fall() -> Self {
        Self { inner: Self::SAFE_FALL }
    }

    pub fn set_safe_fall(&mut self) -> Self {
        self.inner |= Self::SAFE_FALL;
        *self
    }

    pub fn clear_safe_fall(&mut self) -> Self {
        self.inner &= Self::SAFE_FALL.reverse_bits();
        *self
    }

    pub const fn is_hover(&self) -> bool {
        (self.inner & Self::HOVER) != 0
    }

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

    pub const fn as_int(&self) -> u32 {
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

impl From<u32> for MovementFlags {
    fn from(value: u32) -> Self {
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

impl TryFrom<u64> for MovementFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for MovementFlags {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for MovementFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for MovementFlags {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for MovementFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for MovementFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

