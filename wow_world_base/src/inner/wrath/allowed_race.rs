/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/allowed_races.wowm:44`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/allowed_races.wowm#L44):
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
///     BLOODELF = 512;
///     DRAENEI = 1024;
///     FEL_ORC = 2048;
///     NAGA = 4096;
///     BROKEN = 8192;
///     SKELETON = 16384;
///     VRYKUL = 32768;
///     TUSKARR = 65536;
///     FOREST_TROLL = 131072;
///     TAUNKA = 262144;
///     NORTHREND_SKELETON = 524288;
///     ICE_TROLL = 1048576;
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
    pub const BLOODELF: u32 = 0x200;
    pub const DRAENEI: u32 = 0x400;
    pub const FEL_ORC: u32 = 0x800;
    pub const NAGA: u32 = 0x1000;
    pub const BROKEN: u32 = 0x2000;
    pub const SKELETON: u32 = 0x4000;
    pub const VRYKUL: u32 = 0x8000;
    pub const TUSKARR: u32 = 0x10000;
    pub const FOREST_TROLL: u32 = 0x20000;
    pub const TAUNKA: u32 = 0x40000;
    pub const NORTHREND_SKELETON: u32 = 0x80000;
    pub const ICE_TROLL: u32 = 0x100000;

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
                | Self::BLOODELF
                | Self::DRAENEI
                | Self::FEL_ORC
                | Self::NAGA
                | Self::BROKEN
                | Self::SKELETON
                | Self::VRYKUL
                | Self::TUSKARR
                | Self::FOREST_TROLL
                | Self::TAUNKA
                | Self::NORTHREND_SKELETON
                | Self::ICE_TROLL
        }
    }

    pub const fn is_HUMAN(&self) -> bool {
        ((self.inner & Self::HUMAN) != 0) || self.inner == 0
    }

    pub const fn new_HUMAN() -> Self {
        Self { inner: Self::HUMAN }
    }

    pub fn set_HUMAN(&mut self) -> Self {
        self.inner |= Self::HUMAN;
        *self
    }

    pub fn clear_HUMAN(&mut self) -> Self {
        self.inner &= Self::HUMAN.reverse_bits();
        *self
    }

    pub const fn is_ORC(&self) -> bool {
        ((self.inner & Self::ORC) != 0) || self.inner == 0
    }

    pub const fn new_ORC() -> Self {
        Self { inner: Self::ORC }
    }

    pub fn set_ORC(&mut self) -> Self {
        self.inner |= Self::ORC;
        *self
    }

    pub fn clear_ORC(&mut self) -> Self {
        self.inner &= Self::ORC.reverse_bits();
        *self
    }

    pub const fn is_DWARF(&self) -> bool {
        ((self.inner & Self::DWARF) != 0) || self.inner == 0
    }

    pub const fn new_DWARF() -> Self {
        Self { inner: Self::DWARF }
    }

    pub fn set_DWARF(&mut self) -> Self {
        self.inner |= Self::DWARF;
        *self
    }

    pub fn clear_DWARF(&mut self) -> Self {
        self.inner &= Self::DWARF.reverse_bits();
        *self
    }

    pub const fn is_NIGHT_ELF(&self) -> bool {
        ((self.inner & Self::NIGHT_ELF) != 0) || self.inner == 0
    }

    pub const fn new_NIGHT_ELF() -> Self {
        Self { inner: Self::NIGHT_ELF }
    }

    pub fn set_NIGHT_ELF(&mut self) -> Self {
        self.inner |= Self::NIGHT_ELF;
        *self
    }

    pub fn clear_NIGHT_ELF(&mut self) -> Self {
        self.inner &= Self::NIGHT_ELF.reverse_bits();
        *self
    }

    pub const fn is_UNDEAD(&self) -> bool {
        ((self.inner & Self::UNDEAD) != 0) || self.inner == 0
    }

    pub const fn new_UNDEAD() -> Self {
        Self { inner: Self::UNDEAD }
    }

    pub fn set_UNDEAD(&mut self) -> Self {
        self.inner |= Self::UNDEAD;
        *self
    }

    pub fn clear_UNDEAD(&mut self) -> Self {
        self.inner &= Self::UNDEAD.reverse_bits();
        *self
    }

    pub const fn is_TAUREN(&self) -> bool {
        ((self.inner & Self::TAUREN) != 0) || self.inner == 0
    }

    pub const fn new_TAUREN() -> Self {
        Self { inner: Self::TAUREN }
    }

    pub fn set_TAUREN(&mut self) -> Self {
        self.inner |= Self::TAUREN;
        *self
    }

    pub fn clear_TAUREN(&mut self) -> Self {
        self.inner &= Self::TAUREN.reverse_bits();
        *self
    }

    pub const fn is_GNOME(&self) -> bool {
        ((self.inner & Self::GNOME) != 0) || self.inner == 0
    }

    pub const fn new_GNOME() -> Self {
        Self { inner: Self::GNOME }
    }

    pub fn set_GNOME(&mut self) -> Self {
        self.inner |= Self::GNOME;
        *self
    }

    pub fn clear_GNOME(&mut self) -> Self {
        self.inner &= Self::GNOME.reverse_bits();
        *self
    }

    pub const fn is_TROLL(&self) -> bool {
        ((self.inner & Self::TROLL) != 0) || self.inner == 0
    }

    pub const fn new_TROLL() -> Self {
        Self { inner: Self::TROLL }
    }

    pub fn set_TROLL(&mut self) -> Self {
        self.inner |= Self::TROLL;
        *self
    }

    pub fn clear_TROLL(&mut self) -> Self {
        self.inner &= Self::TROLL.reverse_bits();
        *self
    }

    pub const fn is_GOBLIN(&self) -> bool {
        ((self.inner & Self::GOBLIN) != 0) || self.inner == 0
    }

    pub const fn new_GOBLIN() -> Self {
        Self { inner: Self::GOBLIN }
    }

    pub fn set_GOBLIN(&mut self) -> Self {
        self.inner |= Self::GOBLIN;
        *self
    }

    pub fn clear_GOBLIN(&mut self) -> Self {
        self.inner &= Self::GOBLIN.reverse_bits();
        *self
    }

    pub const fn is_BLOODELF(&self) -> bool {
        ((self.inner & Self::BLOODELF) != 0) || self.inner == 0
    }

    pub const fn new_BLOODELF() -> Self {
        Self { inner: Self::BLOODELF }
    }

    pub fn set_BLOODELF(&mut self) -> Self {
        self.inner |= Self::BLOODELF;
        *self
    }

    pub fn clear_BLOODELF(&mut self) -> Self {
        self.inner &= Self::BLOODELF.reverse_bits();
        *self
    }

    pub const fn is_DRAENEI(&self) -> bool {
        ((self.inner & Self::DRAENEI) != 0) || self.inner == 0
    }

    pub const fn new_DRAENEI() -> Self {
        Self { inner: Self::DRAENEI }
    }

    pub fn set_DRAENEI(&mut self) -> Self {
        self.inner |= Self::DRAENEI;
        *self
    }

    pub fn clear_DRAENEI(&mut self) -> Self {
        self.inner &= Self::DRAENEI.reverse_bits();
        *self
    }

    pub const fn is_FEL_ORC(&self) -> bool {
        ((self.inner & Self::FEL_ORC) != 0) || self.inner == 0
    }

    pub const fn new_FEL_ORC() -> Self {
        Self { inner: Self::FEL_ORC }
    }

    pub fn set_FEL_ORC(&mut self) -> Self {
        self.inner |= Self::FEL_ORC;
        *self
    }

    pub fn clear_FEL_ORC(&mut self) -> Self {
        self.inner &= Self::FEL_ORC.reverse_bits();
        *self
    }

    pub const fn is_NAGA(&self) -> bool {
        ((self.inner & Self::NAGA) != 0) || self.inner == 0
    }

    pub const fn new_NAGA() -> Self {
        Self { inner: Self::NAGA }
    }

    pub fn set_NAGA(&mut self) -> Self {
        self.inner |= Self::NAGA;
        *self
    }

    pub fn clear_NAGA(&mut self) -> Self {
        self.inner &= Self::NAGA.reverse_bits();
        *self
    }

    pub const fn is_BROKEN(&self) -> bool {
        ((self.inner & Self::BROKEN) != 0) || self.inner == 0
    }

    pub const fn new_BROKEN() -> Self {
        Self { inner: Self::BROKEN }
    }

    pub fn set_BROKEN(&mut self) -> Self {
        self.inner |= Self::BROKEN;
        *self
    }

    pub fn clear_BROKEN(&mut self) -> Self {
        self.inner &= Self::BROKEN.reverse_bits();
        *self
    }

    pub const fn is_SKELETON(&self) -> bool {
        ((self.inner & Self::SKELETON) != 0) || self.inner == 0
    }

    pub const fn new_SKELETON() -> Self {
        Self { inner: Self::SKELETON }
    }

    pub fn set_SKELETON(&mut self) -> Self {
        self.inner |= Self::SKELETON;
        *self
    }

    pub fn clear_SKELETON(&mut self) -> Self {
        self.inner &= Self::SKELETON.reverse_bits();
        *self
    }

    pub const fn is_VRYKUL(&self) -> bool {
        ((self.inner & Self::VRYKUL) != 0) || self.inner == 0
    }

    pub const fn new_VRYKUL() -> Self {
        Self { inner: Self::VRYKUL }
    }

    pub fn set_VRYKUL(&mut self) -> Self {
        self.inner |= Self::VRYKUL;
        *self
    }

    pub fn clear_VRYKUL(&mut self) -> Self {
        self.inner &= Self::VRYKUL.reverse_bits();
        *self
    }

    pub const fn is_TUSKARR(&self) -> bool {
        ((self.inner & Self::TUSKARR) != 0) || self.inner == 0
    }

    pub const fn new_TUSKARR() -> Self {
        Self { inner: Self::TUSKARR }
    }

    pub fn set_TUSKARR(&mut self) -> Self {
        self.inner |= Self::TUSKARR;
        *self
    }

    pub fn clear_TUSKARR(&mut self) -> Self {
        self.inner &= Self::TUSKARR.reverse_bits();
        *self
    }

    pub const fn is_FOREST_TROLL(&self) -> bool {
        ((self.inner & Self::FOREST_TROLL) != 0) || self.inner == 0
    }

    pub const fn new_FOREST_TROLL() -> Self {
        Self { inner: Self::FOREST_TROLL }
    }

    pub fn set_FOREST_TROLL(&mut self) -> Self {
        self.inner |= Self::FOREST_TROLL;
        *self
    }

    pub fn clear_FOREST_TROLL(&mut self) -> Self {
        self.inner &= Self::FOREST_TROLL.reverse_bits();
        *self
    }

    pub const fn is_TAUNKA(&self) -> bool {
        ((self.inner & Self::TAUNKA) != 0) || self.inner == 0
    }

    pub const fn new_TAUNKA() -> Self {
        Self { inner: Self::TAUNKA }
    }

    pub fn set_TAUNKA(&mut self) -> Self {
        self.inner |= Self::TAUNKA;
        *self
    }

    pub fn clear_TAUNKA(&mut self) -> Self {
        self.inner &= Self::TAUNKA.reverse_bits();
        *self
    }

    pub const fn is_NORTHREND_SKELETON(&self) -> bool {
        ((self.inner & Self::NORTHREND_SKELETON) != 0) || self.inner == 0
    }

    pub const fn new_NORTHREND_SKELETON() -> Self {
        Self { inner: Self::NORTHREND_SKELETON }
    }

    pub fn set_NORTHREND_SKELETON(&mut self) -> Self {
        self.inner |= Self::NORTHREND_SKELETON;
        *self
    }

    pub fn clear_NORTHREND_SKELETON(&mut self) -> Self {
        self.inner &= Self::NORTHREND_SKELETON.reverse_bits();
        *self
    }

    pub const fn is_ICE_TROLL(&self) -> bool {
        ((self.inner & Self::ICE_TROLL) != 0) || self.inner == 0
    }

    pub const fn new_ICE_TROLL() -> Self {
        Self { inner: Self::ICE_TROLL }
    }

    pub fn set_ICE_TROLL(&mut self) -> Self {
        self.inner |= Self::ICE_TROLL;
        *self
    }

    pub fn clear_ICE_TROLL(&mut self) -> Self {
        self.inner &= Self::ICE_TROLL.reverse_bits();
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

