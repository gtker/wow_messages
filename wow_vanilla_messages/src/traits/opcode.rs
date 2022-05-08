#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
use std::future::Future;
use std::pin::Pin;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use wow_srp::header_crypto::{Decrypter, Encrypter};

pub trait OpcodeMessage: Sized {
    type Error;

    #[cfg(feature = "sync")]
    fn read_unencrypted<R: std::io::Read>(r: &mut R) -> Result<Self, Self::Error>;

    #[cfg(feature = "sync")]
    fn write_unencrypted<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    #[cfg(feature = "sync")]
    fn read_encrypted<R: std::io::Read, D: Decrypter>(
        r: &mut R,
        d: &mut D,
    ) -> std::result::Result<Self, Self::Error>;

    #[cfg(feature = "sync")]
    fn write_encrypted<W: std::io::Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> std::result::Result<(), std::io::Error>;

    #[cfg(feature = "async_tokio")]
    fn tokio_read_unencrypted<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async_tokio")]
    fn tokio_write_unencrypted<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async_tokio")]
    fn tokio_read_encrypted<'life0, 'life1, 'async_trait, R, D>(
        r: &'life0 mut R,
        d: &'life1 mut D,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        D: 'async_trait + Decrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async_tokio")]
    fn tokio_write_encrypted<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async_std")]
    fn astd_read_unencrypted<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async_std")]
    fn astd_write_unencrypted<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async_std")]
    fn astd_read_encrypted<'life0, 'life1, 'async_trait, R, D>(
        r: &'life0 mut R,
        d: &'life1 mut D,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + ReadExt + Unpin + Send,
        D: 'async_trait + Decrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async_std")]
    fn astd_write_encrypted<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + WriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait;
}
