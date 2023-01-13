/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L12):
/// ```text
/// flag UpdateFlag : u16 {
///     NONE = 0x0000;
///     SELF = 0x0001;
///     TRANSPORT = 0x0002;
///     HAS_ATTACKING_TARGET = 0x0004;
///     LOW_GUID = 0x0008;
///     HIGH_GUID = 0x0010;
///     LIVING = 0x0020;
///     HAS_POSITION = 0x0040;
///     VEHICLE = 0x0080;
///     POSITION = 0x0100;
///     ROTATION = 0x0200;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct UpdateFlag {
    inner: u16,
}

impl UpdateFlag {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u16 = 0x00;
    pub(crate) const SELF: u16 = 0x01;
    pub(crate) const TRANSPORT: u16 = 0x02;
    pub(crate) const HAS_ATTACKING_TARGET: u16 = 0x04;
    pub(crate) const LOW_GUID: u16 = 0x08;
    pub(crate) const HIGH_GUID: u16 = 0x10;
    pub(crate) const LIVING: u16 = 0x20;
    pub(crate) const HAS_POSITION: u16 = 0x40;
    pub(crate) const VEHICLE: u16 = 0x80;
    pub(crate) const POSITION: u16 = 0x100;
    pub(crate) const ROTATION: u16 = 0x200;

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
                | Self::HAS_ATTACKING_TARGET
                | Self::LOW_GUID
                | Self::HIGH_GUID
                | Self::LIVING
                | Self::HAS_POSITION
                | Self::VEHICLE
                | Self::POSITION
                | Self::ROTATION
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

    pub const fn is_HAS_ATTACKING_TARGET(&self) -> bool {
        (self.inner & Self::HAS_ATTACKING_TARGET) != 0
    }

    pub const fn new_HAS_ATTACKING_TARGET() -> Self {
        Self { inner: Self::HAS_ATTACKING_TARGET }
    }

    pub fn set_HAS_ATTACKING_TARGET(&mut self) -> Self {
        self.inner |= Self::HAS_ATTACKING_TARGET;
        *self
    }

    pub fn clear_HAS_ATTACKING_TARGET(&mut self) -> Self {
        self.inner &= Self::HAS_ATTACKING_TARGET.reverse_bits();
        *self
    }

    pub const fn is_LOW_GUID(&self) -> bool {
        (self.inner & Self::LOW_GUID) != 0
    }

    pub const fn new_LOW_GUID() -> Self {
        Self { inner: Self::LOW_GUID }
    }

    pub fn set_LOW_GUID(&mut self) -> Self {
        self.inner |= Self::LOW_GUID;
        *self
    }

    pub fn clear_LOW_GUID(&mut self) -> Self {
        self.inner &= Self::LOW_GUID.reverse_bits();
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

    pub const fn is_VEHICLE(&self) -> bool {
        (self.inner & Self::VEHICLE) != 0
    }

    pub const fn new_VEHICLE() -> Self {
        Self { inner: Self::VEHICLE }
    }

    pub fn set_VEHICLE(&mut self) -> Self {
        self.inner |= Self::VEHICLE;
        *self
    }

    pub fn clear_VEHICLE(&mut self) -> Self {
        self.inner &= Self::VEHICLE.reverse_bits();
        *self
    }

    pub const fn is_POSITION(&self) -> bool {
        (self.inner & Self::POSITION) != 0
    }

    pub const fn new_POSITION() -> Self {
        Self { inner: Self::POSITION }
    }

    pub fn set_POSITION(&mut self) -> Self {
        self.inner |= Self::POSITION;
        *self
    }

    pub fn clear_POSITION(&mut self) -> Self {
        self.inner &= Self::POSITION.reverse_bits();
        *self
    }

    pub const fn is_ROTATION(&self) -> bool {
        (self.inner & Self::ROTATION) != 0
    }

    pub const fn new_ROTATION() -> Self {
        Self { inner: Self::ROTATION }
    }

    pub fn set_ROTATION(&mut self) -> Self {
        self.inner |= Self::ROTATION;
        *self
    }

    pub fn clear_ROTATION(&mut self) -> Self {
        self.inner &= Self::ROTATION.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u16 {
        self.inner
    }

}

impl std::fmt::UpperHex for UpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for UpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for UpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for UpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

