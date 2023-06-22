/// Used in `AreaTable.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/area_flags.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/area_flags.wowm#L1):
/// ```text
/// flag AreaFlags : u16 {
///     SNOW = 0x01;
///     UNK = 0x02;
///     DEVELOPMENT = 0x04;
///     UNK2 = 0x08;
///     UNK3 = 0x10;
///     CITY_SLAVE = 0x20;
///     CITY_ALLOW_DUELS = 0x40;
///     UNK4 = 0x80;
///     CITY = 0x100;
///     TEST = 0x200;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AreaFlags {
    inner: u16,
}

#[cfg(feature = "print-testcase")]
impl AreaFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_snow() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "SNOW").unwrap();
            first = false;
        }
        if self.is_unk() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNK").unwrap();
            first = false;
        }
        if self.is_development() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DEVELOPMENT").unwrap();
            first = false;
        }
        if self.is_unk2() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNK2").unwrap();
            first = false;
        }
        if self.is_unk3() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNK3").unwrap();
            first = false;
        }
        if self.is_city_slave() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CITY_SLAVE").unwrap();
            first = false;
        }
        if self.is_city_allow_duels() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CITY_ALLOW_DUELS").unwrap();
            first = false;
        }
        if self.is_unk4() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNK4").unwrap();
            first = false;
        }
        if self.is_city() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CITY").unwrap();
            first = false;
        }
        if self.is_test() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "TEST").unwrap();
            first = false;
        }
        s
    }

}

impl AreaFlags {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

    pub const SNOW: u16 = 0x01;
    pub const UNK: u16 = 0x02;
    pub const DEVELOPMENT: u16 = 0x04;
    pub const UNK2: u16 = 0x08;
    pub const UNK3: u16 = 0x10;
    pub const CITY_SLAVE: u16 = 0x20;
    pub const CITY_ALLOW_DUELS: u16 = 0x40;
    pub const UNK4: u16 = 0x80;
    pub const CITY: u16 = 0x100;
    pub const TEST: u16 = 0x200;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::SNOW
                | Self::UNK
                | Self::DEVELOPMENT
                | Self::UNK2
                | Self::UNK3
                | Self::CITY_SLAVE
                | Self::CITY_ALLOW_DUELS
                | Self::UNK4
                | Self::CITY
                | Self::TEST
        }
    }

    pub const fn is_snow(&self) -> bool {
        (self.inner & Self::SNOW) != 0
    }

    pub const fn new_snow() -> Self {
        Self { inner: Self::SNOW }
    }

    pub fn set_snow(&mut self) -> Self {
        self.inner |= Self::SNOW;
        *self
    }

    pub fn clear_snow(&mut self) -> Self {
        self.inner &= Self::SNOW.reverse_bits();
        *self
    }

    pub const fn is_unk(&self) -> bool {
        (self.inner & Self::UNK) != 0
    }

    pub const fn new_unk() -> Self {
        Self { inner: Self::UNK }
    }

    pub fn set_unk(&mut self) -> Self {
        self.inner |= Self::UNK;
        *self
    }

    pub fn clear_unk(&mut self) -> Self {
        self.inner &= Self::UNK.reverse_bits();
        *self
    }

    pub const fn is_development(&self) -> bool {
        (self.inner & Self::DEVELOPMENT) != 0
    }

    pub const fn new_development() -> Self {
        Self { inner: Self::DEVELOPMENT }
    }

    pub fn set_development(&mut self) -> Self {
        self.inner |= Self::DEVELOPMENT;
        *self
    }

    pub fn clear_development(&mut self) -> Self {
        self.inner &= Self::DEVELOPMENT.reverse_bits();
        *self
    }

    pub const fn is_unk2(&self) -> bool {
        (self.inner & Self::UNK2) != 0
    }

    pub const fn new_unk2() -> Self {
        Self { inner: Self::UNK2 }
    }

    pub fn set_unk2(&mut self) -> Self {
        self.inner |= Self::UNK2;
        *self
    }

    pub fn clear_unk2(&mut self) -> Self {
        self.inner &= Self::UNK2.reverse_bits();
        *self
    }

    pub const fn is_unk3(&self) -> bool {
        (self.inner & Self::UNK3) != 0
    }

    pub const fn new_unk3() -> Self {
        Self { inner: Self::UNK3 }
    }

    pub fn set_unk3(&mut self) -> Self {
        self.inner |= Self::UNK3;
        *self
    }

    pub fn clear_unk3(&mut self) -> Self {
        self.inner &= Self::UNK3.reverse_bits();
        *self
    }

    pub const fn is_city_slave(&self) -> bool {
        (self.inner & Self::CITY_SLAVE) != 0
    }

    pub const fn new_city_slave() -> Self {
        Self { inner: Self::CITY_SLAVE }
    }

    pub fn set_city_slave(&mut self) -> Self {
        self.inner |= Self::CITY_SLAVE;
        *self
    }

    pub fn clear_city_slave(&mut self) -> Self {
        self.inner &= Self::CITY_SLAVE.reverse_bits();
        *self
    }

    pub const fn is_city_allow_duels(&self) -> bool {
        (self.inner & Self::CITY_ALLOW_DUELS) != 0
    }

    pub const fn new_city_allow_duels() -> Self {
        Self { inner: Self::CITY_ALLOW_DUELS }
    }

    pub fn set_city_allow_duels(&mut self) -> Self {
        self.inner |= Self::CITY_ALLOW_DUELS;
        *self
    }

    pub fn clear_city_allow_duels(&mut self) -> Self {
        self.inner &= Self::CITY_ALLOW_DUELS.reverse_bits();
        *self
    }

    pub const fn is_unk4(&self) -> bool {
        (self.inner & Self::UNK4) != 0
    }

    pub const fn new_unk4() -> Self {
        Self { inner: Self::UNK4 }
    }

    pub fn set_unk4(&mut self) -> Self {
        self.inner |= Self::UNK4;
        *self
    }

    pub fn clear_unk4(&mut self) -> Self {
        self.inner &= Self::UNK4.reverse_bits();
        *self
    }

    pub const fn is_city(&self) -> bool {
        (self.inner & Self::CITY) != 0
    }

    pub const fn new_city() -> Self {
        Self { inner: Self::CITY }
    }

    pub fn set_city(&mut self) -> Self {
        self.inner |= Self::CITY;
        *self
    }

    pub fn clear_city(&mut self) -> Self {
        self.inner &= Self::CITY.reverse_bits();
        *self
    }

    pub const fn is_test(&self) -> bool {
        (self.inner & Self::TEST) != 0
    }

    pub const fn new_test() -> Self {
        Self { inner: Self::TEST }
    }

    pub fn set_test(&mut self) -> Self {
        self.inner |= Self::TEST;
        *self
    }

    pub fn clear_test(&mut self) -> Self {
        self.inner &= Self::TEST.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u16 {
        self.inner
    }

}

impl std::fmt::UpperHex for AreaFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AreaFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AreaFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AreaFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for AreaFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for AreaFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for AreaFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for AreaFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for AreaFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for AreaFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u16> for AreaFlags {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<u8> for AreaFlags {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u32> for AreaFlags {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for AreaFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for AreaFlags {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i16> for AreaFlags {
    fn from(value: i16) -> Self {
        Self::new(u16::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i32> for AreaFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for AreaFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for AreaFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

