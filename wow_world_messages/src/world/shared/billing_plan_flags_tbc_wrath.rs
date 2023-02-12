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
pub struct BillingPlanFlags {
    inner: u8,
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

    pub const fn is_UNUSED(&self) -> bool {
        (self.inner & Self::UNUSED) != 0
    }

    pub const fn new_UNUSED() -> Self {
        Self { inner: Self::UNUSED }
    }

    pub fn set_UNUSED(&mut self) -> Self {
        self.inner |= Self::UNUSED;
        *self
    }

    pub fn clear_UNUSED(&mut self) -> Self {
        self.inner &= Self::UNUSED.reverse_bits();
        *self
    }

    pub const fn is_RECURRING_BILL(&self) -> bool {
        (self.inner & Self::RECURRING_BILL) != 0
    }

    pub const fn new_RECURRING_BILL() -> Self {
        Self { inner: Self::RECURRING_BILL }
    }

    pub fn set_RECURRING_BILL(&mut self) -> Self {
        self.inner |= Self::RECURRING_BILL;
        *self
    }

    pub fn clear_RECURRING_BILL(&mut self) -> Self {
        self.inner &= Self::RECURRING_BILL.reverse_bits();
        *self
    }

    pub const fn is_FREE_TRIAL(&self) -> bool {
        (self.inner & Self::FREE_TRIAL) != 0
    }

    pub const fn new_FREE_TRIAL() -> Self {
        Self { inner: Self::FREE_TRIAL }
    }

    pub fn set_FREE_TRIAL(&mut self) -> Self {
        self.inner |= Self::FREE_TRIAL;
        *self
    }

    pub fn clear_FREE_TRIAL(&mut self) -> Self {
        self.inner &= Self::FREE_TRIAL.reverse_bits();
        *self
    }

    pub const fn is_IGR(&self) -> bool {
        (self.inner & Self::IGR) != 0
    }

    /// Name meaning unknown
    ///
    pub const fn new_IGR() -> Self {
        Self { inner: Self::IGR }
    }

    pub fn set_IGR(&mut self) -> Self {
        self.inner |= Self::IGR;
        *self
    }

    pub fn clear_IGR(&mut self) -> Self {
        self.inner &= Self::IGR.reverse_bits();
        *self
    }

    pub const fn is_USAGE(&self) -> bool {
        (self.inner & Self::USAGE) != 0
    }

    pub const fn new_USAGE() -> Self {
        Self { inner: Self::USAGE }
    }

    pub fn set_USAGE(&mut self) -> Self {
        self.inner |= Self::USAGE;
        *self
    }

    pub fn clear_USAGE(&mut self) -> Self {
        self.inner &= Self::USAGE.reverse_bits();
        *self
    }

    pub const fn is_TIME_MIXTURE(&self) -> bool {
        (self.inner & Self::TIME_MIXTURE) != 0
    }

    pub const fn new_TIME_MIXTURE() -> Self {
        Self { inner: Self::TIME_MIXTURE }
    }

    pub fn set_TIME_MIXTURE(&mut self) -> Self {
        self.inner |= Self::TIME_MIXTURE;
        *self
    }

    pub fn clear_TIME_MIXTURE(&mut self) -> Self {
        self.inner &= Self::TIME_MIXTURE.reverse_bits();
        *self
    }

    pub const fn is_RESTRICTED(&self) -> bool {
        (self.inner & Self::RESTRICTED) != 0
    }

    pub const fn new_RESTRICTED() -> Self {
        Self { inner: Self::RESTRICTED }
    }

    pub fn set_RESTRICTED(&mut self) -> Self {
        self.inner |= Self::RESTRICTED;
        *self
    }

    pub fn clear_RESTRICTED(&mut self) -> Self {
        self.inner &= Self::RESTRICTED.reverse_bits();
        *self
    }

    pub const fn is_ENABLE_CAIS(&self) -> bool {
        (self.inner & Self::ENABLE_CAIS) != 0
    }

    pub const fn new_ENABLE_CAIS() -> Self {
        Self { inner: Self::ENABLE_CAIS }
    }

    pub fn set_ENABLE_CAIS(&mut self) -> Self {
        self.inner |= Self::ENABLE_CAIS;
        *self
    }

    pub fn clear_ENABLE_CAIS(&mut self) -> Self {
        self.inner &= Self::ENABLE_CAIS.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
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

