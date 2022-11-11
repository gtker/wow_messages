/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm:59`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm#L59):
/// ```text
/// flag GameobjectCastFlags : u32 {
///     LOCK_PLAYER_CAST_ANIM = 0x01;
///     UNKNOWN2 = 0x02;
///     UNKNOWN4 = 0x04;
///     UNKNOWN8 = 0x08;
///     UNKNOWN16 = 0x10;
///     AMMO = 0x20;
///     DEST_LOCATION = 0x040;
///     ITEM_CASTER = 0x100;
///     UNK200 = 0x200;
///     EXTRA_MESSAGE = 0x400;
///     POWER_UPDATE = 0x800;
///     UNK2000 = 0x2000;
///     UNK1000 = 0x1000;
///     UNK8000 = 0x8000;
///     ADJUST_MISSILE = 0x20000;
///     UNK40000 = 0x40000;
///     VISUAL_CHAIN = 0x80000;
///     RUNE_UPDATE = 0x200000;
///     UNK400000 = 0x400000;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct GameobjectCastFlags {
    inner: u32,
}

impl GameobjectCastFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const LOCK_PLAYER_CAST_ANIM: u32 = 0x01;
    pub(crate) const UNKNOWN2: u32 = 0x02;
    pub(crate) const UNKNOWN4: u32 = 0x04;
    pub(crate) const UNKNOWN8: u32 = 0x08;
    pub(crate) const UNKNOWN16: u32 = 0x10;
    pub(crate) const AMMO: u32 = 0x20;
    pub(crate) const DEST_LOCATION: u32 = 0x40;
    pub(crate) const ITEM_CASTER: u32 = 0x100;
    pub(crate) const UNK200: u32 = 0x200;
    pub(crate) const EXTRA_MESSAGE: u32 = 0x400;
    pub(crate) const POWER_UPDATE: u32 = 0x800;
    pub(crate) const UNK2000: u32 = 0x2000;
    pub(crate) const UNK1000: u32 = 0x1000;
    pub(crate) const UNK8000: u32 = 0x8000;
    pub(crate) const ADJUST_MISSILE: u32 = 0x20000;
    pub(crate) const UNK40000: u32 = 0x40000;
    pub(crate) const VISUAL_CHAIN: u32 = 0x80000;
    pub(crate) const RUNE_UPDATE: u32 = 0x200000;
    pub(crate) const UNK400000: u32 = 0x400000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::LOCK_PLAYER_CAST_ANIM
                | Self::UNKNOWN2
                | Self::UNKNOWN4
                | Self::UNKNOWN8
                | Self::UNKNOWN16
                | Self::AMMO
                | Self::DEST_LOCATION
                | Self::ITEM_CASTER
                | Self::UNK200
                | Self::EXTRA_MESSAGE
                | Self::POWER_UPDATE
                | Self::UNK2000
                | Self::UNK1000
                | Self::UNK8000
                | Self::ADJUST_MISSILE
                | Self::UNK40000
                | Self::VISUAL_CHAIN
                | Self::RUNE_UPDATE
                | Self::UNK400000
        }
    }

    pub const fn is_LOCK_PLAYER_CAST_ANIM(&self) -> bool {
        (self.inner & Self::LOCK_PLAYER_CAST_ANIM) != 0
    }

    /// also do not send standstate update
    ///
    pub const fn new_LOCK_PLAYER_CAST_ANIM() -> Self {
        Self { inner: Self::LOCK_PLAYER_CAST_ANIM }
    }

    pub fn set_LOCK_PLAYER_CAST_ANIM(&mut self) -> Self {
        self.inner |= Self::LOCK_PLAYER_CAST_ANIM;
        *self
    }

    pub fn clear_LOCK_PLAYER_CAST_ANIM(&mut self) -> Self {
        self.inner &= Self::LOCK_PLAYER_CAST_ANIM.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN2(&self) -> bool {
        (self.inner & Self::UNKNOWN2) != 0
    }

    pub const fn new_UNKNOWN2() -> Self {
        Self { inner: Self::UNKNOWN2 }
    }

    pub fn set_UNKNOWN2(&mut self) -> Self {
        self.inner |= Self::UNKNOWN2;
        *self
    }

    pub fn clear_UNKNOWN2(&mut self) -> Self {
        self.inner &= Self::UNKNOWN2.reverse_bits();
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

    pub const fn is_AMMO(&self) -> bool {
        (self.inner & Self::AMMO) != 0
    }

    /// 2 functions are called on 2 values
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

    pub const fn is_DEST_LOCATION(&self) -> bool {
        (self.inner & Self::DEST_LOCATION) != 0
    }

    pub const fn new_DEST_LOCATION() -> Self {
        Self { inner: Self::DEST_LOCATION }
    }

    pub fn set_DEST_LOCATION(&mut self) -> Self {
        self.inner |= Self::DEST_LOCATION;
        *self
    }

    pub fn clear_DEST_LOCATION(&mut self) -> Self {
        self.inner &= Self::DEST_LOCATION.reverse_bits();
        *self
    }

    pub const fn is_ITEM_CASTER(&self) -> bool {
        (self.inner & Self::ITEM_CASTER) != 0
    }

    pub const fn new_ITEM_CASTER() -> Self {
        Self { inner: Self::ITEM_CASTER }
    }

    pub fn set_ITEM_CASTER(&mut self) -> Self {
        self.inner |= Self::ITEM_CASTER;
        *self
    }

    pub fn clear_ITEM_CASTER(&mut self) -> Self {
        self.inner &= Self::ITEM_CASTER.reverse_bits();
        *self
    }

    pub const fn is_UNK200(&self) -> bool {
        (self.inner & Self::UNK200) != 0
    }

    pub const fn new_UNK200() -> Self {
        Self { inner: Self::UNK200 }
    }

    pub fn set_UNK200(&mut self) -> Self {
        self.inner |= Self::UNK200;
        *self
    }

    pub fn clear_UNK200(&mut self) -> Self {
        self.inner &= Self::UNK200.reverse_bits();
        *self
    }

    pub const fn is_EXTRA_MESSAGE(&self) -> bool {
        (self.inner & Self::EXTRA_MESSAGE) != 0
    }

    /// TARGET MISSES AND OTHER MESSAGES LIKE 'Resist'
    ///
    pub const fn new_EXTRA_MESSAGE() -> Self {
        Self { inner: Self::EXTRA_MESSAGE }
    }

    pub fn set_EXTRA_MESSAGE(&mut self) -> Self {
        self.inner |= Self::EXTRA_MESSAGE;
        *self
    }

    pub fn clear_EXTRA_MESSAGE(&mut self) -> Self {
        self.inner &= Self::EXTRA_MESSAGE.reverse_bits();
        *self
    }

    pub const fn is_POWER_UPDATE(&self) -> bool {
        (self.inner & Self::POWER_UPDATE) != 0
    }

    /// seems to work hand in hand with some visual effect of update actually
    ///
    pub const fn new_POWER_UPDATE() -> Self {
        Self { inner: Self::POWER_UPDATE }
    }

    pub fn set_POWER_UPDATE(&mut self) -> Self {
        self.inner |= Self::POWER_UPDATE;
        *self
    }

    pub fn clear_POWER_UPDATE(&mut self) -> Self {
        self.inner &= Self::POWER_UPDATE.reverse_bits();
        *self
    }

    pub const fn is_UNK2000(&self) -> bool {
        (self.inner & Self::UNK2000) != 0
    }

    pub const fn new_UNK2000() -> Self {
        Self { inner: Self::UNK2000 }
    }

    pub fn set_UNK2000(&mut self) -> Self {
        self.inner |= Self::UNK2000;
        *self
    }

    pub fn clear_UNK2000(&mut self) -> Self {
        self.inner &= Self::UNK2000.reverse_bits();
        *self
    }

    pub const fn is_UNK1000(&self) -> bool {
        (self.inner & Self::UNK1000) != 0
    }

    /// no idea
    ///
    pub const fn new_UNK1000() -> Self {
        Self { inner: Self::UNK1000 }
    }

    pub fn set_UNK1000(&mut self) -> Self {
        self.inner |= Self::UNK1000;
        *self
    }

    pub fn clear_UNK1000(&mut self) -> Self {
        self.inner &= Self::UNK1000.reverse_bits();
        *self
    }

    pub const fn is_UNK8000(&self) -> bool {
        (self.inner & Self::UNK8000) != 0
    }

    /// seems to make server send extra 2 bytes before UNK1 and after UNK20000
    ///
    pub const fn new_UNK8000() -> Self {
        Self { inner: Self::UNK8000 }
    }

    pub fn set_UNK8000(&mut self) -> Self {
        self.inner |= Self::UNK8000;
        *self
    }

    pub fn clear_UNK8000(&mut self) -> Self {
        self.inner &= Self::UNK8000.reverse_bits();
        *self
    }

    pub const fn is_ADJUST_MISSILE(&self) -> bool {
        (self.inner & Self::ADJUST_MISSILE) != 0
    }

    /// seems to make server send an uint32 after `m_targets.write`
    ///
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

    pub const fn is_UNK40000(&self) -> bool {
        (self.inner & Self::UNK40000) != 0
    }

    /// 1 uint32. this is not confirmed but i have a feeling about it :D
    ///
    pub const fn new_UNK40000() -> Self {
        Self { inner: Self::UNK40000 }
    }

    pub fn set_UNK40000(&mut self) -> Self {
        self.inner |= Self::UNK40000;
        *self
    }

    pub fn clear_UNK40000(&mut self) -> Self {
        self.inner &= Self::UNK40000.reverse_bits();
        *self
    }

    pub const fn is_VISUAL_CHAIN(&self) -> bool {
        (self.inner & Self::VISUAL_CHAIN) != 0
    }

    /// 2 functions called (same ones as for ranged but different)
    ///
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

    pub const fn is_RUNE_UPDATE(&self) -> bool {
        (self.inner & Self::RUNE_UPDATE) != 0
    }

    /// 2 bytes for the rune cur and rune next flags
    ///
    pub const fn new_RUNE_UPDATE() -> Self {
        Self { inner: Self::RUNE_UPDATE }
    }

    pub fn set_RUNE_UPDATE(&mut self) -> Self {
        self.inner |= Self::RUNE_UPDATE;
        *self
    }

    pub fn clear_RUNE_UPDATE(&mut self) -> Self {
        self.inner &= Self::RUNE_UPDATE.reverse_bits();
        *self
    }

    pub const fn is_UNK400000(&self) -> bool {
        (self.inner & Self::UNK400000) != 0
    }

    /// seems to make server send an uint32 after `m_targets.write`
    ///
    pub const fn new_UNK400000() -> Self {
        Self { inner: Self::UNK400000 }
    }

    pub fn set_UNK400000(&mut self) -> Self {
        self.inner |= Self::UNK400000;
        *self
    }

    pub fn clear_UNK400000(&mut self) -> Self {
        self.inner &= Self::UNK400000.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

