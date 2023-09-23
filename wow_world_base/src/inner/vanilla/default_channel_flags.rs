/// Used in `ChatChannels.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/channel_flags.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/channel_flags.wowm#L2):
/// ```text
/// flag DefaultChannelFlags : u32 {
///     NONE = 0x00;
///     INITIAL = 0x01;
///     ZONE_DEPENDENCY = 0x02;
///     GLOBAL = 0x04;
///     TRADE = 0x08;
///     CITY_ONLY = 0x10;
///     CITY_ONLY_2 = 0x20;
///     DEFENCE = 0x10000;
///     UNSELECTED = 0x40000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct DefaultChannelFlags {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl DefaultChannelFlags {
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
        if self.is_initial() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INITIAL").unwrap();
            first = false;
        }
        if self.is_zone_dependency() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ZONE_DEPENDENCY").unwrap();
            first = false;
        }
        if self.is_global() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "GLOBAL").unwrap();
            first = false;
        }
        if self.is_trade() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TRADE").unwrap();
            first = false;
        }
        if self.is_city_only() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CITY_ONLY").unwrap();
            first = false;
        }
        if self.is_city_only_2() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CITY_ONLY_2").unwrap();
            first = false;
        }
        if self.is_defence() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DEFENCE").unwrap();
            first = false;
        }
        if self.is_unselected() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNSELECTED").unwrap();
            first = false;
        }
        s
    }

}

impl DefaultChannelFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const INITIAL: u32 = 0x01;
    pub const ZONE_DEPENDENCY: u32 = 0x02;
    pub const GLOBAL: u32 = 0x04;
    pub const TRADE: u32 = 0x08;
    pub const CITY_ONLY: u32 = 0x10;
    pub const CITY_ONLY_2: u32 = 0x20;
    pub const DEFENCE: u32 = 0x10000;
    pub const UNSELECTED: u32 = 0x40000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::INITIAL
                | Self::ZONE_DEPENDENCY
                | Self::GLOBAL
                | Self::TRADE
                | Self::CITY_ONLY
                | Self::CITY_ONLY_2
                | Self::DEFENCE
                | Self::UNSELECTED
        }
    }

    pub const fn is_initial(&self) -> bool {
        (self.inner & Self::INITIAL) != 0
    }

    pub const fn new_initial() -> Self {
        Self { inner: Self::INITIAL }
    }

    pub fn set_initial(&mut self) -> Self {
        self.inner |= Self::INITIAL;
        *self
    }

    pub fn clear_initial(&mut self) -> Self {
        self.inner &= Self::INITIAL.reverse_bits();
        *self
    }

    pub const fn is_zone_dependency(&self) -> bool {
        (self.inner & Self::ZONE_DEPENDENCY) != 0
    }

    pub const fn new_zone_dependency() -> Self {
        Self { inner: Self::ZONE_DEPENDENCY }
    }

    pub fn set_zone_dependency(&mut self) -> Self {
        self.inner |= Self::ZONE_DEPENDENCY;
        *self
    }

    pub fn clear_zone_dependency(&mut self) -> Self {
        self.inner &= Self::ZONE_DEPENDENCY.reverse_bits();
        *self
    }

    pub const fn is_global(&self) -> bool {
        (self.inner & Self::GLOBAL) != 0
    }

    pub const fn new_global() -> Self {
        Self { inner: Self::GLOBAL }
    }

    pub fn set_global(&mut self) -> Self {
        self.inner |= Self::GLOBAL;
        *self
    }

    pub fn clear_global(&mut self) -> Self {
        self.inner &= Self::GLOBAL.reverse_bits();
        *self
    }

    pub const fn is_trade(&self) -> bool {
        (self.inner & Self::TRADE) != 0
    }

    pub const fn new_trade() -> Self {
        Self { inner: Self::TRADE }
    }

    pub fn set_trade(&mut self) -> Self {
        self.inner |= Self::TRADE;
        *self
    }

    pub fn clear_trade(&mut self) -> Self {
        self.inner &= Self::TRADE.reverse_bits();
        *self
    }

    pub const fn is_city_only(&self) -> bool {
        (self.inner & Self::CITY_ONLY) != 0
    }

    pub const fn new_city_only() -> Self {
        Self { inner: Self::CITY_ONLY }
    }

    pub fn set_city_only(&mut self) -> Self {
        self.inner |= Self::CITY_ONLY;
        *self
    }

    pub fn clear_city_only(&mut self) -> Self {
        self.inner &= Self::CITY_ONLY.reverse_bits();
        *self
    }

    pub const fn is_city_only_2(&self) -> bool {
        (self.inner & Self::CITY_ONLY_2) != 0
    }

    pub const fn new_city_only_2() -> Self {
        Self { inner: Self::CITY_ONLY_2 }
    }

    pub fn set_city_only_2(&mut self) -> Self {
        self.inner |= Self::CITY_ONLY_2;
        *self
    }

    pub fn clear_city_only_2(&mut self) -> Self {
        self.inner &= Self::CITY_ONLY_2.reverse_bits();
        *self
    }

    pub const fn is_defence(&self) -> bool {
        (self.inner & Self::DEFENCE) != 0
    }

    pub const fn new_defence() -> Self {
        Self { inner: Self::DEFENCE }
    }

    pub fn set_defence(&mut self) -> Self {
        self.inner |= Self::DEFENCE;
        *self
    }

    pub fn clear_defence(&mut self) -> Self {
        self.inner &= Self::DEFENCE.reverse_bits();
        *self
    }

    pub const fn is_unselected(&self) -> bool {
        (self.inner & Self::UNSELECTED) != 0
    }

    pub const fn new_unselected() -> Self {
        Self { inner: Self::UNSELECTED }
    }

    pub fn set_unselected(&mut self) -> Self {
        self.inner |= Self::UNSELECTED;
        *self
    }

    pub fn clear_unselected(&mut self) -> Self {
        self.inner &= Self::UNSELECTED.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for DefaultChannelFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for DefaultChannelFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for DefaultChannelFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for DefaultChannelFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for DefaultChannelFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for DefaultChannelFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for DefaultChannelFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for DefaultChannelFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for DefaultChannelFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for DefaultChannelFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for DefaultChannelFlags {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for DefaultChannelFlags {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for DefaultChannelFlags {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for DefaultChannelFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for DefaultChannelFlags {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for DefaultChannelFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for DefaultChannelFlags {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for DefaultChannelFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for DefaultChannelFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

