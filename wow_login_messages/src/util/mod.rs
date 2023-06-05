#[cfg(feature = "tokio")]
mod tokio_impl;

#[cfg(feature = "tokio")]
pub use tokio_impl::*;

#[cfg(feature = "async-std")]
mod async_std_impl;

#[cfg(feature = "async-std")]
pub use async_std_impl::*;

use std::io::Read;

pub(crate) const CSTRING_LARGEST_ALLOWED: usize = 256;

pub fn read_fixed_string_to_vec<R: Read>(
    r: &mut R,
    size: usize,
) -> Result<Vec<u8>, std::io::Error> {
    let mut v = Vec::with_capacity(size);

    for _ in 0..size {
        v.push(read_u8_le(r)?);
    }

    Ok(v)
}

pub fn read_c_string_to_vec<R: Read>(r: &mut R) -> Result<Vec<u8>, std::io::Error> {
    let mut v = Vec::with_capacity(CSTRING_LARGEST_ALLOWED);

    let mut byte = read_u8_le(r)?;
    let mut count = 0;
    while byte != 0 && count != CSTRING_LARGEST_ALLOWED {
        v.push(byte);
        byte = read_u8_le(r)?;
        count += 1;
    }

    Ok(v)
}

// u8
pub fn read_u8_le<R: Read>(r: &mut R) -> Result<u8, std::io::Error> {
    let mut v = [0_u8; 1];
    r.read_exact(&mut v)?;
    Ok(u8::from_le_bytes(v))
}

// u16
pub fn read_u16_le<R: Read>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v)?;
    Ok(u16::from_le_bytes(v))
}

// u32
pub fn read_u32_le<R: Read>(r: &mut R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(u32::from_le_bytes(v))
}

pub fn read_u32_be<R: Read>(r: &mut R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(u32::from_be_bytes(v))
}

// u64
pub fn read_u64_le<R: Read>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v)?;
    Ok(u64::from_le_bytes(v))
}

#[cfg(feature = "print-testcase")]
pub(crate) fn write_bytes(
    s: &mut String,
    bytes: &mut impl Iterator<Item = u8>,
    size: usize,
    name: &str,
    prefix: &str,
) {
    use std::fmt::Write;
    let mut bytes = bytes.take(size);

    write!(s, "{prefix}{:#04X}, ", bytes.next().unwrap()).unwrap();

    for b in bytes {
        write!(s, "{b:#04X}, ").unwrap();
    }
    writeln!(s, "/* {name} */").unwrap();
}
