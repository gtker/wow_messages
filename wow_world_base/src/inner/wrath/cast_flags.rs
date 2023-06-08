/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm#L1):
/// ```text
/// flag CastFlags : u32 {
///     NONE = 0x00000000;
///     PENDING = 0x00000001;
///     HAS_TRAJECTORY = 0x00000002;
///     UNKNOWN_3 = 0x00000004;
///     UNKNOWN_4 = 0x00000008;
///     UNKNOWN_5 = 0x00000010;
///     AMMO = 0x00000020;
///     UNKNOWN_7 = 0x00000040;
///     UNKNOWN_8 = 0x00000080;
///     UNKNOWN_9 = 0x00000100;
///     UNKNOWN_10 = 0x00000200;
///     UNKNOWN_11 = 0x00000400;
///     POWER_LEFT_SELF = 0x00000800;
///     UNKNOWN_13 = 0x00001000;
///     UNKNOWN_14 = 0x00002000;
///     UNKNOWN_15 = 0x00004000;
///     UNKNOWN_16 = 0x00008000;
///     UNKNOWN_17 = 0x00010000;
///     ADJUST_MISSILE = 0x00020000;
///     NO_GCD = 0x00040000;
///     VISUAL_CHAIN = 0x00080000;
///     UNKNOWN_21 = 0x00100000;
///     RUNE_LIST = 0x00200000;
///     UNKNOWN_23 = 0x00400000;
///     UNKNOWN_24 = 0x00800000;
///     UNKNOWN_25 = 0x01000000;
///     UNKNOWN_26 = 0x02000000;
///     IMMUNITY = 0x04000000;
///     UNKNOWN_28 = 0x08000000;
///     UNKNOWN_29 = 0x10000000;
///     UNKNOWN_30 = 0x20000000;
///     HEAL_PREDICTION = 0x40000000;
///     UNKNOWN_32 = 0x80000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct CastFlags {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl CastFlags {
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
        if self.is_pending() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PENDING").unwrap();
            first = false;
        }
        if self.is_has_trajectory() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HAS_TRAJECTORY").unwrap();
            first = false;
        }
        if self.is_unknown_3() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_3").unwrap();
            first = false;
        }
        if self.is_unknown_4() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_4").unwrap();
            first = false;
        }
        if self.is_unknown_5() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_5").unwrap();
            first = false;
        }
        if self.is_ammo() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "AMMO").unwrap();
            first = false;
        }
        if self.is_unknown_7() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_7").unwrap();
            first = false;
        }
        if self.is_unknown_8() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_8").unwrap();
            first = false;
        }
        if self.is_unknown_9() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_9").unwrap();
            first = false;
        }
        if self.is_unknown_10() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_10").unwrap();
            first = false;
        }
        if self.is_unknown_11() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_11").unwrap();
            first = false;
        }
        if self.is_power_left_self() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "POWER_LEFT_SELF").unwrap();
            first = false;
        }
        if self.is_unknown_13() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_13").unwrap();
            first = false;
        }
        if self.is_unknown_14() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_14").unwrap();
            first = false;
        }
        if self.is_unknown_15() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_15").unwrap();
            first = false;
        }
        if self.is_unknown_16() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_16").unwrap();
            first = false;
        }
        if self.is_unknown_17() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_17").unwrap();
            first = false;
        }
        if self.is_adjust_missile() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ADJUST_MISSILE").unwrap();
            first = false;
        }
        if self.is_no_gcd() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_GCD").unwrap();
            first = false;
        }
        if self.is_visual_chain() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "VISUAL_CHAIN").unwrap();
            first = false;
        }
        if self.is_unknown_21() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_21").unwrap();
            first = false;
        }
        if self.is_rune_list() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "RUNE_LIST").unwrap();
            first = false;
        }
        if self.is_unknown_23() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_23").unwrap();
            first = false;
        }
        if self.is_unknown_24() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_24").unwrap();
            first = false;
        }
        if self.is_unknown_25() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_25").unwrap();
            first = false;
        }
        if self.is_unknown_26() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_26").unwrap();
            first = false;
        }
        if self.is_immunity() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IMMUNITY").unwrap();
            first = false;
        }
        if self.is_unknown_28() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_28").unwrap();
            first = false;
        }
        if self.is_unknown_29() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_29").unwrap();
            first = false;
        }
        if self.is_unknown_30() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_30").unwrap();
            first = false;
        }
        if self.is_heal_prediction() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HEAL_PREDICTION").unwrap();
            first = false;
        }
        if self.is_unknown_32() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNKNOWN_32").unwrap();
            first = false;
        }
        s
    }

}

