/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm:66`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm#L66):
/// ```text
/// flag BillingPlanFlags : u8 {
///     NONE = 0x00;
///     UNUSED = 0x01;
///     RECURRING_BILL = 0x02;
///     FREE_TRIAL = 0x04;
///     IGR = 0x08;
///     USAGE = 0x10;
///     TIME_MIXTURE = 0x20;
///     RESTRICTED = 0x40;
///     ENABLE_CAIS = 0x80;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BillingPlanFlags {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl BillingPlanFlags {
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
        if self.is_unused() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNUSED").unwrap();
            first = false;
        }
        if self.is_recurring_bill() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "RECURRING_BILL").unwrap();
            first = false;
        }
        if self.is_free_trial() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FREE_TRIAL").unwrap();
            first = false;
        }
        if self.is_igr() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IGR").unwrap();
            first = false;
        }
        if self.is_usage() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "USAGE").unwrap();
            first = false;
        }
        if self.is_time_mixture() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "TIME_MIXTURE").unwrap();
            first = false;
        }
        if self.is_restricted() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "RESTRICTED").unwrap();
            first = false;
        }
        if self.is_enable_cais() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ENABLE_CAIS").unwrap();
            first = false;
        }
        s
    }

}

impl BillingPlanFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const NONE: u8 = 0x00;
    pub const UNUSED: u8 = 0x01;
    pub const RECURRING_BILL: u8 = 0x02;
    pub const FREE_TRIAL: u8 = 0x04;
    pub const IGR: u8 = 0x08;
    pub const USAGE: u8 = 0x10;
    pub const TIME_MIXTURE: u8 = 0x20;
    pub const RESTRICTED: u8 = 0x40;
    pub const ENABLE_CAIS: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::UNUSED
                | Self::RECURRING_BILL
                | Self::FREE_TRIAL
                | Self::IGR
                | Self::USAGE
                | Self::TIME_MIXTURE
                | Self::RESTRICTED
                | Self::ENABLE_CAIS
        }
    }

    pub const fn is_unused(&self) -> bool {
        (self.inner & Self::UNUSED) != 0
    }

    pub const fn new_unused() -> Self {
        Self { inner: Self::UNUSED }
    }

    pub fn set_unused(&mut self) -> Self {
        self.inner |= Self::UNUSED;
        *self
    }

    pub fn clear_unused(&mut self) -> Self {
        self.inner &= Self::UNUSED.reverse_bits();
        *self
    }

    pub const fn is_recurring_bill(&self) -> bool {
        (self.inner & Self::RECURRING_BILL) != 0
    }

    pub const fn new_recurring_bill() -> Self {
        Self { inner: Self::RECURRING_BILL }
    }

    pub fn set_recurring_bill(&mut self) -> Self {
        self.inner |= Self::RECURRING_BILL;
        *self
    }

    pub fn clear_recurring_bill(&mut self) -> Self {
        self.inner &= Self::RECURRING_BILL.reverse_bits();
        *self
    }

    pub const fn is_free_trial(&self) -> bool {
        (self.inner & Self::FREE_TRIAL) != 0
    }

    pub const fn new_free_trial() -> Self {
        Self { inner: Self::FREE_TRIAL }
    }

    pub fn set_free_trial(&mut self) -> Self {
        self.inner |= Self::FREE_TRIAL;
        *self
    }

    pub fn clear_free_trial(&mut self) -> Self {
        self.inner &= Self::FREE_TRIAL.reverse_bits();
        *self
    }

    pub const fn is_igr(&self) -> bool {
        (self.inner & Self::IGR) != 0
    }

    /// Name meaning unknown
    pub const fn new_igr() -> Self {
        Self { inner: Self::IGR }
    }

    pub fn set_igr(&mut self) -> Self {
        self.inner |= Self::IGR;
        *self
    }

    pub fn clear_igr(&mut self) -> Self {
        self.inner &= Self::IGR.reverse_bits();
        *self
    }

    pub const fn is_usage(&self) -> bool {
        (self.inner & Self::USAGE) != 0
    }

    pub const fn new_usage() -> Self {
        Self { inner: Self::USAGE }
    }

    pub fn set_usage(&mut self) -> Self {
        self.inner |= Self::USAGE;
        *self
    }

    pub fn clear_usage(&mut self) -> Self {
        self.inner &= Self::USAGE.reverse_bits();
        *self
    }

    pub const fn is_time_mixture(&self) -> bool {
        (self.inner & Self::TIME_MIXTURE) != 0
    }

    pub const fn new_time_mixture() -> Self {
        Self { inner: Self::TIME_MIXTURE }
    }

    pub fn set_time_mixture(&mut self) -> Self {
        self.inner |= Self::TIME_MIXTURE;
        *self
    }

    pub fn clear_time_mixture(&mut self) -> Self {
        self.inner &= Self::TIME_MIXTURE.reverse_bits();
        *self
    }

    pub const fn is_restricted(&self) -> bool {
        (self.inner & Self::RESTRICTED) != 0
    }

    pub const fn new_restricted() -> Self {
        Self { inner: Self::RESTRICTED }
    }

    pub fn set_restricted(&mut self) -> Self {
        self.inner |= Self::RESTRICTED;
        *self
    }

    pub fn clear_restricted(&mut self) -> Self {
        self.inner &= Self::RESTRICTED.reverse_bits();
        *self
    }

    pub const fn is_enable_cais(&self) -> bool {
        (self.inner & Self::ENABLE_CAIS) != 0
    }

    pub const fn new_enable_cais() -> Self {
        Self { inner: Self::ENABLE_CAIS }
    }

    pub fn set_enable_cais(&mut self) -> Self {
        self.inner |= Self::ENABLE_CAIS;
        *self
    }

    pub fn clear_enable_cais(&mut self) -> Self {
        self.inner &= Self::ENABLE_CAIS.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for BillingPlanFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for BillingPlanFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for BillingPlanFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for BillingPlanFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for BillingPlanFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for BillingPlanFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for BillingPlanFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for BillingPlanFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for BillingPlanFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for BillingPlanFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

