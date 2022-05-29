#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct UpdateFlag {
    inner: u8,
}

impl UpdateFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl UpdateFlag {
    pub(crate) const NONE: u8 = 0x00;
    pub(crate) const SELF: u8 = 0x01;
    pub(crate) const TRANSPORT: u8 = 0x02;
    pub(crate) const MELEE_ATTACKING: u8 = 0x04;
    pub(crate) const HIGH_GUID: u8 = 0x08;
    pub(crate) const ALL: u8 = 0x10;
    pub(crate) const LIVING: u8 = 0x20;
    pub(crate) const HAS_POSITION: u8 = 0x40;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::SELF
                | Self::TRANSPORT
                | Self::MELEE_ATTACKING
                | Self::HIGH_GUID
                | Self::ALL
                | Self::LIVING
                | Self::HAS_POSITION
        }
    }

    pub const fn is_SELF(&self) -> bool {
        (self.inner & Self::SELF) != 0
    }

    pub const fn new_SELF() -> Self {
        Self { inner: Self::SELF }
    }

    pub fn set_SELF(&mut self) -> Self {
        self.inner |= Self::SELF;
        *self
    }

    pub fn clear_SELF(&mut self) -> Self {
        self.inner &= Self::SELF.reverse_bits();
        *self
    }

    pub const fn is_TRANSPORT(&self) -> bool {
        (self.inner & Self::TRANSPORT) != 0
    }

    pub const fn new_TRANSPORT() -> Self {
        Self { inner: Self::TRANSPORT }
    }

    pub fn set_TRANSPORT(&mut self) -> Self {
        self.inner |= Self::TRANSPORT;
        *self
    }

    pub fn clear_TRANSPORT(&mut self) -> Self {
        self.inner &= Self::TRANSPORT.reverse_bits();
        *self
    }

    pub const fn is_MELEE_ATTACKING(&self) -> bool {
        (self.inner & Self::MELEE_ATTACKING) != 0
    }

    pub const fn new_MELEE_ATTACKING() -> Self {
        Self { inner: Self::MELEE_ATTACKING }
    }

    pub fn set_MELEE_ATTACKING(&mut self) -> Self {
        self.inner |= Self::MELEE_ATTACKING;
        *self
    }

    pub fn clear_MELEE_ATTACKING(&mut self) -> Self {
        self.inner &= Self::MELEE_ATTACKING.reverse_bits();
        *self
    }

    pub const fn is_HIGH_GUID(&self) -> bool {
        (self.inner & Self::HIGH_GUID) != 0
    }

    pub const fn new_HIGH_GUID() -> Self {
        Self { inner: Self::HIGH_GUID }
    }

    pub fn set_HIGH_GUID(&mut self) -> Self {
        self.inner |= Self::HIGH_GUID;
        *self
    }

    pub fn clear_HIGH_GUID(&mut self) -> Self {
        self.inner &= Self::HIGH_GUID.reverse_bits();
        *self
    }

    pub const fn is_ALL(&self) -> bool {
        (self.inner & Self::ALL) != 0
    }

    pub const fn new_ALL() -> Self {
        Self { inner: Self::ALL }
    }

    pub fn set_ALL(&mut self) -> Self {
        self.inner |= Self::ALL;
        *self
    }

    pub fn clear_ALL(&mut self) -> Self {
        self.inner &= Self::ALL.reverse_bits();
        *self
    }

    pub const fn is_LIVING(&self) -> bool {
        (self.inner & Self::LIVING) != 0
    }

    pub const fn new_LIVING() -> Self {
        Self { inner: Self::LIVING }
    }

    pub fn set_LIVING(&mut self) -> Self {
        self.inner |= Self::LIVING;
        *self
    }

    pub fn clear_LIVING(&mut self) -> Self {
        self.inner &= Self::LIVING.reverse_bits();
        *self
    }

    pub const fn is_HAS_POSITION(&self) -> bool {
        (self.inner & Self::HAS_POSITION) != 0
    }

    pub const fn new_HAS_POSITION() -> Self {
        Self { inner: Self::HAS_POSITION }
    }

    pub fn set_HAS_POSITION(&mut self) -> Self {
        self.inner |= Self::HAS_POSITION;
        *self
    }

    pub fn clear_HAS_POSITION(&mut self) -> Self {
        self.inner &= Self::HAS_POSITION.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

