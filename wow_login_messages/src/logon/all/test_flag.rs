use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct TestFlag {
    inner: u8,
}

impl TestFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for TestFlag {
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

impl TestFlag {
    pub const A: u8 = 0x01;
    pub const B: u8 = 0x02;
    pub const C: u8 = 0x04;
    pub const D: u8 = 0x08;
    pub const E: u8 = 0x10;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::A
                | Self::B
                | Self::C
                | Self::D
                | Self::E
        }
    }

    pub const fn is_A(&self) -> bool {
        (self.inner & Self::A) != 0
    }

    pub const fn new_A() -> Self {
        Self { inner: Self::A }
    }

    pub fn set_A(&mut self) -> Self {
        self.inner |= Self::A;
        *self
    }

    pub fn clear_A(&mut self) -> Self {
        self.inner &= Self::A.reverse_bits();
        *self
    }

    pub const fn is_B(&self) -> bool {
        (self.inner & Self::B) != 0
    }

    pub const fn new_B() -> Self {
        Self { inner: Self::B }
    }

    pub fn set_B(&mut self) -> Self {
        self.inner |= Self::B;
        *self
    }

    pub fn clear_B(&mut self) -> Self {
        self.inner &= Self::B.reverse_bits();
        *self
    }

    pub const fn is_C(&self) -> bool {
        (self.inner & Self::C) != 0
    }

    pub const fn new_C() -> Self {
        Self { inner: Self::C }
    }

    pub fn set_C(&mut self) -> Self {
        self.inner |= Self::C;
        *self
    }

    pub fn clear_C(&mut self) -> Self {
        self.inner &= Self::C.reverse_bits();
        *self
    }

    pub const fn is_D(&self) -> bool {
        (self.inner & Self::D) != 0
    }

    pub const fn new_D() -> Self {
        Self { inner: Self::D }
    }

    pub fn set_D(&mut self) -> Self {
        self.inner |= Self::D;
        *self
    }

    pub fn clear_D(&mut self) -> Self {
        self.inner &= Self::D.reverse_bits();
        *self
    }

    pub const fn is_E(&self) -> bool {
        (self.inner & Self::E) != 0
    }

    pub const fn new_E() -> Self {
        Self { inner: Self::E }
    }

    pub fn set_E(&mut self) -> Self {
        self.inner |= Self::E;
        *self
    }

    pub fn clear_E(&mut self) -> Self {
        self.inner &= Self::E.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl ConstantSized for TestFlag {}

impl MaximumPossibleSized for TestFlag {
    fn maximum_possible_size() -> usize {
        1
    }
}

