/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L11):
/// ```text
/// flag RealmFlag : u8 {
///     NONE = 0x00;
///     INVALID = 0x01;
///     OFFLINE = 0x02;
///     FORCE_BLUE_RECOMMENDED = 0x20;
///     FORCE_GREEN_RECOMMENDED = 0x40;
///     FORCE_RED_FULL = 0x80;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct RealmFlag {
    inner: u8,
}

impl RealmFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl RealmFlag {
    pub(crate) const NONE: u8 = 0x00;
    pub(crate) const INVALID: u8 = 0x01;
    pub(crate) const OFFLINE: u8 = 0x02;
    pub(crate) const FORCE_BLUE_RECOMMENDED: u8 = 0x20;
    pub(crate) const FORCE_GREEN_RECOMMENDED: u8 = 0x40;
    pub(crate) const FORCE_RED_FULL: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::INVALID
                | Self::OFFLINE
                | Self::FORCE_BLUE_RECOMMENDED
                | Self::FORCE_GREEN_RECOMMENDED
                | Self::FORCE_RED_FULL
        }
    }

    pub const fn is_INVALID(&self) -> bool {
        (self.inner & Self::INVALID) != 0
    }

    pub const fn new_INVALID() -> Self {
        Self { inner: Self::INVALID }
    }

    pub fn set_INVALID(&mut self) -> Self {
        self.inner |= Self::INVALID;
        *self
    }

    pub fn clear_INVALID(&mut self) -> Self {
        self.inner &= Self::INVALID.reverse_bits();
        *self
    }

    pub const fn is_OFFLINE(&self) -> bool {
        (self.inner & Self::OFFLINE) != 0
    }

    pub const fn new_OFFLINE() -> Self {
        Self { inner: Self::OFFLINE }
    }

    pub fn set_OFFLINE(&mut self) -> Self {
        self.inner |= Self::OFFLINE;
        *self
    }

    pub fn clear_OFFLINE(&mut self) -> Self {
        self.inner &= Self::OFFLINE.reverse_bits();
        *self
    }

    pub const fn is_FORCE_BLUE_RECOMMENDED(&self) -> bool {
        (self.inner & Self::FORCE_BLUE_RECOMMENDED) != 0
    }

    pub const fn new_FORCE_BLUE_RECOMMENDED() -> Self {
        Self { inner: Self::FORCE_BLUE_RECOMMENDED }
    }

    pub fn set_FORCE_BLUE_RECOMMENDED(&mut self) -> Self {
        self.inner |= Self::FORCE_BLUE_RECOMMENDED;
        *self
    }

    pub fn clear_FORCE_BLUE_RECOMMENDED(&mut self) -> Self {
        self.inner &= Self::FORCE_BLUE_RECOMMENDED.reverse_bits();
        *self
    }

    pub const fn is_FORCE_GREEN_RECOMMENDED(&self) -> bool {
        (self.inner & Self::FORCE_GREEN_RECOMMENDED) != 0
    }

    pub const fn new_FORCE_GREEN_RECOMMENDED() -> Self {
        Self { inner: Self::FORCE_GREEN_RECOMMENDED }
    }

    pub fn set_FORCE_GREEN_RECOMMENDED(&mut self) -> Self {
        self.inner |= Self::FORCE_GREEN_RECOMMENDED;
        *self
    }

    pub fn clear_FORCE_GREEN_RECOMMENDED(&mut self) -> Self {
        self.inner &= Self::FORCE_GREEN_RECOMMENDED.reverse_bits();
        *self
    }

    pub const fn is_FORCE_RED_FULL(&self) -> bool {
        (self.inner & Self::FORCE_RED_FULL) != 0
    }

    pub const fn new_FORCE_RED_FULL() -> Self {
        Self { inner: Self::FORCE_RED_FULL }
    }

    pub fn set_FORCE_RED_FULL(&mut self) -> Self {
        self.inner |= Self::FORCE_RED_FULL;
        *self
    }

    pub fn clear_FORCE_RED_FULL(&mut self) -> Self {
        self.inner &= Self::FORCE_RED_FULL.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

