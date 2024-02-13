/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm:54`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm#L54):
/// ```text
/// flag GameobjectCastFlags : u32 {
///     LOCK_PLAYER_CAST_ANIM = 0x01;
///     UNKNOWN2 = 0x02;
///     UNKNOWN4 = 0x04;
///     UNKNOWN8 = 0x08;
///     UNKNOWN16 = 0x10;
///     AMMO = 0x20;
///     DEST_LOCATION = 0x040;
///     ITEM_CASTER = 0x100;
///     UNK200 = 0x200;
///     EXTRA_MESSAGE = 0x400;
///     POWER_UPDATE = 0x800;
///     UNK2000 = 0x2000;
///     UNK1000 = 0x1000;
///     UNK8000 = 0x8000;
///     ADJUST_MISSILE = 0x20000;
///     UNK40000 = 0x40000;
///     VISUAL_CHAIN = 0x80000;
///     RUNE_UPDATE = 0x200000;
///     UNK400000 = 0x400000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GameobjectCastFlags {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl GameobjectCastFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_lock_player_cast_anim() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "LOCK_PLAYER_CAST_ANIM").unwrap();
            first = false;
        }
        if self.is_unknown2() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN2").unwrap();
            first = false;
        }
        if self.is_unknown4() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN4").unwrap();
            first = false;
        }
        if self.is_unknown8() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN8").unwrap();
            first = false;
        }
        if self.is_unknown16() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN16").unwrap();
            first = false;
        }
        if self.is_ammo() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "AMMO").unwrap();
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
        if self.is_item_caster() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ITEM_CASTER").unwrap();
            first = false;
        }
        if self.is_unk200() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK200").unwrap();
            first = false;
        }
        if self.is_extra_message() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "EXTRA_MESSAGE").unwrap();
            first = false;
        }
        if self.is_power_update() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "POWER_UPDATE").unwrap();
            first = false;
        }
        if self.is_unk2000() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK2000").unwrap();
            first = false;
        }
        if self.is_unk1000() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK1000").unwrap();
            first = false;
        }
        if self.is_unk8000() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK8000").unwrap();
            first = false;
        }
        if self.is_adjust_missile() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ADJUST_MISSILE").unwrap();
            first = false;
        }
        if self.is_unk40000() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK40000").unwrap();
            first = false;
        }
        if self.is_visual_chain() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "VISUAL_CHAIN").unwrap();
            first = false;
        }
        if self.is_rune_update() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "RUNE_UPDATE").unwrap();
            first = false;
        }
        if self.is_unk400000() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK400000").unwrap();
            first = false;
        }
        s
    }

}

