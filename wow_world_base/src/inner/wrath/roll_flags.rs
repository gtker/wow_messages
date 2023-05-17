/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm#L14):
/// ```text
/// flag RollFlags : u8 {
///     PASS = 0x01;
///     NEED = 0x02;
///     GREED = 0x04;
///     DISENCHANT = 0x08;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RollFlags {
    inner: u8,
}

impl RollFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const PASS: u8 = 0x01;
    pub const NEED: u8 = 0x02;
    pub const GREED: u8 = 0x04;
    pub const DISENCHANT: u8 = 0x08;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::PASS
                | Self::NEED
                | Self::GREED
                | Self::DISENCHANT
        }
    }

    pub const fn is_pass(&self) -> bool {
        (self.inner & Self::PASS) != 0
    }

    pub const fn new_pass() -> Self {
        Self { inner: Self::PASS }
    }

    pub fn set_pass(&mut self) -> Self {
        self.inner |= Self::PASS;
        *self
    }

    pub fn clear_pass(&mut self) -> Self {
        self.inner &= Self::PASS.reverse_bits();
        *self
    }

    pub const fn is_need(&self) -> bool {
        (self.inner & Self::NEED) != 0
    }

    pub const fn new_need() -> Self {
        Self { inner: Self::NEED }
    }

    pub fn set_need(&mut self) -> Self {
        self.inner |= Self::NEED;
        *self
    }

    pub fn clear_need(&mut self) -> Self {
        self.inner &= Self::NEED.reverse_bits();
        *self
    }

    pub const fn is_greed(&self) -> bool {
        (self.inner & Self::GREED) != 0
    }

    pub const fn new_greed() -> Self {
        Self { inner: Self::GREED }
    }

    pub fn set_greed(&mut self) -> Self {
        self.inner |= Self::GREED;
        *self
    }

    pub fn clear_greed(&mut self) -> Self {
        self.inner &= Self::GREED.reverse_bits();
        *self
    }

    pub const fn is_disenchant(&self) -> bool {
        (self.inner & Self::DISENCHANT) != 0
    }

    pub const fn new_disenchant() -> Self {
        Self { inner: Self::DISENCHANT }
    }

    pub fn set_disenchant(&mut self) -> Self {
        self.inner |= Self::DISENCHANT;
        *self
    }

    pub fn clear_disenchant(&mut self) -> Self {
        self.inner &= Self::DISENCHANT.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for RollFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for RollFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for RollFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for RollFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for RollFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for RollFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for RollFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for RollFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for RollFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for RollFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

