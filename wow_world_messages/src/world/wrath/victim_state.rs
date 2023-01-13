/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm:58`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm#L58):
/// ```text
/// flag VictimState : u8 {
///     INTACT = 0;
///     HIT = 1;
///     DODGE = 2;
///     PARRY = 3;
///     INTERRUPT = 4;
///     BLOCKS = 5;
///     EVADES = 6;
///     IS_IMMUNE = 7;
///     DEFLECTS = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct VictimState {
    inner: u8,
}

impl VictimState {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub(crate) const INTACT: u8 = 0x00;
    pub(crate) const HIT: u8 = 0x01;
    pub(crate) const DODGE: u8 = 0x02;
    pub(crate) const PARRY: u8 = 0x03;
    pub(crate) const INTERRUPT: u8 = 0x04;
    pub(crate) const BLOCKS: u8 = 0x05;
    pub(crate) const EVADES: u8 = 0x06;
    pub(crate) const IS_IMMUNE: u8 = 0x07;
    pub(crate) const DEFLECTS: u8 = 0x08;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::INTACT
                | Self::HIT
                | Self::DODGE
                | Self::PARRY
                | Self::INTERRUPT
                | Self::BLOCKS
                | Self::EVADES
                | Self::IS_IMMUNE
                | Self::DEFLECTS
        }
    }

    pub const fn is_HIT(&self) -> bool {
        (self.inner & Self::HIT) != 0
    }

    /// azerothcore: victim got clear/blocked hit
    ///
    pub const fn new_HIT() -> Self {
        Self { inner: Self::HIT }
    }

    pub fn set_HIT(&mut self) -> Self {
        self.inner |= Self::HIT;
        *self
    }

    pub fn clear_HIT(&mut self) -> Self {
        self.inner &= Self::HIT.reverse_bits();
        *self
    }

    pub const fn is_DODGE(&self) -> bool {
        (self.inner & Self::DODGE) != 0
    }

    pub const fn new_DODGE() -> Self {
        Self { inner: Self::DODGE }
    }

    pub fn set_DODGE(&mut self) -> Self {
        self.inner |= Self::DODGE;
        *self
    }

    pub fn clear_DODGE(&mut self) -> Self {
        self.inner &= Self::DODGE.reverse_bits();
        *self
    }

    pub const fn is_PARRY(&self) -> bool {
        (self.inner & Self::PARRY) != 0
    }

    pub const fn new_PARRY() -> Self {
        Self { inner: Self::PARRY }
    }

    pub fn set_PARRY(&mut self) -> Self {
        self.inner |= Self::PARRY;
        *self
    }

    pub fn clear_PARRY(&mut self) -> Self {
        self.inner &= Self::PARRY.reverse_bits();
        *self
    }

    pub const fn is_INTERRUPT(&self) -> bool {
        (self.inner & Self::INTERRUPT) != 0
    }

    pub const fn new_INTERRUPT() -> Self {
        Self { inner: Self::INTERRUPT }
    }

    pub fn set_INTERRUPT(&mut self) -> Self {
        self.inner |= Self::INTERRUPT;
        *self
    }

    pub fn clear_INTERRUPT(&mut self) -> Self {
        self.inner &= Self::INTERRUPT.reverse_bits();
        *self
    }

    pub const fn is_BLOCKS(&self) -> bool {
        (self.inner & Self::BLOCKS) != 0
    }

    /// azerothcore: unused? not set when blocked, even on full block
    ///
    pub const fn new_BLOCKS() -> Self {
        Self { inner: Self::BLOCKS }
    }

    pub fn set_BLOCKS(&mut self) -> Self {
        self.inner |= Self::BLOCKS;
        *self
    }

    pub fn clear_BLOCKS(&mut self) -> Self {
        self.inner &= Self::BLOCKS.reverse_bits();
        *self
    }

    pub const fn is_EVADES(&self) -> bool {
        (self.inner & Self::EVADES) != 0
    }

    pub const fn new_EVADES() -> Self {
        Self { inner: Self::EVADES }
    }

    pub fn set_EVADES(&mut self) -> Self {
        self.inner |= Self::EVADES;
        *self
    }

    pub fn clear_EVADES(&mut self) -> Self {
        self.inner &= Self::EVADES.reverse_bits();
        *self
    }

    pub const fn is_IS_IMMUNE(&self) -> bool {
        (self.inner & Self::IS_IMMUNE) != 0
    }

    pub const fn new_IS_IMMUNE() -> Self {
        Self { inner: Self::IS_IMMUNE }
    }

    pub fn set_IS_IMMUNE(&mut self) -> Self {
        self.inner |= Self::IS_IMMUNE;
        *self
    }

    pub fn clear_IS_IMMUNE(&mut self) -> Self {
        self.inner &= Self::IS_IMMUNE.reverse_bits();
        *self
    }

    pub const fn is_DEFLECTS(&self) -> bool {
        (self.inner & Self::DEFLECTS) != 0
    }

    pub const fn new_DEFLECTS() -> Self {
        Self { inner: Self::DEFLECTS }
    }

    pub fn set_DEFLECTS(&mut self) -> Self {
        self.inner |= Self::DEFLECTS;
        *self
    }

    pub fn clear_DEFLECTS(&mut self) -> Self {
        self.inner &= Self::DEFLECTS.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for VictimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for VictimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for VictimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for VictimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

