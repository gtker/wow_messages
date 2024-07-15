/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L26):
/// ```text
/// flag SplineFlag : u32 {
///     NONE = 0x00000000;
///     DONE = 0x00000100;
///     FALLING = 0x00000200;
///     NO_SPLINE = 0x00000400;
///     PARABOLIC = 0x00000800;
///     WALK_MODE = 0x00001000;
///     FLYING = 0x00002000;
///     ORIENTATION_FIXED = 0x00004000;
///     FINAL_POINT = 0x00008000;
///     FINAL_TARGET = 0x00010000;
///     FINAL_ANGLE = 0x00020000;
///     CATMULLROM = 0x00040000;
///     CYCLIC = 0x00080000;
///     ENTER_CYCLE = 0x00100000;
///     ANIMATION = 0x00200000;
///     FROZEN = 0x00400000;
///     TRANSPORT_ENTER = 0x00800000;
///     TRANSPORT_EXIT = 0x01000000;
///     UNKNOWN7 = 0x02000000;
///     UNKNOWN8 = 0x04000000;
///     ORIENTATION_INVERSED = 0x08000000;
///     UNKNOWN10 = 0x10000000;
///     UNKNOWN11 = 0x20000000;
///     UNKNOWN12 = 0x40000000;
///     UNKNOWN13 = 0x80000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct SplineFlag {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl SplineFlag {
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
        if self.is_done() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DONE").unwrap();
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
        if self.is_no_spline() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_SPLINE").unwrap();
            first = false;
        }
        if self.is_parabolic() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PARABOLIC").unwrap();
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
        if self.is_flying() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FLYING").unwrap();
            first = false;
        }
        if self.is_orientation_fixed() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ORIENTATION_FIXED").unwrap();
            first = false;
        }
        if self.is_final_point() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FINAL_POINT").unwrap();
            first = false;
        }
        if self.is_final_target() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FINAL_TARGET").unwrap();
            first = false;
        }
        if self.is_final_angle() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FINAL_ANGLE").unwrap();
            first = false;
        }
        if self.is_catmullrom() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CATMULLROM").unwrap();
            first = false;
        }
        if self.is_cyclic() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CYCLIC").unwrap();
            first = false;
        }
        if self.is_enter_cycle() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ENTER_CYCLE").unwrap();
            first = false;
        }
        if self.is_animation() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ANIMATION").unwrap();
            first = false;
        }
        if self.is_frozen() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FROZEN").unwrap();
            first = false;
        }
        if self.is_transport_enter() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TRANSPORT_ENTER").unwrap();
            first = false;
        }
        if self.is_transport_exit() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TRANSPORT_EXIT").unwrap();
            first = false;
        }
        if self.is_unknown7() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN7").unwrap();
            first = false;
        }
        if self.is_unknown8() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN8").unwrap();
            first = false;
        }
        if self.is_orientation_inversed() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ORIENTATION_INVERSED").unwrap();
            first = false;
        }
        if self.is_unknown10() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN10").unwrap();
            first = false;
        }
        if self.is_unknown11() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN11").unwrap();
            first = false;
        }
        if self.is_unknown12() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN12").unwrap();
            first = false;
        }
        if self.is_unknown13() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN13").unwrap();
            first = false;
        }
        s
    }

}

