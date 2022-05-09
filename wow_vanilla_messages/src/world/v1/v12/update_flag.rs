use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};

#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct UpdateFlag {
    inner: u8,
}

impl UpdateFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

}

impl ReadableAndWritable for UpdateFlag {
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

impl UpdateFlag {
    pub const NONE: u8 = 0x00;
    pub const SELF: u8 = 0x01;
    pub const TRANSPORT: u8 = 0x02;
    pub const MELEE_ATTACKING: u8 = 0x04;
    pub const HIGH_GUID: u8 = 0x08;
    pub const ALL: u8 = 0x10;
    pub const LIVING: u8 = 0x20;
    pub const HAS_POSITION: u8 = 0x40;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::SELF
                | Self::TRANSPORT
                | Self::MELEE_ATTACKING
                | Self::HIGH_GUID
                | Self::ALL
                | Self::LIVING
                | Self::HAS_POSITION
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

    pub const fn is_SELF(&self) -> bool {
        (self.inner & Self::SELF) != 0
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

    pub const fn is_TRANSPORT(&self) -> bool {
        (self.inner & Self::TRANSPORT) != 0
    }

    pub const fn new_TRANSPORT() -> Self {
        Self { inner: Self::TRANSPORT }
    }

    pub fn set_TRANSPORT(&mut self) -> Self {
        self.inner |= Self::TRANSPORT;
        *self
    }

    pub fn clear_TRANSPORT(&mut self) -> Self {
        self.inner &= Self::TRANSPORT.reverse_bits();
        *self
    }

    pub const fn is_MELEE_ATTACKING(&self) -> bool {
        (self.inner & Self::MELEE_ATTACKING) != 0
    }

    pub const fn new_MELEE_ATTACKING() -> Self {
        Self { inner: Self::MELEE_ATTACKING }
    }

    pub fn set_MELEE_ATTACKING(&mut self) -> Self {
        self.inner |= Self::MELEE_ATTACKING;
        *self
    }

    pub fn clear_MELEE_ATTACKING(&mut self) -> Self {
        self.inner &= Self::MELEE_ATTACKING.reverse_bits();
        *self
    }

    pub const fn is_HIGH_GUID(&self) -> bool {
        (self.inner & Self::HIGH_GUID) != 0
    }

    pub const fn new_HIGH_GUID() -> Self {
        Self { inner: Self::HIGH_GUID }
    }

    pub fn set_HIGH_GUID(&mut self) -> Self {
        self.inner |= Self::HIGH_GUID;
        *self
    }

    pub fn clear_HIGH_GUID(&mut self) -> Self {
        self.inner &= Self::HIGH_GUID.reverse_bits();
        *self
    }

    pub const fn is_ALL(&self) -> bool {
        (self.inner & Self::ALL) != 0
    }

    pub const fn new_ALL() -> Self {
        Self { inner: Self::ALL }
    }

    pub fn set_ALL(&mut self) -> Self {
        self.inner |= Self::ALL;
        *self
    }

    pub fn clear_ALL(&mut self) -> Self {
        self.inner &= Self::ALL.reverse_bits();
        *self
    }

    pub const fn is_LIVING(&self) -> bool {
        (self.inner & Self::LIVING) != 0
    }

    pub const fn new_LIVING() -> Self {
        Self { inner: Self::LIVING }
    }

    pub fn set_LIVING(&mut self) -> Self {
        self.inner |= Self::LIVING;
        *self
    }

    pub fn clear_LIVING(&mut self) -> Self {
        self.inner &= Self::LIVING.reverse_bits();
        *self
    }

    pub const fn is_HAS_POSITION(&self) -> bool {
        (self.inner & Self::HAS_POSITION) != 0
    }

    pub const fn new_HAS_POSITION() -> Self {
        Self { inner: Self::HAS_POSITION }
    }

    pub fn set_HAS_POSITION(&mut self) -> Self {
        self.inner |= Self::HAS_POSITION;
        *self
    }

    pub fn clear_HAS_POSITION(&mut self) -> Self {
        self.inner &= Self::HAS_POSITION.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl ConstantSized for UpdateFlag {}

impl MaximumPossibleSized for UpdateFlag {
    fn maximum_possible_size() -> usize {
        1
    }
}

