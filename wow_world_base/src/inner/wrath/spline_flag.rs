/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L26):
/// ```text
/// flag SplineFlag : u32 {
///     NONE = 0x00000000;
///     DONE = 0x00000001;
///     FALLING = 0x00000002;
///     UNKNOWN3 = 0x00000004;
///     UNKNOWN4 = 0x00000008;
///     UNKNOWN5 = 0x00000010;
///     UNKNOWN6 = 0x00000020;
///     UNKNOWN7 = 0x00000040;
///     UNKNOWN8 = 0x00000080;
///     RUNMODE = 0x00000100;
///     FLYING = 0x00000200;
///     NO_SPLINE = 0x00000400;
///     PARABOLIC = 0x00000800;
///     UNKNOWN13 = 0x00001000;
///     UNKNOWN14 = 0x00002000;
///     UNKNOWN15 = 0x00004000;
///     UNKNOWN16 = 0x00008000;
///     FINAL_POINT = 0x00010000;
///     FINAL_TARGET = 0x00020000;
///     FINAL_ANGLE = 0x00040000;
///     UNKNOWN19 = 0x00080000;
///     CYCLIC = 0x00100000;
///     ENTER_CYCLE = 0x00200000;
///     FROZEN = 0x00400000;
///     UNKNOWN23 = 0x00800000;
///     UNKNOWN24 = 0x01000000;
///     UNKNOWN25 = 0x02000000;
///     UNKNOWN26 = 0x04000000;
///     UNKNOWN27 = 0x08000000;
///     UNKNOWN28 = 0x10000000;
///     UNKNOWN29 = 0x20000000;
///     UNKNOWN30 = 0x40000000;
///     UNKNOWN31 = 0x80000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
                write!(s, "| ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_done() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DONE").unwrap();
            first = false;
        }
        if self.is_falling() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FALLING").unwrap();
            first = false;
        }
        if self.is_unknown3() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN3").unwrap();
            first = false;
        }
        if self.is_unknown4() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN4").unwrap();
            first = false;
        }
        if self.is_unknown5() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN5").unwrap();
            first = false;
        }
        if self.is_unknown6() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN6").unwrap();
            first = false;
        }
        if self.is_unknown7() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN7").unwrap();
            first = false;
        }
        if self.is_unknown8() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN8").unwrap();
            first = false;
        }
        if self.is_runmode() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "RUNMODE").unwrap();
            first = false;
        }
        if self.is_flying() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FLYING").unwrap();
            first = false;
        }
        if self.is_no_spline() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_SPLINE").unwrap();
            first = false;
        }
        if self.is_parabolic() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PARABOLIC").unwrap();
            first = false;
        }
        if self.is_unknown13() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN13").unwrap();
            first = false;
        }
        if self.is_unknown14() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN14").unwrap();
            first = false;
        }
        if self.is_unknown15() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN15").unwrap();
            first = false;
        }
        if self.is_unknown16() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN16").unwrap();
            first = false;
        }
        if self.is_final_point() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FINAL_POINT").unwrap();
            first = false;
        }
        if self.is_final_target() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FINAL_TARGET").unwrap();
            first = false;
        }
        if self.is_final_angle() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FINAL_ANGLE").unwrap();
            first = false;
        }
        if self.is_unknown19() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN19").unwrap();
            first = false;
        }
        if self.is_cyclic() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CYCLIC").unwrap();
            first = false;
        }
        if self.is_enter_cycle() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ENTER_CYCLE").unwrap();
            first = false;
        }
        if self.is_frozen() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FROZEN").unwrap();
            first = false;
        }
        if self.is_unknown23() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN23").unwrap();
            first = false;
        }
        if self.is_unknown24() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN24").unwrap();
            first = false;
        }
        if self.is_unknown25() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN25").unwrap();
            first = false;
        }
        if self.is_unknown26() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN26").unwrap();
            first = false;
        }
        if self.is_unknown27() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN27").unwrap();
            first = false;
        }
        if self.is_unknown28() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN28").unwrap();
            first = false;
        }
        if self.is_unknown29() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN29").unwrap();
            first = false;
        }
        if self.is_unknown30() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN30").unwrap();
            first = false;
        }
        if self.is_unknown31() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN31").unwrap();
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
    pub const DONE: u32 = 0x01;
    pub const FALLING: u32 = 0x02;
    pub const UNKNOWN3: u32 = 0x04;
    pub const UNKNOWN4: u32 = 0x08;
    pub const UNKNOWN5: u32 = 0x10;
    pub const UNKNOWN6: u32 = 0x20;
    pub const UNKNOWN7: u32 = 0x40;
    pub const UNKNOWN8: u32 = 0x80;
    pub const RUNMODE: u32 = 0x100;
    pub const FLYING: u32 = 0x200;
    pub const NO_SPLINE: u32 = 0x400;
    pub const PARABOLIC: u32 = 0x800;
    pub const UNKNOWN13: u32 = 0x1000;
    pub const UNKNOWN14: u32 = 0x2000;
    pub const UNKNOWN15: u32 = 0x4000;
    pub const UNKNOWN16: u32 = 0x8000;
    pub const FINAL_POINT: u32 = 0x10000;
    pub const FINAL_TARGET: u32 = 0x20000;
    pub const FINAL_ANGLE: u32 = 0x40000;
    pub const UNKNOWN19: u32 = 0x80000;
    pub const CYCLIC: u32 = 0x100000;
    pub const ENTER_CYCLE: u32 = 0x200000;
    pub const FROZEN: u32 = 0x400000;
    pub const UNKNOWN23: u32 = 0x800000;
    pub const UNKNOWN24: u32 = 0x1000000;
    pub const UNKNOWN25: u32 = 0x2000000;
    pub const UNKNOWN26: u32 = 0x4000000;
    pub const UNKNOWN27: u32 = 0x8000000;
    pub const UNKNOWN28: u32 = 0x10000000;
    pub const UNKNOWN29: u32 = 0x20000000;
    pub const UNKNOWN30: u32 = 0x40000000;
    pub const UNKNOWN31: u32 = 0x80000000;

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
                | Self::UNKNOWN3
                | Self::UNKNOWN4
                | Self::UNKNOWN5
                | Self::UNKNOWN6
                | Self::UNKNOWN7
                | Self::UNKNOWN8
                | Self::RUNMODE
                | Self::FLYING
                | Self::NO_SPLINE
                | Self::PARABOLIC
                | Self::UNKNOWN13
                | Self::UNKNOWN14
                | Self::UNKNOWN15
                | Self::UNKNOWN16
                | Self::FINAL_POINT
                | Self::FINAL_TARGET
                | Self::FINAL_ANGLE
                | Self::UNKNOWN19
                | Self::CYCLIC
                | Self::ENTER_CYCLE
                | Self::FROZEN
                | Self::UNKNOWN23
                | Self::UNKNOWN24
                | Self::UNKNOWN25
                | Self::UNKNOWN26
                | Self::UNKNOWN27
                | Self::UNKNOWN28
                | Self::UNKNOWN29
                | Self::UNKNOWN30
                | Self::UNKNOWN31
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

    pub const fn is_unknown3(&self) -> bool {
        (self.inner & Self::UNKNOWN3) != 0
    }

    pub const fn new_unknown3() -> Self {
        Self { inner: Self::UNKNOWN3 }
    }

    pub fn set_unknown3(&mut self) -> Self {
        self.inner |= Self::UNKNOWN3;
        *self
    }

    pub fn clear_unknown3(&mut self) -> Self {
        self.inner &= Self::UNKNOWN3.reverse_bits();
        *self
    }

    pub const fn is_unknown4(&self) -> bool {
        (self.inner & Self::UNKNOWN4) != 0
    }

    pub const fn new_unknown4() -> Self {
        Self { inner: Self::UNKNOWN4 }
    }

    pub fn set_unknown4(&mut self) -> Self {
        self.inner |= Self::UNKNOWN4;
        *self
    }

    pub fn clear_unknown4(&mut self) -> Self {
        self.inner &= Self::UNKNOWN4.reverse_bits();
        *self
    }

    pub const fn is_unknown5(&self) -> bool {
        (self.inner & Self::UNKNOWN5) != 0
    }

    pub const fn new_unknown5() -> Self {
        Self { inner: Self::UNKNOWN5 }
    }

    pub fn set_unknown5(&mut self) -> Self {
        self.inner |= Self::UNKNOWN5;
        *self
    }

    pub fn clear_unknown5(&mut self) -> Self {
        self.inner &= Self::UNKNOWN5.reverse_bits();
        *self
    }

    pub const fn is_unknown6(&self) -> bool {
        (self.inner & Self::UNKNOWN6) != 0
    }

    pub const fn new_unknown6() -> Self {
        Self { inner: Self::UNKNOWN6 }
    }

    pub fn set_unknown6(&mut self) -> Self {
        self.inner |= Self::UNKNOWN6;
        *self
    }

    pub fn clear_unknown6(&mut self) -> Self {
        self.inner &= Self::UNKNOWN6.reverse_bits();
        *self
    }

    pub const fn is_unknown7(&self) -> bool {
        (self.inner & Self::UNKNOWN7) != 0
    }

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

    pub const fn is_runmode(&self) -> bool {
        (self.inner & Self::RUNMODE) != 0
    }

    pub const fn new_runmode() -> Self {
        Self { inner: Self::RUNMODE }
    }

    pub fn set_runmode(&mut self) -> Self {
        self.inner |= Self::RUNMODE;
        *self
    }

    pub fn clear_runmode(&mut self) -> Self {
        self.inner &= Self::RUNMODE.reverse_bits();
        *self
    }

    pub const fn is_flying(&self) -> bool {
        (self.inner & Self::FLYING) != 0
    }

    /// vmangos: Smooth movement(Catmullrom interpolation mode), flying animation
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

    pub const fn is_unknown14(&self) -> bool {
        (self.inner & Self::UNKNOWN14) != 0
    }

    pub const fn new_unknown14() -> Self {
        Self { inner: Self::UNKNOWN14 }
    }

    pub fn set_unknown14(&mut self) -> Self {
        self.inner |= Self::UNKNOWN14;
        *self
    }

    pub fn clear_unknown14(&mut self) -> Self {
        self.inner &= Self::UNKNOWN14.reverse_bits();
        *self
    }

    pub const fn is_unknown15(&self) -> bool {
        (self.inner & Self::UNKNOWN15) != 0
    }

    pub const fn new_unknown15() -> Self {
        Self { inner: Self::UNKNOWN15 }
    }

    pub fn set_unknown15(&mut self) -> Self {
        self.inner |= Self::UNKNOWN15;
        *self
    }

    pub fn clear_unknown15(&mut self) -> Self {
        self.inner &= Self::UNKNOWN15.reverse_bits();
        *self
    }

    pub const fn is_unknown16(&self) -> bool {
        (self.inner & Self::UNKNOWN16) != 0
    }

    pub const fn new_unknown16() -> Self {
        Self { inner: Self::UNKNOWN16 }
    }

    pub fn set_unknown16(&mut self) -> Self {
        self.inner |= Self::UNKNOWN16;
        *self
    }

    pub fn clear_unknown16(&mut self) -> Self {
        self.inner &= Self::UNKNOWN16.reverse_bits();
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

    pub const fn is_unknown19(&self) -> bool {
        (self.inner & Self::UNKNOWN19) != 0
    }

    /// vmangos: exists, but unknown what it does
    pub const fn new_unknown19() -> Self {
        Self { inner: Self::UNKNOWN19 }
    }

    pub fn set_unknown19(&mut self) -> Self {
        self.inner |= Self::UNKNOWN19;
        *self
    }

    pub fn clear_unknown19(&mut self) -> Self {
        self.inner &= Self::UNKNOWN19.reverse_bits();
        *self
    }

    pub const fn is_cyclic(&self) -> bool {
        (self.inner & Self::CYCLIC) != 0
    }

    /// vmangos: Movement by cycled spline
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

    /// vmangos: Everytimes appears with cyclic flag in monster move packet, erases first spline vertex after first cycle done
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

    pub const fn is_unknown23(&self) -> bool {
        (self.inner & Self::UNKNOWN23) != 0
    }

    pub const fn new_unknown23() -> Self {
        Self { inner: Self::UNKNOWN23 }
    }

    pub fn set_unknown23(&mut self) -> Self {
        self.inner |= Self::UNKNOWN23;
        *self
    }

    pub fn clear_unknown23(&mut self) -> Self {
        self.inner &= Self::UNKNOWN23.reverse_bits();
        *self
    }

    pub const fn is_unknown24(&self) -> bool {
        (self.inner & Self::UNKNOWN24) != 0
    }

    pub const fn new_unknown24() -> Self {
        Self { inner: Self::UNKNOWN24 }
    }

    pub fn set_unknown24(&mut self) -> Self {
        self.inner |= Self::UNKNOWN24;
        *self
    }

    pub fn clear_unknown24(&mut self) -> Self {
        self.inner &= Self::UNKNOWN24.reverse_bits();
        *self
    }

    pub const fn is_unknown25(&self) -> bool {
        (self.inner & Self::UNKNOWN25) != 0
    }

    /// vmangos: exists, but unknown what it does
    pub const fn new_unknown25() -> Self {
        Self { inner: Self::UNKNOWN25 }
    }

    pub fn set_unknown25(&mut self) -> Self {
        self.inner |= Self::UNKNOWN25;
        *self
    }

    pub fn clear_unknown25(&mut self) -> Self {
        self.inner &= Self::UNKNOWN25.reverse_bits();
        *self
    }

    pub const fn is_unknown26(&self) -> bool {
        (self.inner & Self::UNKNOWN26) != 0
    }

    pub const fn new_unknown26() -> Self {
        Self { inner: Self::UNKNOWN26 }
    }

    pub fn set_unknown26(&mut self) -> Self {
        self.inner |= Self::UNKNOWN26;
        *self
    }

    pub fn clear_unknown26(&mut self) -> Self {
        self.inner &= Self::UNKNOWN26.reverse_bits();
        *self
    }

    pub const fn is_unknown27(&self) -> bool {
        (self.inner & Self::UNKNOWN27) != 0
    }

    pub const fn new_unknown27() -> Self {
        Self { inner: Self::UNKNOWN27 }
    }

    pub fn set_unknown27(&mut self) -> Self {
        self.inner |= Self::UNKNOWN27;
        *self
    }

    pub fn clear_unknown27(&mut self) -> Self {
        self.inner &= Self::UNKNOWN27.reverse_bits();
        *self
    }

    pub const fn is_unknown28(&self) -> bool {
        (self.inner & Self::UNKNOWN28) != 0
    }

    pub const fn new_unknown28() -> Self {
        Self { inner: Self::UNKNOWN28 }
    }

    pub fn set_unknown28(&mut self) -> Self {
        self.inner |= Self::UNKNOWN28;
        *self
    }

    pub fn clear_unknown28(&mut self) -> Self {
        self.inner &= Self::UNKNOWN28.reverse_bits();
        *self
    }

    pub const fn is_unknown29(&self) -> bool {
        (self.inner & Self::UNKNOWN29) != 0
    }

    pub const fn new_unknown29() -> Self {
        Self { inner: Self::UNKNOWN29 }
    }

    pub fn set_unknown29(&mut self) -> Self {
        self.inner |= Self::UNKNOWN29;
        *self
    }

    pub fn clear_unknown29(&mut self) -> Self {
        self.inner &= Self::UNKNOWN29.reverse_bits();
        *self
    }

    pub const fn is_unknown30(&self) -> bool {
        (self.inner & Self::UNKNOWN30) != 0
    }

    pub const fn new_unknown30() -> Self {
        Self { inner: Self::UNKNOWN30 }
    }

    pub fn set_unknown30(&mut self) -> Self {
        self.inner |= Self::UNKNOWN30;
        *self
    }

    pub fn clear_unknown30(&mut self) -> Self {
        self.inner &= Self::UNKNOWN30.reverse_bits();
        *self
    }

    pub const fn is_unknown31(&self) -> bool {
        (self.inner & Self::UNKNOWN31) != 0
    }

    pub const fn new_unknown31() -> Self {
        Self { inner: Self::UNKNOWN31 }
    }

    pub fn set_unknown31(&mut self) -> Self {
        self.inner |= Self::UNKNOWN31;
        *self
    }

    pub fn clear_unknown31(&mut self) -> Self {
        self.inner &= Self::UNKNOWN31.reverse_bits();
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
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for SplineFlag {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
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

