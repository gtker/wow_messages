/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:70`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L70):
/// ```text
/// flag SpellCastTargetFlags : u32 {
///     SELF = 0x00000000;
///     UNUSED1 = 0x00000001;
///     UNIT = 0x00000002;
///     UNIT_RAID = 0x00000004;
///     UNIT_PARTY = 0x00000008;
///     ITEM = 0x00000010;
///     SOURCE_LOCATION = 0x00000020;
///     DEST_LOCATION = 0x00000040;
///     UNIT_ENEMY = 0x00000080;
///     UNIT_ALLY = 0x00000100;
///     CORPSE_ENEMY = 0x00000200;
///     UNIT_DEAD = 0x00000400;
///     GAMEOBJECT = 0x00000800;
///     TRADE_ITEM = 0x00001000;
///     STRING = 0x00002000;
///     LOCKED = 0x00004000;
///     CORPSE_ALLY = 0x00008000;
///     UNIT_MINIPET = 0x00010000;
///     GLYPH_SLOT = 0x00020000;
///     DEST_TARGET = 0x00040000;
///     UNUSED20 = 0x00080000;
///     UNIT_PASSENGER = 0x00100000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SpellCastTargetFlags {
    inner: u32,
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
        if self.is_unit_raid() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT_RAID").unwrap();
            first = false;
        }
        if self.is_unit_party() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT_PARTY").unwrap();
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
        if self.is_unit_enemy() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT_ENEMY").unwrap();
            first = false;
        }
        if self.is_unit_ally() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT_ALLY").unwrap();
            first = false;
        }
        if self.is_corpse_enemy() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CORPSE_ENEMY").unwrap();
            first = false;
        }
        if self.is_unit_dead() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT_DEAD").unwrap();
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
        if self.is_locked() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "LOCKED").unwrap();
            first = false;
        }
        if self.is_corpse_ally() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CORPSE_ALLY").unwrap();
            first = false;
        }
        if self.is_unit_minipet() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT_MINIPET").unwrap();
            first = false;
        }
        if self.is_glyph_slot() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "GLYPH_SLOT").unwrap();
            first = false;
        }
        if self.is_dest_target() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DEST_TARGET").unwrap();
            first = false;
        }
        if self.is_unused20() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNUSED20").unwrap();
            first = false;
        }
        if self.is_unit_passenger() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNIT_PASSENGER").unwrap();
            first = false;
        }
        s
    }

}

