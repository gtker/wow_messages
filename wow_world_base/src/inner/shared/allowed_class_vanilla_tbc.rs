/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/allowed_races.wowm:72`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/allowed_races.wowm#L72):
/// ```text
/// flag AllowedClass : u32 {
///     ALL = 0;
///     WARRIOR = 1;
///     PALADIN = 2;
///     HUNTER = 4;
///     ROGUE = 8;
///     PRIEST = 16;
///     SHAMAN = 64;
///     MAGE = 128;
///     WARLOCK = 256;
///     DRUID = 1024;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AllowedClass {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl AllowedClass {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALL").unwrap();
            first = false;
        }
        if self.is_warrior() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "WARRIOR").unwrap();
            first = false;
        }
        if self.is_paladin() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PALADIN").unwrap();
            first = false;
        }
        if self.is_hunter() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HUNTER").unwrap();
            first = false;
        }
        if self.is_rogue() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ROGUE").unwrap();
            first = false;
        }
        if self.is_priest() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PRIEST").unwrap();
            first = false;
        }
        if self.is_shaman() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "SHAMAN").unwrap();
            first = false;
        }
        if self.is_mage() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MAGE").unwrap();
            first = false;
        }
        if self.is_warlock() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "WARLOCK").unwrap();
            first = false;
        }
        if self.is_druid() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DRUID").unwrap();
            first = false;
        }
        s
    }

}

impl AllowedClass {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const ALL: u32 = 0x00;
    pub const WARRIOR: u32 = 0x01;
    pub const PALADIN: u32 = 0x02;
    pub const HUNTER: u32 = 0x04;
    pub const ROGUE: u32 = 0x08;
    pub const PRIEST: u32 = 0x10;
    pub const SHAMAN: u32 = 0x40;
    pub const MAGE: u32 = 0x80;
    pub const WARLOCK: u32 = 0x100;
    pub const DRUID: u32 = 0x400;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::ALL
                | Self::WARRIOR
                | Self::PALADIN
                | Self::HUNTER
                | Self::ROGUE
                | Self::PRIEST
                | Self::SHAMAN
                | Self::MAGE
                | Self::WARLOCK
                | Self::DRUID
        }
    }

    pub const fn is_warrior(&self) -> bool {
        ((self.inner & Self::WARRIOR) != 0) || self.inner == 0
    }

    pub const fn new_warrior() -> Self {
        Self { inner: Self::WARRIOR }
    }

    pub fn set_warrior(&mut self) -> Self {
        self.inner |= Self::WARRIOR;
        *self
    }

    pub fn clear_warrior(&mut self) -> Self {
        self.inner &= Self::WARRIOR.reverse_bits();
        *self
    }

    pub const fn is_paladin(&self) -> bool {
        ((self.inner & Self::PALADIN) != 0) || self.inner == 0
    }

    pub const fn new_paladin() -> Self {
        Self { inner: Self::PALADIN }
    }

    pub fn set_paladin(&mut self) -> Self {
        self.inner |= Self::PALADIN;
        *self
    }

    pub fn clear_paladin(&mut self) -> Self {
        self.inner &= Self::PALADIN.reverse_bits();
        *self
    }

    pub const fn is_hunter(&self) -> bool {
        ((self.inner & Self::HUNTER) != 0) || self.inner == 0
    }

    pub const fn new_hunter() -> Self {
        Self { inner: Self::HUNTER }
    }

    pub fn set_hunter(&mut self) -> Self {
        self.inner |= Self::HUNTER;
        *self
    }

    pub fn clear_hunter(&mut self) -> Self {
        self.inner &= Self::HUNTER.reverse_bits();
        *self
    }

    pub const fn is_rogue(&self) -> bool {
        ((self.inner & Self::ROGUE) != 0) || self.inner == 0
    }

    pub const fn new_rogue() -> Self {
        Self { inner: Self::ROGUE }
    }

    pub fn set_rogue(&mut self) -> Self {
        self.inner |= Self::ROGUE;
        *self
    }

    pub fn clear_rogue(&mut self) -> Self {
        self.inner &= Self::ROGUE.reverse_bits();
        *self
    }

    pub const fn is_priest(&self) -> bool {
        ((self.inner & Self::PRIEST) != 0) || self.inner == 0
    }

    pub const fn new_priest() -> Self {
        Self { inner: Self::PRIEST }
    }

    pub fn set_priest(&mut self) -> Self {
        self.inner |= Self::PRIEST;
        *self
    }

    pub fn clear_priest(&mut self) -> Self {
        self.inner &= Self::PRIEST.reverse_bits();
        *self
    }

    pub const fn is_shaman(&self) -> bool {
        ((self.inner & Self::SHAMAN) != 0) || self.inner == 0
    }

    pub const fn new_shaman() -> Self {
        Self { inner: Self::SHAMAN }
    }

    pub fn set_shaman(&mut self) -> Self {
        self.inner |= Self::SHAMAN;
        *self
    }

    pub fn clear_shaman(&mut self) -> Self {
        self.inner &= Self::SHAMAN.reverse_bits();
        *self
    }

    pub const fn is_mage(&self) -> bool {
        ((self.inner & Self::MAGE) != 0) || self.inner == 0
    }

    pub const fn new_mage() -> Self {
        Self { inner: Self::MAGE }
    }

    pub fn set_mage(&mut self) -> Self {
        self.inner |= Self::MAGE;
        *self
    }

    pub fn clear_mage(&mut self) -> Self {
        self.inner &= Self::MAGE.reverse_bits();
        *self
    }

    pub const fn is_warlock(&self) -> bool {
        ((self.inner & Self::WARLOCK) != 0) || self.inner == 0
    }

    pub const fn new_warlock() -> Self {
        Self { inner: Self::WARLOCK }
    }

    pub fn set_warlock(&mut self) -> Self {
        self.inner |= Self::WARLOCK;
        *self
    }

    pub fn clear_warlock(&mut self) -> Self {
        self.inner &= Self::WARLOCK.reverse_bits();
        *self
    }

    pub const fn is_druid(&self) -> bool {
        ((self.inner & Self::DRUID) != 0) || self.inner == 0
    }

    pub const fn new_druid() -> Self {
        Self { inner: Self::DRUID }
    }

    pub fn set_druid(&mut self) -> Self {
        self.inner |= Self::DRUID;
        *self
    }

    pub fn clear_druid(&mut self) -> Self {
        self.inner &= Self::DRUID.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for AllowedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AllowedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AllowedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AllowedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for AllowedClass {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for AllowedClass {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for AllowedClass {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for AllowedClass {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for AllowedClass {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for AllowedClass {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for AllowedClass {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for AllowedClass {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for AllowedClass {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for AllowedClass {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for AllowedClass {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for AllowedClass {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for AllowedClass {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for AllowedClass {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for AllowedClass {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

