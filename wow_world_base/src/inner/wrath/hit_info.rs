/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm#L3):
/// ```text
/// flag HitInfo : u32 {
///     NORMALSWING = 0x00000000;
///     UNK1 = 0x00000001;
///     AFFECTS_VICTIM = 0x00000002;
///     OFFHAND = 0x00000004;
///     UNK2 = 0x00000008;
///     MISS = 0x00000010;
///     FULL_ABSORB = 0x00000020;
///     PARTIAL_ABSORB = 0x00000040;
///     ALL_ABSORB = 0x00000060;
///     FULL_RESIST = 0x00000080;
///     PARTIAL_RESIST = 0x00000100;
///     ALL_RESIST = 0x00000180;
///     CRITICALHIT = 0x00000200;
///     UNK10 = 0x00000400;
///     UNK11 = 0x00000800;
///     UNK12 = 0x00001000;
///     BLOCK = 0x00002000;
///     UNK14 = 0x00004000;
///     UNK15 = 0x00008000;
///     GLANCING = 0x00010000;
///     CRUSHING = 0x00020000;
///     NO_ANIMATION = 0x00040000;
///     UNK19 = 0x00080000;
///     UNK20 = 0x00100000;
///     SWINGNOHITSOUND = 0x00200000;
///     UNK22 = 0x00400000;
///     RAGE_GAIN = 0x00800000;
///     FAKE_DAMAGE = 0x01000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct HitInfo {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl HitInfo {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NORMALSWING").unwrap();
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
        if self.is_affects_victim() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "AFFECTS_VICTIM").unwrap();
            first = false;
        }
        if self.is_offhand() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "OFFHAND").unwrap();
            first = false;
        }
        if self.is_unk2() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK2").unwrap();
            first = false;
        }
        if self.is_miss() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "MISS").unwrap();
            first = false;
        }
        if self.is_full_absorb() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FULL_ABSORB").unwrap();
            first = false;
        }
        if self.is_partial_absorb() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PARTIAL_ABSORB").unwrap();
            first = false;
        }
        if self.is_all_absorb() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ALL_ABSORB").unwrap();
            first = false;
        }
        if self.is_full_resist() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FULL_RESIST").unwrap();
            first = false;
        }
        if self.is_partial_resist() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PARTIAL_RESIST").unwrap();
            first = false;
        }
        if self.is_all_resist() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ALL_RESIST").unwrap();
            first = false;
        }
        if self.is_criticalhit() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CRITICALHIT").unwrap();
            first = false;
        }
        if self.is_unk10() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK10").unwrap();
            first = false;
        }
        if self.is_unk11() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK11").unwrap();
            first = false;
        }
        if self.is_unk12() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK12").unwrap();
            first = false;
        }
        if self.is_block() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "BLOCK").unwrap();
            first = false;
        }
        if self.is_unk14() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK14").unwrap();
            first = false;
        }
        if self.is_unk15() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK15").unwrap();
            first = false;
        }
        if self.is_glancing() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "GLANCING").unwrap();
            first = false;
        }
        if self.is_crushing() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CRUSHING").unwrap();
            first = false;
        }
        if self.is_no_animation() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_ANIMATION").unwrap();
            first = false;
        }
        if self.is_unk19() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK19").unwrap();
            first = false;
        }
        if self.is_unk20() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK20").unwrap();
            first = false;
        }
        if self.is_swingnohitsound() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SWINGNOHITSOUND").unwrap();
            first = false;
        }
        if self.is_unk22() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK22").unwrap();
            first = false;
        }
        if self.is_rage_gain() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "RAGE_GAIN").unwrap();
            first = false;
        }
        if self.is_fake_damage() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FAKE_DAMAGE").unwrap();
            first = false;
        }
        s
    }

}