impl SpellCastTargetFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const SELF: u32 = 0x00;
    pub const UNUSED1: u32 = 0x01;
    pub const UNIT: u32 = 0x02;
    pub const UNIT_RAID: u32 = 0x04;
    pub const UNIT_PARTY: u32 = 0x08;
    pub const ITEM: u32 = 0x10;
    pub const SOURCE_LOCATION: u32 = 0x20;
    pub const DEST_LOCATION: u32 = 0x40;
    pub const UNIT_ENEMY: u32 = 0x80;
    pub const UNIT_ALLY: u32 = 0x100;
    pub const CORPSE_ENEMY: u32 = 0x200;
    pub const UNIT_DEAD: u32 = 0x400;
    pub const GAMEOBJECT: u32 = 0x800;
    pub const TRADE_ITEM: u32 = 0x1000;
    pub const STRING: u32 = 0x2000;
    pub const LOCKED: u32 = 0x4000;
    pub const CORPSE_ALLY: u32 = 0x8000;
    pub const UNIT_MINIPET: u32 = 0x10000;
    pub const GLYPH_SLOT: u32 = 0x20000;
    pub const DEST_TARGET: u32 = 0x40000;
    pub const UNUSED20: u32 = 0x80000;
    pub const UNIT_PASSENGER: u32 = 0x100000;

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
                | Self::UNIT_RAID
                | Self::UNIT_PARTY
                | Self::ITEM
                | Self::SOURCE_LOCATION
                | Self::DEST_LOCATION
                | Self::UNIT_ENEMY
                | Self::UNIT_ALLY
                | Self::CORPSE_ENEMY
                | Self::UNIT_DEAD
                | Self::GAMEOBJECT
                | Self::TRADE_ITEM
                | Self::STRING
                | Self::LOCKED
                | Self::CORPSE_ALLY
                | Self::UNIT_MINIPET
                | Self::GLYPH_SLOT
                | Self::DEST_TARGET
                | Self::UNUSED20
                | Self::UNIT_PASSENGER
        }
    }

    pub const fn is_unused1(&self) -> bool {
        (self.inner & Self::UNUSED1) != 0
    }

    /// not used in any spells as of 2.4.3 (can be set dynamically)
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

    /// pguid
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

    pub const fn is_unit_raid(&self) -> bool {
        (self.inner & Self::UNIT_RAID) != 0
    }

    /// not used in any spells as of 2.4.3 (can be set dynamically) - raid member
    pub const fn new_unit_raid() -> Self {
        Self { inner: Self::UNIT_RAID }
    }

    pub fn set_unit_raid(&mut self) -> Self {
        self.inner |= Self::UNIT_RAID;
        *self
    }

    pub fn clear_unit_raid(&mut self) -> Self {
        self.inner &= Self::UNIT_RAID.reverse_bits();
        *self
    }

    pub const fn is_unit_party(&self) -> bool {
        (self.inner & Self::UNIT_PARTY) != 0
    }

    /// not used in any spells as of 2.4.3 (can be set dynamically) - party member
    pub const fn new_unit_party() -> Self {
        Self { inner: Self::UNIT_PARTY }
    }

    pub fn set_unit_party(&mut self) -> Self {
        self.inner |= Self::UNIT_PARTY;
        *self
    }

    pub fn clear_unit_party(&mut self) -> Self {
        self.inner &= Self::UNIT_PARTY.reverse_bits();
        *self
    }

    pub const fn is_item(&self) -> bool {
        (self.inner & Self::ITEM) != 0
    }

    /// pguid
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

    /// 3xfloat
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

    /// 3xfloat
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

    pub const fn is_unit_enemy(&self) -> bool {
        (self.inner & Self::UNIT_ENEMY) != 0
    }

    /// `CanAttack` == true
    pub const fn new_unit_enemy() -> Self {
        Self { inner: Self::UNIT_ENEMY }
    }

    pub fn set_unit_enemy(&mut self) -> Self {
        self.inner |= Self::UNIT_ENEMY;
        *self
    }

    pub fn clear_unit_enemy(&mut self) -> Self {
        self.inner &= Self::UNIT_ENEMY.reverse_bits();
        *self
    }

    pub const fn is_unit_ally(&self) -> bool {
        (self.inner & Self::UNIT_ALLY) != 0
    }

    /// `CanAssist` == true
    pub const fn new_unit_ally() -> Self {
        Self { inner: Self::UNIT_ALLY }
    }

    pub fn set_unit_ally(&mut self) -> Self {
        self.inner |= Self::UNIT_ALLY;
        *self
    }

    pub fn clear_unit_ally(&mut self) -> Self {
        self.inner &= Self::UNIT_ALLY.reverse_bits();
        *self
    }

    pub const fn is_corpse_enemy(&self) -> bool {
        (self.inner & Self::CORPSE_ENEMY) != 0
    }

    /// pguid, `CanAssist` == false
    pub const fn new_corpse_enemy() -> Self {
        Self { inner: Self::CORPSE_ENEMY }
    }

    pub fn set_corpse_enemy(&mut self) -> Self {
        self.inner |= Self::CORPSE_ENEMY;
        *self
    }

    pub fn clear_corpse_enemy(&mut self) -> Self {
        self.inner &= Self::CORPSE_ENEMY.reverse_bits();
        *self
    }

    pub const fn is_unit_dead(&self) -> bool {
        (self.inner & Self::UNIT_DEAD) != 0
    }

    /// skinning-like effects
    pub const fn new_unit_dead() -> Self {
        Self { inner: Self::UNIT_DEAD }
    }

    pub fn set_unit_dead(&mut self) -> Self {
        self.inner |= Self::UNIT_DEAD;
        *self
    }

    pub fn clear_unit_dead(&mut self) -> Self {
        self.inner &= Self::UNIT_DEAD.reverse_bits();
        *self
    }

    pub const fn is_gameobject(&self) -> bool {
        (self.inner & Self::GAMEOBJECT) != 0
    }

    /// pguid, 0 spells in 2.4.3
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

    pub const fn is_locked(&self) -> bool {
        (self.inner & Self::LOCKED) != 0
    }

    /// 199 spells, opening object/lock
    pub const fn new_locked() -> Self {
        Self { inner: Self::LOCKED }
    }

    pub fn set_locked(&mut self) -> Self {
        self.inner |= Self::LOCKED;
        *self
    }

    pub fn clear_locked(&mut self) -> Self {
        self.inner &= Self::LOCKED.reverse_bits();
        *self
    }

    pub const fn is_corpse_ally(&self) -> bool {
        (self.inner & Self::CORPSE_ALLY) != 0
    }

    /// pguid, `CanAssist` == true
    pub const fn new_corpse_ally() -> Self {
        Self { inner: Self::CORPSE_ALLY }
    }

    pub fn set_corpse_ally(&mut self) -> Self {
        self.inner |= Self::CORPSE_ALLY;
        *self
    }

    pub fn clear_corpse_ally(&mut self) -> Self {
        self.inner &= Self::CORPSE_ALLY.reverse_bits();
        *self
    }

    pub const fn is_unit_minipet(&self) -> bool {
        (self.inner & Self::UNIT_MINIPET) != 0
    }

    /// pguid, not used in any spells as of 2.4.3 (can be set dynamically)
    pub const fn new_unit_minipet() -> Self {
        Self { inner: Self::UNIT_MINIPET }
    }

    pub fn set_unit_minipet(&mut self) -> Self {
        self.inner |= Self::UNIT_MINIPET;
        *self
    }

    pub fn clear_unit_minipet(&mut self) -> Self {
        self.inner &= Self::UNIT_MINIPET.reverse_bits();
        *self
    }

    pub const fn is_glyph_slot(&self) -> bool {
        (self.inner & Self::GLYPH_SLOT) != 0
    }

    /// used in glyph spells
    pub const fn new_glyph_slot() -> Self {
        Self { inner: Self::GLYPH_SLOT }
    }

    pub fn set_glyph_slot(&mut self) -> Self {
        self.inner |= Self::GLYPH_SLOT;
        *self
    }

    pub fn clear_glyph_slot(&mut self) -> Self {
        self.inner &= Self::GLYPH_SLOT.reverse_bits();
        *self
    }

    pub const fn is_dest_target(&self) -> bool {
        (self.inner & Self::DEST_TARGET) != 0
    }

    /// sometimes appears with `DEST_TARGET` spells (may appear or not for a given spell)
    pub const fn new_dest_target() -> Self {
        Self { inner: Self::DEST_TARGET }
    }

    pub fn set_dest_target(&mut self) -> Self {
        self.inner |= Self::DEST_TARGET;
        *self
    }

    pub fn clear_dest_target(&mut self) -> Self {
        self.inner &= Self::DEST_TARGET.reverse_bits();
        *self
    }

    pub const fn is_unused20(&self) -> bool {
        (self.inner & Self::UNUSED20) != 0
    }

    /// uint32 counter loop, vec3 - screen position (?) guid, not used so far
    pub const fn new_unused20() -> Self {
        Self { inner: Self::UNUSED20 }
    }

    pub fn set_unused20(&mut self) -> Self {
        self.inner |= Self::UNUSED20;
        *self
    }

    pub fn clear_unused20(&mut self) -> Self {
        self.inner &= Self::UNUSED20.reverse_bits();
        *self
    }

    pub const fn is_unit_passenger(&self) -> bool {
        (self.inner & Self::UNIT_PASSENGER) != 0
    }

    /// guessed, used to validate target (if vehicle passenger)
    pub const fn new_unit_passenger() -> Self {
        Self { inner: Self::UNIT_PASSENGER }
    }

    pub fn set_unit_passenger(&mut self) -> Self {
        self.inner |= Self::UNIT_PASSENGER;
        *self
    }

    pub fn clear_unit_passenger(&mut self) -> Self {
        self.inner &= Self::UNIT_PASSENGER.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
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

impl From<u32> for SpellCastTargetFlags {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for SpellCastTargetFlags {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for SpellCastTargetFlags {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for SpellCastTargetFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for SpellCastTargetFlags {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for SpellCastTargetFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for SpellCastTargetFlags {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for SpellCastTargetFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for SpellCastTargetFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

