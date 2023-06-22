/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_list.wowm#L3):
/// ```text
/// flag ChannelFlags : u8 {
///     NONE = 0x00;
///     CUSTOM = 0x01;
///     TRADE = 0x04;
///     NOT_LFG = 0x08;
///     GENERAL = 0x10;
///     CITY = 0x20;
///     LFG = 0x40;
///     VOICE = 0x80;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ChannelFlags {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl ChannelFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_custom() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CUSTOM").unwrap();
            first = false;
        }
        if self.is_trade() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "TRADE").unwrap();
            first = false;
        }
        if self.is_not_lfg() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NOT_LFG").unwrap();
            first = false;
        }
        if self.is_general() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "GENERAL").unwrap();
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
        if self.is_lfg() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "LFG").unwrap();
            first = false;
        }
        if self.is_voice() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "VOICE").unwrap();
            first = false;
        }
        s
    }

}

impl ChannelFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const NONE: u8 = 0x00;
    pub const CUSTOM: u8 = 0x01;
    pub const TRADE: u8 = 0x04;
    pub const NOT_LFG: u8 = 0x08;
    pub const GENERAL: u8 = 0x10;
    pub const CITY: u8 = 0x20;
    pub const LFG: u8 = 0x40;
    pub const VOICE: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::CUSTOM
                | Self::TRADE
                | Self::NOT_LFG
                | Self::GENERAL
                | Self::CITY
                | Self::LFG
                | Self::VOICE
        }
    }

    pub const fn is_custom(&self) -> bool {
        (self.inner & Self::CUSTOM) != 0
    }

    pub const fn new_custom() -> Self {
        Self { inner: Self::CUSTOM }
    }

    pub fn set_custom(&mut self) -> Self {
        self.inner |= Self::CUSTOM;
        *self
    }

    pub fn clear_custom(&mut self) -> Self {
        self.inner &= Self::CUSTOM.reverse_bits();
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

    pub const fn is_not_lfg(&self) -> bool {
        (self.inner & Self::NOT_LFG) != 0
    }

    pub const fn new_not_lfg() -> Self {
        Self { inner: Self::NOT_LFG }
    }

    pub fn set_not_lfg(&mut self) -> Self {
        self.inner |= Self::NOT_LFG;
        *self
    }

    pub fn clear_not_lfg(&mut self) -> Self {
        self.inner &= Self::NOT_LFG.reverse_bits();
        *self
    }

    pub const fn is_general(&self) -> bool {
        (self.inner & Self::GENERAL) != 0
    }

    pub const fn new_general() -> Self {
        Self { inner: Self::GENERAL }
    }

    pub fn set_general(&mut self) -> Self {
        self.inner |= Self::GENERAL;
        *self
    }

    pub fn clear_general(&mut self) -> Self {
        self.inner &= Self::GENERAL.reverse_bits();
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

    pub const fn is_lfg(&self) -> bool {
        (self.inner & Self::LFG) != 0
    }

    pub const fn new_lfg() -> Self {
        Self { inner: Self::LFG }
    }

    pub fn set_lfg(&mut self) -> Self {
        self.inner |= Self::LFG;
        *self
    }

    pub fn clear_lfg(&mut self) -> Self {
        self.inner &= Self::LFG.reverse_bits();
        *self
    }

    pub const fn is_voice(&self) -> bool {
        (self.inner & Self::VOICE) != 0
    }

    pub const fn new_voice() -> Self {
        Self { inner: Self::VOICE }
    }

    pub fn set_voice(&mut self) -> Self {
        self.inner |= Self::VOICE;
        *self
    }

    pub fn clear_voice(&mut self) -> Self {
        self.inner &= Self::VOICE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for ChannelFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for ChannelFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for ChannelFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for ChannelFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for ChannelFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for ChannelFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for ChannelFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for ChannelFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for ChannelFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for ChannelFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u8> for ChannelFlags {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl TryFrom<u16> for ChannelFlags {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u32> for ChannelFlags {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for ChannelFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i8> for ChannelFlags {
    fn from(value: i8) -> Self {
        Self::new(u8::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i16> for ChannelFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i32> for ChannelFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for ChannelFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for ChannelFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

