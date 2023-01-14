/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_aura_update_all.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_aura_update_all.wowm#L1):
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
pub struct AuraFlag {
    inner: u8,
}

impl AuraFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub(crate) const EMPTY: u8 = 0x00;
    pub(crate) const EFFECT_1: u8 = 0x01;
    pub(crate) const EFFECT_2: u8 = 0x02;
    pub(crate) const EFFECT_3: u8 = 0x04;
    pub(crate) const NOT_CASTER: u8 = 0x08;
    pub(crate) const SET: u8 = 0x09;
    pub(crate) const CANCELLABLE: u8 = 0x10;
    pub(crate) const DURATION: u8 = 0x20;
    pub(crate) const HIDE: u8 = 0x40;
    pub(crate) const NEGATIVE: u8 = 0x80;

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

    pub const fn is_EFFECT_1(&self) -> bool {
        (self.inner & Self::EFFECT_1) != 0
    }

    pub const fn new_EFFECT_1() -> Self {
        Self { inner: Self::EFFECT_1 }
    }

    pub fn set_EFFECT_1(&mut self) -> Self {
        self.inner |= Self::EFFECT_1;
        *self
    }

    pub fn clear_EFFECT_1(&mut self) -> Self {
        self.inner &= Self::EFFECT_1.reverse_bits();
        *self
    }

    pub const fn is_EFFECT_2(&self) -> bool {
        (self.inner & Self::EFFECT_2) != 0
    }

    pub const fn new_EFFECT_2() -> Self {
        Self { inner: Self::EFFECT_2 }
    }

    pub fn set_EFFECT_2(&mut self) -> Self {
        self.inner |= Self::EFFECT_2;
        *self
    }

    pub fn clear_EFFECT_2(&mut self) -> Self {
        self.inner &= Self::EFFECT_2.reverse_bits();
        *self
    }

    pub const fn is_EFFECT_3(&self) -> bool {
        (self.inner & Self::EFFECT_3) != 0
    }

    pub const fn new_EFFECT_3() -> Self {
        Self { inner: Self::EFFECT_3 }
    }

    pub fn set_EFFECT_3(&mut self) -> Self {
        self.inner |= Self::EFFECT_3;
        *self
    }

    pub fn clear_EFFECT_3(&mut self) -> Self {
        self.inner &= Self::EFFECT_3.reverse_bits();
        *self
    }

    pub const fn is_NOT_CASTER(&self) -> bool {
        (self.inner & Self::NOT_CASTER) != 0
    }

    pub const fn new_NOT_CASTER() -> Self {
        Self { inner: Self::NOT_CASTER }
    }

    pub fn set_NOT_CASTER(&mut self) -> Self {
        self.inner |= Self::NOT_CASTER;
        *self
    }

    pub fn clear_NOT_CASTER(&mut self) -> Self {
        self.inner &= Self::NOT_CASTER.reverse_bits();
        *self
    }

    pub const fn is_SET(&self) -> bool {
        (self.inner & Self::SET) != 0
    }

    pub const fn new_SET() -> Self {
        Self { inner: Self::SET }
    }

    pub fn set_SET(&mut self) -> Self {
        self.inner |= Self::SET;
        *self
    }

    pub fn clear_SET(&mut self) -> Self {
        self.inner &= Self::SET.reverse_bits();
        *self
    }

    pub const fn is_CANCELLABLE(&self) -> bool {
        (self.inner & Self::CANCELLABLE) != 0
    }

    pub const fn new_CANCELLABLE() -> Self {
        Self { inner: Self::CANCELLABLE }
    }

    pub fn set_CANCELLABLE(&mut self) -> Self {
        self.inner |= Self::CANCELLABLE;
        *self
    }

    pub fn clear_CANCELLABLE(&mut self) -> Self {
        self.inner &= Self::CANCELLABLE.reverse_bits();
        *self
    }

    pub const fn is_DURATION(&self) -> bool {
        (self.inner & Self::DURATION) != 0
    }

    pub const fn new_DURATION() -> Self {
        Self { inner: Self::DURATION }
    }

    pub fn set_DURATION(&mut self) -> Self {
        self.inner |= Self::DURATION;
        *self
    }

    pub fn clear_DURATION(&mut self) -> Self {
        self.inner &= Self::DURATION.reverse_bits();
        *self
    }

    pub const fn is_HIDE(&self) -> bool {
        (self.inner & Self::HIDE) != 0
    }

    /// Seems to hide the aura and tell client the aura was removed
    ///
    pub const fn new_HIDE() -> Self {
        Self { inner: Self::HIDE }
    }

    pub fn set_HIDE(&mut self) -> Self {
        self.inner |= Self::HIDE;
        *self
    }

    pub fn clear_HIDE(&mut self) -> Self {
        self.inner &= Self::HIDE.reverse_bits();
        *self
    }

    pub const fn is_NEGATIVE(&self) -> bool {
        (self.inner & Self::NEGATIVE) != 0
    }

    pub const fn new_NEGATIVE() -> Self {
        Self { inner: Self::NEGATIVE }
    }

    pub fn set_NEGATIVE(&mut self) -> Self {
        self.inner |= Self::NEGATIVE;
        *self
    }

    pub fn clear_NEGATIVE(&mut self) -> Self {
        self.inner &= Self::NEGATIVE.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
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

