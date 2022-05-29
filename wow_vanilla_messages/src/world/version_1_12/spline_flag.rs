#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct SplineFlag {
    inner: u32,
}

impl SplineFlag {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

}

impl SplineFlag {
    pub(crate) const NONE: u32 = 0x00;
    pub(crate) const DONE: u32 = 0x01;
    pub(crate) const FALLING: u32 = 0x02;
    pub(crate) const UNKNOWN3: u32 = 0x04;
    pub(crate) const UNKNOWN4: u32 = 0x08;
    pub(crate) const UNKNOWN5: u32 = 0x10;
    pub(crate) const UNKNOWN6: u32 = 0x20;
    pub(crate) const UNKNOWN7: u32 = 0x40;
    pub(crate) const UNKNOWN8: u32 = 0x80;
    pub(crate) const RUNMODE: u32 = 0x100;
    pub(crate) const FLYING: u32 = 0x200;
    pub(crate) const NO_SPLINE: u32 = 0x400;
    pub(crate) const UNKNOWN12: u32 = 0x800;
    pub(crate) const UNKNOWN13: u32 = 0x1000;
    pub(crate) const UNKNOWN14: u32 = 0x2000;
    pub(crate) const UNKNOWN15: u32 = 0x4000;
    pub(crate) const UNKNOWN16: u32 = 0x8000;
    pub(crate) const FINAL_POINT: u32 = 0x10000;
    pub(crate) const FINAL_TARGET: u32 = 0x20000;
    pub(crate) const FINAL_ANGLE: u32 = 0x40000;
    pub(crate) const UNKNOWN19: u32 = 0x80000;
    pub(crate) const CYCLIC: u32 = 0x100000;
    pub(crate) const ENTER_CYCLE: u32 = 0x200000;
    pub(crate) const FROZEN: u32 = 0x400000;
    pub(crate) const UNKNOWN23: u32 = 0x800000;
    pub(crate) const UNKNOWN24: u32 = 0x1000000;
    pub(crate) const UNKNOWN25: u32 = 0x2000000;
    pub(crate) const UNKNOWN26: u32 = 0x4000000;
    pub(crate) const UNKNOWN27: u32 = 0x8000000;
    pub(crate) const UNKNOWN28: u32 = 0x10000000;
    pub(crate) const UNKNOWN29: u32 = 0x20000000;
    pub(crate) const UNKNOWN30: u32 = 0x40000000;
    pub(crate) const UNKNOWN31: u32 = 0x80000000;

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
                | Self::UNKNOWN12
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

    pub const fn is_DONE(&self) -> bool {
        (self.inner & Self::DONE) != 0
    }

    pub const fn new_DONE() -> Self {
        Self { inner: Self::DONE }
    }

    pub fn set_DONE(&mut self) -> Self {
        self.inner |= Self::DONE;
        *self
    }

    pub fn clear_DONE(&mut self) -> Self {
        self.inner &= Self::DONE.reverse_bits();
        *self
    }

    pub const fn is_FALLING(&self) -> bool {
        (self.inner & Self::FALLING) != 0
    }

    pub const fn new_FALLING() -> Self {
        Self { inner: Self::FALLING }
    }

    pub fn set_FALLING(&mut self) -> Self {
        self.inner |= Self::FALLING;
        *self
    }