impl SplineFlag {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const DONE: u32 = 0x100;
    pub const FALLING: u32 = 0x200;
    pub const NO_SPLINE: u32 = 0x400;
    pub const PARABOLIC: u32 = 0x800;
    pub const WALK_MODE: u32 = 0x1000;
    pub const FLYING: u32 = 0x2000;
    pub const ORIENTATION_FIXED: u32 = 0x4000;
    pub const FINAL_POINT: u32 = 0x8000;
    pub const FINAL_TARGET: u32 = 0x10000;
    pub const FINAL_ANGLE: u32 = 0x20000;
    pub const CATMULLROM: u32 = 0x40000;
    pub const CYCLIC: u32 = 0x80000;
    pub const ENTER_CYCLE: u32 = 0x100000;
    pub const ANIMATION: u32 = 0x200000;
    pub const FROZEN: u32 = 0x400000;
    pub const TRANSPORT_ENTER: u32 = 0x800000;
    pub const TRANSPORT_EXIT: u32 = 0x1000000;
    pub const UNKNOWN7: u32 = 0x2000000;
    pub const UNKNOWN8: u32 = 0x4000000;
    pub const ORIENTATION_INVERSED: u32 = 0x8000000;
    pub const UNKNOWN10: u32 = 0x10000000;
    pub const UNKNOWN11: u32 = 0x20000000;
    pub const UNKNOWN12: u32 = 0x40000000;
    pub const UNKNOWN13: u32 = 0x80000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::DONE
                | Self::FALLING
                | Self::NO_SPLINE
                | Self::PARABOLIC
                | Self::WALK_MODE
                | Self::FLYING
                | Self::ORIENTATION_FIXED
                | Self::FINAL_POINT
                | Self::FINAL_TARGET
                | Self::FINAL_ANGLE
                | Self::CATMULLROM
                | Self::CYCLIC
                | Self::ENTER_CYCLE
                | Self::ANIMATION
                | Self::FROZEN
                | Self::TRANSPORT_ENTER
                | Self::TRANSPORT_EXIT
                | Self::UNKNOWN7
                | Self::UNKNOWN8
                | Self::ORIENTATION_INVERSED
                | Self::UNKNOWN10
                | Self::UNKNOWN11
                | Self::UNKNOWN12
                | Self::UNKNOWN13
        }
    }

    pub const fn is_done(&self) -> bool {
        (self.inner & Self::DONE) != 0
    }

    pub const fn new_done() -> Self {
        Self { inner: Self::DONE }
    }

    pub fn set_done(&mut self) -> Self {
        self.inner |= Self::DONE;
        *self
    }

    pub fn clear_done(&mut self) -> Self {
        self.inner &= Self::DONE.reverse_bits();
        *self
    }

    pub const fn is_falling(&self) -> bool {
        (self.inner & Self::FALLING) != 0
    }

    /// vmangos: Affects elevation computation
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

    pub const fn is_no_spline(&self) -> bool {
        (self.inner & Self::NO_SPLINE) != 0
    }

    pub const fn new_no_spline() -> Self {
        Self { inner: Self::NO_SPLINE }
    }

    pub fn set_no_spline(&mut self) -> Self {
        self.inner |= Self::NO_SPLINE;
        *self
    }

    pub fn clear_no_spline(&mut self) -> Self {
        self.inner &= Self::NO_SPLINE.reverse_bits();
        *self
    }

    pub const fn is_parabolic(&self) -> bool {
        (self.inner & Self::PARABOLIC) != 0
    }

    pub const fn new_parabolic() -> Self {
        Self { inner: Self::PARABOLIC }
    }

    pub fn set_parabolic(&mut self) -> Self {
        self.inner |= Self::PARABOLIC;
        *self
    }

    pub fn clear_parabolic(&mut self) -> Self {
        self.inner &= Self::PARABOLIC.reverse_bits();
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

    pub const fn is_orientation_fixed(&self) -> bool {
        (self.inner & Self::ORIENTATION_FIXED) != 0
    }

    pub const fn new_orientation_fixed() -> Self {
        Self { inner: Self::ORIENTATION_FIXED }
    }

    pub fn set_orientation_fixed(&mut self) -> Self {
        self.inner |= Self::ORIENTATION_FIXED;
        *self
    }

    pub fn clear_orientation_fixed(&mut self) -> Self {
        self.inner &= Self::ORIENTATION_FIXED.reverse_bits();
        *self
    }

    pub const fn is_final_point(&self) -> bool {
        (self.inner & Self::FINAL_POINT) != 0
    }

    pub const fn new_final_point() -> Self {
        Self { inner: Self::FINAL_POINT }
    }

    pub fn set_final_point(&mut self) -> Self {
        self.inner |= Self::FINAL_POINT;
        *self
    }

    pub fn clear_final_point(&mut self) -> Self {
        self.inner &= Self::FINAL_POINT.reverse_bits();
        *self
    }

    pub const fn is_final_target(&self) -> bool {
        (self.inner & Self::FINAL_TARGET) != 0
    }

    pub const fn new_final_target() -> Self {
        Self { inner: Self::FINAL_TARGET }
    }

    pub fn set_final_target(&mut self) -> Self {
        self.inner |= Self::FINAL_TARGET;
        *self
    }

    pub fn clear_final_target(&mut self) -> Self {
        self.inner &= Self::FINAL_TARGET.reverse_bits();
        *self
    }

    pub const fn is_final_angle(&self) -> bool {
        (self.inner & Self::FINAL_ANGLE) != 0
    }

    pub const fn new_final_angle() -> Self {
        Self { inner: Self::FINAL_ANGLE }
    }

    pub fn set_final_angle(&mut self) -> Self {
        self.inner |= Self::FINAL_ANGLE;
        *self
    }

    pub fn clear_final_angle(&mut self) -> Self {
        self.inner &= Self::FINAL_ANGLE.reverse_bits();
        *self
    }

    pub const fn is_catmullrom(&self) -> bool {
        (self.inner & Self::CATMULLROM) != 0
    }

    /// azerothcore: Used Catmullrom interpolation mode
    pub const fn new_catmullrom() -> Self {
        Self { inner: Self::CATMULLROM }
    }

    pub fn set_catmullrom(&mut self) -> Self {
        self.inner |= Self::CATMULLROM;
        *self
    }

    pub fn clear_catmullrom(&mut self) -> Self {
        self.inner &= Self::CATMULLROM.reverse_bits();
        *self
    }

    pub const fn is_cyclic(&self) -> bool {
        (self.inner & Self::CYCLIC) != 0
    }

    /// azerothcore: Movement by cycled spline
    pub const fn new_cyclic() -> Self {
        Self { inner: Self::CYCLIC }
    }

    pub fn set_cyclic(&mut self) -> Self {
        self.inner |= Self::CYCLIC;
        *self
    }

    pub fn clear_cyclic(&mut self) -> Self {
        self.inner &= Self::CYCLIC.reverse_bits();
        *self
    }

    pub const fn is_enter_cycle(&self) -> bool {
        (self.inner & Self::ENTER_CYCLE) != 0
    }

    /// azerothcore: Everytimes appears with cyclic flag in monster move packet, erases first spline vertex after first cycle done
    pub const fn new_enter_cycle() -> Self {
        Self { inner: Self::ENTER_CYCLE }
    }

    pub fn set_enter_cycle(&mut self) -> Self {
        self.inner |= Self::ENTER_CYCLE;
        *self
    }

    pub fn clear_enter_cycle(&mut self) -> Self {
        self.inner &= Self::ENTER_CYCLE.reverse_bits();
        *self
    }

    pub const fn is_animation(&self) -> bool {
        (self.inner & Self::ANIMATION) != 0
    }

    /// azerothcore: Plays animation after some time passed
    pub const fn new_animation() -> Self {
        Self { inner: Self::ANIMATION }
    }

    pub fn set_animation(&mut self) -> Self {
        self.inner |= Self::ANIMATION;
        *self
    }

    pub fn clear_animation(&mut self) -> Self {
        self.inner &= Self::ANIMATION.reverse_bits();
        *self
    }

    pub const fn is_frozen(&self) -> bool {
        (self.inner & Self::FROZEN) != 0
    }

    /// vmangos: Will never arrive
    pub const fn new_frozen() -> Self {
        Self { inner: Self::FROZEN }
    }

    pub fn set_frozen(&mut self) -> Self {
        self.inner |= Self::FROZEN;
        *self
    }

    pub fn clear_frozen(&mut self) -> Self {
        self.inner &= Self::FROZEN.reverse_bits();
        *self
    }

    pub const fn is_transport_enter(&self) -> bool {
        (self.inner & Self::TRANSPORT_ENTER) != 0
    }

    pub const fn new_transport_enter() -> Self {
        Self { inner: Self::TRANSPORT_ENTER }
    }

    pub fn set_transport_enter(&mut self) -> Self {
        self.inner |= Self::TRANSPORT_ENTER;
        *self
    }

    pub fn clear_transport_enter(&mut self) -> Self {
        self.inner &= Self::TRANSPORT_ENTER.reverse_bits();
        *self
    }

    pub const fn is_transport_exit(&self) -> bool {
        (self.inner & Self::TRANSPORT_EXIT) != 0
    }

    pub const fn new_transport_exit() -> Self {
        Self { inner: Self::TRANSPORT_EXIT }
    }

    pub fn set_transport_exit(&mut self) -> Self {
        self.inner |= Self::TRANSPORT_EXIT;
        *self
    }

    pub fn clear_transport_exit(&mut self) -> Self {
        self.inner &= Self::TRANSPORT_EXIT.reverse_bits();
        *self
    }

    pub const fn is_unknown7(&self) -> bool {
        (self.inner & Self::UNKNOWN7) != 0
    }

    /// vmangos: exists, but unknown what it does
    pub const fn new_unknown7() -> Self {
        Self { inner: Self::UNKNOWN7 }
    }

    pub fn set_unknown7(&mut self) -> Self {
        self.inner |= Self::UNKNOWN7;
        *self
    }

    pub fn clear_unknown7(&mut self) -> Self {
        self.inner &= Self::UNKNOWN7.reverse_bits();
        *self
    }

    pub const fn is_unknown8(&self) -> bool {
        (self.inner & Self::UNKNOWN8) != 0
    }

    pub const fn new_unknown8() -> Self {
        Self { inner: Self::UNKNOWN8 }
    }

    pub fn set_unknown8(&mut self) -> Self {
        self.inner |= Self::UNKNOWN8;
        *self
    }

    pub fn clear_unknown8(&mut self) -> Self {
        self.inner &= Self::UNKNOWN8.reverse_bits();
        *self
    }

    pub const fn is_orientation_inversed(&self) -> bool {
        (self.inner & Self::ORIENTATION_INVERSED) != 0
    }

    pub const fn new_orientation_inversed() -> Self {
        Self { inner: Self::ORIENTATION_INVERSED }
    }

    pub fn set_orientation_inversed(&mut self) -> Self {
        self.inner |= Self::ORIENTATION_INVERSED;
        *self
    }

    pub fn clear_orientation_inversed(&mut self) -> Self {
        self.inner &= Self::ORIENTATION_INVERSED.reverse_bits();
        *self
    }

    pub const fn is_unknown10(&self) -> bool {
        (self.inner & Self::UNKNOWN10) != 0
    }

    pub const fn new_unknown10() -> Self {
        Self { inner: Self::UNKNOWN10 }
    }

    pub fn set_unknown10(&mut self) -> Self {
        self.inner |= Self::UNKNOWN10;
        *self
    }

    pub fn clear_unknown10(&mut self) -> Self {
        self.inner &= Self::UNKNOWN10.reverse_bits();
        *self
    }

    pub const fn is_unknown11(&self) -> bool {
        (self.inner & Self::UNKNOWN11) != 0
    }

    pub const fn new_unknown11() -> Self {
        Self { inner: Self::UNKNOWN11 }
    }

    pub fn set_unknown11(&mut self) -> Self {
        self.inner |= Self::UNKNOWN11;
        *self
    }

    pub fn clear_unknown11(&mut self) -> Self {
        self.inner &= Self::UNKNOWN11.reverse_bits();
        *self
    }

    pub const fn is_unknown12(&self) -> bool {
        (self.inner & Self::UNKNOWN12) != 0
    }

    pub const fn new_unknown12() -> Self {
        Self { inner: Self::UNKNOWN12 }
    }

    pub fn set_unknown12(&mut self) -> Self {
        self.inner |= Self::UNKNOWN12;
        *self
    }

    pub fn clear_unknown12(&mut self) -> Self {
        self.inner &= Self::UNKNOWN12.reverse_bits();
        *self
    }

    pub const fn is_unknown13(&self) -> bool {
        (self.inner & Self::UNKNOWN13) != 0
    }

    pub const fn new_unknown13() -> Self {
        Self { inner: Self::UNKNOWN13 }
    }

    pub fn set_unknown13(&mut self) -> Self {
        self.inner |= Self::UNKNOWN13;
        *self
    }

    pub fn clear_unknown13(&mut self) -> Self {
        self.inner &= Self::UNKNOWN13.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for SplineFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for SplineFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for SplineFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for SplineFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for SplineFlag {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for SplineFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for SplineFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for SplineFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for SplineFlag {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for SplineFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for SplineFlag {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for SplineFlag {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for SplineFlag {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for SplineFlag {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for SplineFlag {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl TryFrom<i16> for SplineFlag {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl From<i32> for SplineFlag {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for SplineFlag {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for SplineFlag {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

