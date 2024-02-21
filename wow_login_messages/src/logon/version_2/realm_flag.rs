/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L11):
/// ```text
/// flag RealmFlag : u8 {
///     NONE = 0x00;
///     INVALID = 0x01;
///     OFFLINE = 0x02;
///     FORCE_BLUE_RECOMMENDED = 0x20;
///     FORCE_GREEN_RECOMMENDED = 0x40;
///     FORCE_RED_FULL = 0x80;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct RealmFlag {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl RealmFlag {
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
        if self.is_invalid() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INVALID").unwrap();
            first = false;
        }
        if self.is_offline() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "OFFLINE").unwrap();
            first = false;
        }
        if self.is_force_blue_recommended() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FORCE_BLUE_RECOMMENDED").unwrap();
            first = false;
        }
        if self.is_force_green_recommended() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FORCE_GREEN_RECOMMENDED").unwrap();
            first = false;
        }
        if self.is_force_red_full() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FORCE_RED_FULL").unwrap();
            first = false;
        }
        s
    }

}

impl RealmFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const NONE: u8 = 0x00;
    pub const INVALID: u8 = 0x01;
    pub const OFFLINE: u8 = 0x02;
    pub const FORCE_BLUE_RECOMMENDED: u8 = 0x20;
    pub const FORCE_GREEN_RECOMMENDED: u8 = 0x40;
    pub const FORCE_RED_FULL: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::INVALID
                | Self::OFFLINE
                | Self::FORCE_BLUE_RECOMMENDED
                | Self::FORCE_GREEN_RECOMMENDED
                | Self::FORCE_RED_FULL
        }
    }

    pub const fn is_invalid(&self) -> bool {
        (self.inner & Self::INVALID) != 0
    }

    pub const fn new_invalid() -> Self {
        Self { inner: Self::INVALID }
    }

    pub fn set_invalid(&mut self) -> Self {
        self.inner |= Self::INVALID;
        *self
    }

    pub fn clear_invalid(&mut self) -> Self {
        self.inner &= Self::INVALID.reverse_bits();
        *self
    }

    pub const fn is_offline(&self) -> bool {
        (self.inner & Self::OFFLINE) != 0
    }

    pub const fn new_offline() -> Self {
        Self { inner: Self::OFFLINE }
    }

    pub fn set_offline(&mut self) -> Self {
        self.inner |= Self::OFFLINE;
        *self
    }

    pub fn clear_offline(&mut self) -> Self {
        self.inner &= Self::OFFLINE.reverse_bits();
        *self
    }

    pub const fn is_force_blue_recommended(&self) -> bool {
        (self.inner & Self::FORCE_BLUE_RECOMMENDED) != 0
    }

    pub const fn new_force_blue_recommended() -> Self {
        Self { inner: Self::FORCE_BLUE_RECOMMENDED }
    }

    pub fn set_force_blue_recommended(&mut self) -> Self {
        self.inner |= Self::FORCE_BLUE_RECOMMENDED;
        *self
    }

    pub fn clear_force_blue_recommended(&mut self) -> Self {
        self.inner &= Self::FORCE_BLUE_RECOMMENDED.reverse_bits();
        *self
    }

    pub const fn is_force_green_recommended(&self) -> bool {
        (self.inner & Self::FORCE_GREEN_RECOMMENDED) != 0
    }

    pub const fn new_force_green_recommended() -> Self {
        Self { inner: Self::FORCE_GREEN_RECOMMENDED }
    }

    pub fn set_force_green_recommended(&mut self) -> Self {
        self.inner |= Self::FORCE_GREEN_RECOMMENDED;
        *self
    }

    pub fn clear_force_green_recommended(&mut self) -> Self {
        self.inner &= Self::FORCE_GREEN_RECOMMENDED.reverse_bits();
        *self
    }

    pub const fn is_force_red_full(&self) -> bool {
        (self.inner & Self::FORCE_RED_FULL) != 0
    }

    pub const fn new_force_red_full() -> Self {
        Self { inner: Self::FORCE_RED_FULL }
    }

    pub fn set_force_red_full(&mut self) -> Self {
        self.inner |= Self::FORCE_RED_FULL;
        *self
    }

    pub fn clear_force_red_full(&mut self) -> Self {
        self.inner &= Self::FORCE_RED_FULL.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for RealmFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for RealmFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for RealmFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for RealmFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for RealmFlag {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for RealmFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for RealmFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for RealmFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for RealmFlag {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for RealmFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u8> for RealmFlag {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl TryFrom<u16> for RealmFlag {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u32> for RealmFlag {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for RealmFlag {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i8> for RealmFlag {
    fn from(value: i8) -> Self {
        Self::new(u8::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i16> for RealmFlag {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i32> for RealmFlag {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for RealmFlag {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for RealmFlag {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

