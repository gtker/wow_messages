/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm:67`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm#L67):
/// ```text
/// flag HitInfo : u32 {
///     NORMALSWING = 0x00000000;
///     UNK0 = 0x00000001;
///     NORMALSWING2 = 0x00000002;
///     LEFTSWING = 0x00000004;
///     UNK3 = 0x00000008;
///     MISS = 0x00000010;
///     ABSORB = 0x00000020;
///     RESIST = 0x00000040;
///     CRITICALHIT = 0x00000080;
///     UNK8 = 0x00000100;
///     UNK9 = 0x00002000;
///     GLANCING = 0x00004000;
///     CRUSHING = 0x00008000;
///     NOACTION = 0x00010000;
///     SWINGNOHITSOUND = 0x00080000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct HitInfo {
    inner: u32,
}

impl HitInfo {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const NORMALSWING: u32 = 0x00;
    pub(crate) const UNK0: u32 = 0x01;
    pub(crate) const NORMALSWING2: u32 = 0x02;
    pub(crate) const LEFTSWING: u32 = 0x04;
    pub(crate) const UNK3: u32 = 0x08;
    pub(crate) const MISS: u32 = 0x10;
    pub(crate) const ABSORB: u32 = 0x20;
    pub(crate) const RESIST: u32 = 0x40;
    pub(crate) const CRITICALHIT: u32 = 0x80;
    pub(crate) const UNK8: u32 = 0x100;
    pub(crate) const UNK9: u32 = 0x2000;
    pub(crate) const GLANCING: u32 = 0x4000;
    pub(crate) const CRUSHING: u32 = 0x8000;
    pub(crate) const NOACTION: u32 = 0x10000;
    pub(crate) const SWINGNOHITSOUND: u32 = 0x80000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NORMALSWING
                | Self::UNK0
                | Self::NORMALSWING2
                | Self::LEFTSWING
                | Self::UNK3
                | Self::MISS
                | Self::ABSORB
                | Self::RESIST
                | Self::CRITICALHIT
                | Self::UNK8
                | Self::UNK9
                | Self::GLANCING
                | Self::CRUSHING
                | Self::NOACTION
                | Self::SWINGNOHITSOUND
        }
    }

    pub const fn is_UNK0(&self) -> bool {
        (self.inner & Self::UNK0) != 0
    }

    /// req correct packet structure
    ///
    pub const fn new_UNK0() -> Self {
        Self { inner: Self::UNK0 }
    }

    pub fn set_UNK0(&mut self) -> Self {
        self.inner |= Self::UNK0;
        *self
    }

    pub fn clear_UNK0(&mut self) -> Self {
        self.inner &= Self::UNK0.reverse_bits();
        *self
    }

    pub const fn is_NORMALSWING2(&self) -> bool {
        (self.inner & Self::NORMALSWING2) != 0
    }

    pub const fn new_NORMALSWING2() -> Self {
        Self { inner: Self::NORMALSWING2 }
    }

    pub fn set_NORMALSWING2(&mut self) -> Self {
        self.inner |= Self::NORMALSWING2;
        *self
    }

    pub fn clear_NORMALSWING2(&mut self) -> Self {
        self.inner &= Self::NORMALSWING2.reverse_bits();
        *self
    }

    pub const fn is_LEFTSWING(&self) -> bool {
        (self.inner & Self::LEFTSWING) != 0
    }

    pub const fn new_LEFTSWING() -> Self {
        Self { inner: Self::LEFTSWING }
    }

    pub fn set_LEFTSWING(&mut self) -> Self {
        self.inner |= Self::LEFTSWING;
        *self
    }

    pub fn clear_LEFTSWING(&mut self) -> Self {
        self.inner &= Self::LEFTSWING.reverse_bits();
        *self
    }

    pub const fn is_UNK3(&self) -> bool {
        (self.inner & Self::UNK3) != 0
    }

    pub const fn new_UNK3() -> Self {
        Self { inner: Self::UNK3 }
    }

    pub fn set_UNK3(&mut self) -> Self {
        self.inner |= Self::UNK3;
        *self
    }

    pub fn clear_UNK3(&mut self) -> Self {
        self.inner &= Self::UNK3.reverse_bits();
        *self
    }

    pub const fn is_MISS(&self) -> bool {
        (self.inner & Self::MISS) != 0
    }

    pub const fn new_MISS() -> Self {
        Self { inner: Self::MISS }
    }

    pub fn set_MISS(&mut self) -> Self {
        self.inner |= Self::MISS;
        *self
    }

    pub fn clear_MISS(&mut self) -> Self {
        self.inner &= Self::MISS.reverse_bits();
        *self
    }

    pub const fn is_ABSORB(&self) -> bool {
        (self.inner & Self::ABSORB) != 0
    }

    /// plays absorb sound
    ///
    pub const fn new_ABSORB() -> Self {
        Self { inner: Self::ABSORB }
    }

    pub fn set_ABSORB(&mut self) -> Self {
        self.inner |= Self::ABSORB;
        *self
    }

    pub fn clear_ABSORB(&mut self) -> Self {
        self.inner &= Self::ABSORB.reverse_bits();
        *self
    }

    pub const fn is_RESIST(&self) -> bool {
        (self.inner & Self::RESIST) != 0
    }

    /// resisted atleast some damage
    ///
    pub const fn new_RESIST() -> Self {
        Self { inner: Self::RESIST }
    }

    pub fn set_RESIST(&mut self) -> Self {
        self.inner |= Self::RESIST;
        *self
    }

    pub fn clear_RESIST(&mut self) -> Self {
        self.inner &= Self::RESIST.reverse_bits();
        *self
    }

    pub const fn is_CRITICALHIT(&self) -> bool {
        (self.inner & Self::CRITICALHIT) != 0
    }

    pub const fn new_CRITICALHIT() -> Self {
        Self { inner: Self::CRITICALHIT }
    }

    pub fn set_CRITICALHIT(&mut self) -> Self {
        self.inner |= Self::CRITICALHIT;
        *self
    }

    pub fn clear_CRITICALHIT(&mut self) -> Self {
        self.inner &= Self::CRITICALHIT.reverse_bits();
        *self
    }

    pub const fn is_UNK8(&self) -> bool {
        (self.inner & Self::UNK8) != 0
    }

    /// wotlk?
    ///
    pub const fn new_UNK8() -> Self {
        Self { inner: Self::UNK8 }
    }

    pub fn set_UNK8(&mut self) -> Self {
        self.inner |= Self::UNK8;
        *self
    }

    pub fn clear_UNK8(&mut self) -> Self {
        self.inner &= Self::UNK8.reverse_bits();
        *self
    }

    pub const fn is_UNK9(&self) -> bool {
        (self.inner & Self::UNK9) != 0
    }

    /// wotlk?
    ///
    pub const fn new_UNK9() -> Self {
        Self { inner: Self::UNK9 }
    }

    pub fn set_UNK9(&mut self) -> Self {
        self.inner |= Self::UNK9;
        *self
    }

    pub fn clear_UNK9(&mut self) -> Self {
        self.inner &= Self::UNK9.reverse_bits();
        *self
    }

    pub const fn is_GLANCING(&self) -> bool {
        (self.inner & Self::GLANCING) != 0
    }

    pub const fn new_GLANCING() -> Self {
        Self { inner: Self::GLANCING }
    }

    pub fn set_GLANCING(&mut self) -> Self {
        self.inner |= Self::GLANCING;
        *self
    }

    pub fn clear_GLANCING(&mut self) -> Self {
        self.inner &= Self::GLANCING.reverse_bits();
        *self
    }

    pub const fn is_CRUSHING(&self) -> bool {
        (self.inner & Self::CRUSHING) != 0
    }

    pub const fn new_CRUSHING() -> Self {
        Self { inner: Self::CRUSHING }
    }

    pub fn set_CRUSHING(&mut self) -> Self {
        self.inner |= Self::CRUSHING;
        *self
    }

    pub fn clear_CRUSHING(&mut self) -> Self {
        self.inner &= Self::CRUSHING.reverse_bits();
        *self
    }

    pub const fn is_NOACTION(&self) -> bool {
        (self.inner & Self::NOACTION) != 0
    }

    pub const fn new_NOACTION() -> Self {
        Self { inner: Self::NOACTION }
    }

    pub fn set_NOACTION(&mut self) -> Self {
        self.inner |= Self::NOACTION;
        *self
    }

    pub fn clear_NOACTION(&mut self) -> Self {
        self.inner &= Self::NOACTION.reverse_bits();
        *self
    }

    pub const fn is_SWINGNOHITSOUND(&self) -> bool {
        (self.inner & Self::SWINGNOHITSOUND) != 0
    }

    pub const fn new_SWINGNOHITSOUND() -> Self {
        Self { inner: Self::SWINGNOHITSOUND }
    }

    pub fn set_SWINGNOHITSOUND(&mut self) -> Self {
        self.inner |= Self::SWINGNOHITSOUND;
        *self
    }

    pub fn clear_SWINGNOHITSOUND(&mut self) -> Self {
        self.inner &= Self::SWINGNOHITSOUND.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

