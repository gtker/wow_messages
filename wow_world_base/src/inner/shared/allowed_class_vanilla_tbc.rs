/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/allowed_races.wowm:72`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/allowed_races.wowm#L72):
/// ```text
/// flag AllowedClass : u32 {
///     ALL = 0;
///     WARRIOR = 1;
///     PALADIN = 2;
///     HUNTER = 4;
///     ROGUE = 8;
///     PRIEST = 16;
///     SHAMAN = 64;
///     MAGE = 128;
///     WARLOCK = 256;
///     DRUID = 1024;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AllowedClass {
    inner: u32,
}

impl AllowedClass {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const ALL: u32 = 0x00;
    pub const WARRIOR: u32 = 0x01;
    pub const PALADIN: u32 = 0x02;
    pub const HUNTER: u32 = 0x04;
    pub const ROGUE: u32 = 0x08;
    pub const PRIEST: u32 = 0x10;
    pub const SHAMAN: u32 = 0x40;
    pub const MAGE: u32 = 0x80;
    pub const WARLOCK: u32 = 0x100;
    pub const DRUID: u32 = 0x400;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::ALL
                | Self::WARRIOR
                | Self::PALADIN
                | Self::HUNTER
                | Self::ROGUE
                | Self::PRIEST
                | Self::SHAMAN
                | Self::MAGE
                | Self::WARLOCK
                | Self::DRUID
        }
    }

    pub const fn is_WARRIOR(&self) -> bool {
        ((self.inner & Self::WARRIOR) != 0) || self.inner == 0
    }

    pub const fn new_WARRIOR() -> Self {
        Self { inner: Self::WARRIOR }
    }

    pub fn set_WARRIOR(&mut self) -> Self {
        self.inner |= Self::WARRIOR;
        *self
    }

    pub fn clear_WARRIOR(&mut self) -> Self {
        self.inner &= Self::WARRIOR.reverse_bits();
        *self
    }

    pub const fn is_PALADIN(&self) -> bool {
        ((self.inner & Self::PALADIN) != 0) || self.inner == 0
    }

    pub const fn new_PALADIN() -> Self {
        Self { inner: Self::PALADIN }
    }

    pub fn set_PALADIN(&mut self) -> Self {
        self.inner |= Self::PALADIN;
        *self
    }

    pub fn clear_PALADIN(&mut self) -> Self {
        self.inner &= Self::PALADIN.reverse_bits();
        *self
    }

    pub const fn is_HUNTER(&self) -> bool {
        ((self.inner & Self::HUNTER) != 0) || self.inner == 0
    }

    pub const fn new_HUNTER() -> Self {
        Self { inner: Self::HUNTER }
    }

    pub fn set_HUNTER(&mut self) -> Self {
        self.inner |= Self::HUNTER;
        *self
    }

    pub fn clear_HUNTER(&mut self) -> Self {
        self.inner &= Self::HUNTER.reverse_bits();
        *self
    }

    pub const fn is_ROGUE(&self) -> bool {
        ((self.inner & Self::ROGUE) != 0) || self.inner == 0
    }

    pub const fn new_ROGUE() -> Self {
        Self { inner: Self::ROGUE }
    }

    pub fn set_ROGUE(&mut self) -> Self {
        self.inner |= Self::ROGUE;
        *self
    }

    pub fn clear_ROGUE(&mut self) -> Self {
        self.inner &= Self::ROGUE.reverse_bits();
        *self
    }

    pub const fn is_PRIEST(&self) -> bool {
        ((self.inner & Self::PRIEST) != 0) || self.inner == 0
    }

    pub const fn new_PRIEST() -> Self {
        Self { inner: Self::PRIEST }
    }

    pub fn set_PRIEST(&mut self) -> Self {
        self.inner |= Self::PRIEST;
        *self
    }

    pub fn clear_PRIEST(&mut self) -> Self {
        self.inner &= Self::PRIEST.reverse_bits();
        *self
    }

    pub const fn is_SHAMAN(&self) -> bool {
        ((self.inner & Self::SHAMAN) != 0) || self.inner == 0
    }

    pub const fn new_SHAMAN() -> Self {
        Self { inner: Self::SHAMAN }
    }

    pub fn set_SHAMAN(&mut self) -> Self {
        self.inner |= Self::SHAMAN;
        *self
    }

    pub fn clear_SHAMAN(&mut self) -> Self {
        self.inner &= Self::SHAMAN.reverse_bits();
        *self
    }

    pub const fn is_MAGE(&self) -> bool {
        ((self.inner & Self::MAGE) != 0) || self.inner == 0
    }

    pub const fn new_MAGE() -> Self {
        Self { inner: Self::MAGE }
    }

    pub fn set_MAGE(&mut self) -> Self {
        self.inner |= Self::MAGE;
        *self
    }

    pub fn clear_MAGE(&mut self) -> Self {
        self.inner &= Self::MAGE.reverse_bits();
        *self
    }

    pub const fn is_WARLOCK(&self) -> bool {
        ((self.inner & Self::WARLOCK) != 0) || self.inner == 0
    }

    pub const fn new_WARLOCK() -> Self {
        Self { inner: Self::WARLOCK }
    }

    pub fn set_WARLOCK(&mut self) -> Self {
        self.inner |= Self::WARLOCK;
        *self
    }

    pub fn clear_WARLOCK(&mut self) -> Self {
        self.inner &= Self::WARLOCK.reverse_bits();
        *self
    }

    pub const fn is_DRUID(&self) -> bool {
        ((self.inner & Self::DRUID) != 0) || self.inner == 0
    }

    pub const fn new_DRUID() -> Self {
        Self { inner: Self::DRUID }
    }

    pub fn set_DRUID(&mut self) -> Self {
        self.inner |= Self::DRUID;
        *self
    }

    pub fn clear_DRUID(&mut self) -> Self {
        self.inner &= Self::DRUID.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for AllowedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AllowedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AllowedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AllowedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

