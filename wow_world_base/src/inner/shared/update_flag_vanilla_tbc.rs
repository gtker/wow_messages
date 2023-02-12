/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm#L14):
/// ```text
/// flag UpdateFlag : u8 {
///     NONE = 0x00;
///     SELF = 0x01;
///     TRANSPORT = 0x02;
///     MELEE_ATTACKING = 0x04;
///     HIGH_GUID = 0x08;
///     ALL = 0x10;
///     LIVING = 0x20;
///     HAS_POSITION = 0x40;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UpdateFlag {
    inner: u8,
}

impl UpdateFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const NONE: u8 = 0x00;
    pub const SELF: u8 = 0x01;
    pub const TRANSPORT: u8 = 0x02;
    pub const MELEE_ATTACKING: u8 = 0x04;
    pub const HIGH_GUID: u8 = 0x08;
    pub const ALL: u8 = 0x10;
    pub const LIVING: u8 = 0x20;
    pub const HAS_POSITION: u8 = 0x40;

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

    pub const fn as_int(&self) -> u8 {
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

