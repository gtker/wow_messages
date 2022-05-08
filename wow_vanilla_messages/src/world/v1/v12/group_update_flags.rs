use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct GroupUpdateFlags {
    inner: u32,
}

impl GroupUpdateFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for GroupUpdateFlags {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::read_u32_le(r)?;
        Ok(Self { inner })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes())?;
        Ok(())
    }

    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = crate::util::tokio_read_u32_le(r).await?;
            Ok(Self { inner })
        })
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

    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = crate::util::astd_read_u32_le(r).await?;
            Ok(Self { inner })
        })
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

impl GroupUpdateFlags {
    pub const FLAG_NONE: u32 = 0x00;
    pub const FLAG_STATUS: u32 = 0x01;
    pub const FLAG_CUR_HP: u32 = 0x02;
    pub const FLAG_MAX_HP: u32 = 0x04;
    pub const FLAG_POWER_TYPE: u32 = 0x08;
    pub const FLAG_CUR_POWER: u32 = 0x10;
    pub const FLAG_MAX_POWER: u32 = 0x20;
    pub const FLAG_LEVEL: u32 = 0x40;
    pub const FLAG_ZONE: u32 = 0x80;
    pub const FLAG_POSITION: u32 = 0x100;
    pub const FLAG_AURAS: u32 = 0x200;
    pub const FLAG_AURAS_2: u32 = 0x400;
    pub const FLAG_PET_GUID: u32 = 0x800;
    pub const FLAG_PET_NAME: u32 = 0x1000;
    pub const FLAG_PET_MODEL_ID: u32 = 0x2000;
    pub const FLAG_PET_CUR_HP: u32 = 0x4000;
    pub const FLAG_PET_MAX_HP: u32 = 0x8000;
    pub const FLAG_PET_POWER_TYPE: u32 = 0x10000;
    pub const FLAG_PET_CUR_POWER: u32 = 0x20000;
    pub const FLAG_PET_MAX_POWER: u32 = 0x40000;
    pub const FLAG_PET_AURAS: u32 = 0x80000;
    pub const FLAG_PET_AURAS_2: u32 = 0x100000;
    pub const MODE_OFFLINE: u32 = 0x10000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::FLAG_NONE
                | Self::FLAG_STATUS
                | Self::FLAG_CUR_HP
                | Self::FLAG_MAX_HP
                | Self::FLAG_POWER_TYPE
                | Self::FLAG_CUR_POWER
                | Self::FLAG_MAX_POWER
                | Self::FLAG_LEVEL
                | Self::FLAG_ZONE
                | Self::FLAG_POSITION
                | Self::FLAG_AURAS
                | Self::FLAG_AURAS_2
                | Self::FLAG_PET_GUID
                | Self::FLAG_PET_NAME
                | Self::FLAG_PET_MODEL_ID
                | Self::FLAG_PET_CUR_HP
                | Self::FLAG_PET_MAX_HP
                | Self::FLAG_PET_POWER_TYPE
                | Self::FLAG_PET_CUR_POWER
                | Self::FLAG_PET_MAX_POWER
                | Self::FLAG_PET_AURAS
                | Self::FLAG_PET_AURAS_2
                | Self::MODE_OFFLINE
        }
    }

    pub const fn is_FLAG_NONE(&self) -> bool {
        // Underlying value is 0
        self.inner == Self::FLAG_NONE
    }

    pub const fn new_FLAG_NONE() -> Self {
        Self { inner: Self::FLAG_NONE }
    }

    pub fn set_FLAG_NONE(&mut self) -> Self {
        self.inner |= Self::FLAG_NONE;
        *self
    }

    pub fn clear_FLAG_NONE(&mut self) -> Self {
        self.inner &= Self::FLAG_NONE.reverse_bits();
        *self
    }

    pub const fn is_FLAG_STATUS(&self) -> bool {
        (self.inner & Self::FLAG_STATUS) != 0
    }

    pub const fn new_FLAG_STATUS() -> Self {
        Self { inner: Self::FLAG_STATUS }
    }

    pub fn set_FLAG_STATUS(&mut self) -> Self {
        self.inner |= Self::FLAG_STATUS;
        *self
    }

    pub fn clear_FLAG_STATUS(&mut self) -> Self {
        self.inner &= Self::FLAG_STATUS.reverse_bits();
        *self
    }

    pub const fn is_FLAG_CUR_HP(&self) -> bool {
        (self.inner & Self::FLAG_CUR_HP) != 0
    }

    pub const fn new_FLAG_CUR_HP() -> Self {
        Self { inner: Self::FLAG_CUR_HP }
    }

    pub fn set_FLAG_CUR_HP(&mut self) -> Self {
        self.inner |= Self::FLAG_CUR_HP;
        *self
    }

    pub fn clear_FLAG_CUR_HP(&mut self) -> Self {
        self.inner &= Self::FLAG_CUR_HP.reverse_bits();
        *self
    }

    pub const fn is_FLAG_MAX_HP(&self) -> bool {
        (self.inner & Self::FLAG_MAX_HP) != 0
    }

    pub const fn new_FLAG_MAX_HP() -> Self {
        Self { inner: Self::FLAG_MAX_HP }
    }

    pub fn set_FLAG_MAX_HP(&mut self) -> Self {
        self.inner |= Self::FLAG_MAX_HP;
        *self
    }

    pub fn clear_FLAG_MAX_HP(&mut self) -> Self {
        self.inner &= Self::FLAG_MAX_HP.reverse_bits();
        *self
    }

    pub const fn is_FLAG_POWER_TYPE(&self) -> bool {
        (self.inner & Self::FLAG_POWER_TYPE) != 0
    }

    pub const fn new_FLAG_POWER_TYPE() -> Self {
        Self { inner: Self::FLAG_POWER_TYPE }
    }

    pub fn set_FLAG_POWER_TYPE(&mut self) -> Self {
        self.inner |= Self::FLAG_POWER_TYPE;
        *self
    }

    pub fn clear_FLAG_POWER_TYPE(&mut self) -> Self {
        self.inner &= Self::FLAG_POWER_TYPE.reverse_bits();
        *self
    }

    pub const fn is_FLAG_CUR_POWER(&self) -> bool {
        (self.inner & Self::FLAG_CUR_POWER) != 0
    }

    pub const fn new_FLAG_CUR_POWER() -> Self {
        Self { inner: Self::FLAG_CUR_POWER }
    }

    pub fn set_FLAG_CUR_POWER(&mut self) -> Self {
        self.inner |= Self::FLAG_CUR_POWER;
        *self
    }

    pub fn clear_FLAG_CUR_POWER(&mut self) -> Self {
        self.inner &= Self::FLAG_CUR_POWER.reverse_bits();
        *self
    }

    pub const fn is_FLAG_MAX_POWER(&self) -> bool {
        (self.inner & Self::FLAG_MAX_POWER) != 0
    }

    pub const fn new_FLAG_MAX_POWER() -> Self {
        Self { inner: Self::FLAG_MAX_POWER }
    }

    pub fn set_FLAG_MAX_POWER(&mut self) -> Self {
        self.inner |= Self::FLAG_MAX_POWER;
        *self
    }

    pub fn clear_FLAG_MAX_POWER(&mut self) -> Self {
        self.inner &= Self::FLAG_MAX_POWER.reverse_bits();
        *self
    }

    pub const fn is_FLAG_LEVEL(&self) -> bool {
        (self.inner & Self::FLAG_LEVEL) != 0
    }

    pub const fn new_FLAG_LEVEL() -> Self {
        Self { inner: Self::FLAG_LEVEL }
    }

    pub fn set_FLAG_LEVEL(&mut self) -> Self {
        self.inner |= Self::FLAG_LEVEL;
        *self
    }

    pub fn clear_FLAG_LEVEL(&mut self) -> Self {
        self.inner &= Self::FLAG_LEVEL.reverse_bits();
        *self
    }

    pub const fn is_FLAG_ZONE(&self) -> bool {
        (self.inner & Self::FLAG_ZONE) != 0
    }

    pub const fn new_FLAG_ZONE() -> Self {
        Self { inner: Self::FLAG_ZONE }
    }

    pub fn set_FLAG_ZONE(&mut self) -> Self {
        self.inner |= Self::FLAG_ZONE;
        *self
    }

    pub fn clear_FLAG_ZONE(&mut self) -> Self {
        self.inner &= Self::FLAG_ZONE.reverse_bits();
        *self
    }

    pub const fn is_FLAG_POSITION(&self) -> bool {
        (self.inner & Self::FLAG_POSITION) != 0
    }

    pub const fn new_FLAG_POSITION() -> Self {
        Self { inner: Self::FLAG_POSITION }
    }

    pub fn set_FLAG_POSITION(&mut self) -> Self {
        self.inner |= Self::FLAG_POSITION;
        *self
    }

    pub fn clear_FLAG_POSITION(&mut self) -> Self {
        self.inner &= Self::FLAG_POSITION.reverse_bits();
        *self
    }

    pub const fn is_FLAG_AURAS(&self) -> bool {
        (self.inner & Self::FLAG_AURAS) != 0
    }

    pub const fn new_FLAG_AURAS() -> Self {
        Self { inner: Self::FLAG_AURAS }
    }

    pub fn set_FLAG_AURAS(&mut self) -> Self {
        self.inner |= Self::FLAG_AURAS;
        *self
    }

    pub fn clear_FLAG_AURAS(&mut self) -> Self {
        self.inner &= Self::FLAG_AURAS.reverse_bits();
        *self
    }

    pub const fn is_FLAG_AURAS_2(&self) -> bool {
        (self.inner & Self::FLAG_AURAS_2) != 0
    }

    pub const fn new_FLAG_AURAS_2() -> Self {
        Self { inner: Self::FLAG_AURAS_2 }
    }

    pub fn set_FLAG_AURAS_2(&mut self) -> Self {
        self.inner |= Self::FLAG_AURAS_2;
        *self
    }

    pub fn clear_FLAG_AURAS_2(&mut self) -> Self {
        self.inner &= Self::FLAG_AURAS_2.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_GUID(&self) -> bool {
        (self.inner & Self::FLAG_PET_GUID) != 0
    }

    pub const fn new_FLAG_PET_GUID() -> Self {
        Self { inner: Self::FLAG_PET_GUID }
    }

    pub fn set_FLAG_PET_GUID(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_GUID;
        *self
    }

    pub fn clear_FLAG_PET_GUID(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_GUID.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_NAME(&self) -> bool {
        (self.inner & Self::FLAG_PET_NAME) != 0
    }

    pub const fn new_FLAG_PET_NAME() -> Self {
        Self { inner: Self::FLAG_PET_NAME }
    }

    pub fn set_FLAG_PET_NAME(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_NAME;
        *self
    }

    pub fn clear_FLAG_PET_NAME(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_NAME.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_MODEL_ID(&self) -> bool {
        (self.inner & Self::FLAG_PET_MODEL_ID) != 0
    }

    pub const fn new_FLAG_PET_MODEL_ID() -> Self {
        Self { inner: Self::FLAG_PET_MODEL_ID }
    }

    pub fn set_FLAG_PET_MODEL_ID(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_MODEL_ID;
        *self
    }

    pub fn clear_FLAG_PET_MODEL_ID(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_MODEL_ID.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_CUR_HP(&self) -> bool {
        (self.inner & Self::FLAG_PET_CUR_HP) != 0
    }

    pub const fn new_FLAG_PET_CUR_HP() -> Self {
        Self { inner: Self::FLAG_PET_CUR_HP }
    }

    pub fn set_FLAG_PET_CUR_HP(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_CUR_HP;
        *self
    }

    pub fn clear_FLAG_PET_CUR_HP(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_CUR_HP.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_MAX_HP(&self) -> bool {
        (self.inner & Self::FLAG_PET_MAX_HP) != 0
    }

    pub const fn new_FLAG_PET_MAX_HP() -> Self {
        Self { inner: Self::FLAG_PET_MAX_HP }
    }

    pub fn set_FLAG_PET_MAX_HP(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_MAX_HP;
        *self
    }

    pub fn clear_FLAG_PET_MAX_HP(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_MAX_HP.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_POWER_TYPE(&self) -> bool {
        (self.inner & Self::FLAG_PET_POWER_TYPE) != 0
    }

    pub const fn new_FLAG_PET_POWER_TYPE() -> Self {
        Self { inner: Self::FLAG_PET_POWER_TYPE }
    }

    pub fn set_FLAG_PET_POWER_TYPE(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_POWER_TYPE;
        *self
    }

    pub fn clear_FLAG_PET_POWER_TYPE(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_POWER_TYPE.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_CUR_POWER(&self) -> bool {
        (self.inner & Self::FLAG_PET_CUR_POWER) != 0
    }

    pub const fn new_FLAG_PET_CUR_POWER() -> Self {
        Self { inner: Self::FLAG_PET_CUR_POWER }
    }

    pub fn set_FLAG_PET_CUR_POWER(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_CUR_POWER;
        *self
    }

    pub fn clear_FLAG_PET_CUR_POWER(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_CUR_POWER.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_MAX_POWER(&self) -> bool {
        (self.inner & Self::FLAG_PET_MAX_POWER) != 0
    }

    pub const fn new_FLAG_PET_MAX_POWER() -> Self {
        Self { inner: Self::FLAG_PET_MAX_POWER }
    }

    pub fn set_FLAG_PET_MAX_POWER(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_MAX_POWER;
        *self
    }

    pub fn clear_FLAG_PET_MAX_POWER(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_MAX_POWER.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_AURAS(&self) -> bool {
        (self.inner & Self::FLAG_PET_AURAS) != 0
    }

    pub const fn new_FLAG_PET_AURAS() -> Self {
        Self { inner: Self::FLAG_PET_AURAS }
    }

    pub fn set_FLAG_PET_AURAS(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_AURAS;
        *self
    }

    pub fn clear_FLAG_PET_AURAS(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_AURAS.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_AURAS_2(&self) -> bool {
        (self.inner & Self::FLAG_PET_AURAS_2) != 0
    }

    pub const fn new_FLAG_PET_AURAS_2() -> Self {
        Self { inner: Self::FLAG_PET_AURAS_2 }
    }

    pub fn set_FLAG_PET_AURAS_2(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_AURAS_2;
        *self
    }

    pub fn clear_FLAG_PET_AURAS_2(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_AURAS_2.reverse_bits();
        *self
    }

    pub const fn is_MODE_OFFLINE(&self) -> bool {
        (self.inner & Self::MODE_OFFLINE) != 0
    }

    pub const fn new_MODE_OFFLINE() -> Self {
        Self { inner: Self::MODE_OFFLINE }
    }

    pub fn set_MODE_OFFLINE(&mut self) -> Self {
        self.inner |= Self::MODE_OFFLINE;
        *self
    }

    pub fn clear_MODE_OFFLINE(&mut self) -> Self {
        self.inner &= Self::MODE_OFFLINE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl ConstantSized for GroupUpdateFlags {}

impl MaximumPossibleSized for GroupUpdateFlags {
    fn maximum_possible_size() -> usize {
        4
    }
}

