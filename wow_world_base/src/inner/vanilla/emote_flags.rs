/// Used in `Emotes.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/emote_flags.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/emote_flags.wowm#L2):
/// ```text
/// flag EmoteFlags : u8 {
///     TALK = 0x08;
///     QUESTION = 0x10;
///     EXCLAMATION = 0x20;
///     SHOUT = 0x40;
///     LAUGH = 0x80;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct EmoteFlags {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl EmoteFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_talk() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TALK").unwrap();
            first = false;
        }
        if self.is_question() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "QUESTION").unwrap();
            first = false;
        }
        if self.is_exclamation() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "EXCLAMATION").unwrap();
            first = false;
        }
        if self.is_shout() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SHOUT").unwrap();
            first = false;
        }
        if self.is_laugh() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "LAUGH").unwrap();
            first = false;
        }
        s
    }

}

impl EmoteFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const TALK: u8 = 0x08;
    pub const QUESTION: u8 = 0x10;
    pub const EXCLAMATION: u8 = 0x20;
    pub const SHOUT: u8 = 0x40;
    pub const LAUGH: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::TALK
                | Self::QUESTION
                | Self::EXCLAMATION
                | Self::SHOUT
                | Self::LAUGH
        }
    }

    pub const fn is_talk(&self) -> bool {
        (self.inner & Self::TALK) != 0
    }

    pub const fn new_talk() -> Self {
        Self { inner: Self::TALK }
    }

    pub fn set_talk(&mut self) -> Self {
        self.inner |= Self::TALK;
        *self
    }

    pub fn clear_talk(&mut self) -> Self {
        self.inner &= Self::TALK.reverse_bits();
        *self
    }

    pub const fn is_question(&self) -> bool {
        (self.inner & Self::QUESTION) != 0
    }

    pub const fn new_question() -> Self {
        Self { inner: Self::QUESTION }
    }

    pub fn set_question(&mut self) -> Self {
        self.inner |= Self::QUESTION;
        *self
    }

    pub fn clear_question(&mut self) -> Self {
        self.inner &= Self::QUESTION.reverse_bits();
        *self
    }

    pub const fn is_exclamation(&self) -> bool {
        (self.inner & Self::EXCLAMATION) != 0
    }

    pub const fn new_exclamation() -> Self {
        Self { inner: Self::EXCLAMATION }
    }

    pub fn set_exclamation(&mut self) -> Self {
        self.inner |= Self::EXCLAMATION;
        *self
    }

    pub fn clear_exclamation(&mut self) -> Self {
        self.inner &= Self::EXCLAMATION.reverse_bits();
        *self
    }

    pub const fn is_shout(&self) -> bool {
        (self.inner & Self::SHOUT) != 0
    }

    pub const fn new_shout() -> Self {
        Self { inner: Self::SHOUT }
    }

    pub fn set_shout(&mut self) -> Self {
        self.inner |= Self::SHOUT;
        *self
    }

    pub fn clear_shout(&mut self) -> Self {
        self.inner &= Self::SHOUT.reverse_bits();
        *self
    }

    pub const fn is_laugh(&self) -> bool {
        (self.inner & Self::LAUGH) != 0
    }

    pub const fn new_laugh() -> Self {
        Self { inner: Self::LAUGH }
    }

    pub fn set_laugh(&mut self) -> Self {
        self.inner |= Self::LAUGH;
        *self
    }

    pub fn clear_laugh(&mut self) -> Self {
        self.inner &= Self::LAUGH.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for EmoteFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for EmoteFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for EmoteFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for EmoteFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for EmoteFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for EmoteFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for EmoteFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for EmoteFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for EmoteFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for EmoteFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u8> for EmoteFlags {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl TryFrom<u16> for EmoteFlags {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u32> for EmoteFlags {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for EmoteFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i8> for EmoteFlags {
    fn from(value: i8) -> Self {
        Self::new(u8::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i16> for EmoteFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i32> for EmoteFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for EmoteFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for EmoteFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

