/// Reply to [`CMD_AUTH_LOGON_PROOF_Client`](crate::logon::version_8::CMD_AUTH_LOGON_PROOF_Client).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm#L27):
/// ```text
/// flag AccountFlag : u32 {
///     GM = 0x000001;
///     TRIAL = 0x000008;
///     PROPASS = 0x800000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct AccountFlag {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl AccountFlag {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_gm() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "GM").unwrap();
            first = false;
        }
        if self.is_trial() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "TRIAL").unwrap();
            first = false;
        }
        if self.is_propass() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PROPASS").unwrap();
            first = false;
        }
        s
    }

}

impl AccountFlag {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const GM: u32 = 0x01;
    pub const TRIAL: u32 = 0x08;
    pub const PROPASS: u32 = 0x800000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::GM
                | Self::TRIAL
                | Self::PROPASS
        }
    }

    pub const fn is_gm(&self) -> bool {
        (self.inner & Self::GM) != 0
    }

    pub const fn new_gm() -> Self {
        Self { inner: Self::GM }
    }

    pub fn set_gm(&mut self) -> Self {
        self.inner |= Self::GM;
        *self
    }

    pub fn clear_gm(&mut self) -> Self {
        self.inner &= Self::GM.reverse_bits();
        *self
    }

    pub const fn is_trial(&self) -> bool {
        (self.inner & Self::TRIAL) != 0
    }

    pub const fn new_trial() -> Self {
        Self { inner: Self::TRIAL }
    }

    pub fn set_trial(&mut self) -> Self {
        self.inner |= Self::TRIAL;
        *self
    }

    pub fn clear_trial(&mut self) -> Self {
        self.inner &= Self::TRIAL.reverse_bits();
        *self
    }

    pub const fn is_propass(&self) -> bool {
        (self.inner & Self::PROPASS) != 0
    }

    pub const fn new_propass() -> Self {
        Self { inner: Self::PROPASS }
    }

    pub fn set_propass(&mut self) -> Self {
        self.inner |= Self::PROPASS;
        *self
    }

    pub fn clear_propass(&mut self) -> Self {
        self.inner &= Self::PROPASS.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for AccountFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AccountFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AccountFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AccountFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for AccountFlag {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for AccountFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for AccountFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for AccountFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for AccountFlag {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for AccountFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

