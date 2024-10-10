use crate::util::CSTRING_LARGEST_ALLOWED;
use tokio::io::AsyncReadExt;

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
    let mut v = Vec::with_capacity(CSTRING_LARGEST_ALLOWED);

    let mut byte = tokio_read_u8_le(r).await?;
    let mut count = 0;
    while byte != 0 && count != CSTRING_LARGEST_ALLOWED {
        v.push(byte);
        byte = tokio_read_u8_le(r).await?;
        count += 1;
    }

    Ok(v)
}

pub async fn tokio_read_bool_u8<R: AsyncReadExt + Unpin + Unpin>(
    mut r: R,
) -> Result<bool, std::io::Error> {
    let v = tokio_read_u8_le(&mut r).await?;

    Ok(match v {
        0 => false,
        1 => true,
        v => panic!("invalid integer for bool: {v}"),
    })
}

// u8
pub async fn tokio_read_u8_le<R: AsyncReadExt + Unpin + Unpin>(
    r: &mut R,
) -> Result<u8, std::io::Error> {
    let mut v = [0_u8; 1];
    r.read_exact(&mut v).await?;
    Ok(u8::from_le_bytes(v))
}

// u16
pub async fn tokio_read_u16_le<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v).await?;
    Ok(u16::from_le_bytes(v))
}

// u32
pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u32, std::io::Error> {
    r.read_u32_le().await
}

pub async fn tokio_read_f32_le<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<f32, std::io::Error> {
    r.read_f32_le().await
}

pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v).await?;
    Ok(u32::from_be_bytes(v))
}

// u64
pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v).await?;
    Ok(u64::from_le_bytes(v))
}

// i32
pub async fn tokio_read_i32_le<R: AsyncReadExt + Unpin>(r: &mut R) -> Result<i32, std::io::Error> {
    r.read_i32_le().await
}