impl CastFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const PENDING: u32 = 0x01;
    pub const HAS_TRAJECTORY: u32 = 0x02;
    pub const UNKNOWN_3: u32 = 0x04;
    pub const UNKNOWN_4: u32 = 0x08;
    pub const UNKNOWN_5: u32 = 0x10;
    pub const AMMO: u32 = 0x20;
    pub const UNKNOWN_7: u32 = 0x40;
    pub const UNKNOWN_8: u32 = 0x80;
    pub const UNKNOWN_9: u32 = 0x100;
    pub const UNKNOWN_10: u32 = 0x200;
    pub const UNKNOWN_11: u32 = 0x400;
    pub const POWER_LEFT_SELF: u32 = 0x800;
    pub const UNKNOWN_13: u32 = 0x1000;
    pub const UNKNOWN_14: u32 = 0x2000;
    pub const UNKNOWN_15: u32 = 0x4000;
    pub const UNKNOWN_16: u32 = 0x8000;
    pub const UNKNOWN_17: u32 = 0x10000;
    pub const ADJUST_MISSILE: u32 = 0x20000;
    pub const NO_GCD: u32 = 0x40000;
    pub const VISUAL_CHAIN: u32 = 0x80000;
    pub const UNKNOWN_21: u32 = 0x100000;
    pub const RUNE_LIST: u32 = 0x200000;
    pub const UNKNOWN_23: u32 = 0x400000;
    pub const UNKNOWN_24: u32 = 0x800000;
    pub const UNKNOWN_25: u32 = 0x1000000;
    pub const UNKNOWN_26: u32 = 0x2000000;
    pub const IMMUNITY: u32 = 0x4000000;
    pub const UNKNOWN_28: u32 = 0x8000000;
    pub const UNKNOWN_29: u32 = 0x10000000;
    pub const UNKNOWN_30: u32 = 0x20000000;
    pub const HEAL_PREDICTION: u32 = 0x40000000;
    pub const UNKNOWN_32: u32 = 0x80000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::PENDING
                | Self::HAS_TRAJECTORY
                | Self::UNKNOWN_3
                | Self::UNKNOWN_4
                | Self::UNKNOWN_5
                | Self::AMMO
                | Self::UNKNOWN_7
                | Self::UNKNOWN_8
                | Self::UNKNOWN_9
                | Self::UNKNOWN_10
                | Self::UNKNOWN_11
                | Self::POWER_LEFT_SELF
                | Self::UNKNOWN_13
                | Self::UNKNOWN_14
                | Self::UNKNOWN_15
                | Self::UNKNOWN_16
                | Self::UNKNOWN_17
                | Self::ADJUST_MISSILE
                | Self::NO_GCD
                | Self::VISUAL_CHAIN
                | Self::UNKNOWN_21
                | Self::RUNE_LIST
                | Self::UNKNOWN_23
                | Self::UNKNOWN_24
                | Self::UNKNOWN_25
                | Self::UNKNOWN_26
                | Self::IMMUNITY
                | Self::UNKNOWN_28
                | Self::UNKNOWN_29
                | Self::UNKNOWN_30
                | Self::HEAL_PREDICTION
                | Self::UNKNOWN_32
        }
    }

    pub const fn is_pending(&self) -> bool {
        (self.inner & Self::PENDING) != 0
    }

    /// aoe combat log?
    pub const fn new_pending() -> Self {
        Self { inner: Self::PENDING }
    }

    pub fn set_pending(&mut self) -> Self {
        self.inner |= Self::PENDING;
        *self
    }

    pub fn clear_pending(&mut self) -> Self {
        self.inner &= Self::PENDING.reverse_bits();
        *self
    }

    pub const fn is_has_trajectory(&self) -> bool {
        (self.inner & Self::HAS_TRAJECTORY) != 0
    }

    pub const fn new_has_trajectory() -> Self {
        Self { inner: Self::HAS_TRAJECTORY }
    }

    pub fn set_has_trajectory(&mut self) -> Self {
        self.inner |= Self::HAS_TRAJECTORY;
        *self
    }

    pub fn clear_has_trajectory(&mut self) -> Self {
        self.inner &= Self::HAS_TRAJECTORY.reverse_bits();
        *self
    }

    pub const fn is_unknown_3(&self) -> bool {
        (self.inner & Self::UNKNOWN_3) != 0
    }

    pub const fn new_unknown_3() -> Self {
        Self { inner: Self::UNKNOWN_3 }
    }

    pub fn set_unknown_3(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_3;
        *self
    }

    pub fn clear_unknown_3(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_3.reverse_bits();
        *self
    }

    pub const fn is_unknown_4(&self) -> bool {
        (self.inner & Self::UNKNOWN_4) != 0
    }

    /// ignore AOE visual
    pub const fn new_unknown_4() -> Self {
        Self { inner: Self::UNKNOWN_4 }
    }

    pub fn set_unknown_4(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_4;
        *self
    }

    pub fn clear_unknown_4(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_4.reverse_bits();
        *self
    }

    pub const fn is_unknown_5(&self) -> bool {
        (self.inner & Self::UNKNOWN_5) != 0
    }

    pub const fn new_unknown_5() -> Self {
        Self { inner: Self::UNKNOWN_5 }
    }

    pub fn set_unknown_5(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_5;
        *self
    }

    pub fn clear_unknown_5(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_5.reverse_bits();
        *self
    }

    pub const fn is_ammo(&self) -> bool {
        (self.inner & Self::AMMO) != 0
    }

    /// Projectiles visual
    pub const fn new_ammo() -> Self {
        Self { inner: Self::AMMO }
    }

    pub fn set_ammo(&mut self) -> Self {
        self.inner |= Self::AMMO;
        *self
    }

    pub fn clear_ammo(&mut self) -> Self {
        self.inner &= Self::AMMO.reverse_bits();
        *self
    }

    pub const fn is_unknown_7(&self) -> bool {
        (self.inner & Self::UNKNOWN_7) != 0
    }

    pub const fn new_unknown_7() -> Self {
        Self { inner: Self::UNKNOWN_7 }
    }

    pub fn set_unknown_7(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_7;
        *self
    }

    pub fn clear_unknown_7(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_7.reverse_bits();
        *self
    }

    pub const fn is_unknown_8(&self) -> bool {
        (self.inner & Self::UNKNOWN_8) != 0
    }

    pub const fn new_unknown_8() -> Self {
        Self { inner: Self::UNKNOWN_8 }
    }

    pub fn set_unknown_8(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_8;
        *self
    }

    pub fn clear_unknown_8(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_8.reverse_bits();
        *self
    }

    pub const fn is_unknown_9(&self) -> bool {
        (self.inner & Self::UNKNOWN_9) != 0
    }

    pub const fn new_unknown_9() -> Self {
        Self { inner: Self::UNKNOWN_9 }
    }

    pub fn set_unknown_9(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_9;
        *self
    }

    pub fn clear_unknown_9(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_9.reverse_bits();
        *self
    }

    pub const fn is_unknown_10(&self) -> bool {
        (self.inner & Self::UNKNOWN_10) != 0
    }

    pub const fn new_unknown_10() -> Self {
        Self { inner: Self::UNKNOWN_10 }
    }

    pub fn set_unknown_10(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_10;
        *self
    }

    pub fn clear_unknown_10(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_10.reverse_bits();
        *self
    }

    pub const fn is_unknown_11(&self) -> bool {
        (self.inner & Self::UNKNOWN_11) != 0
    }

    pub const fn new_unknown_11() -> Self {
        Self { inner: Self::UNKNOWN_11 }
    }

    pub fn set_unknown_11(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_11;
        *self
    }

    pub fn clear_unknown_11(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_11.reverse_bits();
        *self
    }

    pub const fn is_power_left_self(&self) -> bool {
        (self.inner & Self::POWER_LEFT_SELF) != 0
    }

    pub const fn new_power_left_self() -> Self {
        Self { inner: Self::POWER_LEFT_SELF }
    }

    pub fn set_power_left_self(&mut self) -> Self {
        self.inner |= Self::POWER_LEFT_SELF;
        *self
    }

    pub fn clear_power_left_self(&mut self) -> Self {
        self.inner &= Self::POWER_LEFT_SELF.reverse_bits();
        *self
    }

    pub const fn is_unknown_13(&self) -> bool {
        (self.inner & Self::UNKNOWN_13) != 0
    }

    pub const fn new_unknown_13() -> Self {
        Self { inner: Self::UNKNOWN_13 }
    }

    pub fn set_unknown_13(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_13;
        *self
    }

    pub fn clear_unknown_13(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_13.reverse_bits();
        *self
    }

    pub const fn is_unknown_14(&self) -> bool {
        (self.inner & Self::UNKNOWN_14) != 0
    }

    pub const fn new_unknown_14() -> Self {
        Self { inner: Self::UNKNOWN_14 }
    }

    pub fn set_unknown_14(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_14;
        *self
    }

    pub fn clear_unknown_14(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_14.reverse_bits();
        *self
    }

    pub const fn is_unknown_15(&self) -> bool {
        (self.inner & Self::UNKNOWN_15) != 0
    }

    pub const fn new_unknown_15() -> Self {
        Self { inner: Self::UNKNOWN_15 }
    }

    pub fn set_unknown_15(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_15;
        *self
    }

    pub fn clear_unknown_15(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_15.reverse_bits();
        *self
    }

    pub const fn is_unknown_16(&self) -> bool {
        (self.inner & Self::UNKNOWN_16) != 0
    }

    pub const fn new_unknown_16() -> Self {
        Self { inner: Self::UNKNOWN_16 }
    }

    pub fn set_unknown_16(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_16;
        *self
    }

    pub fn clear_unknown_16(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_16.reverse_bits();
        *self
    }

    pub const fn is_unknown_17(&self) -> bool {
        (self.inner & Self::UNKNOWN_17) != 0
    }

    pub const fn new_unknown_17() -> Self {
        Self { inner: Self::UNKNOWN_17 }
    }

    pub fn set_unknown_17(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_17;
        *self
    }

    pub fn clear_unknown_17(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_17.reverse_bits();
        *self
    }

    pub const fn is_adjust_missile(&self) -> bool {
        (self.inner & Self::ADJUST_MISSILE) != 0
    }

    pub const fn new_adjust_missile() -> Self {
        Self { inner: Self::ADJUST_MISSILE }
    }

    pub fn set_adjust_missile(&mut self) -> Self {
        self.inner |= Self::ADJUST_MISSILE;
        *self
    }

    pub fn clear_adjust_missile(&mut self) -> Self {
        self.inner &= Self::ADJUST_MISSILE.reverse_bits();
        *self
    }

    pub const fn is_no_gcd(&self) -> bool {
        (self.inner & Self::NO_GCD) != 0
    }

    /// no GCD for spell casts from charm/summon (vehicle spells is an example)
    pub const fn new_no_gcd() -> Self {
        Self { inner: Self::NO_GCD }
    }

    pub fn set_no_gcd(&mut self) -> Self {
        self.inner |= Self::NO_GCD;
        *self
    }

    pub fn clear_no_gcd(&mut self) -> Self {
        self.inner &= Self::NO_GCD.reverse_bits();
        *self
    }

    pub const fn is_visual_chain(&self) -> bool {
        (self.inner & Self::VISUAL_CHAIN) != 0
    }

    pub const fn new_visual_chain() -> Self {
        Self { inner: Self::VISUAL_CHAIN }
    }

    pub fn set_visual_chain(&mut self) -> Self {
        self.inner |= Self::VISUAL_CHAIN;
        *self
    }

    pub fn clear_visual_chain(&mut self) -> Self {
        self.inner &= Self::VISUAL_CHAIN.reverse_bits();
        *self
    }

    pub const fn is_unknown_21(&self) -> bool {
        (self.inner & Self::UNKNOWN_21) != 0
    }

    pub const fn new_unknown_21() -> Self {
        Self { inner: Self::UNKNOWN_21 }
    }

    pub fn set_unknown_21(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_21;
        *self
    }

    pub fn clear_unknown_21(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_21.reverse_bits();
        *self
    }

    pub const fn is_rune_list(&self) -> bool {
        (self.inner & Self::RUNE_LIST) != 0
    }

    pub const fn new_rune_list() -> Self {
        Self { inner: Self::RUNE_LIST }
    }

    pub fn set_rune_list(&mut self) -> Self {
        self.inner |= Self::RUNE_LIST;
        *self
    }

    pub fn clear_rune_list(&mut self) -> Self {
        self.inner &= Self::RUNE_LIST.reverse_bits();
        *self
    }

    pub const fn is_unknown_23(&self) -> bool {
        (self.inner & Self::UNKNOWN_23) != 0
    }

    pub const fn new_unknown_23() -> Self {
        Self { inner: Self::UNKNOWN_23 }
    }

    pub fn set_unknown_23(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_23;
        *self
    }

    pub fn clear_unknown_23(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_23.reverse_bits();
        *self
    }

    pub const fn is_unknown_24(&self) -> bool {
        (self.inner & Self::UNKNOWN_24) != 0
    }

    pub const fn new_unknown_24() -> Self {
        Self { inner: Self::UNKNOWN_24 }
    }

    pub fn set_unknown_24(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_24;
        *self
    }

    pub fn clear_unknown_24(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_24.reverse_bits();
        *self
    }

    pub const fn is_unknown_25(&self) -> bool {
        (self.inner & Self::UNKNOWN_25) != 0
    }

    pub const fn new_unknown_25() -> Self {
        Self { inner: Self::UNKNOWN_25 }
    }

    pub fn set_unknown_25(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_25;
        *self
    }

    pub fn clear_unknown_25(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_25.reverse_bits();
        *self
    }

    pub const fn is_unknown_26(&self) -> bool {
        (self.inner & Self::UNKNOWN_26) != 0
    }

    pub const fn new_unknown_26() -> Self {
        Self { inner: Self::UNKNOWN_26 }
    }

    pub fn set_unknown_26(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_26;
        *self
    }

    pub fn clear_unknown_26(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_26.reverse_bits();
        *self
    }

    pub const fn is_immunity(&self) -> bool {
        (self.inner & Self::IMMUNITY) != 0
    }

    pub const fn new_immunity() -> Self {
        Self { inner: Self::IMMUNITY }
    }

    pub fn set_immunity(&mut self) -> Self {
        self.inner |= Self::IMMUNITY;
        *self
    }

    pub fn clear_immunity(&mut self) -> Self {
        self.inner &= Self::IMMUNITY.reverse_bits();
        *self
    }

    pub const fn is_unknown_28(&self) -> bool {
        (self.inner & Self::UNKNOWN_28) != 0
    }

    pub const fn new_unknown_28() -> Self {
        Self { inner: Self::UNKNOWN_28 }
    }

    pub fn set_unknown_28(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_28;
        *self
    }

    pub fn clear_unknown_28(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_28.reverse_bits();
        *self
    }

    pub const fn is_unknown_29(&self) -> bool {
        (self.inner & Self::UNKNOWN_29) != 0
    }

    pub const fn new_unknown_29() -> Self {
        Self { inner: Self::UNKNOWN_29 }
    }

    pub fn set_unknown_29(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_29;
        *self
    }

    pub fn clear_unknown_29(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_29.reverse_bits();
        *self
    }

    pub const fn is_unknown_30(&self) -> bool {
        (self.inner & Self::UNKNOWN_30) != 0
    }

    pub const fn new_unknown_30() -> Self {
        Self { inner: Self::UNKNOWN_30 }
    }

    pub fn set_unknown_30(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_30;
        *self
    }

    pub fn clear_unknown_30(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_30.reverse_bits();
        *self
    }

    pub const fn is_heal_prediction(&self) -> bool {
        (self.inner & Self::HEAL_PREDICTION) != 0
    }

    /// Unused on TC 3.3.5a. Defined from TC Master.
    pub const fn new_heal_prediction() -> Self {
        Self { inner: Self::HEAL_PREDICTION }
    }

    pub fn set_heal_prediction(&mut self) -> Self {
        self.inner |= Self::HEAL_PREDICTION;
        *self
    }

    pub fn clear_heal_prediction(&mut self) -> Self {
        self.inner &= Self::HEAL_PREDICTION.reverse_bits();
        *self
    }

    pub const fn is_unknown_32(&self) -> bool {
        (self.inner & Self::UNKNOWN_32) != 0
    }

    pub const fn new_unknown_32() -> Self {
        Self { inner: Self::UNKNOWN_32 }
    }

    pub fn set_unknown_32(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_32;
        *self
    }

    pub fn clear_unknown_32(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_32.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for CastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for CastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for CastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for CastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for CastFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for CastFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for CastFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for CastFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for CastFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for CastFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

