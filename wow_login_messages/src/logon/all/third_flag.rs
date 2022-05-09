use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct ThirdFlag {
    inner: u8,
}

impl ThirdFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for ThirdFlag {
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

impl ThirdFlag {
    pub const X: u8 = 0x01;
    pub const Y: u8 = 0x02;
    pub const Z: u8 = 0x04;
    pub const W: u8 = 0x08;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::X
                | Self::Y
                | Self::Z
                | Self::W
        }
    }

    pub const fn is_X(&self) -> bool {
        (self.inner & Self::X) != 0
    }

    pub const fn new_X() -> Self {
        Self { inner: Self::X }
    }

    pub fn set_X(&mut self) -> Self {
        self.inner |= Self::X;
        *self
    }

    pub fn clear_X(&mut self) -> Self {
        self.inner &= Self::X.reverse_bits();
        *self
    }

    pub const fn is_Y(&self) -> bool {
        (self.inner & Self::Y) != 0
    }

    pub const fn new_Y() -> Self {
        Self { inner: Self::Y }
    }

    pub fn set_Y(&mut self) -> Self {
        self.inner |= Self::Y;
        *self
    }

    pub fn clear_Y(&mut self) -> Self {
        self.inner &= Self::Y.reverse_bits();
        *self
    }

    pub const fn is_Z(&self) -> bool {
        (self.inner & Self::Z) != 0
    }

    pub const fn new_Z() -> Self {
        Self { inner: Self::Z }
    }

    pub fn set_Z(&mut self) -> Self {
        self.inner |= Self::Z;
        *self
    }

    pub fn clear_Z(&mut self) -> Self {
        self.inner &= Self::Z.reverse_bits();
        *self
    }

    pub const fn is_W(&self) -> bool {
        (self.inner & Self::W) != 0
    }

    pub const fn new_W() -> Self {
        Self { inner: Self::W }
    }

    pub fn set_W(&mut self) -> Self {
        self.inner |= Self::W;
        *self
    }

    pub fn clear_W(&mut self) -> Self {
        self.inner &= Self::W.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl ConstantSized for ThirdFlag {}

impl MaximumPossibleSized for ThirdFlag {
    fn maximum_possible_size() -> usize {
        1
    }
}

