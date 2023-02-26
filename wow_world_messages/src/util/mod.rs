#[cfg(feature = "tokio")]
mod tokio_impl;

#[cfg(feature = "async-std")]
mod async_std_impl;
mod trait_helpers;

pub(crate) use trait_helpers::*;

#[cfg(feature = "async-std")]
pub use async_std_impl::*;

#[cfg(feature = "tokio")]
pub use tokio_impl::*;

use std::io::{Read, Write};

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

pub fn read_sized_c_string_to_vec<R: Read>(
    r: &mut R,
    size: u32,
) -> Result<Vec<u8>, std::io::Error> {
    let mut v = vec![0_u8; (size - 1) as usize];

    r.read_exact(&mut v)?;

    let mut null_terminator = [0_u8; 1];
    r.read_exact(&mut null_terminator)?;

    Ok(v)
}

// u8
pub fn read_u8_le<R: Read>(r: &mut R) -> Result<u8, std::io::Error> {
    let mut v = [0_u8; 1];
    r.read_exact(&mut v)?;
    Ok(u8::from_le_bytes(v))
}

pub fn write_u8_le<W: Write>(w: &mut W, v: u8) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes())?;
    Ok(())
}

// u16
pub fn read_u16_le<R: Read>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v)?;
    Ok(u16::from_le_bytes(v))
}

pub fn read_u16_be<R: Read>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v)?;
    Ok(u16::from_be_bytes(v))
}

pub fn write_u16_le<W: Write>(w: &mut W, v: u16) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes())?;
    Ok(())
}

pub fn write_u16_be<W: Write>(w: &mut W, v: u16) -> Result<(), std::io::Error> {
    w.write_all(&v.to_be_bytes())?;
    Ok(())
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

pub fn write_u32_le<W: Write>(w: &mut W, v: u32) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes())?;
    Ok(())
}

pub fn write_u32_be<W: Write>(w: &mut W, v: u32) -> Result<(), std::io::Error> {
    w.write_all(&v.to_be_bytes())?;
    Ok(())
}

// u64
pub fn read_u64_le<R: Read>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v)?;
    Ok(u64::from_le_bytes(v))
}

pub fn read_u64_be<R: Read>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v)?;
    Ok(u64::from_be_bytes(v))
}

pub fn write_u64_le<W: Write>(w: &mut W, v: u64) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes())?;
    Ok(())
}

pub fn write_u64_be<W: Write>(w: &mut W, v: u64) -> Result<(), std::io::Error> {
    w.write_all(&v.to_be_bytes())?;
    Ok(())
}

// u32
pub fn read_i32_le<R: Read>(r: &mut R) -> Result<i32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(i32::from_le_bytes(v))
}

pub fn read_i32_be<R: Read>(r: &mut R) -> Result<i32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(i32::from_be_bytes(v))
}

// f32
pub fn read_f32_le<R: Read>(r: &mut R) -> Result<f32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(f32::from_le_bytes(v))
}

pub fn read_f32_be<R: Read>(r: &mut R) -> Result<f32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(f32::from_be_bytes(v))
}

// f64
pub fn read_f64_le<R: Read>(r: &mut R) -> Result<f64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v)?;
    Ok(f64::from_le_bytes(v))
}

pub fn read_f64_be<R: Read>(r: &mut R) -> Result<f64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v)?;
    Ok(f64::from_be_bytes(v))
}

pub fn zlib_compressed_size(data: &[u8]) -> usize {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder
        .write_all(data)
        .expect("Failed to compress data when calculating message size.");
    let compressed_data = encoder
        .finish()
        .expect("Failed to flush compressed data when calculating message size.");
    compressed_data.len()
}

pub const fn u16s_to_u32(high: u16, low: u16) -> u32 {
    (high as u32) << 16 | low as u32
}

pub const fn u32_to_u16s(a: u32) -> (u16, u16) {
    let low = a as u16;
    let high = ((a & (u16::MAX as u32) << 16) >> 16) as u16;

    (high, low)
}
