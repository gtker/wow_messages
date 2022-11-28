/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:56`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L56):
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
pub struct SpellCastTargetFlags {
    inner: u32,
}

impl SpellCastTargetFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const SELF: u32 = 0x00;
    pub(crate) const UNUSED1: u32 = 0x01;
    pub(crate) const UNIT: u32 = 0x02;
    pub(crate) const UNIT_RAID: u32 = 0x04;
    pub(crate) const UNIT_PARTY: u32 = 0x08;
    pub(crate) const ITEM: u32 = 0x10;
    pub(crate) const SOURCE_LOCATION: u32 = 0x20;
    pub(crate) const DEST_LOCATION: u32 = 0x40;
    pub(crate) const UNIT_ENEMY: u32 = 0x80;
    pub(crate) const UNIT_ALLY: u32 = 0x100;
    pub(crate) const CORPSE_ENEMY: u32 = 0x200;
    pub(crate) const UNIT_DEAD: u32 = 0x400;
    pub(crate) const GAMEOBJECT: u32 = 0x800;
    pub(crate) const TRADE_ITEM: u32 = 0x1000;
    pub(crate) const STRING: u32 = 0x2000;
    pub(crate) const LOCKED: u32 = 0x4000;
    pub(crate) const CORPSE_ALLY: u32 = 0x8000;
    pub(crate) const UNIT_MINIPET: u32 = 0x10000;
    pub(crate) const GLYPH_SLOT: u32 = 0x20000;
    pub(crate) const DEST_TARGET: u32 = 0x40000;
    pub(crate) const UNUSED20: u32 = 0x80000;
    pub(crate) const UNIT_PASSENGER: u32 = 0x100000;

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

    pub const fn is_UNUSED1(&self) -> bool {
        (self.inner & Self::UNUSED1) != 0
    }

    /// not used in any spells as of 2.4.3 (can be set dynamically)
    ///
    pub const fn new_UNUSED1() -> Self {
        Self { inner: Self::UNUSED1 }
    }

    pub fn set_UNUSED1(&mut self) -> Self {
        self.inner |= Self::UNUSED1;
        *self
    }

    pub fn clear_UNUSED1(&mut self) -> Self {
        self.inner &= Self::UNUSED1.reverse_bits();
        *self
    }

    pub const fn is_UNIT(&self) -> bool {
        (self.inner & Self::UNIT) != 0
    }

    /// pguid
    ///
    pub const fn new_UNIT() -> Self {
        Self { inner: Self::UNIT }
    }

    pub fn set_UNIT(&mut self) -> Self {
        self.inner |= Self::UNIT;
        *self
    }

    pub fn clear_UNIT(&mut self) -> Self {
        self.inner &= Self::UNIT.reverse_bits();
        *self
    }

    pub const fn is_UNIT_RAID(&self) -> bool {
        (self.inner & Self::UNIT_RAID) != 0
    }

    /// not used in any spells as of 2.4.3 (can be set dynamically) - raid member
    ///
    pub const fn new_UNIT_RAID() -> Self {
        Self { inner: Self::UNIT_RAID }
    }

    pub fn set_UNIT_RAID(&mut self) -> Self {
        self.inner |= Self::UNIT_RAID;
        *self
    }

    pub fn clear_UNIT_RAID(&mut self) -> Self {
        self.inner &= Self::UNIT_RAID.reverse_bits();
        *self
    }

    pub const fn is_UNIT_PARTY(&self) -> bool {
        (self.inner & Self::UNIT_PARTY) != 0
    }

    /// not used in any spells as of 2.4.3 (can be set dynamically) - party member
    ///
    pub const fn new_UNIT_PARTY() -> Self {
        Self { inner: Self::UNIT_PARTY }
    }

    pub fn set_UNIT_PARTY(&mut self) -> Self {
        self.inner |= Self::UNIT_PARTY;
        *self
    }

    pub fn clear_UNIT_PARTY(&mut self) -> Self {
        self.inner &= Self::UNIT_PARTY.reverse_bits();
        *self
    }

    pub const fn is_ITEM(&self) -> bool {
        (self.inner & Self::ITEM) != 0
    }

    /// pguid
    ///
    pub const fn new_ITEM() -> Self {
        Self { inner: Self::ITEM }
    }

    pub fn set_ITEM(&mut self) -> Self {
        self.inner |= Self::ITEM;
        *self
    }

    pub fn clear_ITEM(&mut self) -> Self {
        self.inner &= Self::ITEM.reverse_bits();
        *self
    }

    pub const fn is_SOURCE_LOCATION(&self) -> bool {
        (self.inner & Self::SOURCE_LOCATION) != 0
    }

    /// 3xfloat
    ///
    pub const fn new_SOURCE_LOCATION() -> Self {
        Self { inner: Self::SOURCE_LOCATION }
    }

    pub fn set_SOURCE_LOCATION(&mut self) -> Self {
        self.inner |= Self::SOURCE_LOCATION;
        *self
    }

    pub fn clear_SOURCE_LOCATION(&mut self) -> Self {
        self.inner &= Self::SOURCE_LOCATION.reverse_bits();
        *self
    }

    pub const fn is_DEST_LOCATION(&self) -> bool {
        (self.inner & Self::DEST_LOCATION) != 0
    }

    /// 3xfloat
    ///
    pub const fn new_DEST_LOCATION() -> Self {
        Self { inner: Self::DEST_LOCATION }
    }

    pub fn set_DEST_LOCATION(&mut self) -> Self {
        self.inner |= Self::DEST_LOCATION;
        *self
    }

    pub fn clear_DEST_LOCATION(&mut self) -> Self {
        self.inner &= Self::DEST_LOCATION.reverse_bits();
        *self
    }

    pub const fn is_UNIT_ENEMY(&self) -> bool {
        (self.inner & Self::UNIT_ENEMY) != 0
    }

    /// `CanAttack` == true
    ///
    pub const fn new_UNIT_ENEMY() -> Self {
        Self { inner: Self::UNIT_ENEMY }
    }

    pub fn set_UNIT_ENEMY(&mut self) -> Self {
        self.inner |= Self::UNIT_ENEMY;
        *self
    }

    pub fn clear_UNIT_ENEMY(&mut self) -> Self {
        self.inner &= Self::UNIT_ENEMY.reverse_bits();
        *self
    }

    pub const fn is_UNIT_ALLY(&self) -> bool {
        (self.inner & Self::UNIT_ALLY) != 0
    }

    /// `CanAssist` == true
    ///
    pub const fn new_UNIT_ALLY() -> Self {
        Self { inner: Self::UNIT_ALLY }
    }

    pub fn set_UNIT_ALLY(&mut self) -> Self {
        self.inner |= Self::UNIT_ALLY;
        *self
    }

    pub fn clear_UNIT_ALLY(&mut self) -> Self {
        self.inner &= Self::UNIT_ALLY.reverse_bits();
        *self
    }

    pub const fn is_CORPSE_ENEMY(&self) -> bool {
        (self.inner & Self::CORPSE_ENEMY) != 0
    }

    /// pguid, `CanAssist` == false
    ///
    pub const fn new_CORPSE_ENEMY() -> Self {
        Self { inner: Self::CORPSE_ENEMY }
    }

    pub fn set_CORPSE_ENEMY(&mut self) -> Self {
        self.inner |= Self::CORPSE_ENEMY;
        *self
    }

    pub fn clear_CORPSE_ENEMY(&mut self) -> Self {
        self.inner &= Self::CORPSE_ENEMY.reverse_bits();
        *self
    }

    pub const fn is_UNIT_DEAD(&self) -> bool {
        (self.inner & Self::UNIT_DEAD) != 0
    }

    /// skinning-like effects
    ///
    pub const fn new_UNIT_DEAD() -> Self {
        Self { inner: Self::UNIT_DEAD }
    }

    pub fn set_UNIT_DEAD(&mut self) -> Self {
        self.inner |= Self::UNIT_DEAD;
        *self
    }

    pub fn clear_UNIT_DEAD(&mut self) -> Self {
        self.inner &= Self::UNIT_DEAD.reverse_bits();
        *self
    }

    pub const fn is_GAMEOBJECT(&self) -> bool {
        (self.inner & Self::GAMEOBJECT) != 0
    }

    /// pguid, 0 spells in 2.4.3
    ///
    pub const fn new_GAMEOBJECT() -> Self {
        Self { inner: Self::GAMEOBJECT }
    }

    pub fn set_GAMEOBJECT(&mut self) -> Self {
        self.inner |= Self::GAMEOBJECT;
        *self
    }

    pub fn clear_GAMEOBJECT(&mut self) -> Self {
        self.inner &= Self::GAMEOBJECT.reverse_bits();
        *self
    }

    pub const fn is_TRADE_ITEM(&self) -> bool {
        (self.inner & Self::TRADE_ITEM) != 0
    }

    /// pguid, 0 spells
    ///
    pub const fn new_TRADE_ITEM() -> Self {
        Self { inner: Self::TRADE_ITEM }
    }

    pub fn set_TRADE_ITEM(&mut self) -> Self {
        self.inner |= Self::TRADE_ITEM;
        *self
    }

    pub fn clear_TRADE_ITEM(&mut self) -> Self {
        self.inner &= Self::TRADE_ITEM.reverse_bits();
        *self
    }

    pub const fn is_STRING(&self) -> bool {
        (self.inner & Self::STRING) != 0
    }

    /// string, 0 spells
    ///
    pub const fn new_STRING() -> Self {
        Self { inner: Self::STRING }
    }

    pub fn set_STRING(&mut self) -> Self {
        self.inner |= Self::STRING;
        *self
    }

    pub fn clear_STRING(&mut self) -> Self {
        self.inner &= Self::STRING.reverse_bits();
        *self
    }

    pub const fn is_LOCKED(&self) -> bool {
        (self.inner & Self::LOCKED) != 0
    }

    /// 199 spells, opening object/lock
    ///
    pub const fn new_LOCKED() -> Self {
        Self { inner: Self::LOCKED }
    }

    pub fn set_LOCKED(&mut self) -> Self {
        self.inner |= Self::LOCKED;
        *self
    }

    pub fn clear_LOCKED(&mut self) -> Self {
        self.inner &= Self::LOCKED.reverse_bits();
        *self
    }

    pub const fn is_CORPSE_ALLY(&self) -> bool {
        (self.inner & Self::CORPSE_ALLY) != 0
    }

    /// pguid, `CanAssist` == true
    ///
    pub const fn new_CORPSE_ALLY() -> Self {
        Self { inner: Self::CORPSE_ALLY }
    }

    pub fn set_CORPSE_ALLY(&mut self) -> Self {
        self.inner |= Self::CORPSE_ALLY;
        *self
    }

    pub fn clear_CORPSE_ALLY(&mut self) -> Self {
        self.inner &= Self::CORPSE_ALLY.reverse_bits();
        *self
    }

    pub const fn is_UNIT_MINIPET(&self) -> bool {
        (self.inner & Self::UNIT_MINIPET) != 0
    }

    /// pguid, not used in any spells as of 2.4.3 (can be set dynamically)
    ///
    pub const fn new_UNIT_MINIPET() -> Self {
        Self { inner: Self::UNIT_MINIPET }
    }

    pub fn set_UNIT_MINIPET(&mut self) -> Self {
        self.inner |= Self::UNIT_MINIPET;
        *self
    }

    pub fn clear_UNIT_MINIPET(&mut self) -> Self {
        self.inner &= Self::UNIT_MINIPET.reverse_bits();
        *self
    }

    pub const fn is_GLYPH_SLOT(&self) -> bool {
        (self.inner & Self::GLYPH_SLOT) != 0
    }

    /// used in glyph spells
    ///
    pub const fn new_GLYPH_SLOT() -> Self {
        Self { inner: Self::GLYPH_SLOT }
    }

    pub fn set_GLYPH_SLOT(&mut self) -> Self {
        self.inner |= Self::GLYPH_SLOT;
        *self
    }

    pub fn clear_GLYPH_SLOT(&mut self) -> Self {
        self.inner &= Self::GLYPH_SLOT.reverse_bits();
        *self
    }

    pub const fn is_DEST_TARGET(&self) -> bool {
        (self.inner & Self::DEST_TARGET) != 0
    }

    /// sometimes appears with `DEST_TARGET` spells (may appear or not for a given spell)
    ///
    pub const fn new_DEST_TARGET() -> Self {
        Self { inner: Self::DEST_TARGET }
    }

    pub fn set_DEST_TARGET(&mut self) -> Self {
        self.inner |= Self::DEST_TARGET;
        *self
    }

    pub fn clear_DEST_TARGET(&mut self) -> Self {
        self.inner &= Self::DEST_TARGET.reverse_bits();
        *self
    }

    pub const fn is_UNUSED20(&self) -> bool {
        (self.inner & Self::UNUSED20) != 0
    }

    /// uint32 counter loop, vec3 - screen position (?) guid, not used so far
    ///
    pub const fn new_UNUSED20() -> Self {
        Self { inner: Self::UNUSED20 }
    }

    pub fn set_UNUSED20(&mut self) -> Self {
        self.inner |= Self::UNUSED20;
        *self
    }

    pub fn clear_UNUSED20(&mut self) -> Self {
        self.inner &= Self::UNUSED20.reverse_bits();
        *self
    }

    pub const fn is_UNIT_PASSENGER(&self) -> bool {
        (self.inner & Self::UNIT_PASSENGER) != 0
    }

    /// guessed, used to validate target (if vehicle passenger)
    ///
    pub const fn new_UNIT_PASSENGER() -> Self {
        Self { inner: Self::UNIT_PASSENGER }
    }

    pub fn set_UNIT_PASSENGER(&mut self) -> Self {
        self.inner |= Self::UNIT_PASSENGER;
        *self
    }

    pub fn clear_UNIT_PASSENGER(&mut self) -> Self {
        self.inner &= Self::UNIT_PASSENGER.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

