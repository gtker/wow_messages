use async_std::io::ReadExt;

// u16
pub async fn astd_read_u16_le<R: ReadExt + Unpin>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v).await?;
    Ok(u16::from_le_bytes(v))
}

pub async fn astd_read_u16_be<R: ReadExt + Unpin>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v).await?;
    Ok(u16::from_be_bytes(v))
}

// u32
pub async fn astd_read_u32_le<R: ReadExt + Unpin>(r: &mut R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v).await?;
    Ok(u32::from_le_bytes(v))
}
