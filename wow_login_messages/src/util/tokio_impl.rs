use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn tokio_read_fixed_string_to_vec<R: AsyncReadExt + Unpin + Unpin>(
    r: &mut R,
    size: usize,
) -> Result<Vec<u8>, std::io::Error> {
    let mut v = Vec::with_capacity(size);

    for _ in 0..size {
        v.push(tokio_read_u8_le(r).await?);
    }

    Ok(v)
}

pub async fn tokio_read_c_string_to_vec<R: AsyncReadExt + Unpin>(
    r: &mut R,
) -> Result<Vec<u8>, std::io::Error> {
    const CSTRING_LARGEST_ALLOWED: usize = 256;

    let mut v = Vec::with_capacity(CSTRING_LARGEST_ALLOWED as usize);

    let mut byte = tokio_read_u8_le(r).await?;
    let mut count = 0;
    while byte != 0 && count != CSTRING_LARGEST_ALLOWED {
        v.push(byte);
        byte = tokio_read_u8_le(r).await?;
        count += 1;
    }

    Ok(v)
}
// u8
pub async fn tokio_read_u8_le<R: AsyncReadExt + Unpin + Unpin>(
    r: &mut R,
) -> Result<u8, std::io::Error> {
    let mut v = [0_u8; 1];
    r.read_exact(&mut v).await?;
    Ok(u8::from_le_bytes(v))
}

pub async fn tokio_write_u8_le<W: AsyncWriteExt + Unpin>(
    w: &mut W,
    v: u8,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes()).await?;
    Ok(())
}

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

pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin>(
    w: &mut W,
    v: u16,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes()).await?;
    Ok(())
}

pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin>(
    w: &mut W,
    v: u16,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_be_bytes()).await?;
    Ok(())
}

// u32
pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u32, std::io::Error> {
    Ok(r.read_u32_le().await?)
}

pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v).await?;
    Ok(u32::from_be_bytes(v))
}

pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin>(
    w: &mut W,
    v: u32,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes()).await?;
    Ok(())
}

pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin>(
    w: &mut W,
    v: u32,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_be_bytes()).await?;
    Ok(())
}

// u64
pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v).await?;
    Ok(u64::from_le_bytes(v))
}

pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v).await?;
    Ok(u64::from_be_bytes(v))
}

pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin>(
    w: &mut W,
    v: u64,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes()).await?;
    Ok(())
}

pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin>(
    w: &mut W,
    v: u64,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_be_bytes()).await?;
    Ok(())
}
