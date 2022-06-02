/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm#L26):
/// ```text
/// flag AccountFlag : u32 {
///     GM = 0x000001;
///     TRIAL = 0x000008;
///     PROPASS = 0x800000;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct AccountFlag {
    inner: u32,
}

impl AccountFlag {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const GM: u32 = 0x01;
    pub(crate) const TRIAL: u32 = 0x08;
    pub(crate) const PROPASS: u32 = 0x800000;

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

    pub const fn is_GM(&self) -> bool {
        (self.inner & Self::GM) != 0
    }

    pub const fn new_GM() -> Self {
        Self { inner: Self::GM }
    }

    pub fn set_GM(&mut self) -> Self {
        self.inner |= Self::GM;
        *self
    }

    pub fn clear_GM(&mut self) -> Self {
        self.inner &= Self::GM.reverse_bits();
        *self
    }

    pub const fn is_TRIAL(&self) -> bool {
        (self.inner & Self::TRIAL) != 0
    }

    pub const fn new_TRIAL() -> Self {
        Self { inner: Self::TRIAL }
    }

    pub fn set_TRIAL(&mut self) -> Self {
        self.inner |= Self::TRIAL;
        *self
    }

    pub fn clear_TRIAL(&mut self) -> Self {
        self.inner &= Self::TRIAL.reverse_bits();
        *self
    }

    pub const fn is_PROPASS(&self) -> bool {
        (self.inner & Self::PROPASS) != 0
    }

    pub const fn new_PROPASS() -> Self {
        Self { inner: Self::PROPASS }
    }

    pub fn set_PROPASS(&mut self) -> Self {
        self.inner |= Self::PROPASS;
        *self
    }

    pub fn clear_PROPASS(&mut self) -> Self {
        self.inner &= Self::PROPASS.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

