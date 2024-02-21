/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm:49`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm#L49):
/// ```text
/// flag VictimState : u8 {
///     INTACT = 0;
///     HIT = 1;
///     DODGE = 2;
///     PARRY = 3;
///     INTERRUPT = 4;
///     BLOCKS = 5;
///     EVADES = 6;
///     IS_IMMUNE = 7;
///     DEFLECTS = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct VictimState {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl VictimState {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INTACT").unwrap();
            first = false;
        }
        if self.is_hit() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HIT").unwrap();
            first = false;
        }
        if self.is_dodge() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DODGE").unwrap();
            first = false;
        }
        if self.is_parry() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PARRY").unwrap();
            first = false;
        }
        if self.is_interrupt() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INTERRUPT").unwrap();
            first = false;
        }
        if self.is_blocks() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "BLOCKS").unwrap();
            first = false;
        }
        if self.is_evades() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "EVADES").unwrap();
            first = false;
        }
        if self.is_is_immune() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "IS_IMMUNE").unwrap();
            first = false;
        }
        if self.is_deflects() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DEFLECTS").unwrap();
            first = false;
        }
        s
    }

}

impl VictimState {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const INTACT: u8 = 0x00;
    pub const HIT: u8 = 0x01;
    pub const DODGE: u8 = 0x02;
    pub const PARRY: u8 = 0x03;
    pub const INTERRUPT: u8 = 0x04;
    pub const BLOCKS: u8 = 0x05;
    pub const EVADES: u8 = 0x06;
    pub const IS_IMMUNE: u8 = 0x07;
    pub const DEFLECTS: u8 = 0x08;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::INTACT
                | Self::HIT
                | Self::DODGE
                | Self::PARRY
                | Self::INTERRUPT
                | Self::BLOCKS
                | Self::EVADES
                | Self::IS_IMMUNE
                | Self::DEFLECTS
        }
    }

    pub const fn is_hit(&self) -> bool {
        (self.inner & Self::HIT) != 0
    }

    /// azerothcore: victim got clear/blocked hit
    pub const fn new_hit() -> Self {
        Self { inner: Self::HIT }
    }

    pub fn set_hit(&mut self) -> Self {
        self.inner |= Self::HIT;
        *self
    }

    pub fn clear_hit(&mut self) -> Self {
        self.inner &= Self::HIT.reverse_bits();
        *self
    }

    pub const fn is_dodge(&self) -> bool {
        (self.inner & Self::DODGE) != 0
    }

    pub const fn new_dodge() -> Self {
        Self { inner: Self::DODGE }
    }

    pub fn set_dodge(&mut self) -> Self {
        self.inner |= Self::DODGE;
        *self
    }

    pub fn clear_dodge(&mut self) -> Self {
        self.inner &= Self::DODGE.reverse_bits();
        *self
    }

    pub const fn is_parry(&self) -> bool {
        (self.inner & Self::PARRY) != 0
    }

    pub const fn new_parry() -> Self {
        Self { inner: Self::PARRY }
    }

    pub fn set_parry(&mut self) -> Self {
        self.inner |= Self::PARRY;
        *self
    }

    pub fn clear_parry(&mut self) -> Self {
        self.inner &= Self::PARRY.reverse_bits();
        *self
    }

    pub const fn is_interrupt(&self) -> bool {
        (self.inner & Self::INTERRUPT) != 0
    }

    pub const fn new_interrupt() -> Self {
        Self { inner: Self::INTERRUPT }
    }

    pub fn set_interrupt(&mut self) -> Self {
        self.inner |= Self::INTERRUPT;
        *self
    }

    pub fn clear_interrupt(&mut self) -> Self {
        self.inner &= Self::INTERRUPT.reverse_bits();
        *self
    }

    pub const fn is_blocks(&self) -> bool {
        (self.inner & Self::BLOCKS) != 0
    }

    /// azerothcore: unused? not set when blocked, even on full block
    pub const fn new_blocks() -> Self {
        Self { inner: Self::BLOCKS }
    }

    pub fn set_blocks(&mut self) -> Self {
        self.inner |= Self::BLOCKS;
        *self
    }

    pub fn clear_blocks(&mut self) -> Self {
        self.inner &= Self::BLOCKS.reverse_bits();
        *self
    }

    pub const fn is_evades(&self) -> bool {
        (self.inner & Self::EVADES) != 0
    }

    pub const fn new_evades() -> Self {
        Self { inner: Self::EVADES }
    }

    pub fn set_evades(&mut self) -> Self {
        self.inner |= Self::EVADES;
        *self
    }

    pub fn clear_evades(&mut self) -> Self {
        self.inner &= Self::EVADES.reverse_bits();
        *self
    }

    pub const fn is_is_immune(&self) -> bool {
        (self.inner & Self::IS_IMMUNE) != 0
    }

    pub const fn new_is_immune() -> Self {
        Self { inner: Self::IS_IMMUNE }
    }

    pub fn set_is_immune(&mut self) -> Self {
        self.inner |= Self::IS_IMMUNE;
        *self
    }

    pub fn clear_is_immune(&mut self) -> Self {
        self.inner &= Self::IS_IMMUNE.reverse_bits();
        *self
    }

    pub const fn is_deflects(&self) -> bool {
        (self.inner & Self::DEFLECTS) != 0
    }

    pub const fn new_deflects() -> Self {
        Self { inner: Self::DEFLECTS }
    }

    pub fn set_deflects(&mut self) -> Self {
        self.inner |= Self::DEFLECTS;
        *self
    }

    pub fn clear_deflects(&mut self) -> Self {
        self.inner &= Self::DEFLECTS.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for VictimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for VictimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for VictimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for VictimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for VictimState {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for VictimState {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for VictimState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for VictimState {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for VictimState {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for VictimState {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u8> for VictimState {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl TryFrom<u16> for VictimState {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u32> for VictimState {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for VictimState {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i8> for VictimState {
    fn from(value: i8) -> Self {
        Self::new(u8::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i16> for VictimState {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i32> for VictimState {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for VictimState {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for VictimState {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

