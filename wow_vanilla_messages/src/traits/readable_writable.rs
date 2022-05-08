#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait ReadableAndWritable: Sized {
    type Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> Result<Self, Self::Error>;

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W)
        -> Result<(), std::io::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;
}
