use tokio::io::AsyncReadExt;

// u16
pub async fn tokio_read_u16_le<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v).await?;
    Ok(u16::from_le_bytes(v))
}

pub async fn tokio_read_u16_be<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v).await?;
    Ok(u16::from_be_bytes(v))
}

// u32
pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u32, std::io::Error> {
    r.read_u32_le().await
}
