/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L1):
/// ```text
/// flag SpellCastTargetFlags : u16 {
///     SELF = 0x00000000;
///     UNUSED1 = 0x00000001;
///     UNIT = 0x00000002;
///     UNUSED2 = 0x00000004;
///     UNUSED3 = 0x00000008;
///     ITEM = 0x00000010;
///     SOURCE_LOCATION = 0x00000020;
///     DEST_LOCATION = 0x00000040;
///     OBJECT_UNK = 0x00000080;
///     UNIT_UNK = 0x00000100;
///     PVP_CORPSE = 0x00000200;
///     UNIT_CORPSE = 0x00000400;
///     GAMEOBJECT = 0x00000800;
///     TRADE_ITEM = 0x00001000;
///     STRING = 0x00002000;
///     UNK1 = 0x00004000;
///     CORPSE = 0x00008000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SpellCastTargetFlags {
    inner: u16,
}

#[cfg(feature = "print-testcase")]
impl SpellCastTargetFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SELF").unwrap();
            first = false;
        }
        if self.is_unused1() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNUSED1").unwrap();
            first = false;
        }
        if self.is_unit() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT").unwrap();
            first = false;
        }
        if self.is_unused2() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNUSED2").unwrap();
            first = false;
        }
        if self.is_unused3() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNUSED3").unwrap();
            first = false;
        }
        if self.is_item() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ITEM").unwrap();
            first = false;
        }
        if self.is_source_location() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SOURCE_LOCATION").unwrap();
            first = false;
        }
        if self.is_dest_location() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DEST_LOCATION").unwrap();
            first = false;
        }
        if self.is_object_unk() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "OBJECT_UNK").unwrap();
            first = false;
        }
        if self.is_unit_unk() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT_UNK").unwrap();
            first = false;
        }
        if self.is_pvp_corpse() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PVP_CORPSE").unwrap();
            first = false;
        }
        if self.is_unit_corpse() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT_CORPSE").unwrap();
            first = false;
        }
        if self.is_gameobject() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "GAMEOBJECT").unwrap();
            first = false;
        }
        if self.is_trade_item() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TRADE_ITEM").unwrap();
            first = false;
        }
        if self.is_string() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "STRING").unwrap();
            first = false;
        }
        if self.is_unk1() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK1").unwrap();
            first = false;
        }
        if self.is_corpse() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CORPSE").unwrap();
            first = false;
        }
        s
    }

}

impl SpellCastTargetFlags {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

