/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm#L3):
/// ```text
/// flag HitInfo : u32 {
///     NORMALSWING = 0x00000000;
///     UNK1 = 0x00000001;
///     AFFECTS_VICTIM = 0x00000002;
///     OFFHAND = 0x00000004;
///     UNK2 = 0x00000008;
///     MISS = 0x00000010;
///     FULL_ABSORB = 0x00000020;
///     PARTIAL_ABSORB = 0x00000040;
///     ALL_ABSORB = 0x00000060;
///     FULL_RESIST = 0x00000080;
///     PARTIAL_RESIST = 0x00000100;
///     ALL_RESIST = 0x00000180;
///     CRITICALHIT = 0x00000200;
///     UNK10 = 0x00000400;
///     UNK11 = 0x00000800;
///     UNK12 = 0x00001000;
///     BLOCK = 0x00002000;
///     UNK14 = 0x00004000;
///     UNK15 = 0x00008000;
///     GLANCING = 0x00010000;
///     CRUSHING = 0x00020000;
///     NO_ANIMATION = 0x00040000;
///     UNK19 = 0x00080000;
///     UNK20 = 0x00100000;
///     SWINGNOHITSOUND = 0x00200000;
///     UNK22 = 0x00400000;
///     RAGE_GAIN = 0x00800000;
///     FAKE_DAMAGE = 0x01000000;
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
    pub(crate) const UNK1: u32 = 0x01;
    pub(crate) const AFFECTS_VICTIM: u32 = 0x02;
    pub(crate) const OFFHAND: u32 = 0x04;
    pub(crate) const UNK2: u32 = 0x08;
    pub(crate) const MISS: u32 = 0x10;
    pub(crate) const FULL_ABSORB: u32 = 0x20;
    pub(crate) const PARTIAL_ABSORB: u32 = 0x40;
    pub(crate) const ALL_ABSORB: u32 = 0x60;
    pub(crate) const FULL_RESIST: u32 = 0x80;
    pub(crate) const PARTIAL_RESIST: u32 = 0x100;
    pub(crate) const ALL_RESIST: u32 = 0x180;
    pub(crate) const CRITICALHIT: u32 = 0x200;
    pub(crate) const UNK10: u32 = 0x400;
    pub(crate) const UNK11: u32 = 0x800;
    pub(crate) const UNK12: u32 = 0x1000;
    pub(crate) const BLOCK: u32 = 0x2000;
    pub(crate) const UNK14: u32 = 0x4000;
    pub(crate) const UNK15: u32 = 0x8000;
    pub(crate) const GLANCING: u32 = 0x10000;
    pub(crate) const CRUSHING: u32 = 0x20000;
    pub(crate) const NO_ANIMATION: u32 = 0x40000;
    pub(crate) const UNK19: u32 = 0x80000;
    pub(crate) const UNK20: u32 = 0x100000;
    pub(crate) const SWINGNOHITSOUND: u32 = 0x200000;
    pub(crate) const UNK22: u32 = 0x400000;
    pub(crate) const RAGE_GAIN: u32 = 0x800000;
    pub(crate) const FAKE_DAMAGE: u32 = 0x1000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NORMALSWING
                | Self::UNK1
                | Self::AFFECTS_VICTIM
                | Self::OFFHAND
                | Self::UNK2
                | Self::MISS
                | Self::FULL_ABSORB
                | Self::PARTIAL_ABSORB
                | Self::ALL_ABSORB
                | Self::FULL_RESIST
                | Self::PARTIAL_RESIST
                | Self::ALL_RESIST
                | Self::CRITICALHIT
                | Self::UNK10
                | Self::UNK11
                | Self::UNK12
                | Self::BLOCK
                | Self::UNK14
                | Self::UNK15
                | Self::GLANCING
                | Self::CRUSHING
                | Self::NO_ANIMATION
                | Self::UNK19
                | Self::UNK20
                | Self::SWINGNOHITSOUND
                | Self::UNK22
                | Self::RAGE_GAIN
                | Self::FAKE_DAMAGE
        }
    }

    pub const fn is_UNK1(&self) -> bool {
        (self.inner & Self::UNK1) != 0
    }

    /// req correct packet structure
    ///
    pub const fn new_UNK1() -> Self {
        Self { inner: Self::UNK1 }
    }

    pub fn set_UNK1(&mut self) -> Self {
        self.inner |= Self::UNK1;
        *self
    }

    pub fn clear_UNK1(&mut self) -> Self {
        self.inner &= Self::UNK1.reverse_bits();
        *self
    }

    pub const fn is_AFFECTS_VICTIM(&self) -> bool {
        (self.inner & Self::AFFECTS_VICTIM) != 0
    }

    pub const fn new_AFFECTS_VICTIM() -> Self {
        Self { inner: Self::AFFECTS_VICTIM }
    }

    pub fn set_AFFECTS_VICTIM(&mut self) -> Self {
        self.inner |= Self::AFFECTS_VICTIM;
        *self
    }

    pub fn clear_AFFECTS_VICTIM(&mut self) -> Self {
        self.inner &= Self::AFFECTS_VICTIM.reverse_bits();
        *self
    }

    pub const fn is_OFFHAND(&self) -> bool {
        (self.inner & Self::OFFHAND) != 0
    }

    pub const fn new_OFFHAND() -> Self {
        Self { inner: Self::OFFHAND }
    }

    pub fn set_OFFHAND(&mut self) -> Self {
        self.inner |= Self::OFFHAND;
        *self
    }

    pub fn clear_OFFHAND(&mut self) -> Self {
        self.inner &= Self::OFFHAND.reverse_bits();
        *self
    }

    pub const fn is_UNK2(&self) -> bool {
        (self.inner & Self::UNK2) != 0
    }

    pub const fn new_UNK2() -> Self {
        Self { inner: Self::UNK2 }
    }

    pub fn set_UNK2(&mut self) -> Self {
        self.inner |= Self::UNK2;
        *self
    }

    pub fn clear_UNK2(&mut self) -> Self {
        self.inner &= Self::UNK2.reverse_bits();
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

    pub const fn is_FULL_ABSORB(&self) -> bool {
        (self.inner & Self::FULL_ABSORB) != 0
    }

    pub const fn new_FULL_ABSORB() -> Self {
        Self { inner: Self::FULL_ABSORB }
    }

    pub fn set_FULL_ABSORB(&mut self) -> Self {
        self.inner |= Self::FULL_ABSORB;
        *self
    }

    pub fn clear_FULL_ABSORB(&mut self) -> Self {
        self.inner &= Self::FULL_ABSORB.reverse_bits();
        *self
    }

    pub const fn is_PARTIAL_ABSORB(&self) -> bool {
        (self.inner & Self::PARTIAL_ABSORB) != 0
    }

    pub const fn new_PARTIAL_ABSORB() -> Self {
        Self { inner: Self::PARTIAL_ABSORB }
    }

    pub fn set_PARTIAL_ABSORB(&mut self) -> Self {
        self.inner |= Self::PARTIAL_ABSORB;
        *self
    }

    pub fn clear_PARTIAL_ABSORB(&mut self) -> Self {
        self.inner &= Self::PARTIAL_ABSORB.reverse_bits();
        *self
    }

    pub const fn is_ALL_ABSORB(&self) -> bool {
        (self.inner & Self::ALL_ABSORB) != 0
    }

    pub const fn new_ALL_ABSORB() -> Self {
        Self { inner: Self::ALL_ABSORB }
    }

    pub fn set_ALL_ABSORB(&mut self) -> Self {
        self.inner |= Self::ALL_ABSORB;
        *self
    }

    pub fn clear_ALL_ABSORB(&mut self) -> Self {
        self.inner &= Self::ALL_ABSORB.reverse_bits();
        *self
    }

    pub const fn is_FULL_RESIST(&self) -> bool {
        (self.inner & Self::FULL_RESIST) != 0
    }

    pub const fn new_FULL_RESIST() -> Self {
        Self { inner: Self::FULL_RESIST }
    }

    pub fn set_FULL_RESIST(&mut self) -> Self {
        self.inner |= Self::FULL_RESIST;
        *self
    }

    pub fn clear_FULL_RESIST(&mut self) -> Self {
        self.inner &= Self::FULL_RESIST.reverse_bits();
        *self
    }

    pub const fn is_PARTIAL_RESIST(&self) -> bool {
        (self.inner & Self::PARTIAL_RESIST) != 0
    }

    pub const fn new_PARTIAL_RESIST() -> Self {
        Self { inner: Self::PARTIAL_RESIST }
    }

    pub fn set_PARTIAL_RESIST(&mut self) -> Self {
        self.inner |= Self::PARTIAL_RESIST;
        *self
    }

    pub fn clear_PARTIAL_RESIST(&mut self) -> Self {
        self.inner &= Self::PARTIAL_RESIST.reverse_bits();
        *self
    }

    pub const fn is_ALL_RESIST(&self) -> bool {
        (self.inner & Self::ALL_RESIST) != 0
    }

    pub const fn new_ALL_RESIST() -> Self {
        Self { inner: Self::ALL_RESIST }
    }

    pub fn set_ALL_RESIST(&mut self) -> Self {
        self.inner |= Self::ALL_RESIST;
        *self
    }

    pub fn clear_ALL_RESIST(&mut self) -> Self {
        self.inner &= Self::ALL_RESIST.reverse_bits();
        *self
    }

    pub const fn is_CRITICALHIT(&self) -> bool {
        (self.inner & Self::CRITICALHIT) != 0
    }

    /// critical hit
    ///
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

    pub const fn is_UNK10(&self) -> bool {
        (self.inner & Self::UNK10) != 0
    }

    pub const fn new_UNK10() -> Self {
        Self { inner: Self::UNK10 }
    }

    pub fn set_UNK10(&mut self) -> Self {
        self.inner |= Self::UNK10;
        *self
    }

    pub fn clear_UNK10(&mut self) -> Self {
        self.inner &= Self::UNK10.reverse_bits();
        *self
    }

    pub const fn is_UNK11(&self) -> bool {
        (self.inner & Self::UNK11) != 0
    }

    pub const fn new_UNK11() -> Self {
        Self { inner: Self::UNK11 }
    }

    pub fn set_UNK11(&mut self) -> Self {
        self.inner |= Self::UNK11;
        *self
    }

    pub fn clear_UNK11(&mut self) -> Self {
        self.inner &= Self::UNK11.reverse_bits();
        *self
    }

    pub const fn is_UNK12(&self) -> bool {
        (self.inner & Self::UNK12) != 0
    }

    pub const fn new_UNK12() -> Self {
        Self { inner: Self::UNK12 }
    }

    pub fn set_UNK12(&mut self) -> Self {
        self.inner |= Self::UNK12;
        *self
    }

    pub fn clear_UNK12(&mut self) -> Self {
        self.inner &= Self::UNK12.reverse_bits();
        *self
    }

    pub const fn is_BLOCK(&self) -> bool {
        (self.inner & Self::BLOCK) != 0
    }

    /// blocked damage
    ///
    pub const fn new_BLOCK() -> Self {
        Self { inner: Self::BLOCK }
    }

    pub fn set_BLOCK(&mut self) -> Self {
        self.inner |= Self::BLOCK;
        *self
    }

    pub fn clear_BLOCK(&mut self) -> Self {
        self.inner &= Self::BLOCK.reverse_bits();
        *self
    }

    pub const fn is_UNK14(&self) -> bool {
        (self.inner & Self::UNK14) != 0
    }

    /// set only if meleespellid is present. no world text when victim is hit for 0 dmg(HideWorldTextForNoDamage?)
    ///
    pub const fn new_UNK14() -> Self {
        Self { inner: Self::UNK14 }
    }

    pub fn set_UNK14(&mut self) -> Self {
        self.inner |= Self::UNK14;
        *self
    }

    pub fn clear_UNK14(&mut self) -> Self {
        self.inner &= Self::UNK14.reverse_bits();
        *self
    }

    pub const fn is_UNK15(&self) -> bool {
        (self.inner & Self::UNK15) != 0
    }

    /// player victim? something related to blod sprut visual (BloodSpurtInBack?)
    ///
    pub const fn new_UNK15() -> Self {
        Self { inner: Self::UNK15 }
    }

    pub fn set_UNK15(&mut self) -> Self {
        self.inner |= Self::UNK15;
        *self
    }

    pub fn clear_UNK15(&mut self) -> Self {
        self.inner &= Self::UNK15.reverse_bits();
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

    pub const fn is_NO_ANIMATION(&self) -> bool {
        (self.inner & Self::NO_ANIMATION) != 0
    }

    pub const fn new_NO_ANIMATION() -> Self {
        Self { inner: Self::NO_ANIMATION }
    }

    pub fn set_NO_ANIMATION(&mut self) -> Self {
        self.inner |= Self::NO_ANIMATION;
        *self
    }

    pub fn clear_NO_ANIMATION(&mut self) -> Self {
        self.inner &= Self::NO_ANIMATION.reverse_bits();
        *self
    }

    pub const fn is_UNK19(&self) -> bool {
        (self.inner & Self::UNK19) != 0
    }

    pub const fn new_UNK19() -> Self {
        Self { inner: Self::UNK19 }
    }

    pub fn set_UNK19(&mut self) -> Self {
        self.inner |= Self::UNK19;
        *self
    }

    pub fn clear_UNK19(&mut self) -> Self {
        self.inner &= Self::UNK19.reverse_bits();
        *self
    }

    pub const fn is_UNK20(&self) -> bool {
        (self.inner & Self::UNK20) != 0
    }

    pub const fn new_UNK20() -> Self {
        Self { inner: Self::UNK20 }
    }

    pub fn set_UNK20(&mut self) -> Self {
        self.inner |= Self::UNK20;
        *self
    }

    pub fn clear_UNK20(&mut self) -> Self {
        self.inner &= Self::UNK20.reverse_bits();
        *self
    }

    pub const fn is_SWINGNOHITSOUND(&self) -> bool {
        (self.inner & Self::SWINGNOHITSOUND) != 0
    }

    /// unused?
    ///
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

    pub const fn is_UNK22(&self) -> bool {
        (self.inner & Self::UNK22) != 0
    }

    pub const fn new_UNK22() -> Self {
        Self { inner: Self::UNK22 }
    }

    pub fn set_UNK22(&mut self) -> Self {
        self.inner |= Self::UNK22;
        *self
    }

    pub fn clear_UNK22(&mut self) -> Self {
        self.inner &= Self::UNK22.reverse_bits();
        *self
    }

    pub const fn is_RAGE_GAIN(&self) -> bool {
        (self.inner & Self::RAGE_GAIN) != 0
    }

    pub const fn new_RAGE_GAIN() -> Self {
        Self { inner: Self::RAGE_GAIN }
    }

    pub fn set_RAGE_GAIN(&mut self) -> Self {
        self.inner |= Self::RAGE_GAIN;
        *self
    }

    pub fn clear_RAGE_GAIN(&mut self) -> Self {
        self.inner &= Self::RAGE_GAIN.reverse_bits();
        *self
    }

    pub const fn is_FAKE_DAMAGE(&self) -> bool {
        (self.inner & Self::FAKE_DAMAGE) != 0
    }

    /// enables damage animation even if no damage done. set only if no damage
    ///
    pub const fn new_FAKE_DAMAGE() -> Self {
        Self { inner: Self::FAKE_DAMAGE }
    }

    pub fn set_FAKE_DAMAGE(&mut self) -> Self {
        self.inner |= Self::FAKE_DAMAGE;
        *self
    }

    pub fn clear_FAKE_DAMAGE(&mut self) -> Self {
        self.inner &= Self::FAKE_DAMAGE.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

