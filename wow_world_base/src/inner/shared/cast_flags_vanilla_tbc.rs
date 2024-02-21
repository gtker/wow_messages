/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:49`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L49):
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CastFlags {
    inner: u16,
}

#[cfg(feature = "print-testcase")]
impl CastFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_hidden_combatlog() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HIDDEN_COMBATLOG").unwrap();
            first = false;
        }
        if self.is_unknown2() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN2").unwrap();
            first = false;
        }
        if self.is_unknown3() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN3").unwrap();
            first = false;
        }
        if self.is_unknown4() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN4").unwrap();
            first = false;
        }
        if self.is_unknown5() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN5").unwrap();
            first = false;
        }
        if self.is_ammo() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "AMMO").unwrap();
            first = false;
        }
        if self.is_unknown7() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN7").unwrap();
            first = false;
        }
        if self.is_unknown8() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN8").unwrap();
            first = false;
        }
        if self.is_unknown9() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN9").unwrap();
            first = false;
        }
        s
    }

}

impl CastFlags {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

    pub const NONE: u16 = 0x00;
    pub const HIDDEN_COMBATLOG: u16 = 0x01;
    pub const UNKNOWN2: u16 = 0x02;
    pub const UNKNOWN3: u16 = 0x04;
    pub const UNKNOWN4: u16 = 0x08;
    pub const UNKNOWN5: u16 = 0x10;
    pub const AMMO: u16 = 0x20;
    pub const UNKNOWN7: u16 = 0x40;
    pub const UNKNOWN8: u16 = 0x80;
    pub const UNKNOWN9: u16 = 0x100;

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

    pub const fn is_hidden_combatlog(&self) -> bool {
        (self.inner & Self::HIDDEN_COMBATLOG) != 0
    }

    /// mangoszero/cmangos/vmangos: hide in combat log?
    pub const fn new_hidden_combatlog() -> Self {
        Self { inner: Self::HIDDEN_COMBATLOG }
    }

    pub fn set_hidden_combatlog(&mut self) -> Self {
        self.inner |= Self::HIDDEN_COMBATLOG;
        *self
    }

    pub fn clear_hidden_combatlog(&mut self) -> Self {
        self.inner &= Self::HIDDEN_COMBATLOG.reverse_bits();
        *self
    }

    pub const fn is_unknown2(&self) -> bool {
        (self.inner & Self::UNKNOWN2) != 0
    }

    pub const fn new_unknown2() -> Self {
        Self { inner: Self::UNKNOWN2 }
    }

    pub fn set_unknown2(&mut self) -> Self {
        self.inner |= Self::UNKNOWN2;
        *self
    }

    pub fn clear_unknown2(&mut self) -> Self {
        self.inner &= Self::UNKNOWN2.reverse_bits();
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

    pub const fn is_ammo(&self) -> bool {
        (self.inner & Self::AMMO) != 0
    }

    /// cmangos/vmangos/mangoszero: Projectiles visual
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

    pub const fn is_unknown7(&self) -> bool {
        (self.inner & Self::UNKNOWN7) != 0
    }

    /// cmangos/vmangos/mangoszero: !0x41 mask used to call `CGTradeSkillInfo::DoRecast`
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

    pub const fn is_unknown9(&self) -> bool {
        (self.inner & Self::UNKNOWN9) != 0
    }

    pub const fn new_unknown9() -> Self {
        Self { inner: Self::UNKNOWN9 }
    }

    pub fn set_unknown9(&mut self) -> Self {
        self.inner |= Self::UNKNOWN9;
        *self
    }

    pub fn clear_unknown9(&mut self) -> Self {
        self.inner &= Self::UNKNOWN9.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u16 {
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

impl From<u16> for CastFlags {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<u8> for CastFlags {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u32> for CastFlags {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for CastFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for CastFlags {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl From<i16> for CastFlags {
    fn from(value: i16) -> Self {
        Self::new(u16::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i32> for CastFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for CastFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for CastFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

