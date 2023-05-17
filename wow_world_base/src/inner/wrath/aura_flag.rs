/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm#L1):
/// ```text
/// flag AuraFlag : u8 {
///     EMPTY = 0x0;
///     EFFECT_1 = 0x1;
///     EFFECT_2 = 0x2;
///     EFFECT_3 = 0x4;
///     NOT_CASTER = 0x8;
///     SET = 0x9;
///     CANCELLABLE = 0x10;
///     DURATION = 0x20;
///     HIDE = 0x40;
///     NEGATIVE = 0x80;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AuraFlag {
    inner: u8,
}

impl AuraFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const EMPTY: u8 = 0x00;
    pub const EFFECT_1: u8 = 0x01;
    pub const EFFECT_2: u8 = 0x02;
    pub const EFFECT_3: u8 = 0x04;
    pub const NOT_CASTER: u8 = 0x08;
    pub const SET: u8 = 0x09;
    pub const CANCELLABLE: u8 = 0x10;
    pub const DURATION: u8 = 0x20;
    pub const HIDE: u8 = 0x40;
    pub const NEGATIVE: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::EMPTY
                | Self::EFFECT_1
                | Self::EFFECT_2
                | Self::EFFECT_3
                | Self::NOT_CASTER
                | Self::SET
                | Self::CANCELLABLE
                | Self::DURATION
                | Self::HIDE
                | Self::NEGATIVE
        }
    }

    pub const fn is_effect_1(&self) -> bool {
        (self.inner & Self::EFFECT_1) != 0
    }

    pub const fn new_effect_1() -> Self {
        Self { inner: Self::EFFECT_1 }
    }

    pub fn set_effect_1(&mut self) -> Self {
        self.inner |= Self::EFFECT_1;
        *self
    }

    pub fn clear_effect_1(&mut self) -> Self {
        self.inner &= Self::EFFECT_1.reverse_bits();
        *self
    }

    pub const fn is_effect_2(&self) -> bool {
        (self.inner & Self::EFFECT_2) != 0
    }

    pub const fn new_effect_2() -> Self {
        Self { inner: Self::EFFECT_2 }
    }

    pub fn set_effect_2(&mut self) -> Self {
        self.inner |= Self::EFFECT_2;
        *self
    }

    pub fn clear_effect_2(&mut self) -> Self {
        self.inner &= Self::EFFECT_2.reverse_bits();
        *self
    }

    pub const fn is_effect_3(&self) -> bool {
        (self.inner & Self::EFFECT_3) != 0
    }

    pub const fn new_effect_3() -> Self {
        Self { inner: Self::EFFECT_3 }
    }

    pub fn set_effect_3(&mut self) -> Self {
        self.inner |= Self::EFFECT_3;
        *self
    }

    pub fn clear_effect_3(&mut self) -> Self {
        self.inner &= Self::EFFECT_3.reverse_bits();
        *self
    }

    pub const fn is_not_caster(&self) -> bool {
        (self.inner & Self::NOT_CASTER) != 0
    }

    pub const fn new_not_caster() -> Self {
        Self { inner: Self::NOT_CASTER }
    }

    pub fn set_not_caster(&mut self) -> Self {
        self.inner |= Self::NOT_CASTER;
        *self
    }

    pub fn clear_not_caster(&mut self) -> Self {
        self.inner &= Self::NOT_CASTER.reverse_bits();
        *self
    }

    pub const fn is_set(&self) -> bool {
        (self.inner & Self::SET) != 0
    }

    pub const fn new_set() -> Self {
        Self { inner: Self::SET }
    }

    pub fn set_set(&mut self) -> Self {
        self.inner |= Self::SET;
        *self
    }

    pub fn clear_set(&mut self) -> Self {
        self.inner &= Self::SET.reverse_bits();
        *self
    }

    pub const fn is_cancellable(&self) -> bool {
        (self.inner & Self::CANCELLABLE) != 0
    }

    pub const fn new_cancellable() -> Self {
        Self { inner: Self::CANCELLABLE }
    }

    pub fn set_cancellable(&mut self) -> Self {
        self.inner |= Self::CANCELLABLE;
        *self
    }

    pub fn clear_cancellable(&mut self) -> Self {
        self.inner &= Self::CANCELLABLE.reverse_bits();
        *self
    }

    pub const fn is_duration(&self) -> bool {
        (self.inner & Self::DURATION) != 0
    }

    pub const fn new_duration() -> Self {
        Self { inner: Self::DURATION }
    }

    pub fn set_duration(&mut self) -> Self {
        self.inner |= Self::DURATION;
        *self
    }

    pub fn clear_duration(&mut self) -> Self {
        self.inner &= Self::DURATION.reverse_bits();
        *self
    }

    pub const fn is_hide(&self) -> bool {
        (self.inner & Self::HIDE) != 0
    }

    /// Seems to hide the aura and tell client the aura was removed
    ///
    pub const fn new_hide() -> Self {
        Self { inner: Self::HIDE }
    }

    pub fn set_hide(&mut self) -> Self {
        self.inner |= Self::HIDE;
        *self
    }

    pub fn clear_hide(&mut self) -> Self {
        self.inner &= Self::HIDE.reverse_bits();
        *self
    }

    pub const fn is_negative(&self) -> bool {
        (self.inner & Self::NEGATIVE) != 0
    }

    pub const fn new_negative() -> Self {
        Self { inner: Self::NEGATIVE }
    }

    pub fn set_negative(&mut self) -> Self {
        self.inner |= Self::NEGATIVE;
        *self
    }

    pub fn clear_negative(&mut self) -> Self {
        self.inner &= Self::NEGATIVE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for AuraFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AuraFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AuraFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AuraFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for AuraFlag {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for AuraFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for AuraFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for AuraFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for AuraFlag {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for AuraFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

