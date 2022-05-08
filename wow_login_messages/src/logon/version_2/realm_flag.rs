use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct RealmFlag {
    inner: u8,
}

impl RealmFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for RealmFlag {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::read_u8_le(r)?;
        Ok(Self { inner })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::tokio_read_u8_le(r).await?;
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
        let inner = crate::util::astd_read_u8_le(r).await?;
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

impl RealmFlag {
    pub const NONE: u8 = 0x00;
    pub const INVALID: u8 = 0x01;
    pub const OFFLINE: u8 = 0x02;
    pub const FORCE_BLUE_RECOMMENDED: u8 = 0x20;
    pub const FORCE_GREEN_RECOMMENDED: u8 = 0x40;
    pub const FORCE_RED_FULL: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::INVALID
                | Self::OFFLINE
                | Self::FORCE_BLUE_RECOMMENDED
                | Self::FORCE_GREEN_RECOMMENDED
                | Self::FORCE_RED_FULL
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

    pub const fn is_INVALID(&self) -> bool {
        (self.inner & Self::INVALID) != 0
    }

    pub const fn new_INVALID() -> Self {
        Self { inner: Self::INVALID }
    }

    pub fn set_INVALID(&mut self) -> Self {
        self.inner |= Self::INVALID;
        *self
    }

    pub fn clear_INVALID(&mut self) -> Self {
        self.inner &= Self::INVALID.reverse_bits();
        *self
    }

    pub const fn is_OFFLINE(&self) -> bool {
        (self.inner & Self::OFFLINE) != 0
    }

    pub const fn new_OFFLINE() -> Self {
        Self { inner: Self::OFFLINE }
    }

    pub fn set_OFFLINE(&mut self) -> Self {
        self.inner |= Self::OFFLINE;
        *self
    }

    pub fn clear_OFFLINE(&mut self) -> Self {
        self.inner &= Self::OFFLINE.reverse_bits();
        *self
    }

    pub const fn is_FORCE_BLUE_RECOMMENDED(&self) -> bool {
        (self.inner & Self::FORCE_BLUE_RECOMMENDED) != 0
    }

    pub const fn new_FORCE_BLUE_RECOMMENDED() -> Self {
        Self { inner: Self::FORCE_BLUE_RECOMMENDED }
    }

    pub fn set_FORCE_BLUE_RECOMMENDED(&mut self) -> Self {
        self.inner |= Self::FORCE_BLUE_RECOMMENDED;
        *self
    }

    pub fn clear_FORCE_BLUE_RECOMMENDED(&mut self) -> Self {
        self.inner &= Self::FORCE_BLUE_RECOMMENDED.reverse_bits();
        *self
    }

    pub const fn is_FORCE_GREEN_RECOMMENDED(&self) -> bool {
        (self.inner & Self::FORCE_GREEN_RECOMMENDED) != 0
    }

    pub const fn new_FORCE_GREEN_RECOMMENDED() -> Self {
        Self { inner: Self::FORCE_GREEN_RECOMMENDED }
    }

    pub fn set_FORCE_GREEN_RECOMMENDED(&mut self) -> Self {
        self.inner |= Self::FORCE_GREEN_RECOMMENDED;
        *self
    }

    pub fn clear_FORCE_GREEN_RECOMMENDED(&mut self) -> Self {
        self.inner &= Self::FORCE_GREEN_RECOMMENDED.reverse_bits();
        *self
    }

    pub const fn is_FORCE_RED_FULL(&self) -> bool {
        (self.inner & Self::FORCE_RED_FULL) != 0
    }

    pub const fn new_FORCE_RED_FULL() -> Self {
        Self { inner: Self::FORCE_RED_FULL }
    }

    pub fn set_FORCE_RED_FULL(&mut self) -> Self {
        self.inner |= Self::FORCE_RED_FULL;
        *self
    }

    pub fn clear_FORCE_RED_FULL(&mut self) -> Self {
        self.inner &= Self::FORCE_RED_FULL.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl ConstantSized for RealmFlag {}

impl MaximumPossibleSized for RealmFlag {
    fn maximum_possible_size() -> usize {
        1
    }
}

