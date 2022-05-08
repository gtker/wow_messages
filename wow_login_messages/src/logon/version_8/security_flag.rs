use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct SecurityFlag {
    inner: u8,
}

impl SecurityFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for SecurityFlag {
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
            let inner = crate::util::astd_read_u8_le(r).await?;
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

impl SecurityFlag {
    pub const NONE: u8 = 0x00;
    pub const PIN: u8 = 0x01;
    pub const UNKNOWN0: u8 = 0x02;
    pub const AUTHENTICATOR: u8 = 0x04;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::PIN
                | Self::UNKNOWN0
                | Self::AUTHENTICATOR
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

    pub const fn is_PIN(&self) -> bool {
        (self.inner & Self::PIN) != 0
    }

    pub const fn new_PIN() -> Self {
        Self { inner: Self::PIN }
    }

    pub fn set_PIN(&mut self) -> Self {
        self.inner |= Self::PIN;
        *self
    }

    pub fn clear_PIN(&mut self) -> Self {
        self.inner &= Self::PIN.reverse_bits();
        *self
    }

    pub const fn is_UNKNOWN0(&self) -> bool {
        (self.inner & Self::UNKNOWN0) != 0
    }

    pub const fn new_UNKNOWN0() -> Self {
        Self { inner: Self::UNKNOWN0 }
    }

    pub fn set_UNKNOWN0(&mut self) -> Self {
        self.inner |= Self::UNKNOWN0;
        *self
    }

    pub fn clear_UNKNOWN0(&mut self) -> Self {
        self.inner &= Self::UNKNOWN0.reverse_bits();
        *self
    }

    pub const fn is_AUTHENTICATOR(&self) -> bool {
        (self.inner & Self::AUTHENTICATOR) != 0
    }

    pub const fn new_AUTHENTICATOR() -> Self {
        Self { inner: Self::AUTHENTICATOR }
    }

    pub fn set_AUTHENTICATOR(&mut self) -> Self {
        self.inner |= Self::AUTHENTICATOR;
        *self
    }

    pub fn clear_AUTHENTICATOR(&mut self) -> Self {
        self.inner &= Self::AUTHENTICATOR.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl ConstantSized for SecurityFlag {}

impl MaximumPossibleSized for SecurityFlag {
    fn maximum_possible_size() -> usize {
        1
    }
}

