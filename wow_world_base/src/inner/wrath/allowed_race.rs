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

#[cfg(feature = "print-testcase")]
impl AllowedRace {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ALL").unwrap();
            first = false;
        }
        if self.is_human() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HUMAN").unwrap();
            first = false;
        }
        if self.is_orc() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ORC").unwrap();
            first = false;
        }
        if self.is_dwarf() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DWARF").unwrap();
            first = false;
        }
        if self.is_night_elf() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NIGHT_ELF").unwrap();
            first = false;
        }
        if self.is_undead() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNDEAD").unwrap();
            first = false;
        }
        if self.is_tauren() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TAUREN").unwrap();
            first = false;
        }
        if self.is_gnome() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "GNOME").unwrap();
            first = false;
        }
        if self.is_troll() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TROLL").unwrap();
            first = false;
        }
        if self.is_goblin() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "GOBLIN").unwrap();
            first = false;
        }
        if self.is_bloodelf() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "BLOODELF").unwrap();
            first = false;
        }
        if self.is_draenei() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DRAENEI").unwrap();
            first = false;
        }
        if self.is_fel_orc() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FEL_ORC").unwrap();
            first = false;
        }
        if self.is_naga() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NAGA").unwrap();
            first = false;
        }
        if self.is_broken() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "BROKEN").unwrap();
            first = false;
        }
        if self.is_skeleton() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SKELETON").unwrap();
            first = false;
        }
        if self.is_vrykul() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "VRYKUL").unwrap();
            first = false;
        }
        if self.is_tuskarr() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TUSKARR").unwrap();
            first = false;
        }
        if self.is_forest_troll() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FOREST_TROLL").unwrap();
            first = false;
        }
        if self.is_taunka() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TAUNKA").unwrap();
            first = false;
        }
        if self.is_northrend_skeleton() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NORTHREND_SKELETON").unwrap();
            first = false;
        }
        if self.is_ice_troll() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ICE_TROLL").unwrap();
            first = false;
        }
        s
    }

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

    pub const fn is_bloodelf(&self) -> bool {
        ((self.inner & Self::BLOODELF) != 0) || self.inner == 0
    }

    pub const fn new_bloodelf() -> Self {
        Self { inner: Self::BLOODELF }
    }

    pub fn set_bloodelf(&mut self) -> Self {
        self.inner |= Self::BLOODELF;
        *self
    }

    pub fn clear_bloodelf(&mut self) -> Self {
        self.inner &= Self::BLOODELF.reverse_bits();
        *self
    }

    pub const fn is_draenei(&self) -> bool {
        ((self.inner & Self::DRAENEI) != 0) || self.inner == 0
    }

    pub const fn new_draenei() -> Self {
        Self { inner: Self::DRAENEI }
    }

    pub fn set_draenei(&mut self) -> Self {
        self.inner |= Self::DRAENEI;
        *self
    }

    pub fn clear_draenei(&mut self) -> Self {
        self.inner &= Self::DRAENEI.reverse_bits();
        *self
    }

    pub const fn is_fel_orc(&self) -> bool {
        ((self.inner & Self::FEL_ORC) != 0) || self.inner == 0
    }

    pub const fn new_fel_orc() -> Self {
        Self { inner: Self::FEL_ORC }
    }

    pub fn set_fel_orc(&mut self) -> Self {
        self.inner |= Self::FEL_ORC;
        *self
    }

    pub fn clear_fel_orc(&mut self) -> Self {
        self.inner &= Self::FEL_ORC.reverse_bits();
        *self
    }

    pub const fn is_naga(&self) -> bool {
        ((self.inner & Self::NAGA) != 0) || self.inner == 0
    }

    pub const fn new_naga() -> Self {
        Self { inner: Self::NAGA }
    }

    pub fn set_naga(&mut self) -> Self {
        self.inner |= Self::NAGA;
        *self
    }

    pub fn clear_naga(&mut self) -> Self {
        self.inner &= Self::NAGA.reverse_bits();
        *self
    }

    pub const fn is_broken(&self) -> bool {
        ((self.inner & Self::BROKEN) != 0) || self.inner == 0
    }

    pub const fn new_broken() -> Self {
        Self { inner: Self::BROKEN }
    }

    pub fn set_broken(&mut self) -> Self {
        self.inner |= Self::BROKEN;
        *self
    }

    pub fn clear_broken(&mut self) -> Self {
        self.inner &= Self::BROKEN.reverse_bits();
        *self
    }

    pub const fn is_skeleton(&self) -> bool {
        ((self.inner & Self::SKELETON) != 0) || self.inner == 0
    }

    pub const fn new_skeleton() -> Self {
        Self { inner: Self::SKELETON }
    }

    pub fn set_skeleton(&mut self) -> Self {
        self.inner |= Self::SKELETON;
        *self
    }

    pub fn clear_skeleton(&mut self) -> Self {
        self.inner &= Self::SKELETON.reverse_bits();
        *self
    }

    pub const fn is_vrykul(&self) -> bool {
        ((self.inner & Self::VRYKUL) != 0) || self.inner == 0
    }

    pub const fn new_vrykul() -> Self {
        Self { inner: Self::VRYKUL }
    }

    pub fn set_vrykul(&mut self) -> Self {
        self.inner |= Self::VRYKUL;
        *self
    }

    pub fn clear_vrykul(&mut self) -> Self {
        self.inner &= Self::VRYKUL.reverse_bits();
        *self
    }

    pub const fn is_tuskarr(&self) -> bool {
        ((self.inner & Self::TUSKARR) != 0) || self.inner == 0
    }

    pub const fn new_tuskarr() -> Self {
        Self { inner: Self::TUSKARR }
    }

    pub fn set_tuskarr(&mut self) -> Self {
        self.inner |= Self::TUSKARR;
        *self
    }

    pub fn clear_tuskarr(&mut self) -> Self {
        self.inner &= Self::TUSKARR.reverse_bits();
        *self
    }

    pub const fn is_forest_troll(&self) -> bool {
        ((self.inner & Self::FOREST_TROLL) != 0) || self.inner == 0
    }

    pub const fn new_forest_troll() -> Self {
        Self { inner: Self::FOREST_TROLL }
    }

    pub fn set_forest_troll(&mut self) -> Self {
        self.inner |= Self::FOREST_TROLL;
        *self
    }

    pub fn clear_forest_troll(&mut self) -> Self {
        self.inner &= Self::FOREST_TROLL.reverse_bits();
        *self
    }

    pub const fn is_taunka(&self) -> bool {
        ((self.inner & Self::TAUNKA) != 0) || self.inner == 0
    }

    pub const fn new_taunka() -> Self {
        Self { inner: Self::TAUNKA }
    }

    pub fn set_taunka(&mut self) -> Self {
        self.inner |= Self::TAUNKA;
        *self
    }

    pub fn clear_taunka(&mut self) -> Self {
        self.inner &= Self::TAUNKA.reverse_bits();
        *self
    }

    pub const fn is_northrend_skeleton(&self) -> bool {
        ((self.inner & Self::NORTHREND_SKELETON) != 0) || self.inner == 0
    }

    pub const fn new_northrend_skeleton() -> Self {
        Self { inner: Self::NORTHREND_SKELETON }
    }

    pub fn set_northrend_skeleton(&mut self) -> Self {
        self.inner |= Self::NORTHREND_SKELETON;
        *self
    }

    pub fn clear_northrend_skeleton(&mut self) -> Self {
        self.inner &= Self::NORTHREND_SKELETON.reverse_bits();
        *self
    }

    pub const fn is_ice_troll(&self) -> bool {
        ((self.inner & Self::ICE_TROLL) != 0) || self.inner == 0
    }

    pub const fn new_ice_troll() -> Self {
        Self { inner: Self::ICE_TROLL }
    }

    pub fn set_ice_troll(&mut self) -> Self {
        self.inner |= Self::ICE_TROLL;
        *self
    }

    pub fn clear_ice_troll(&mut self) -> Self {
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

impl From<u32> for AllowedRace {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for AllowedRace {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for AllowedRace {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for AllowedRace {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for AllowedRace {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for AllowedRace {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for AllowedRace {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for AllowedRace {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for AllowedRace {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