    pub fn clear_FALLING(&mut self) -> Self {
        self.inner &= Self::FALLING.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN3(&self) -> bool {
        (self.inner & Self::UNKNOWN3) != 0
    }

    pub const fn new_UNKNOWN3() -> Self {
        Self { inner: Self::UNKNOWN3 }
    }

    pub fn set_UNKNOWN3(&mut self) -> Self {
        self.inner |= Self::UNKNOWN3;
        *self
    }

    pub fn clear_UNKNOWN3(&mut self) -> Self {
        self.inner &= Self::UNKNOWN3.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN4(&self) -> bool {
        (self.inner & Self::UNKNOWN4) != 0
    }

    pub const fn new_UNKNOWN4() -> Self {
        Self { inner: Self::UNKNOWN4 }
    }

    pub fn set_UNKNOWN4(&mut self) -> Self {
        self.inner |= Self::UNKNOWN4;
        *self
    }

    pub fn clear_UNKNOWN4(&mut self) -> Self {
        self.inner &= Self::UNKNOWN4.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN5(&self) -> bool {
        (self.inner & Self::UNKNOWN5) != 0
    }

    pub const fn new_UNKNOWN5() -> Self {
        Self { inner: Self::UNKNOWN5 }
    }

    pub fn set_UNKNOWN5(&mut self) -> Self {
        self.inner |= Self::UNKNOWN5;
        *self
    }

    pub fn clear_UNKNOWN5(&mut self) -> Self {
        self.inner &= Self::UNKNOWN5.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN6(&self) -> bool {
        (self.inner & Self::UNKNOWN6) != 0
    }

    pub const fn new_UNKNOWN6() -> Self {
        Self { inner: Self::UNKNOWN6 }
    }

    pub fn set_UNKNOWN6(&mut self) -> Self {
        self.inner |= Self::UNKNOWN6;
        *self
    }

    pub fn clear_UNKNOWN6(&mut self) -> Self {
        self.inner &= Self::UNKNOWN6.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN7(&self) -> bool {
        (self.inner & Self::UNKNOWN7) != 0
    }

    pub const fn new_UNKNOWN7() -> Self {
        Self { inner: Self::UNKNOWN7 }
    }

    pub fn set_UNKNOWN7(&mut self) -> Self {
        self.inner |= Self::UNKNOWN7;
        *self
    }

    pub fn clear_UNKNOWN7(&mut self) -> Self {
        self.inner &= Self::UNKNOWN7.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN8(&self) -> bool {
        (self.inner & Self::UNKNOWN8) != 0
    }

    pub const fn new_UNKNOWN8() -> Self {
        Self { inner: Self::UNKNOWN8 }
    }

    pub fn set_UNKNOWN8(&mut self) -> Self {
        self.inner |= Self::UNKNOWN8;
        *self
    }

    pub fn clear_UNKNOWN8(&mut self) -> Self {
        self.inner &= Self::UNKNOWN8.reverse_bits();
        *self
    }

    pub const fn is_RUNMODE(&self) -> bool {
        (self.inner & Self::RUNMODE) != 0
    }

    pub const fn new_RUNMODE() -> Self {
        Self { inner: Self::RUNMODE }
    }

    pub fn set_RUNMODE(&mut self) -> Self {
        self.inner |= Self::RUNMODE;
        *self
    }

    pub fn clear_RUNMODE(&mut self) -> Self {
        self.inner &= Self::RUNMODE.reverse_bits();
        *self
    }

    pub const fn is_FLYING(&self) -> bool {
        (self.inner & Self::FLYING) != 0
    }

    pub const fn new_FLYING() -> Self {
        Self { inner: Self::FLYING }
    }

    pub fn set_FLYING(&mut self) -> Self {
        self.inner |= Self::FLYING;
        *self
    }

    pub fn clear_FLYING(&mut self) -> Self {
        self.inner &= Self::FLYING.reverse_bits();
        *self
    }

    pub const fn is_NO_SPLINE(&self) -> bool {
        (self.inner & Self::NO_SPLINE) != 0
    }

    pub const fn new_NO_SPLINE() -> Self {
        Self { inner: Self::NO_SPLINE }
    }

    pub fn set_NO_SPLINE(&mut self) -> Self {
        self.inner |= Self::NO_SPLINE;
        *self
    }

    pub fn clear_NO_SPLINE(&mut self) -> Self {
        self.inner &= Self::NO_SPLINE.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN12(&self) -> bool {
        (self.inner & Self::UNKNOWN12) != 0
    }

    pub const fn new_UNKNOWN12() -> Self {
        Self { inner: Self::UNKNOWN12 }
    }

    pub fn set_UNKNOWN12(&mut self) -> Self {
        self.inner |= Self::UNKNOWN12;
        *self
    }

    pub fn clear_UNKNOWN12(&mut self) -> Self {
        self.inner &= Self::UNKNOWN12.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN13(&self) -> bool {
        (self.inner & Self::UNKNOWN13) != 0
    }

    pub const fn new_UNKNOWN13() -> Self {
        Self { inner: Self::UNKNOWN13 }
    }

    pub fn set_UNKNOWN13(&mut self) -> Self {
        self.inner |= Self::UNKNOWN13;
        *self
    }

    pub fn clear_UNKNOWN13(&mut self) -> Self {
        self.inner &= Self::UNKNOWN13.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN14(&self) -> bool {
        (self.inner & Self::UNKNOWN14) != 0
    }

    pub const fn new_UNKNOWN14() -> Self {
        Self { inner: Self::UNKNOWN14 }
    }

    pub fn set_UNKNOWN14(&mut self) -> Self {
        self.inner |= Self::UNKNOWN14;
        *self
    }

    pub fn clear_UNKNOWN14(&mut self) -> Self {
        self.inner &= Self::UNKNOWN14.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN15(&self) -> bool {
        (self.inner & Self::UNKNOWN15) != 0
    }

    pub const fn new_UNKNOWN15() -> Self {
        Self { inner: Self::UNKNOWN15 }
    }

    pub fn set_UNKNOWN15(&mut self) -> Self {
        self.inner |= Self::UNKNOWN15;
        *self
    }

    pub fn clear_UNKNOWN15(&mut self) -> Self {
        self.inner &= Self::UNKNOWN15.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN16(&self) -> bool {
        (self.inner & Self::UNKNOWN16) != 0
    }

    pub const fn new_UNKNOWN16() -> Self {
        Self { inner: Self::UNKNOWN16 }
    }

    pub fn set_UNKNOWN16(&mut self) -> Self {
        self.inner |= Self::UNKNOWN16;
        *self
    }

    pub fn clear_UNKNOWN16(&mut self) -> Self {
        self.inner &= Self::UNKNOWN16.reverse_bits();
        *self
    }

    pub const fn is_FINAL_POINT(&self) -> bool {
        (self.inner & Self::FINAL_POINT) != 0
    }

    pub const fn new_FINAL_POINT() -> Self {
        Self { inner: Self::FINAL_POINT }
    }

    pub fn set_FINAL_POINT(&mut self) -> Self {
        self.inner |= Self::FINAL_POINT;
        *self
    }

    pub fn clear_FINAL_POINT(&mut self) -> Self {
        self.inner &= Self::FINAL_POINT.reverse_bits();
        *self
    }

    pub const fn is_FINAL_TARGET(&self) -> bool {
        (self.inner & Self::FINAL_TARGET) != 0
    }

    pub const fn new_FINAL_TARGET() -> Self {
        Self { inner: Self::FINAL_TARGET }
    }

    pub fn set_FINAL_TARGET(&mut self) -> Self {
        self.inner |= Self::FINAL_TARGET;
        *self
    }

    pub fn clear_FINAL_TARGET(&mut self) -> Self {
        self.inner &= Self::FINAL_TARGET.reverse_bits();
        *self
    }

    pub const fn is_FINAL_ANGLE(&self) -> bool {
        (self.inner & Self::FINAL_ANGLE) != 0
    }

    pub const fn new_FINAL_ANGLE() -> Self {
        Self { inner: Self::FINAL_ANGLE }
    }

    pub fn set_FINAL_ANGLE(&mut self) -> Self {
        self.inner |= Self::FINAL_ANGLE;
        *self
    }

    pub fn clear_FINAL_ANGLE(&mut self) -> Self {
        self.inner &= Self::FINAL_ANGLE.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN19(&self) -> bool {
        (self.inner & Self::UNKNOWN19) != 0
    }

    pub const fn new_UNKNOWN19() -> Self {
        Self { inner: Self::UNKNOWN19 }
    }

    pub fn set_UNKNOWN19(&mut self) -> Self {
        self.inner |= Self::UNKNOWN19;
        *self
    }

    pub fn clear_UNKNOWN19(&mut self) -> Self {
        self.inner &= Self::UNKNOWN19.reverse_bits();
        *self
    }

    pub const fn is_CYCLIC(&self) -> bool {
        (self.inner & Self::CYCLIC) != 0
    }

    pub const fn new_CYCLIC() -> Self {
        Self { inner: Self::CYCLIC }
    }

    pub fn set_CYCLIC(&mut self) -> Self {
        self.inner |= Self::CYCLIC;
        *self
    }

    pub fn clear_CYCLIC(&mut self) -> Self {
        self.inner &= Self::CYCLIC.reverse_bits();
        *self
    }

    pub const fn is_ENTER_CYCLE(&self) -> bool {
        (self.inner & Self::ENTER_CYCLE) != 0
    }

    pub const fn new_ENTER_CYCLE() -> Self {
        Self { inner: Self::ENTER_CYCLE }
    }

    pub fn set_ENTER_CYCLE(&mut self) -> Self {
        self.inner |= Self::ENTER_CYCLE;
        *self
    }

    pub fn clear_ENTER_CYCLE(&mut self) -> Self {
        self.inner &= Self::ENTER_CYCLE.reverse_bits();
        *self
    }

    pub const fn is_FROZEN(&self) -> bool {
        (self.inner & Self::FROZEN) != 0
    }

    pub const fn new_FROZEN() -> Self {
        Self { inner: Self::FROZEN }
    }

    pub fn set_FROZEN(&mut self) -> Self {
        self.inner |= Self::FROZEN;
        *self
    }

    pub fn clear_FROZEN(&mut self) -> Self {
        self.inner &= Self::FROZEN.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN23(&self) -> bool {
        (self.inner & Self::UNKNOWN23) != 0
    }

    pub const fn new_UNKNOWN23() -> Self {
        Self { inner: Self::UNKNOWN23 }
    }

    pub fn set_UNKNOWN23(&mut self) -> Self {
        self.inner |= Self::UNKNOWN23;
        *self
    }

    pub fn clear_UNKNOWN23(&mut self) -> Self {
        self.inner &= Self::UNKNOWN23.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN24(&self) -> bool {
        (self.inner & Self::UNKNOWN24) != 0
    }

    pub const fn new_UNKNOWN24() -> Self {
        Self { inner: Self::UNKNOWN24 }
    }

    pub fn set_UNKNOWN24(&mut self) -> Self {
        self.inner |= Self::UNKNOWN24;
        *self
    }

    pub fn clear_UNKNOWN24(&mut self) -> Self {
        self.inner &= Self::UNKNOWN24.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN25(&self) -> bool {
        (self.inner & Self::UNKNOWN25) != 0
    }

    pub const fn new_UNKNOWN25() -> Self {
        Self { inner: Self::UNKNOWN25 }
    }

    pub fn set_UNKNOWN25(&mut self) -> Self {
        self.inner |= Self::UNKNOWN25;
        *self
    }

    pub fn clear_UNKNOWN25(&mut self) -> Self {
        self.inner &= Self::UNKNOWN25.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN26(&self) -> bool {
        (self.inner & Self::UNKNOWN26) != 0
    }

    pub const fn new_UNKNOWN26() -> Self {
        Self { inner: Self::UNKNOWN26 }
    }

    pub fn set_UNKNOWN26(&mut self) -> Self {
        self.inner |= Self::UNKNOWN26;
        *self
    }

    pub fn clear_UNKNOWN26(&mut self) -> Self {
        self.inner &= Self::UNKNOWN26.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN27(&self) -> bool {
        (self.inner & Self::UNKNOWN27) != 0
    }

    pub const fn new_UNKNOWN27() -> Self {
        Self { inner: Self::UNKNOWN27 }
    }

    pub fn set_UNKNOWN27(&mut self) -> Self {
        self.inner |= Self::UNKNOWN27;
        *self
    }

    pub fn clear_UNKNOWN27(&mut self) -> Self {
        self.inner &= Self::UNKNOWN27.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN28(&self) -> bool {
        (self.inner & Self::UNKNOWN28) != 0
    }

    pub const fn new_UNKNOWN28() -> Self {
        Self { inner: Self::UNKNOWN28 }
    }

    pub fn set_UNKNOWN28(&mut self) -> Self {
        self.inner |= Self::UNKNOWN28;
        *self
    }

    pub fn clear_UNKNOWN28(&mut self) -> Self {
        self.inner &= Self::UNKNOWN28.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN29(&self) -> bool {
        (self.inner & Self::UNKNOWN29) != 0
    }

    pub const fn new_UNKNOWN29() -> Self {
        Self { inner: Self::UNKNOWN29 }
    }

    pub fn set_UNKNOWN29(&mut self) -> Self {
        self.inner |= Self::UNKNOWN29;
        *self
    }

    pub fn clear_UNKNOWN29(&mut self) -> Self {
        self.inner &= Self::UNKNOWN29.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN30(&self) -> bool {
        (self.inner & Self::UNKNOWN30) != 0
    }

    pub const fn new_UNKNOWN30() -> Self {
        Self { inner: Self::UNKNOWN30 }
    }

    pub fn set_UNKNOWN30(&mut self) -> Self {
        self.inner |= Self::UNKNOWN30;
        *self
    }

    pub fn clear_UNKNOWN30(&mut self) -> Self {
        self.inner &= Self::UNKNOWN30.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN31(&self) -> bool {
        (self.inner & Self::UNKNOWN31) != 0
    }

    pub const fn new_UNKNOWN31() -> Self {
        Self { inner: Self::UNKNOWN31 }
    }

    pub fn set_UNKNOWN31(&mut self) -> Self {
        self.inner |= Self::UNKNOWN31;
        *self
    }

    pub fn clear_UNKNOWN31(&mut self) -> Self {
        self.inner &= Self::UNKNOWN31.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

