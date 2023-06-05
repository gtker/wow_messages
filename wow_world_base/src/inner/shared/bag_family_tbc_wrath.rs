/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:275`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L275):
/// ```text
/// flag BagFamily : u32 {
///     NONE = 0x00000000;
///     ARROWS = 0x00000001;
///     BULLETS = 0x00000002;
///     SOUL_SHARDS = 0x00000004;
///     LEATHERWORKING_SUPPLIES = 0x00000008;
///     INSCRIPTION_SUPPLIES = 0x00000010;
///     HERBS = 0x00000020;
///     ENCHANTING_SUPPLIES = 0x00000040;
///     ENGINEERING_SUPPLIES = 0x00000080;
///     KEYS = 0x00000100;
///     GEMS = 0x00000200;
///     MINING_SUPPLIES = 0x00000400;
///     SOULBOUND_EQUIPMENT = 0x00000800;
///     VANITY_PETS = 0x00001000;
///     CURRENCY_TOKENS = 0x00002000;
///     QUEST_ITEMS = 0x00004000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BagFamily {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl BagFamily {
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_arrows() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ARROWS").unwrap();
            first = false;
        }
        if self.is_bullets() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "BULLETS").unwrap();
            first = false;
        }
        if self.is_soul_shards() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "SOUL_SHARDS").unwrap();
            first = false;
        }
        if self.is_leatherworking_supplies() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "LEATHERWORKING_SUPPLIES").unwrap();
            first = false;
        }
        if self.is_inscription_supplies() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "INSCRIPTION_SUPPLIES").unwrap();
            first = false;
        }
        if self.is_herbs() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HERBS").unwrap();
            first = false;
        }
        if self.is_enchanting_supplies() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ENCHANTING_SUPPLIES").unwrap();
            first = false;
        }
        if self.is_engineering_supplies() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ENGINEERING_SUPPLIES").unwrap();
            first = false;
        }
        if self.is_keys() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "KEYS").unwrap();
            first = false;
        }
        if self.is_gems() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "GEMS").unwrap();
            first = false;
        }
        if self.is_mining_supplies() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MINING_SUPPLIES").unwrap();
            first = false;
        }
        if self.is_soulbound_equipment() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "SOULBOUND_EQUIPMENT").unwrap();
            first = false;
        }
        if self.is_vanity_pets() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "VANITY_PETS").unwrap();
            first = false;
        }
        if self.is_currency_tokens() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CURRENCY_TOKENS").unwrap();
            first = false;
        }
        if self.is_quest_items() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "QUEST_ITEMS").unwrap();
            first = false;
        }
        s
    }

}

