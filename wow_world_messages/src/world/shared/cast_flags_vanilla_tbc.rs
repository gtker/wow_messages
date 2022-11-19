/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:51`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L51):
/// ```text
/// flag CastFlags : u16 {
///     NONE = 0x00000000;
///     HIDDEN_COMBATLOG = 0x00000001;
///     UNKNOWN2 = 0x00000002;
///     UNKNOWN3 = 0x00000004;
///     UNKNOWN4 = 0x00000008;
///     UNKNOWN5 = 0x00000010;
///     AMMO = 0x00000020;
///     UNKNOWN7 = 0x00000040;
///     UNKNOWN8 = 0x00000080;
///     UNKNOWN9 = 0x00000100;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct CastFlags {
    inner: u16,
}

impl CastFlags {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u16 = 0x00;
    pub(crate) const HIDDEN_COMBATLOG: u16 = 0x01;
    pub(crate) const UNKNOWN2: u16 = 0x02;
    pub(crate) const UNKNOWN3: u16 = 0x04;
    pub(crate) const UNKNOWN4: u16 = 0x08;
    pub(crate) const UNKNOWN5: u16 = 0x10;
    pub(crate) const AMMO: u16 = 0x20;
    pub(crate) const UNKNOWN7: u16 = 0x40;
    pub(crate) const UNKNOWN8: u16 = 0x80;
    pub(crate) const UNKNOWN9: u16 = 0x100;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::HIDDEN_COMBATLOG
                | Self::UNKNOWN2
                | Self::UNKNOWN3
                | Self::UNKNOWN4
                | Self::UNKNOWN5
                | Self::AMMO
                | Self::UNKNOWN7
                | Self::UNKNOWN8
                | Self::UNKNOWN9
        }
    }

    pub const fn is_HIDDEN_COMBATLOG(&self) -> bool {
        (self.inner & Self::HIDDEN_COMBATLOG) != 0
    }

    /// mangoszero/cmangos/vmangos: hide in combat log?
    ///
    pub const fn new_HIDDEN_COMBATLOG() -> Self {
        Self { inner: Self::HIDDEN_COMBATLOG }
    }

    pub fn set_HIDDEN_COMBATLOG(&mut self) -> Self {
        self.inner |= Self::HIDDEN_COMBATLOG;
        *self
    }

    pub fn clear_HIDDEN_COMBATLOG(&mut self) -> Self {
        self.inner &= Self::HIDDEN_COMBATLOG.reverse_bits();
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

    pub const fn is_AMMO(&self) -> bool {
        (self.inner & Self::AMMO) != 0
    }

    /// cmangos/vmangos/mangoszero: Projectiles visual
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

    pub const fn is_UNKNOWN7(&self) -> bool {
        (self.inner & Self::UNKNOWN7) != 0
    }

    /// cmangos/vmangos/mangoszero: !0x41 mask used to call `CGTradeSkillInfo::DoRecast`
    ///
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

    pub const fn is_UNKNOWN9(&self) -> bool {
        (self.inner & Self::UNKNOWN9) != 0
    }

    pub const fn new_UNKNOWN9() -> Self {
        Self { inner: Self::UNKNOWN9 }
    }

    pub fn set_UNKNOWN9(&mut self) -> Self {
        self.inner |= Self::UNKNOWN9;
        *self
    }

    pub fn clear_UNKNOWN9(&mut self) -> Self {
        self.inner &= Self::UNKNOWN9.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u16 {
        self.inner
    }

}

