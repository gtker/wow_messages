#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait MessageBody: Sized {
    const OPCODE: u16;

    fn size_without_size_or_opcode_fields(&self) -> u16;

    type Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> Result<Self, Self::Error>;

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(
        r: &mut R,
        body_size: u32,
    ) -> Result<Self, Self::Error>;
    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(
        r: &mut R,
        body_size: u32,
    ) -> Result<Self, Self::Error>;

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error>;
}
