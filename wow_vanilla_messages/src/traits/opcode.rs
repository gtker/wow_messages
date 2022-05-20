#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use std::future::Future;
use std::pin::Pin;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use wow_srp::header_crypto::{Decrypter, Encrypter};

pub trait OpcodeMessage: Sized {
    type Error;

    #[cfg(feature = "sync")]
    fn read_unencrypted<R: std::io::Read>(r: &mut R) -> Result<Self, Self::Error>;

    #[cfg(feature = "sync")]
    fn read_encrypted<R: std::io::Read, D: Decrypter>(
        r: &mut R,
        d: &mut D,
    ) -> std::result::Result<Self, Self::Error>;

    #[cfg(feature = "tokio")]
    fn tokio_read_unencrypted<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "tokio")]
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

    #[cfg(feature = "async-std")]
    fn astd_read_unencrypted<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async-std")]
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
}
