use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct AccountFlag {
    inner: u32,
}

impl AccountFlag {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for AccountFlag {
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
            let inner = crate::util::tokio_read_u32_le(r).await?;
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
            let inner = crate::util::astd_read_u32_le(r).await?;
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

impl AccountFlag {
    pub const GM: u32 = 0x01;
    pub const TRIAL: u32 = 0x08;
    pub const PROPASS: u32 = 0x800000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::GM
                | Self::TRIAL
                | Self::PROPASS
        }
    }

    pub const fn is_GM(&self) -> bool {
        (self.inner & Self::GM) != 0
    }

    pub const fn new_GM() -> Self {
        Self { inner: Self::GM }
    }

    pub fn set_GM(&mut self) -> Self {
        self.inner |= Self::GM;
        *self
    }

    pub fn clear_GM(&mut self) -> Self {
        self.inner &= Self::GM.reverse_bits();
        *self
    }

    pub const fn is_TRIAL(&self) -> bool {
        (self.inner & Self::TRIAL) != 0
    }

    pub const fn new_TRIAL() -> Self {
        Self { inner: Self::TRIAL }
    }

    pub fn set_TRIAL(&mut self) -> Self {
        self.inner |= Self::TRIAL;
        *self
    }

    pub fn clear_TRIAL(&mut self) -> Self {
        self.inner &= Self::TRIAL.reverse_bits();
        *self
    }

    pub const fn is_PROPASS(&self) -> bool {
        (self.inner & Self::PROPASS) != 0
    }

    pub const fn new_PROPASS() -> Self {
        Self { inner: Self::PROPASS }
    }

    pub fn set_PROPASS(&mut self) -> Self {
        self.inner |= Self::PROPASS;
        *self
    }

    pub fn clear_PROPASS(&mut self) -> Self {
        self.inner &= Self::PROPASS.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl ConstantSized for AccountFlag {}

impl MaximumPossibleSized for AccountFlag {
    fn maximum_possible_size() -> usize {
        4
    }
}