impl GameobjectCastFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const LOCK_PLAYER_CAST_ANIM: u32 = 0x01;
    pub const UNKNOWN2: u32 = 0x02;
    pub const UNKNOWN4: u32 = 0x04;
    pub const UNKNOWN8: u32 = 0x08;
    pub const UNKNOWN16: u32 = 0x10;
    pub const AMMO: u32 = 0x20;
    pub const DEST_LOCATION: u32 = 0x40;
    pub const ITEM_CASTER: u32 = 0x100;
    pub const UNK200: u32 = 0x200;
    pub const EXTRA_MESSAGE: u32 = 0x400;
    pub const POWER_UPDATE: u32 = 0x800;
    pub const UNK2000: u32 = 0x2000;
    pub const UNK1000: u32 = 0x1000;
    pub const UNK8000: u32 = 0x8000;
    pub const ADJUST_MISSILE: u32 = 0x20000;
    pub const UNK40000: u32 = 0x40000;
    pub const VISUAL_CHAIN: u32 = 0x80000;
    pub const RUNE_UPDATE: u32 = 0x200000;
    pub const UNK400000: u32 = 0x400000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::LOCK_PLAYER_CAST_ANIM
                | Self::UNKNOWN2
                | Self::UNKNOWN4
                | Self::UNKNOWN8
                | Self::UNKNOWN16
                | Self::AMMO
                | Self::DEST_LOCATION
                | Self::ITEM_CASTER
                | Self::UNK200
                | Self::EXTRA_MESSAGE
                | Self::POWER_UPDATE
                | Self::UNK2000
                | Self::UNK1000
                | Self::UNK8000
                | Self::ADJUST_MISSILE
                | Self::UNK40000
                | Self::VISUAL_CHAIN
                | Self::RUNE_UPDATE
                | Self::UNK400000
        }
    }

    pub const fn is_lock_player_cast_anim(&self) -> bool {
        (self.inner & Self::LOCK_PLAYER_CAST_ANIM) != 0
    }

    /// also do not send standstate update
    pub const fn new_lock_player_cast_anim() -> Self {
        Self { inner: Self::LOCK_PLAYER_CAST_ANIM }
    }

    pub fn set_lock_player_cast_anim(&mut self) -> Self {
        self.inner |= Self::LOCK_PLAYER_CAST_ANIM;
        *self
    }

    pub fn clear_lock_player_cast_anim(&mut self) -> Self {
        self.inner &= Self::LOCK_PLAYER_CAST_ANIM.reverse_bits();
        *self
    }

    pub const fn is_unknown2(&self) -> bool {
        (self.inner & Self::UNKNOWN2) != 0
    }

    pub const fn new_unknown2() -> Self {
        Self { inner: Self::UNKNOWN2 }
    }

    pub fn set_unknown2(&mut self) -> Self {
        self.inner |= Self::UNKNOWN2;
        *self
    }

    pub fn clear_unknown2(&mut self) -> Self {
        self.inner &= Self::UNKNOWN2.reverse_bits();
        *self
    }

    pub const fn is_unknown4(&self) -> bool {
        (self.inner & Self::UNKNOWN4) != 0
    }

    pub const fn new_unknown4() -> Self {
        Self { inner: Self::UNKNOWN4 }
    }

    pub fn set_unknown4(&mut self) -> Self {
        self.inner |= Self::UNKNOWN4;
        *self
    }

    pub fn clear_unknown4(&mut self) -> Self {
        self.inner &= Self::UNKNOWN4.reverse_bits();
        *self
    }

    pub const fn is_unknown8(&self) -> bool {
        (self.inner & Self::UNKNOWN8) != 0
    }

    pub const fn new_unknown8() -> Self {
        Self { inner: Self::UNKNOWN8 }
    }

    pub fn set_unknown8(&mut self) -> Self {
        self.inner |= Self::UNKNOWN8;
        *self
    }

    pub fn clear_unknown8(&mut self) -> Self {
        self.inner &= Self::UNKNOWN8.reverse_bits();
        *self
    }

    pub const fn is_unknown16(&self) -> bool {
        (self.inner & Self::UNKNOWN16) != 0
    }

    pub const fn new_unknown16() -> Self {
        Self { inner: Self::UNKNOWN16 }
    }

    pub fn set_unknown16(&mut self) -> Self {
        self.inner |= Self::UNKNOWN16;
        *self
    }

    pub fn clear_unknown16(&mut self) -> Self {
        self.inner &= Self::UNKNOWN16.reverse_bits();
        *self
    }

    pub const fn is_ammo(&self) -> bool {
        (self.inner & Self::AMMO) != 0
    }

    /// 2 functions are called on 2 values
    pub const fn new_ammo() -> Self {
        Self { inner: Self::AMMO }
    }

    pub fn set_ammo(&mut self) -> Self {
        self.inner |= Self::AMMO;
        *self
    }

    pub fn clear_ammo(&mut self) -> Self {
        self.inner &= Self::AMMO.reverse_bits();
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

    pub const fn is_item_caster(&self) -> bool {
        (self.inner & Self::ITEM_CASTER) != 0
    }

    pub const fn new_item_caster() -> Self {
        Self { inner: Self::ITEM_CASTER }
    }

    pub fn set_item_caster(&mut self) -> Self {
        self.inner |= Self::ITEM_CASTER;
        *self
    }

    pub fn clear_item_caster(&mut self) -> Self {
        self.inner &= Self::ITEM_CASTER.reverse_bits();
        *self
    }

    pub const fn is_unk200(&self) -> bool {
        (self.inner & Self::UNK200) != 0
    }

    pub const fn new_unk200() -> Self {
        Self { inner: Self::UNK200 }
    }

    pub fn set_unk200(&mut self) -> Self {
        self.inner |= Self::UNK200;
        *self
    }

    pub fn clear_unk200(&mut self) -> Self {
        self.inner &= Self::UNK200.reverse_bits();
        *self
    }

    pub const fn is_extra_message(&self) -> bool {
        (self.inner & Self::EXTRA_MESSAGE) != 0
    }

    /// TARGET MISSES AND OTHER MESSAGES LIKE 'Resist'
    pub const fn new_extra_message() -> Self {
        Self { inner: Self::EXTRA_MESSAGE }
    }

    pub fn set_extra_message(&mut self) -> Self {
        self.inner |= Self::EXTRA_MESSAGE;
        *self
    }

    pub fn clear_extra_message(&mut self) -> Self {
        self.inner &= Self::EXTRA_MESSAGE.reverse_bits();
        *self
    }

    pub const fn is_power_update(&self) -> bool {
        (self.inner & Self::POWER_UPDATE) != 0
    }

    /// seems to work hand in hand with some visual effect of update actually
    pub const fn new_power_update() -> Self {
        Self { inner: Self::POWER_UPDATE }
    }

    pub fn set_power_update(&mut self) -> Self {
        self.inner |= Self::POWER_UPDATE;
        *self
    }

    pub fn clear_power_update(&mut self) -> Self {
        self.inner &= Self::POWER_UPDATE.reverse_bits();
        *self
    }

    pub const fn is_unk2000(&self) -> bool {
        (self.inner & Self::UNK2000) != 0
    }

    pub const fn new_unk2000() -> Self {
        Self { inner: Self::UNK2000 }
    }

    pub fn set_unk2000(&mut self) -> Self {
        self.inner |= Self::UNK2000;
        *self
    }

    pub fn clear_unk2000(&mut self) -> Self {
        self.inner &= Self::UNK2000.reverse_bits();
        *self
    }

    pub const fn is_unk1000(&self) -> bool {
        (self.inner & Self::UNK1000) != 0
    }

    /// no idea
    pub const fn new_unk1000() -> Self {
        Self { inner: Self::UNK1000 }
    }

    pub fn set_unk1000(&mut self) -> Self {
        self.inner |= Self::UNK1000;
        *self
    }

    pub fn clear_unk1000(&mut self) -> Self {
        self.inner &= Self::UNK1000.reverse_bits();
        *self
    }

    pub const fn is_unk8000(&self) -> bool {
        (self.inner & Self::UNK8000) != 0
    }

    /// seems to make server send extra 2 bytes before UNK1 and after UNK20000
    pub const fn new_unk8000() -> Self {
        Self { inner: Self::UNK8000 }
    }

    pub fn set_unk8000(&mut self) -> Self {
        self.inner |= Self::UNK8000;
        *self
    }

    pub fn clear_unk8000(&mut self) -> Self {
        self.inner &= Self::UNK8000.reverse_bits();
        *self
    }

    pub const fn is_adjust_missile(&self) -> bool {
        (self.inner & Self::ADJUST_MISSILE) != 0
    }

    /// seems to make server send an uint32 after `m_targets.write`
    pub const fn new_adjust_missile() -> Self {
        Self { inner: Self::ADJUST_MISSILE }
    }

    pub fn set_adjust_missile(&mut self) -> Self {
        self.inner |= Self::ADJUST_MISSILE;
        *self
    }

    pub fn clear_adjust_missile(&mut self) -> Self {
        self.inner &= Self::ADJUST_MISSILE.reverse_bits();
        *self
    }

    pub const fn is_unk40000(&self) -> bool {
        (self.inner & Self::UNK40000) != 0
    }

    /// 1 uint32. this is not confirmed but i have a feeling about it :D
    pub const fn new_unk40000() -> Self {
        Self { inner: Self::UNK40000 }
    }

    pub fn set_unk40000(&mut self) -> Self {
        self.inner |= Self::UNK40000;
        *self
    }

    pub fn clear_unk40000(&mut self) -> Self {
        self.inner &= Self::UNK40000.reverse_bits();
        *self
    }

    pub const fn is_visual_chain(&self) -> bool {
        (self.inner & Self::VISUAL_CHAIN) != 0
    }

    /// 2 functions called (same ones as for ranged but different)
    pub const fn new_visual_chain() -> Self {
        Self { inner: Self::VISUAL_CHAIN }
    }

    pub fn set_visual_chain(&mut self) -> Self {
        self.inner |= Self::VISUAL_CHAIN;
        *self
    }

    pub fn clear_visual_chain(&mut self) -> Self {
        self.inner &= Self::VISUAL_CHAIN.reverse_bits();
        *self
    }

    pub const fn is_rune_update(&self) -> bool {
        (self.inner & Self::RUNE_UPDATE) != 0
    }

    /// 2 bytes for the rune cur and rune next flags
    pub const fn new_rune_update() -> Self {
        Self { inner: Self::RUNE_UPDATE }
    }

    pub fn set_rune_update(&mut self) -> Self {
        self.inner |= Self::RUNE_UPDATE;
        *self
    }

    pub fn clear_rune_update(&mut self) -> Self {
        self.inner &= Self::RUNE_UPDATE.reverse_bits();
        *self
    }

    pub const fn is_unk400000(&self) -> bool {
        (self.inner & Self::UNK400000) != 0
    }

    /// seems to make server send an uint32 after `m_targets.write`
    pub const fn new_unk400000() -> Self {
        Self { inner: Self::UNK400000 }
    }

    pub fn set_unk400000(&mut self) -> Self {
        self.inner |= Self::UNK400000;
        *self
    }

    pub fn clear_unk400000(&mut self) -> Self {
        self.inner &= Self::UNK400000.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for GameobjectCastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for GameobjectCastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for GameobjectCastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for GameobjectCastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for GameobjectCastFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for GameobjectCastFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for GameobjectCastFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for GameobjectCastFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for GameobjectCastFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for GameobjectCastFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for GameobjectCastFlags {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for GameobjectCastFlags {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for GameobjectCastFlags {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for GameobjectCastFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for GameobjectCastFlags {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl TryFrom<i16> for GameobjectCastFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl From<i32> for GameobjectCastFlags {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for GameobjectCastFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for GameobjectCastFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

