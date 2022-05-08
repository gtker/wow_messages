use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct SpellCastTargetFlags {
    inner: u16,
}

impl SpellCastTargetFlags {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for SpellCastTargetFlags {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::read_u16_le(r)?;
        Ok(Self { inner })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::tokio_read_u16_le(r).await?;
        Ok(Self { inner })
    }

    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            w.write_all(&self.inner.to_le_bytes()).await?;
            Ok(())
        })
    }
    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::astd_read_u16_le(r).await?;
        Ok(Self { inner })
    }

    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            w.write_all(&self.inner.to_le_bytes()).await?;
            Ok(())
        })
    }
}

impl SpellCastTargetFlags {
    pub const SELF: u16 = 0x00;
    pub const UNUSED1: u16 = 0x01;
    pub const UNIT: u16 = 0x02;
    pub const UNIT_RAID: u16 = 0x04;
    pub const UNIT_PARTY: u16 = 0x08;
    pub const ITEM: u16 = 0x10;
    pub const SOURCE_LOCATION: u16 = 0x20;
    pub const DEST_LOCATION: u16 = 0x40;
    pub const UNIT_ENEMY: u16 = 0x80;
    pub const UNIT_ALLY: u16 = 0x100;
    pub const CORPSE_ENEMY: u16 = 0x200;
    pub const UNIT_DEAD: u16 = 0x400;
    pub const GAMEOBJECT: u16 = 0x800;
    pub const TRADE_ITEM: u16 = 0x1000;
    pub const STRING: u16 = 0x2000;
    pub const LOCKED: u16 = 0x4000;
    pub const CORPSE_ALLY: u16 = 0x8000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
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
        }
    }

    pub const fn is_SELF(&self) -> bool {
        // Underlying value is 0
        self.inner == Self::SELF
    }

    pub const fn new_SELF() -> Self {
        Self { inner: Self::SELF }
    }

    pub fn set_SELF(&mut self) -> Self {
        self.inner |= Self::SELF;
        *self
    }

    pub fn clear_SELF(&mut self) -> Self {
        self.inner &= Self::SELF.reverse_bits();
        *self
    }

    pub const fn is_UNUSED1(&self) -> bool {
        (self.inner & Self::UNUSED1) != 0
    }

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

    pub const fn as_int(&self) -> u16 {
        self.inner
    }

}

impl ConstantSized for SpellCastTargetFlags {}

impl MaximumPossibleSized for SpellCastTargetFlags {
    fn maximum_possible_size() -> usize {
        2
    }
}