impl BagFamily {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const ARROWS: u32 = 0x01;
    pub const BULLETS: u32 = 0x02;
    pub const SOUL_SHARDS: u32 = 0x04;
    pub const LEATHERWORKING_SUPPLIES: u32 = 0x08;
    pub const INSCRIPTION_SUPPLIES: u32 = 0x10;
    pub const HERBS: u32 = 0x20;
    pub const ENCHANTING_SUPPLIES: u32 = 0x40;
    pub const ENGINEERING_SUPPLIES: u32 = 0x80;
    pub const KEYS: u32 = 0x100;
    pub const GEMS: u32 = 0x200;
    pub const MINING_SUPPLIES: u32 = 0x400;
    pub const SOULBOUND_EQUIPMENT: u32 = 0x800;
    pub const VANITY_PETS: u32 = 0x1000;
    pub const CURRENCY_TOKENS: u32 = 0x2000;
    pub const QUEST_ITEMS: u32 = 0x4000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::ARROWS
                | Self::BULLETS
                | Self::SOUL_SHARDS
                | Self::LEATHERWORKING_SUPPLIES
                | Self::INSCRIPTION_SUPPLIES
                | Self::HERBS
                | Self::ENCHANTING_SUPPLIES
                | Self::ENGINEERING_SUPPLIES
                | Self::KEYS
                | Self::GEMS
                | Self::MINING_SUPPLIES
                | Self::SOULBOUND_EQUIPMENT
                | Self::VANITY_PETS
                | Self::CURRENCY_TOKENS
                | Self::QUEST_ITEMS
        }
    }

    pub const fn is_arrows(&self) -> bool {
        (self.inner & Self::ARROWS) != 0
    }

    pub const fn new_arrows() -> Self {
        Self { inner: Self::ARROWS }
    }

    pub fn set_arrows(&mut self) -> Self {
        self.inner |= Self::ARROWS;
        *self
    }

    pub fn clear_arrows(&mut self) -> Self {
        self.inner &= Self::ARROWS.reverse_bits();
        *self
    }

    pub const fn is_bullets(&self) -> bool {
        (self.inner & Self::BULLETS) != 0
    }

    pub const fn new_bullets() -> Self {
        Self { inner: Self::BULLETS }
    }

    pub fn set_bullets(&mut self) -> Self {
        self.inner |= Self::BULLETS;
        *self
    }

    pub fn clear_bullets(&mut self) -> Self {
        self.inner &= Self::BULLETS.reverse_bits();
        *self
    }

    pub const fn is_soul_shards(&self) -> bool {
        (self.inner & Self::SOUL_SHARDS) != 0
    }

    pub const fn new_soul_shards() -> Self {
        Self { inner: Self::SOUL_SHARDS }
    }

    pub fn set_soul_shards(&mut self) -> Self {
        self.inner |= Self::SOUL_SHARDS;
        *self
    }

    pub fn clear_soul_shards(&mut self) -> Self {
        self.inner &= Self::SOUL_SHARDS.reverse_bits();
        *self
    }

    pub const fn is_leatherworking_supplies(&self) -> bool {
        (self.inner & Self::LEATHERWORKING_SUPPLIES) != 0
    }

    pub const fn new_leatherworking_supplies() -> Self {
        Self { inner: Self::LEATHERWORKING_SUPPLIES }
    }

    pub fn set_leatherworking_supplies(&mut self) -> Self {
        self.inner |= Self::LEATHERWORKING_SUPPLIES;
        *self
    }

    pub fn clear_leatherworking_supplies(&mut self) -> Self {
        self.inner &= Self::LEATHERWORKING_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_inscription_supplies(&self) -> bool {
        (self.inner & Self::INSCRIPTION_SUPPLIES) != 0
    }

    pub const fn new_inscription_supplies() -> Self {
        Self { inner: Self::INSCRIPTION_SUPPLIES }
    }

    pub fn set_inscription_supplies(&mut self) -> Self {
        self.inner |= Self::INSCRIPTION_SUPPLIES;
        *self
    }

    pub fn clear_inscription_supplies(&mut self) -> Self {
        self.inner &= Self::INSCRIPTION_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_herbs(&self) -> bool {
        (self.inner & Self::HERBS) != 0
    }

    pub const fn new_herbs() -> Self {
        Self { inner: Self::HERBS }
    }

    pub fn set_herbs(&mut self) -> Self {
        self.inner |= Self::HERBS;
        *self
    }

    pub fn clear_herbs(&mut self) -> Self {
        self.inner &= Self::HERBS.reverse_bits();
        *self
    }

    pub const fn is_enchanting_supplies(&self) -> bool {
        (self.inner & Self::ENCHANTING_SUPPLIES) != 0
    }

    pub const fn new_enchanting_supplies() -> Self {
        Self { inner: Self::ENCHANTING_SUPPLIES }
    }

    pub fn set_enchanting_supplies(&mut self) -> Self {
        self.inner |= Self::ENCHANTING_SUPPLIES;
        *self
    }

    pub fn clear_enchanting_supplies(&mut self) -> Self {
        self.inner &= Self::ENCHANTING_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_engineering_supplies(&self) -> bool {
        (self.inner & Self::ENGINEERING_SUPPLIES) != 0
    }

    pub const fn new_engineering_supplies() -> Self {
        Self { inner: Self::ENGINEERING_SUPPLIES }
    }

    pub fn set_engineering_supplies(&mut self) -> Self {
        self.inner |= Self::ENGINEERING_SUPPLIES;
        *self
    }

    pub fn clear_engineering_supplies(&mut self) -> Self {
        self.inner &= Self::ENGINEERING_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_keys(&self) -> bool {
        (self.inner & Self::KEYS) != 0
    }

    pub const fn new_keys() -> Self {
        Self { inner: Self::KEYS }
    }

    pub fn set_keys(&mut self) -> Self {
        self.inner |= Self::KEYS;
        *self
    }

    pub fn clear_keys(&mut self) -> Self {
        self.inner &= Self::KEYS.reverse_bits();
        *self
    }

    pub const fn is_gems(&self) -> bool {
        (self.inner & Self::GEMS) != 0
    }

    pub const fn new_gems() -> Self {
        Self { inner: Self::GEMS }
    }

    pub fn set_gems(&mut self) -> Self {
        self.inner |= Self::GEMS;
        *self
    }

    pub fn clear_gems(&mut self) -> Self {
        self.inner &= Self::GEMS.reverse_bits();
        *self
    }

    pub const fn is_mining_supplies(&self) -> bool {
        (self.inner & Self::MINING_SUPPLIES) != 0
    }

    pub const fn new_mining_supplies() -> Self {
        Self { inner: Self::MINING_SUPPLIES }
    }

    pub fn set_mining_supplies(&mut self) -> Self {
        self.inner |= Self::MINING_SUPPLIES;
        *self
    }

    pub fn clear_mining_supplies(&mut self) -> Self {
        self.inner &= Self::MINING_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_soulbound_equipment(&self) -> bool {
        (self.inner & Self::SOULBOUND_EQUIPMENT) != 0
    }

    pub const fn new_soulbound_equipment() -> Self {
        Self { inner: Self::SOULBOUND_EQUIPMENT }
    }

    pub fn set_soulbound_equipment(&mut self) -> Self {
        self.inner |= Self::SOULBOUND_EQUIPMENT;
        *self
    }

    pub fn clear_soulbound_equipment(&mut self) -> Self {
        self.inner &= Self::SOULBOUND_EQUIPMENT.reverse_bits();
        *self
    }

    pub const fn is_vanity_pets(&self) -> bool {
        (self.inner & Self::VANITY_PETS) != 0
    }

    pub const fn new_vanity_pets() -> Self {
        Self { inner: Self::VANITY_PETS }
    }

    pub fn set_vanity_pets(&mut self) -> Self {
        self.inner |= Self::VANITY_PETS;
        *self
    }

    pub fn clear_vanity_pets(&mut self) -> Self {
        self.inner &= Self::VANITY_PETS.reverse_bits();
        *self
    }

    pub const fn is_currency_tokens(&self) -> bool {
        (self.inner & Self::CURRENCY_TOKENS) != 0
    }

    pub const fn new_currency_tokens() -> Self {
        Self { inner: Self::CURRENCY_TOKENS }
    }

    pub fn set_currency_tokens(&mut self) -> Self {
        self.inner |= Self::CURRENCY_TOKENS;
        *self
    }

    pub fn clear_currency_tokens(&mut self) -> Self {
        self.inner &= Self::CURRENCY_TOKENS.reverse_bits();
        *self
    }

    pub const fn is_quest_items(&self) -> bool {
        (self.inner & Self::QUEST_ITEMS) != 0
    }

    pub const fn new_quest_items() -> Self {
        Self { inner: Self::QUEST_ITEMS }
    }

    pub fn set_quest_items(&mut self) -> Self {
        self.inner |= Self::QUEST_ITEMS;
        *self
    }

    pub fn clear_quest_items(&mut self) -> Self {
        self.inner &= Self::QUEST_ITEMS.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for BagFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for BagFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for BagFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for BagFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for BagFamily {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for BagFamily {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for BagFamily {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for BagFamily {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for BagFamily {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for BagFamily {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

