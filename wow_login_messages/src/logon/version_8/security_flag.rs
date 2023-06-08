/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/common.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/common.wowm#L8):
/// ```text
/// flag SecurityFlag : u8 {
///     NONE = 0x00;
///     PIN = 0x01;
///     MATRIX_CARD = 0x02;
///     AUTHENTICATOR = 0x04;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct SecurityFlag {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl SecurityFlag {
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
        if self.is_pin() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PIN").unwrap();
            first = false;
        }
        if self.is_matrix_card() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MATRIX_CARD").unwrap();
            first = false;
        }
        if self.is_authenticator() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "AUTHENTICATOR").unwrap();
            first = false;
        }
        s
    }

}

impl SecurityFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const NONE: u8 = 0x00;
    pub const PIN: u8 = 0x01;
    pub const MATRIX_CARD: u8 = 0x02;
    pub const AUTHENTICATOR: u8 = 0x04;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::PIN
                | Self::MATRIX_CARD
                | Self::AUTHENTICATOR
        }
    }

    pub const fn is_pin(&self) -> bool {
        (self.inner & Self::PIN) != 0
    }

    pub const fn new_pin() -> Self {
        Self { inner: Self::PIN }
    }

    pub fn set_pin(&mut self) -> Self {
        self.inner |= Self::PIN;
        *self
    }

    pub fn clear_pin(&mut self) -> Self {
        self.inner &= Self::PIN.reverse_bits();
        *self
    }

    pub const fn is_matrix_card(&self) -> bool {
        (self.inner & Self::MATRIX_CARD) != 0
    }

    /// Matrix Card 2FA which requires a matrix card.
    /// `https://forum.xentax.com/viewtopic.php?f=13&p=186022`
    pub const fn new_matrix_card() -> Self {
        Self { inner: Self::MATRIX_CARD }
    }

    pub fn set_matrix_card(&mut self) -> Self {
        self.inner |= Self::MATRIX_CARD;
        *self
    }

    pub fn clear_matrix_card(&mut self) -> Self {
        self.inner &= Self::MATRIX_CARD.reverse_bits();
        *self
    }

    pub const fn is_authenticator(&self) -> bool {
        (self.inner & Self::AUTHENTICATOR) != 0
    }

    pub const fn new_authenticator() -> Self {
        Self { inner: Self::AUTHENTICATOR }
    }

    pub fn set_authenticator(&mut self) -> Self {
        self.inner |= Self::AUTHENTICATOR;
        *self
    }

    pub fn clear_authenticator(&mut self) -> Self {
        self.inner &= Self::AUTHENTICATOR.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for SecurityFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for SecurityFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for SecurityFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for SecurityFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for SecurityFlag {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for SecurityFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for SecurityFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for SecurityFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for SecurityFlag {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for SecurityFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

