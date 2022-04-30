use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::read_u32_le(r)?;
        Ok(Self { inner })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes())?;
        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for AccountFlag {
    type Error = std::io::Error;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let inner = crate::util::tokio_read_u32_le(r).await?;
        Ok(Self { inner })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes()).await?;
        Ok(())
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

    pub const fn as_u32(&self) -> u32 {
        self.inner
    }

}

impl ConstantSized for AccountFlag {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for AccountFlag {
    fn maximum_possible_size() -> usize {
        4
    }
}

