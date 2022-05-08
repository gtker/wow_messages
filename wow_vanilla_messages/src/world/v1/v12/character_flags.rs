use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct CharacterFlags {
    inner: u32,
}

impl CharacterFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for CharacterFlags {
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

impl CharacterFlags {
    pub const NONE: u32 = 0x00;
    pub const LOCKED_FOR_TRANSFER: u32 = 0x04;
    pub const HIDE_HELM: u32 = 0x400;
    pub const HIDE_CLOAK: u32 = 0x800;
    pub const GHOST: u32 = 0x2000;
    pub const RENAME: u32 = 0x4000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::LOCKED_FOR_TRANSFER
                | Self::HIDE_HELM
                | Self::HIDE_CLOAK
                | Self::GHOST
                | Self::RENAME
        }
    }

    pub const fn is_NONE(&self) -> bool {
        // Underlying value is 0
        self.inner == Self::NONE
    }

    pub const fn new_NONE() -> Self {
        Self { inner: Self::NONE }
    }

    pub fn set_NONE(&mut self) -> Self {
        self.inner |= Self::NONE;
        *self
    }

    pub fn clear_NONE(&mut self) -> Self {
        self.inner &= Self::NONE.reverse_bits();
        *self
    }

    pub const fn is_LOCKED_FOR_TRANSFER(&self) -> bool {
        (self.inner & Self::LOCKED_FOR_TRANSFER) != 0
    }

    pub const fn new_LOCKED_FOR_TRANSFER() -> Self {
        Self { inner: Self::LOCKED_FOR_TRANSFER }
    }

    pub fn set_LOCKED_FOR_TRANSFER(&mut self) -> Self {
        self.inner |= Self::LOCKED_FOR_TRANSFER;
        *self
    }

    pub fn clear_LOCKED_FOR_TRANSFER(&mut self) -> Self {
        self.inner &= Self::LOCKED_FOR_TRANSFER.reverse_bits();
        *self
    }

    pub const fn is_HIDE_HELM(&self) -> bool {
        (self.inner & Self::HIDE_HELM) != 0
    }

    pub const fn new_HIDE_HELM() -> Self {
        Self { inner: Self::HIDE_HELM }
    }

    pub fn set_HIDE_HELM(&mut self) -> Self {
        self.inner |= Self::HIDE_HELM;
        *self
    }

    pub fn clear_HIDE_HELM(&mut self) -> Self {
        self.inner &= Self::HIDE_HELM.reverse_bits();
        *self
    }

    pub const fn is_HIDE_CLOAK(&self) -> bool {
        (self.inner & Self::HIDE_CLOAK) != 0
    }

    pub const fn new_HIDE_CLOAK() -> Self {
        Self { inner: Self::HIDE_CLOAK }
    }

    pub fn set_HIDE_CLOAK(&mut self) -> Self {
        self.inner |= Self::HIDE_CLOAK;
        *self
    }

    pub fn clear_HIDE_CLOAK(&mut self) -> Self {
        self.inner &= Self::HIDE_CLOAK.reverse_bits();
        *self
    }

    pub const fn is_GHOST(&self) -> bool {
        (self.inner & Self::GHOST) != 0
    }

    pub const fn new_GHOST() -> Self {
        Self { inner: Self::GHOST }
    }

    pub fn set_GHOST(&mut self) -> Self {
        self.inner |= Self::GHOST;
        *self
    }

    pub fn clear_GHOST(&mut self) -> Self {
        self.inner &= Self::GHOST.reverse_bits();
        *self
    }

    pub const fn is_RENAME(&self) -> bool {
        (self.inner & Self::RENAME) != 0
    }

    pub const fn new_RENAME() -> Self {
        Self { inner: Self::RENAME }
    }

    pub fn set_RENAME(&mut self) -> Self {
        self.inner |= Self::RENAME;
        *self
    }

    pub fn clear_RENAME(&mut self) -> Self {
        self.inner &= Self::RENAME.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl ConstantSized for CharacterFlags {}

impl MaximumPossibleSized for CharacterFlags {
    fn maximum_possible_size() -> usize {
        4
    }
}