    pub const SELF: u16 = 0x00;
    pub const UNUSED1: u16 = 0x01;
    pub const UNIT: u16 = 0x02;
    pub const UNUSED2: u16 = 0x04;
    pub const UNUSED3: u16 = 0x08;
    pub const ITEM: u16 = 0x10;
    pub const SOURCE_LOCATION: u16 = 0x20;
    pub const DEST_LOCATION: u16 = 0x40;
    pub const OBJECT_UNK: u16 = 0x80;
    pub const UNIT_UNK: u16 = 0x100;
    pub const PVP_CORPSE: u16 = 0x200;
    pub const UNIT_CORPSE: u16 = 0x400;
    pub const GAMEOBJECT: u16 = 0x800;
    pub const TRADE_ITEM: u16 = 0x1000;
    pub const STRING: u16 = 0x2000;
    pub const UNK1: u16 = 0x4000;
    pub const CORPSE: u16 = 0x8000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::SELF
                | Self::UNUSED1
                | Self::UNIT
                | Self::UNUSED2
                | Self::UNUSED3
                | Self::ITEM
                | Self::SOURCE_LOCATION
                | Self::DEST_LOCATION
                | Self::OBJECT_UNK
                | Self::UNIT_UNK
                | Self::PVP_CORPSE
                | Self::UNIT_CORPSE
                | Self::GAMEOBJECT
                | Self::TRADE_ITEM
                | Self::STRING
                | Self::UNK1
                | Self::CORPSE
        }
    }

    pub const fn is_unused1(&self) -> bool {
        (self.inner & Self::UNUSED1) != 0
    }

    /// not used in any spells (can be set dynamically)
    pub const fn new_unused1() -> Self {
        Self { inner: Self::UNUSED1 }
    }

    pub fn set_unused1(&mut self) -> Self {
        self.inner |= Self::UNUSED1;
        *self
    }

    pub fn clear_unused1(&mut self) -> Self {
        self.inner &= Self::UNUSED1.reverse_bits();
        *self
    }

    pub const fn is_unit(&self) -> bool {
        (self.inner & Self::UNIT) != 0
    }

    pub const fn new_unit() -> Self {
        Self { inner: Self::UNIT }
    }

    pub fn set_unit(&mut self) -> Self {
        self.inner |= Self::UNIT;
        *self
    }

    pub fn clear_unit(&mut self) -> Self {
        self.inner &= Self::UNIT.reverse_bits();
        *self
    }

    pub const fn is_unused2(&self) -> bool {
        (self.inner & Self::UNUSED2) != 0
    }

    /// not used in any spells (can be set dynamically)
    pub const fn new_unused2() -> Self {
        Self { inner: Self::UNUSED2 }
    }

    pub fn set_unused2(&mut self) -> Self {
        self.inner |= Self::UNUSED2;
        *self
    }

    pub fn clear_unused2(&mut self) -> Self {
        self.inner &= Self::UNUSED2.reverse_bits();
        *self
    }

    pub const fn is_unused3(&self) -> bool {
        (self.inner & Self::UNUSED3) != 0
    }

    /// not used in any spells (can be set dynamically)
    pub const fn new_unused3() -> Self {
        Self { inner: Self::UNUSED3 }
    }

    pub fn set_unused3(&mut self) -> Self {
        self.inner |= Self::UNUSED3;
        *self
    }

    pub fn clear_unused3(&mut self) -> Self {
        self.inner &= Self::UNUSED3.reverse_bits();
        *self
    }

    pub const fn is_item(&self) -> bool {
        (self.inner & Self::ITEM) != 0
    }

    pub const fn new_item() -> Self {
        Self { inner: Self::ITEM }
    }

    pub fn set_item(&mut self) -> Self {
        self.inner |= Self::ITEM;
        *self
    }

    pub fn clear_item(&mut self) -> Self {
        self.inner &= Self::ITEM.reverse_bits();
        *self
    }

    pub const fn is_source_location(&self) -> bool {
        (self.inner & Self::SOURCE_LOCATION) != 0
    }

    pub const fn new_source_location() -> Self {
        Self { inner: Self::SOURCE_LOCATION }
    }

    pub fn set_source_location(&mut self) -> Self {
        self.inner |= Self::SOURCE_LOCATION;
        *self
    }

    pub fn clear_source_location(&mut self) -> Self {
        self.inner &= Self::SOURCE_LOCATION.reverse_bits();
        *self
    }

    pub const fn is_dest_location(&self) -> bool {
        (self.inner & Self::DEST_LOCATION) != 0
    }

    pub const fn new_dest_location() -> Self {
        Self { inner: Self::DEST_LOCATION }
    }

    pub fn set_dest_location(&mut self) -> Self {
        self.inner |= Self::DEST_LOCATION;
        *self
    }

    pub fn clear_dest_location(&mut self) -> Self {
        self.inner &= Self::DEST_LOCATION.reverse_bits();
        *self
    }

    pub const fn is_object_unk(&self) -> bool {
        (self.inner & Self::OBJECT_UNK) != 0
    }

    /// used in 7 spells only
    pub const fn new_object_unk() -> Self {
        Self { inner: Self::OBJECT_UNK }
    }

    pub fn set_object_unk(&mut self) -> Self {
        self.inner |= Self::OBJECT_UNK;
        *self
    }

    pub fn clear_object_unk(&mut self) -> Self {
        self.inner &= Self::OBJECT_UNK.reverse_bits();
        *self
    }

    pub const fn is_unit_unk(&self) -> bool {
        (self.inner & Self::UNIT_UNK) != 0
    }

    /// looks like self target (389 spells)
    pub const fn new_unit_unk() -> Self {
        Self { inner: Self::UNIT_UNK }
    }

    pub fn set_unit_unk(&mut self) -> Self {
        self.inner |= Self::UNIT_UNK;
        *self
    }

    pub fn clear_unit_unk(&mut self) -> Self {
        self.inner &= Self::UNIT_UNK.reverse_bits();
        *self
    }

    pub const fn is_pvp_corpse(&self) -> bool {
        (self.inner & Self::PVP_CORPSE) != 0
    }

    pub const fn new_pvp_corpse() -> Self {
        Self { inner: Self::PVP_CORPSE }
    }

    pub fn set_pvp_corpse(&mut self) -> Self {
        self.inner |= Self::PVP_CORPSE;
        *self
    }

    pub fn clear_pvp_corpse(&mut self) -> Self {
        self.inner &= Self::PVP_CORPSE.reverse_bits();
        *self
    }

    pub const fn is_unit_corpse(&self) -> bool {
        (self.inner & Self::UNIT_CORPSE) != 0
    }

    /// 10 spells (gathering professions)
    pub const fn new_unit_corpse() -> Self {
        Self { inner: Self::UNIT_CORPSE }
    }

    pub fn set_unit_corpse(&mut self) -> Self {
        self.inner |= Self::UNIT_CORPSE;
        *self
    }

    pub fn clear_unit_corpse(&mut self) -> Self {
        self.inner &= Self::UNIT_CORPSE.reverse_bits();
        *self
    }

    pub const fn is_gameobject(&self) -> bool {
        (self.inner & Self::GAMEOBJECT) != 0
    }

    /// pguid, 0 spells
    pub const fn new_gameobject() -> Self {
        Self { inner: Self::GAMEOBJECT }
    }

    pub fn set_gameobject(&mut self) -> Self {
        self.inner |= Self::GAMEOBJECT;
        *self
    }

    pub fn clear_gameobject(&mut self) -> Self {
        self.inner &= Self::GAMEOBJECT.reverse_bits();
        *self
    }

    pub const fn is_trade_item(&self) -> bool {
        (self.inner & Self::TRADE_ITEM) != 0
    }

    /// pguid, 0 spells
    pub const fn new_trade_item() -> Self {
        Self { inner: Self::TRADE_ITEM }
    }

    pub fn set_trade_item(&mut self) -> Self {
        self.inner |= Self::TRADE_ITEM;
        *self
    }

    pub fn clear_trade_item(&mut self) -> Self {
        self.inner &= Self::TRADE_ITEM.reverse_bits();
        *self
    }

    pub const fn is_string(&self) -> bool {
        (self.inner & Self::STRING) != 0
    }

    /// string, 0 spells
    pub const fn new_string() -> Self {
        Self { inner: Self::STRING }
    }

    pub fn set_string(&mut self) -> Self {
        self.inner |= Self::STRING;
        *self
    }

    pub fn clear_string(&mut self) -> Self {
        self.inner &= Self::STRING.reverse_bits();
        *self
    }

    pub const fn is_unk1(&self) -> bool {
        (self.inner & Self::UNK1) != 0
    }

    /// 199 spells, opening object/lock
    pub const fn new_unk1() -> Self {
        Self { inner: Self::UNK1 }
    }

    pub fn set_unk1(&mut self) -> Self {
        self.inner |= Self::UNK1;
        *self
    }

    pub fn clear_unk1(&mut self) -> Self {
        self.inner &= Self::UNK1.reverse_bits();
        *self
    }

    pub const fn is_corpse(&self) -> bool {
        (self.inner & Self::CORPSE) != 0
    }

    /// pguid, resurrection spells
    pub const fn new_corpse() -> Self {
        Self { inner: Self::CORPSE }
    }

    pub fn set_corpse(&mut self) -> Self {
        self.inner |= Self::CORPSE;
        *self
    }

    pub fn clear_corpse(&mut self) -> Self {
        self.inner &= Self::CORPSE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u16 {
        self.inner
    }

}

impl std::fmt::UpperHex for SpellCastTargetFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for SpellCastTargetFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for SpellCastTargetFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for SpellCastTargetFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for SpellCastTargetFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for SpellCastTargetFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for SpellCastTargetFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for SpellCastTargetFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for SpellCastTargetFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for SpellCastTargetFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u16> for SpellCastTargetFlags {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<u8> for SpellCastTargetFlags {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u32> for SpellCastTargetFlags {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for SpellCastTargetFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for SpellCastTargetFlags {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl From<i16> for SpellCastTargetFlags {
    fn from(value: i16) -> Self {
        Self::new(u16::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i32> for SpellCastTargetFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for SpellCastTargetFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for SpellCastTargetFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

