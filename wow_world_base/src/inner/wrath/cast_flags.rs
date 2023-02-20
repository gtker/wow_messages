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

    pub const fn is_PENDING(&self) -> bool {
        (self.inner & Self::PENDING) != 0
    }

    /// aoe combat log?
    ///
    pub const fn new_PENDING() -> Self {
        Self { inner: Self::PENDING }
    }

    pub fn set_PENDING(&mut self) -> Self {
        self.inner |= Self::PENDING;
        *self
    }

    pub fn clear_PENDING(&mut self) -> Self {
        self.inner &= Self::PENDING.reverse_bits();
        *self
    }

    pub const fn is_HAS_TRAJECTORY(&self) -> bool {
        (self.inner & Self::HAS_TRAJECTORY) != 0
    }

    pub const fn new_HAS_TRAJECTORY() -> Self {
        Self { inner: Self::HAS_TRAJECTORY }
    }

    pub fn set_HAS_TRAJECTORY(&mut self) -> Self {
        self.inner |= Self::HAS_TRAJECTORY;
        *self
    }

    pub fn clear_HAS_TRAJECTORY(&mut self) -> Self {
        self.inner &= Self::HAS_TRAJECTORY.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_3(&self) -> bool {
        (self.inner & Self::UNKNOWN_3) != 0
    }

    pub const fn new_UNKNOWN_3() -> Self {
        Self { inner: Self::UNKNOWN_3 }
    }

    pub fn set_UNKNOWN_3(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_3;
        *self
    }

    pub fn clear_UNKNOWN_3(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_3.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_4(&self) -> bool {
        (self.inner & Self::UNKNOWN_4) != 0
    }

    /// ignore AOE visual
    ///
    pub const fn new_UNKNOWN_4() -> Self {
        Self { inner: Self::UNKNOWN_4 }
    }

    pub fn set_UNKNOWN_4(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_4;
        *self
    }

    pub fn clear_UNKNOWN_4(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_4.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_5(&self) -> bool {
        (self.inner & Self::UNKNOWN_5) != 0
    }

    pub const fn new_UNKNOWN_5() -> Self {
        Self { inner: Self::UNKNOWN_5 }
    }

    pub fn set_UNKNOWN_5(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_5;
        *self
    }

    pub fn clear_UNKNOWN_5(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_5.reverse_bits();
        *self
    }

    pub const fn is_AMMO(&self) -> bool {
        (self.inner & Self::AMMO) != 0
    }

    /// Projectiles visual
    ///
    pub const fn new_AMMO() -> Self {
        Self { inner: Self::AMMO }
    }

    pub fn set_AMMO(&mut self) -> Self {
        self.inner |= Self::AMMO;
        *self
    }

    pub fn clear_AMMO(&mut self) -> Self {
        self.inner &= Self::AMMO.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_7(&self) -> bool {
        (self.inner & Self::UNKNOWN_7) != 0
    }

    pub const fn new_UNKNOWN_7() -> Self {
        Self { inner: Self::UNKNOWN_7 }
    }

    pub fn set_UNKNOWN_7(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_7;
        *self
    }

    pub fn clear_UNKNOWN_7(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_7.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_8(&self) -> bool {
        (self.inner & Self::UNKNOWN_8) != 0
    }

    pub const fn new_UNKNOWN_8() -> Self {
        Self { inner: Self::UNKNOWN_8 }
    }

    pub fn set_UNKNOWN_8(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_8;
        *self
    }

    pub fn clear_UNKNOWN_8(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_8.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_9(&self) -> bool {
        (self.inner & Self::UNKNOWN_9) != 0
    }

    pub const fn new_UNKNOWN_9() -> Self {
        Self { inner: Self::UNKNOWN_9 }
    }

    pub fn set_UNKNOWN_9(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_9;
        *self
    }

    pub fn clear_UNKNOWN_9(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_9.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_10(&self) -> bool {
        (self.inner & Self::UNKNOWN_10) != 0
    }

    pub const fn new_UNKNOWN_10() -> Self {
        Self { inner: Self::UNKNOWN_10 }
    }

    pub fn set_UNKNOWN_10(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_10;
        *self
    }

    pub fn clear_UNKNOWN_10(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_10.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_11(&self) -> bool {
        (self.inner & Self::UNKNOWN_11) != 0
    }

    pub const fn new_UNKNOWN_11() -> Self {
        Self { inner: Self::UNKNOWN_11 }
    }

    pub fn set_UNKNOWN_11(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_11;
        *self
    }

    pub fn clear_UNKNOWN_11(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_11.reverse_bits();
        *self
    }

    pub const fn is_POWER_LEFT_SELF(&self) -> bool {
        (self.inner & Self::POWER_LEFT_SELF) != 0
    }

    pub const fn new_POWER_LEFT_SELF() -> Self {
        Self { inner: Self::POWER_LEFT_SELF }
    }

    pub fn set_POWER_LEFT_SELF(&mut self) -> Self {
        self.inner |= Self::POWER_LEFT_SELF;
        *self
    }

    pub fn clear_POWER_LEFT_SELF(&mut self) -> Self {
        self.inner &= Self::POWER_LEFT_SELF.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_13(&self) -> bool {
        (self.inner & Self::UNKNOWN_13) != 0
    }

    pub const fn new_UNKNOWN_13() -> Self {
        Self { inner: Self::UNKNOWN_13 }
    }

    pub fn set_UNKNOWN_13(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_13;
        *self
    }

    pub fn clear_UNKNOWN_13(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_13.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_14(&self) -> bool {
        (self.inner & Self::UNKNOWN_14) != 0
    }

    pub const fn new_UNKNOWN_14() -> Self {
        Self { inner: Self::UNKNOWN_14 }
    }

    pub fn set_UNKNOWN_14(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_14;
        *self
    }

    pub fn clear_UNKNOWN_14(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_14.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_15(&self) -> bool {
        (self.inner & Self::UNKNOWN_15) != 0
    }

    pub const fn new_UNKNOWN_15() -> Self {
        Self { inner: Self::UNKNOWN_15 }
    }

    pub fn set_UNKNOWN_15(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_15;
        *self
    }

    pub fn clear_UNKNOWN_15(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_15.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_16(&self) -> bool {
        (self.inner & Self::UNKNOWN_16) != 0
    }

    pub const fn new_UNKNOWN_16() -> Self {
        Self { inner: Self::UNKNOWN_16 }
    }

    pub fn set_UNKNOWN_16(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_16;
        *self
    }

    pub fn clear_UNKNOWN_16(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_16.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_17(&self) -> bool {
        (self.inner & Self::UNKNOWN_17) != 0
    }

    pub const fn new_UNKNOWN_17() -> Self {
        Self { inner: Self::UNKNOWN_17 }
    }

    pub fn set_UNKNOWN_17(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_17;
        *self
    }

    pub fn clear_UNKNOWN_17(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_17.reverse_bits();
        *self
    }

    pub const fn is_ADJUST_MISSILE(&self) -> bool {
        (self.inner & Self::ADJUST_MISSILE) != 0
    }

    pub const fn new_ADJUST_MISSILE() -> Self {
        Self { inner: Self::ADJUST_MISSILE }
    }

    pub fn set_ADJUST_MISSILE(&mut self) -> Self {
        self.inner |= Self::ADJUST_MISSILE;
        *self
    }

    pub fn clear_ADJUST_MISSILE(&mut self) -> Self {
        self.inner &= Self::ADJUST_MISSILE.reverse_bits();
        *self
    }

    pub const fn is_NO_GCD(&self) -> bool {
        (self.inner & Self::NO_GCD) != 0
    }

    /// no GCD for spell casts from charm/summon (vehicle spells is an example)
    ///
    pub const fn new_NO_GCD() -> Self {
        Self { inner: Self::NO_GCD }
    }

    pub fn set_NO_GCD(&mut self) -> Self {
        self.inner |= Self::NO_GCD;
        *self
    }

    pub fn clear_NO_GCD(&mut self) -> Self {
        self.inner &= Self::NO_GCD.reverse_bits();
        *self
    }

    pub const fn is_VISUAL_CHAIN(&self) -> bool {
        (self.inner & Self::VISUAL_CHAIN) != 0
    }

    pub const fn new_VISUAL_CHAIN() -> Self {
        Self { inner: Self::VISUAL_CHAIN }
    }

    pub fn set_VISUAL_CHAIN(&mut self) -> Self {
        self.inner |= Self::VISUAL_CHAIN;
        *self
    }

    pub fn clear_VISUAL_CHAIN(&mut self) -> Self {
        self.inner &= Self::VISUAL_CHAIN.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_21(&self) -> bool {
        (self.inner & Self::UNKNOWN_21) != 0
    }

    pub const fn new_UNKNOWN_21() -> Self {
        Self { inner: Self::UNKNOWN_21 }
    }

    pub fn set_UNKNOWN_21(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_21;
        *self
    }

    pub fn clear_UNKNOWN_21(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_21.reverse_bits();
        *self
    }

    pub const fn is_RUNE_LIST(&self) -> bool {
        (self.inner & Self::RUNE_LIST) != 0
    }

    pub const fn new_RUNE_LIST() -> Self {
        Self { inner: Self::RUNE_LIST }
    }

    pub fn set_RUNE_LIST(&mut self) -> Self {
        self.inner |= Self::RUNE_LIST;
        *self
    }

    pub fn clear_RUNE_LIST(&mut self) -> Self {
        self.inner &= Self::RUNE_LIST.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_23(&self) -> bool {
        (self.inner & Self::UNKNOWN_23) != 0
    }

    pub const fn new_UNKNOWN_23() -> Self {
        Self { inner: Self::UNKNOWN_23 }
    }

    pub fn set_UNKNOWN_23(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_23;
        *self
    }

    pub fn clear_UNKNOWN_23(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_23.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_24(&self) -> bool {
        (self.inner & Self::UNKNOWN_24) != 0
    }

    pub const fn new_UNKNOWN_24() -> Self {
        Self { inner: Self::UNKNOWN_24 }
    }

    pub fn set_UNKNOWN_24(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_24;
        *self
    }

    pub fn clear_UNKNOWN_24(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_24.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_25(&self) -> bool {
        (self.inner & Self::UNKNOWN_25) != 0
    }

    pub const fn new_UNKNOWN_25() -> Self {
        Self { inner: Self::UNKNOWN_25 }
    }

    pub fn set_UNKNOWN_25(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_25;
        *self
    }

    pub fn clear_UNKNOWN_25(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_25.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_26(&self) -> bool {
        (self.inner & Self::UNKNOWN_26) != 0
    }

    pub const fn new_UNKNOWN_26() -> Self {
        Self { inner: Self::UNKNOWN_26 }
    }

    pub fn set_UNKNOWN_26(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_26;
        *self
    }

    pub fn clear_UNKNOWN_26(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_26.reverse_bits();
        *self
    }

    pub const fn is_IMMUNITY(&self) -> bool {
        (self.inner & Self::IMMUNITY) != 0
    }

    pub const fn new_IMMUNITY() -> Self {
        Self { inner: Self::IMMUNITY }
    }

    pub fn set_IMMUNITY(&mut self) -> Self {
        self.inner |= Self::IMMUNITY;
        *self
    }

    pub fn clear_IMMUNITY(&mut self) -> Self {
        self.inner &= Self::IMMUNITY.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_28(&self) -> bool {
        (self.inner & Self::UNKNOWN_28) != 0
    }

    pub const fn new_UNKNOWN_28() -> Self {
        Self { inner: Self::UNKNOWN_28 }
    }

    pub fn set_UNKNOWN_28(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_28;
        *self
    }

    pub fn clear_UNKNOWN_28(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_28.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_29(&self) -> bool {
        (self.inner & Self::UNKNOWN_29) != 0
    }

    pub const fn new_UNKNOWN_29() -> Self {
        Self { inner: Self::UNKNOWN_29 }
    }

    pub fn set_UNKNOWN_29(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_29;
        *self
    }

    pub fn clear_UNKNOWN_29(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_29.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_30(&self) -> bool {
        (self.inner & Self::UNKNOWN_30) != 0
    }

    pub const fn new_UNKNOWN_30() -> Self {
        Self { inner: Self::UNKNOWN_30 }
    }

    pub fn set_UNKNOWN_30(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_30;
        *self
    }

    pub fn clear_UNKNOWN_30(&mut self) -> Self {
        self.inner &= Self::UNKNOWN_30.reverse_bits();
        *self
    }

    pub const fn is_HEAL_PREDICTION(&self) -> bool {
        (self.inner & Self::HEAL_PREDICTION) != 0
    }

    /// Unused on TC 3.3.5a. Defined from TC Master.
    ///
    pub const fn new_HEAL_PREDICTION() -> Self {
        Self { inner: Self::HEAL_PREDICTION }
    }

    pub fn set_HEAL_PREDICTION(&mut self) -> Self {
        self.inner |= Self::HEAL_PREDICTION;
        *self
    }

    pub fn clear_HEAL_PREDICTION(&mut self) -> Self {
        self.inner &= Self::HEAL_PREDICTION.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN_32(&self) -> bool {
        (self.inner & Self::UNKNOWN_32) != 0
    }

    pub const fn new_UNKNOWN_32() -> Self {
        Self { inner: Self::UNKNOWN_32 }
    }

    pub fn set_UNKNOWN_32(&mut self) -> Self {
        self.inner |= Self::UNKNOWN_32;
        *self
    }

    pub fn clear_UNKNOWN_32(&mut self) -> Self {
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

