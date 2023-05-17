/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/allowed_races.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/allowed_races.wowm#L3):
/// ```text
/// flag AllowedRace : u32 {
///     ALL = 0;
///     HUMAN = 1;
///     ORC = 2;
///     DWARF = 4;
///     NIGHT_ELF = 8;
///     UNDEAD = 16;
///     TAUREN = 32;
///     GNOME = 64;
///     TROLL = 128;
///     GOBLIN = 256;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AllowedRace {
    inner: u32,
}

impl AllowedRace {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const ALL: u32 = 0x00;
    pub const HUMAN: u32 = 0x01;
    pub const ORC: u32 = 0x02;
    pub const DWARF: u32 = 0x04;
    pub const NIGHT_ELF: u32 = 0x08;
    pub const UNDEAD: u32 = 0x10;
    pub const TAUREN: u32 = 0x20;
    pub const GNOME: u32 = 0x40;
    pub const TROLL: u32 = 0x80;
    pub const GOBLIN: u32 = 0x100;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::ALL
                | Self::HUMAN
                | Self::ORC
                | Self::DWARF
                | Self::NIGHT_ELF
                | Self::UNDEAD
                | Self::TAUREN
                | Self::GNOME
                | Self::TROLL
                | Self::GOBLIN
        }
    }

    pub const fn is_human(&self) -> bool {
        ((self.inner & Self::HUMAN) != 0) || self.inner == 0
    }

    pub const fn new_human() -> Self {
        Self { inner: Self::HUMAN }
    }

    pub fn set_human(&mut self) -> Self {
        self.inner |= Self::HUMAN;
        *self
    }

    pub fn clear_human(&mut self) -> Self {
        self.inner &= Self::HUMAN.reverse_bits();
        *self
    }

    pub const fn is_orc(&self) -> bool {
        ((self.inner & Self::ORC) != 0) || self.inner == 0
    }

    pub const fn new_orc() -> Self {
        Self { inner: Self::ORC }
    }

    pub fn set_orc(&mut self) -> Self {
        self.inner |= Self::ORC;
        *self
    }

    pub fn clear_orc(&mut self) -> Self {
        self.inner &= Self::ORC.reverse_bits();
        *self
    }

    pub const fn is_dwarf(&self) -> bool {
        ((self.inner & Self::DWARF) != 0) || self.inner == 0
    }

    pub const fn new_dwarf() -> Self {
        Self { inner: Self::DWARF }
    }

    pub fn set_dwarf(&mut self) -> Self {
        self.inner |= Self::DWARF;
        *self
    }

    pub fn clear_dwarf(&mut self) -> Self {
        self.inner &= Self::DWARF.reverse_bits();
        *self
    }

    pub const fn is_night_elf(&self) -> bool {
        ((self.inner & Self::NIGHT_ELF) != 0) || self.inner == 0
    }

    pub const fn new_night_elf() -> Self {
        Self { inner: Self::NIGHT_ELF }
    }

    pub fn set_night_elf(&mut self) -> Self {
        self.inner |= Self::NIGHT_ELF;
        *self
    }

    pub fn clear_night_elf(&mut self) -> Self {
        self.inner &= Self::NIGHT_ELF.reverse_bits();
        *self
    }

    pub const fn is_undead(&self) -> bool {
        ((self.inner & Self::UNDEAD) != 0) || self.inner == 0
    }

    pub const fn new_undead() -> Self {
        Self { inner: Self::UNDEAD }
    }

    pub fn set_undead(&mut self) -> Self {
        self.inner |= Self::UNDEAD;
        *self
    }

    pub fn clear_undead(&mut self) -> Self {
        self.inner &= Self::UNDEAD.reverse_bits();
        *self
    }

    pub const fn is_tauren(&self) -> bool {
        ((self.inner & Self::TAUREN) != 0) || self.inner == 0
    }

    pub const fn new_tauren() -> Self {
        Self { inner: Self::TAUREN }
    }

    pub fn set_tauren(&mut self) -> Self {
        self.inner |= Self::TAUREN;
        *self
    }

    pub fn clear_tauren(&mut self) -> Self {
        self.inner &= Self::TAUREN.reverse_bits();
        *self
    }

    pub const fn is_gnome(&self) -> bool {
        ((self.inner & Self::GNOME) != 0) || self.inner == 0
    }

    pub const fn new_gnome() -> Self {
        Self { inner: Self::GNOME }
    }

    pub fn set_gnome(&mut self) -> Self {
        self.inner |= Self::GNOME;
        *self
    }

    pub fn clear_gnome(&mut self) -> Self {
        self.inner &= Self::GNOME.reverse_bits();
        *self
    }

    pub const fn is_troll(&self) -> bool {
        ((self.inner & Self::TROLL) != 0) || self.inner == 0
    }

    pub const fn new_troll() -> Self {
        Self { inner: Self::TROLL }
    }

    pub fn set_troll(&mut self) -> Self {
        self.inner |= Self::TROLL;
        *self
    }

    pub fn clear_troll(&mut self) -> Self {
        self.inner &= Self::TROLL.reverse_bits();
        *self
    }

    pub const fn is_goblin(&self) -> bool {
        ((self.inner & Self::GOBLIN) != 0) || self.inner == 0
    }

    pub const fn new_goblin() -> Self {
        Self { inner: Self::GOBLIN }
    }

    pub fn set_goblin(&mut self) -> Self {
        self.inner |= Self::GOBLIN;
        *self
    }

    pub fn clear_goblin(&mut self) -> Self {
        self.inner &= Self::GOBLIN.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for AllowedRace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AllowedRace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AllowedRace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AllowedRace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for AllowedRace {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for AllowedRace {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for AllowedRace {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for AllowedRace {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for AllowedRace {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for AllowedRace {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

