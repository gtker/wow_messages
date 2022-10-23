pub fn read_f32_le<R: std::io::Read>(r: &mut R) -> Result<f32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(f32::from_le_bytes(v))
}