impl HitInfo {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NORMALSWING: u32 = 0x00;
    pub const UNK1: u32 = 0x01;
    pub const AFFECTS_VICTIM: u32 = 0x02;
    pub const OFFHAND: u32 = 0x04;
    pub const UNK2: u32 = 0x08;
    pub const MISS: u32 = 0x10;
    pub const FULL_ABSORB: u32 = 0x20;
    pub const PARTIAL_ABSORB: u32 = 0x40;
    pub const ALL_ABSORB: u32 = 0x60;
    pub const FULL_RESIST: u32 = 0x80;
    pub const PARTIAL_RESIST: u32 = 0x100;
    pub const ALL_RESIST: u32 = 0x180;
    pub const CRITICALHIT: u32 = 0x200;
    pub const UNK10: u32 = 0x400;
    pub const UNK11: u32 = 0x800;
    pub const UNK12: u32 = 0x1000;
    pub const BLOCK: u32 = 0x2000;
    pub const UNK14: u32 = 0x4000;
    pub const UNK15: u32 = 0x8000;
    pub const GLANCING: u32 = 0x10000;
    pub const CRUSHING: u32 = 0x20000;
    pub const NO_ANIMATION: u32 = 0x40000;
    pub const UNK19: u32 = 0x80000;
    pub const UNK20: u32 = 0x100000;
    pub const SWINGNOHITSOUND: u32 = 0x200000;
    pub const UNK22: u32 = 0x400000;
    pub const RAGE_GAIN: u32 = 0x800000;
    pub const FAKE_DAMAGE: u32 = 0x1000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NORMALSWING
                | Self::UNK1
                | Self::AFFECTS_VICTIM
                | Self::OFFHAND
                | Self::UNK2
                | Self::MISS
                | Self::FULL_ABSORB
                | Self::PARTIAL_ABSORB
                | Self::ALL_ABSORB
                | Self::FULL_RESIST
                | Self::PARTIAL_RESIST
                | Self::ALL_RESIST
                | Self::CRITICALHIT
                | Self::UNK10
                | Self::UNK11
                | Self::UNK12
                | Self::BLOCK
                | Self::UNK14
                | Self::UNK15
                | Self::GLANCING
                | Self::CRUSHING
                | Self::NO_ANIMATION
                | Self::UNK19
                | Self::UNK20
                | Self::SWINGNOHITSOUND
                | Self::UNK22
                | Self::RAGE_GAIN
                | Self::FAKE_DAMAGE
        }
    }

    pub const fn is_unk1(&self) -> bool {
        (self.inner & Self::UNK1) != 0
    }

    /// req correct packet structure
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

    pub const fn is_affects_victim(&self) -> bool {
        (self.inner & Self::AFFECTS_VICTIM) != 0
    }

    pub const fn new_affects_victim() -> Self {
        Self { inner: Self::AFFECTS_VICTIM }
    }

    pub fn set_affects_victim(&mut self) -> Self {
        self.inner |= Self::AFFECTS_VICTIM;
        *self
    }

    pub fn clear_affects_victim(&mut self) -> Self {
        self.inner &= Self::AFFECTS_VICTIM.reverse_bits();
        *self
    }

    pub const fn is_offhand(&self) -> bool {
        (self.inner & Self::OFFHAND) != 0
    }

    pub const fn new_offhand() -> Self {
        Self { inner: Self::OFFHAND }
    }

    pub fn set_offhand(&mut self) -> Self {
        self.inner |= Self::OFFHAND;
        *self
    }

    pub fn clear_offhand(&mut self) -> Self {
        self.inner &= Self::OFFHAND.reverse_bits();
        *self
    }

    pub const fn is_unk2(&self) -> bool {
        (self.inner & Self::UNK2) != 0
    }

    pub const fn new_unk2() -> Self {
        Self { inner: Self::UNK2 }
    }

    pub fn set_unk2(&mut self) -> Self {
        self.inner |= Self::UNK2;
        *self
    }

    pub fn clear_unk2(&mut self) -> Self {
        self.inner &= Self::UNK2.reverse_bits();
        *self
    }

    pub const fn is_miss(&self) -> bool {
        (self.inner & Self::MISS) != 0
    }

    pub const fn new_miss() -> Self {
        Self { inner: Self::MISS }
    }

    pub fn set_miss(&mut self) -> Self {
        self.inner |= Self::MISS;
        *self
    }

    pub fn clear_miss(&mut self) -> Self {
        self.inner &= Self::MISS.reverse_bits();
        *self
    }

    pub const fn is_full_absorb(&self) -> bool {
        (self.inner & Self::FULL_ABSORB) != 0
    }

    pub const fn new_full_absorb() -> Self {
        Self { inner: Self::FULL_ABSORB }
    }

    pub fn set_full_absorb(&mut self) -> Self {
        self.inner |= Self::FULL_ABSORB;
        *self
    }

    pub fn clear_full_absorb(&mut self) -> Self {
        self.inner &= Self::FULL_ABSORB.reverse_bits();
        *self
    }

    pub const fn is_partial_absorb(&self) -> bool {
        (self.inner & Self::PARTIAL_ABSORB) != 0
    }

    pub const fn new_partial_absorb() -> Self {
        Self { inner: Self::PARTIAL_ABSORB }
    }

    pub fn set_partial_absorb(&mut self) -> Self {
        self.inner |= Self::PARTIAL_ABSORB;
        *self
    }

    pub fn clear_partial_absorb(&mut self) -> Self {
        self.inner &= Self::PARTIAL_ABSORB.reverse_bits();
        *self
    }

    pub const fn is_all_absorb(&self) -> bool {
        (self.inner & Self::ALL_ABSORB) != 0
    }

    pub const fn new_all_absorb() -> Self {
        Self { inner: Self::ALL_ABSORB }
    }

    pub fn set_all_absorb(&mut self) -> Self {
        self.inner |= Self::ALL_ABSORB;
        *self
    }

    pub fn clear_all_absorb(&mut self) -> Self {
        self.inner &= Self::ALL_ABSORB.reverse_bits();
        *self
    }

    pub const fn is_full_resist(&self) -> bool {
        (self.inner & Self::FULL_RESIST) != 0
    }

    pub const fn new_full_resist() -> Self {
        Self { inner: Self::FULL_RESIST }
    }

    pub fn set_full_resist(&mut self) -> Self {
        self.inner |= Self::FULL_RESIST;
        *self
    }

    pub fn clear_full_resist(&mut self) -> Self {
        self.inner &= Self::FULL_RESIST.reverse_bits();
        *self
    }

    pub const fn is_partial_resist(&self) -> bool {
        (self.inner & Self::PARTIAL_RESIST) != 0
    }

    pub const fn new_partial_resist() -> Self {
        Self { inner: Self::PARTIAL_RESIST }
    }

    pub fn set_partial_resist(&mut self) -> Self {
        self.inner |= Self::PARTIAL_RESIST;
        *self
    }

    pub fn clear_partial_resist(&mut self) -> Self {
        self.inner &= Self::PARTIAL_RESIST.reverse_bits();
        *self
    }

    pub const fn is_all_resist(&self) -> bool {
        (self.inner & Self::ALL_RESIST) != 0
    }

    pub const fn new_all_resist() -> Self {
        Self { inner: Self::ALL_RESIST }
    }

    pub fn set_all_resist(&mut self) -> Self {
        self.inner |= Self::ALL_RESIST;
        *self
    }

    pub fn clear_all_resist(&mut self) -> Self {
        self.inner &= Self::ALL_RESIST.reverse_bits();
        *self
    }

    pub const fn is_criticalhit(&self) -> bool {
        (self.inner & Self::CRITICALHIT) != 0
    }

    /// critical hit
    pub const fn new_criticalhit() -> Self {
        Self { inner: Self::CRITICALHIT }
    }

    pub fn set_criticalhit(&mut self) -> Self {
        self.inner |= Self::CRITICALHIT;
        *self
    }

    pub fn clear_criticalhit(&mut self) -> Self {
        self.inner &= Self::CRITICALHIT.reverse_bits();
        *self
    }

    pub const fn is_unk10(&self) -> bool {
        (self.inner & Self::UNK10) != 0
    }

    pub const fn new_unk10() -> Self {
        Self { inner: Self::UNK10 }
    }

    pub fn set_unk10(&mut self) -> Self {
        self.inner |= Self::UNK10;
        *self
    }

    pub fn clear_unk10(&mut self) -> Self {
        self.inner &= Self::UNK10.reverse_bits();
        *self
    }

    pub const fn is_unk11(&self) -> bool {
        (self.inner & Self::UNK11) != 0
    }

    pub const fn new_unk11() -> Self {
        Self { inner: Self::UNK11 }
    }

    pub fn set_unk11(&mut self) -> Self {
        self.inner |= Self::UNK11;
        *self
    }

    pub fn clear_unk11(&mut self) -> Self {
        self.inner &= Self::UNK11.reverse_bits();
        *self
    }

    pub const fn is_unk12(&self) -> bool {
        (self.inner & Self::UNK12) != 0
    }

    pub const fn new_unk12() -> Self {
        Self { inner: Self::UNK12 }
    }

    pub fn set_unk12(&mut self) -> Self {
        self.inner |= Self::UNK12;
        *self
    }

    pub fn clear_unk12(&mut self) -> Self {
        self.inner &= Self::UNK12.reverse_bits();
        *self
    }

    pub const fn is_block(&self) -> bool {
        (self.inner & Self::BLOCK) != 0
    }

    /// blocked damage
    pub const fn new_block() -> Self {
        Self { inner: Self::BLOCK }
    }

    pub fn set_block(&mut self) -> Self {
        self.inner |= Self::BLOCK;
        *self
    }

    pub fn clear_block(&mut self) -> Self {
        self.inner &= Self::BLOCK.reverse_bits();
        *self
    }

    pub const fn is_unk14(&self) -> bool {
        (self.inner & Self::UNK14) != 0
    }

    /// set only if meleespellid is present. no world text when victim is hit for 0 dmg(HideWorldTextForNoDamage?)
    pub const fn new_unk14() -> Self {
        Self { inner: Self::UNK14 }
    }

    pub fn set_unk14(&mut self) -> Self {
        self.inner |= Self::UNK14;
        *self
    }

    pub fn clear_unk14(&mut self) -> Self {
        self.inner &= Self::UNK14.reverse_bits();
        *self
    }

    pub const fn is_unk15(&self) -> bool {
        (self.inner & Self::UNK15) != 0
    }

    /// player victim? something related to blod sprut visual (`BloodSpurtInBack`?)
    pub const fn new_unk15() -> Self {
        Self { inner: Self::UNK15 }
    }

    pub fn set_unk15(&mut self) -> Self {
        self.inner |= Self::UNK15;
        *self
    }

    pub fn clear_unk15(&mut self) -> Self {
        self.inner &= Self::UNK15.reverse_bits();
        *self
    }

    pub const fn is_glancing(&self) -> bool {
        (self.inner & Self::GLANCING) != 0
    }

    pub const fn new_glancing() -> Self {
        Self { inner: Self::GLANCING }
    }

    pub fn set_glancing(&mut self) -> Self {
        self.inner |= Self::GLANCING;
        *self
    }

    pub fn clear_glancing(&mut self) -> Self {
        self.inner &= Self::GLANCING.reverse_bits();
        *self
    }

    pub const fn is_crushing(&self) -> bool {
        (self.inner & Self::CRUSHING) != 0
    }

    pub const fn new_crushing() -> Self {
        Self { inner: Self::CRUSHING }
    }

    pub fn set_crushing(&mut self) -> Self {
        self.inner |= Self::CRUSHING;
        *self
    }

    pub fn clear_crushing(&mut self) -> Self {
        self.inner &= Self::CRUSHING.reverse_bits();
        *self
    }

    pub const fn is_no_animation(&self) -> bool {
        (self.inner & Self::NO_ANIMATION) != 0
    }

    pub const fn new_no_animation() -> Self {
        Self { inner: Self::NO_ANIMATION }
    }

    pub fn set_no_animation(&mut self) -> Self {
        self.inner |= Self::NO_ANIMATION;
        *self
    }

    pub fn clear_no_animation(&mut self) -> Self {
        self.inner &= Self::NO_ANIMATION.reverse_bits();
        *self
    }

    pub const fn is_unk19(&self) -> bool {
        (self.inner & Self::UNK19) != 0
    }

    pub const fn new_unk19() -> Self {
        Self { inner: Self::UNK19 }
    }

    pub fn set_unk19(&mut self) -> Self {
        self.inner |= Self::UNK19;
        *self
    }

    pub fn clear_unk19(&mut self) -> Self {
        self.inner &= Self::UNK19.reverse_bits();
        *self
    }

    pub const fn is_unk20(&self) -> bool {
        (self.inner & Self::UNK20) != 0
    }

    pub const fn new_unk20() -> Self {
        Self { inner: Self::UNK20 }
    }

    pub fn set_unk20(&mut self) -> Self {
        self.inner |= Self::UNK20;
        *self
    }

    pub fn clear_unk20(&mut self) -> Self {
        self.inner &= Self::UNK20.reverse_bits();
        *self
    }

    pub const fn is_swingnohitsound(&self) -> bool {
        (self.inner & Self::SWINGNOHITSOUND) != 0
    }

    /// unused?
    pub const fn new_swingnohitsound() -> Self {
        Self { inner: Self::SWINGNOHITSOUND }
    }

    pub fn set_swingnohitsound(&mut self) -> Self {
        self.inner |= Self::SWINGNOHITSOUND;
        *self
    }

    pub fn clear_swingnohitsound(&mut self) -> Self {
        self.inner &= Self::SWINGNOHITSOUND.reverse_bits();
        *self
    }

    pub const fn is_unk22(&self) -> bool {
        (self.inner & Self::UNK22) != 0
    }

    pub const fn new_unk22() -> Self {
        Self { inner: Self::UNK22 }
    }

    pub fn set_unk22(&mut self) -> Self {
        self.inner |= Self::UNK22;
        *self
    }

    pub fn clear_unk22(&mut self) -> Self {
        self.inner &= Self::UNK22.reverse_bits();
        *self
    }

    pub const fn is_rage_gain(&self) -> bool {
        (self.inner & Self::RAGE_GAIN) != 0
    }

    pub const fn new_rage_gain() -> Self {
        Self { inner: Self::RAGE_GAIN }
    }

    pub fn set_rage_gain(&mut self) -> Self {
        self.inner |= Self::RAGE_GAIN;
        *self
    }

    pub fn clear_rage_gain(&mut self) -> Self {
        self.inner &= Self::RAGE_GAIN.reverse_bits();
        *self
    }

    pub const fn is_fake_damage(&self) -> bool {
        (self.inner & Self::FAKE_DAMAGE) != 0
    }

    /// enables damage animation even if no damage done. set only if no damage
    pub const fn new_fake_damage() -> Self {
        Self { inner: Self::FAKE_DAMAGE }
    }

    pub fn set_fake_damage(&mut self) -> Self {
        self.inner |= Self::FAKE_DAMAGE;
        *self
    }

    pub fn clear_fake_damage(&mut self) -> Self {
        self.inner &= Self::FAKE_DAMAGE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for HitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for HitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for HitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for HitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for HitInfo {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for HitInfo {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for HitInfo {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for HitInfo {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for HitInfo {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for HitInfo {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for HitInfo {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for HitInfo {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for HitInfo {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for HitInfo {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for HitInfo {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for HitInfo {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for HitInfo {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for HitInfo {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for HitInfo {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

