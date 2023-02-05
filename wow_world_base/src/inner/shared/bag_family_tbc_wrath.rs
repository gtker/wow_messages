/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:277`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L277):
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

impl BagFamily {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u32 = 0x00;
    pub(crate) const ARROWS: u32 = 0x01;
    pub(crate) const BULLETS: u32 = 0x02;
    pub(crate) const SOUL_SHARDS: u32 = 0x04;
    pub(crate) const LEATHERWORKING_SUPPLIES: u32 = 0x08;
    pub(crate) const INSCRIPTION_SUPPLIES: u32 = 0x10;
    pub(crate) const HERBS: u32 = 0x20;
    pub(crate) const ENCHANTING_SUPPLIES: u32 = 0x40;
    pub(crate) const ENGINEERING_SUPPLIES: u32 = 0x80;
    pub(crate) const KEYS: u32 = 0x100;
    pub(crate) const GEMS: u32 = 0x200;
    pub(crate) const MINING_SUPPLIES: u32 = 0x400;
    pub(crate) const SOULBOUND_EQUIPMENT: u32 = 0x800;
    pub(crate) const VANITY_PETS: u32 = 0x1000;
    pub(crate) const CURRENCY_TOKENS: u32 = 0x2000;
    pub(crate) const QUEST_ITEMS: u32 = 0x4000;

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

    pub const fn is_ARROWS(&self) -> bool {
        (self.inner & Self::ARROWS) != 0
    }

    pub const fn new_ARROWS() -> Self {
        Self { inner: Self::ARROWS }
    }

    pub fn set_ARROWS(&mut self) -> Self {
        self.inner |= Self::ARROWS;
        *self
    }

    pub fn clear_ARROWS(&mut self) -> Self {
        self.inner &= Self::ARROWS.reverse_bits();
        *self
    }

    pub const fn is_BULLETS(&self) -> bool {
        (self.inner & Self::BULLETS) != 0
    }

    pub const fn new_BULLETS() -> Self {
        Self { inner: Self::BULLETS }
    }

    pub fn set_BULLETS(&mut self) -> Self {
        self.inner |= Self::BULLETS;
        *self
    }

    pub fn clear_BULLETS(&mut self) -> Self {
        self.inner &= Self::BULLETS.reverse_bits();
        *self
    }

    pub const fn is_SOUL_SHARDS(&self) -> bool {
        (self.inner & Self::SOUL_SHARDS) != 0
    }

    pub const fn new_SOUL_SHARDS() -> Self {
        Self { inner: Self::SOUL_SHARDS }
    }

    pub fn set_SOUL_SHARDS(&mut self) -> Self {
        self.inner |= Self::SOUL_SHARDS;
        *self
    }

    pub fn clear_SOUL_SHARDS(&mut self) -> Self {
        self.inner &= Self::SOUL_SHARDS.reverse_bits();
        *self
    }

    pub const fn is_LEATHERWORKING_SUPPLIES(&self) -> bool {
        (self.inner & Self::LEATHERWORKING_SUPPLIES) != 0
    }

    pub const fn new_LEATHERWORKING_SUPPLIES() -> Self {
        Self { inner: Self::LEATHERWORKING_SUPPLIES }
    }

    pub fn set_LEATHERWORKING_SUPPLIES(&mut self) -> Self {
        self.inner |= Self::LEATHERWORKING_SUPPLIES;
        *self
    }

    pub fn clear_LEATHERWORKING_SUPPLIES(&mut self) -> Self {
        self.inner &= Self::LEATHERWORKING_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_INSCRIPTION_SUPPLIES(&self) -> bool {
        (self.inner & Self::INSCRIPTION_SUPPLIES) != 0
    }

    pub const fn new_INSCRIPTION_SUPPLIES() -> Self {
        Self { inner: Self::INSCRIPTION_SUPPLIES }
    }

    pub fn set_INSCRIPTION_SUPPLIES(&mut self) -> Self {
        self.inner |= Self::INSCRIPTION_SUPPLIES;
        *self
    }

    pub fn clear_INSCRIPTION_SUPPLIES(&mut self) -> Self {
        self.inner &= Self::INSCRIPTION_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_HERBS(&self) -> bool {
        (self.inner & Self::HERBS) != 0
    }

    pub const fn new_HERBS() -> Self {
        Self { inner: Self::HERBS }
    }

    pub fn set_HERBS(&mut self) -> Self {
        self.inner |= Self::HERBS;
        *self
    }

    pub fn clear_HERBS(&mut self) -> Self {
        self.inner &= Self::HERBS.reverse_bits();
        *self
    }

    pub const fn is_ENCHANTING_SUPPLIES(&self) -> bool {
        (self.inner & Self::ENCHANTING_SUPPLIES) != 0
    }

    pub const fn new_ENCHANTING_SUPPLIES() -> Self {
        Self { inner: Self::ENCHANTING_SUPPLIES }
    }

    pub fn set_ENCHANTING_SUPPLIES(&mut self) -> Self {
        self.inner |= Self::ENCHANTING_SUPPLIES;
        *self
    }

    pub fn clear_ENCHANTING_SUPPLIES(&mut self) -> Self {
        self.inner &= Self::ENCHANTING_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_ENGINEERING_SUPPLIES(&self) -> bool {
        (self.inner & Self::ENGINEERING_SUPPLIES) != 0
    }

    pub const fn new_ENGINEERING_SUPPLIES() -> Self {
        Self { inner: Self::ENGINEERING_SUPPLIES }
    }

    pub fn set_ENGINEERING_SUPPLIES(&mut self) -> Self {
        self.inner |= Self::ENGINEERING_SUPPLIES;
        *self
    }

    pub fn clear_ENGINEERING_SUPPLIES(&mut self) -> Self {
        self.inner &= Self::ENGINEERING_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_KEYS(&self) -> bool {
        (self.inner & Self::KEYS) != 0
    }

    pub const fn new_KEYS() -> Self {
        Self { inner: Self::KEYS }
    }

    pub fn set_KEYS(&mut self) -> Self {
        self.inner |= Self::KEYS;
        *self
    }

    pub fn clear_KEYS(&mut self) -> Self {
        self.inner &= Self::KEYS.reverse_bits();
        *self
    }

    pub const fn is_GEMS(&self) -> bool {
        (self.inner & Self::GEMS) != 0
    }

    pub const fn new_GEMS() -> Self {
        Self { inner: Self::GEMS }
    }

    pub fn set_GEMS(&mut self) -> Self {
        self.inner |= Self::GEMS;
        *self
    }

    pub fn clear_GEMS(&mut self) -> Self {
        self.inner &= Self::GEMS.reverse_bits();
        *self
    }

    pub const fn is_MINING_SUPPLIES(&self) -> bool {
        (self.inner & Self::MINING_SUPPLIES) != 0
    }

    pub const fn new_MINING_SUPPLIES() -> Self {
        Self { inner: Self::MINING_SUPPLIES }
    }

    pub fn set_MINING_SUPPLIES(&mut self) -> Self {
        self.inner |= Self::MINING_SUPPLIES;
        *self
    }

    pub fn clear_MINING_SUPPLIES(&mut self) -> Self {
        self.inner &= Self::MINING_SUPPLIES.reverse_bits();
        *self
    }

    pub const fn is_SOULBOUND_EQUIPMENT(&self) -> bool {
        (self.inner & Self::SOULBOUND_EQUIPMENT) != 0
    }

    pub const fn new_SOULBOUND_EQUIPMENT() -> Self {
        Self { inner: Self::SOULBOUND_EQUIPMENT }
    }

    pub fn set_SOULBOUND_EQUIPMENT(&mut self) -> Self {
        self.inner |= Self::SOULBOUND_EQUIPMENT;
        *self
    }

    pub fn clear_SOULBOUND_EQUIPMENT(&mut self) -> Self {
        self.inner &= Self::SOULBOUND_EQUIPMENT.reverse_bits();
        *self
    }

    pub const fn is_VANITY_PETS(&self) -> bool {
        (self.inner & Self::VANITY_PETS) != 0
    }

    pub const fn new_VANITY_PETS() -> Self {
        Self { inner: Self::VANITY_PETS }
    }

    pub fn set_VANITY_PETS(&mut self) -> Self {
        self.inner |= Self::VANITY_PETS;
        *self
    }

    pub fn clear_VANITY_PETS(&mut self) -> Self {
        self.inner &= Self::VANITY_PETS.reverse_bits();
        *self
    }

    pub const fn is_CURRENCY_TOKENS(&self) -> bool {
        (self.inner & Self::CURRENCY_TOKENS) != 0
    }

    pub const fn new_CURRENCY_TOKENS() -> Self {
        Self { inner: Self::CURRENCY_TOKENS }
    }

    pub fn set_CURRENCY_TOKENS(&mut self) -> Self {
        self.inner |= Self::CURRENCY_TOKENS;
        *self
    }

    pub fn clear_CURRENCY_TOKENS(&mut self) -> Self {
        self.inner &= Self::CURRENCY_TOKENS.reverse_bits();
        *self
    }

    pub const fn is_QUEST_ITEMS(&self) -> bool {
        (self.inner & Self::QUEST_ITEMS) != 0
    }

    pub const fn new_QUEST_ITEMS() -> Self {
        Self { inner: Self::QUEST_ITEMS }
    }

    pub fn set_QUEST_ITEMS(&mut self) -> Self {
        self.inner |= Self::QUEST_ITEMS;
        *self
    }

    pub fn clear_QUEST_ITEMS(&mut self) -> Self {
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

