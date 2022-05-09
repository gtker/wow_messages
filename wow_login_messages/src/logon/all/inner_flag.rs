use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct InnerFlag {
    inner: u8,
}

impl InnerFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for InnerFlag {
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
            let inner = crate::util::tokio_read_u8_le(r).await?;
            Ok(Self { inner })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            let inner = crate::util::astd_read_u8_le(r).await?;
            Ok(Self { inner })
        })
    }

    #[cfg(feature = "async_std")]
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

impl InnerFlag {
    pub const H: u8 = 0x01;
    pub const I: u8 = 0x02;
    pub const J: u8 = 0x04;
    pub const K: u8 = 0x08;
    pub const L: u8 = 0x10;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::H
                | Self::I
                | Self::J
                | Self::K
                | Self::L
        }
    }

    pub const fn is_H(&self) -> bool {
        (self.inner & Self::H) != 0
    }

    pub const fn new_H() -> Self {
        Self { inner: Self::H }
    }

    pub fn set_H(&mut self) -> Self {
        self.inner |= Self::H;
        *self
    }

    pub fn clear_H(&mut self) -> Self {
        self.inner &= Self::H.reverse_bits();
        *self
    }

    pub const fn is_I(&self) -> bool {
        (self.inner & Self::I) != 0
    }

    pub const fn new_I() -> Self {
        Self { inner: Self::I }
    }

    pub fn set_I(&mut self) -> Self {
        self.inner |= Self::I;
        *self
    }

    pub fn clear_I(&mut self) -> Self {
        self.inner &= Self::I.reverse_bits();
        *self
    }

    pub const fn is_J(&self) -> bool {
        (self.inner & Self::J) != 0
    }

    pub const fn new_J() -> Self {
        Self { inner: Self::J }
    }

    pub fn set_J(&mut self) -> Self {
        self.inner |= Self::J;
        *self
    }

    pub fn clear_J(&mut self) -> Self {
        self.inner &= Self::J.reverse_bits();
        *self
    }

    pub const fn is_K(&self) -> bool {
        (self.inner & Self::K) != 0
    }

    pub const fn new_K() -> Self {
        Self { inner: Self::K }
    }

    pub fn set_K(&mut self) -> Self {
        self.inner |= Self::K;
        *self
    }

    pub fn clear_K(&mut self) -> Self {
        self.inner &= Self::K.reverse_bits();
        *self
    }

    pub const fn is_L(&self) -> bool {
        (self.inner & Self::L) != 0
    }

    pub const fn new_L() -> Self {
        Self { inner: Self::L }
    }

    pub fn set_L(&mut self) -> Self {
        self.inner |= Self::L;
        *self
    }

    pub fn clear_L(&mut self) -> Self {
        self.inner &= Self::L.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl ConstantSized for InnerFlag {}

impl MaximumPossibleSized for InnerFlag {
    fn maximum_possible_size() -> usize {
        1
    }
}

