#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use wow_srp::header_crypto::{Decrypter, Encrypter};

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
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
    async fn tokio_read_unencrypted<R: AsyncReadExt + Unpin + Send>(
        r: &mut R,
    ) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_unencrypted<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_encrypted<R: AsyncReadExt + Unpin + Send, D: Decrypter + Send>(
        r: &mut R,
        d: &mut D,
    ) -> std::result::Result<Self, Self::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_encrypted<W: AsyncWriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> std::result::Result<(), std::io::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_read_unencrypted<R: ReadExt + Unpin + Send>(
        r: &mut R,
    ) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_write_unencrypted<W: WriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_read_encrypted<R: ReadExt + Unpin + Send, D: Decrypter + Send>(
        r: &mut R,
        d: &mut D,
    ) -> std::result::Result<Self, Self::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_write_encrypted<W: WriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> std::result::Result<(), std::io::Error>;
}
