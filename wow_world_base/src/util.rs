pub fn read_u8_le<R: std::io::Read>(r: &mut R) -> Result<u8, std::io::Error> {
    let mut v = [0_u8; 1];
    r.read_exact(&mut v)?;
    Ok(u8::from_le_bytes(v))
}

pub fn read_f32_le<R: std::io::Read>(r: &mut R) -> Result<f32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(f32::from_le_bytes(v))
}

pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(u32::from_le_bytes(v))
}

pub fn read_i32_le<R: std::io::Read>(r: &mut R) -> Result<i32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(i32::from_le_bytes(v))
}

pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v)?;
    Ok(u64::from_le_bytes(v))
}
