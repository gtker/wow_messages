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

use crate::errors::ExpectedOpcodeError;
use std::io::{Read, Write};

#[cfg(any(feature = "wrath", feature = "tbc"))]
use crate::shared::addon_tbc_wrath::Addon;
#[cfg(feature = "wrath")]
use crate::wrath::{AchievementDone, AchievementInProgress};

#[cfg(any(feature = "wrath", feature = "tbc", feature = "vanilla"))]
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

pub(crate) const CSTRING_LARGEST_ALLOWED: usize = 256;

pub(crate) fn assert_empty(
    body_size: u32,
    opcode: impl Into<u32>,
) -> Result<(), ExpectedOpcodeError> {
    if body_size != 0 {
        Err(ExpectedOpcodeError::Parse(
            crate::errors::ParseError::InvalidSize {
                opcode: opcode.into(),
                size: body_size,
            },
        ))
    } else {
        Ok(())
    }
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

// u16
pub fn read_u16_le<R: Read>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v)?;
    Ok(u16::from_le_bytes(v))
}

#[cfg(feature = "sync")]
pub fn read_u16_be<R: Read>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v)?;
    Ok(u16::from_be_bytes(v))
}

// u32
pub fn read_u32_le<R: Read>(r: &mut R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(u32::from_le_bytes(v))
}

// u64
pub fn read_u64_le<R: Read>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v)?;
    Ok(u64::from_le_bytes(v))
}

// u32
pub fn read_i32_le<R: Read>(r: &mut R) -> Result<i32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(i32::from_le_bytes(v))
}

// f32
pub fn read_f32_le<R: Read>(r: &mut R) -> Result<f32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v)?;
    Ok(f32::from_le_bytes(v))
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

pub const fn packed_guid_size(guid: &crate::Guid) -> usize {
    let mut amount_of_bytes = 1;

    let mut i = 0;
    while i < 8 {
        if (guid.guid() & (0xFF << (i * 8))) != 0 {
            amount_of_bytes += 1;
        }

        i += 1;
    }

    amount_of_bytes
}

pub fn write_packed_guid(
    guid: &crate::Guid,
    mut v: impl std::io::Write,
) -> Result<(), std::io::Error> {
    let guid = guid.guid().to_le_bytes();
    let mut bit_pattern: u8 = 0;

    let mut placeholder = [0_u8; 9];
    let mut index = 1;
    for (i, &b) in guid.iter().enumerate() {
        if b != 0 {
            bit_pattern |= 1 << i;
            placeholder[index] = b;
            index += 1;
        }
    }

    placeholder[0] = bit_pattern;

    v.write_all(&placeholder[0..index])
}

pub fn read_guid(r: &mut impl Read) -> Result<crate::Guid, std::io::Error> {
    Ok(crate::Guid::new(crate::util::read_u64_le(r)?))
}

pub fn read_packed_guid(r: &mut impl Read) -> Result<crate::Guid, std::io::Error> {
    let bit_pattern = read_u8_le(r)?;
    let mut guid: u64 = 0;

    for index in 0..8 {
        let bit = bit_pattern & (1 << index);

        if bit != 0 {
            let byte = read_u8_le(r)?;
            guid |= (byte as u64) << (index * 8);
        }
    }

    Ok(crate::Guid::new(guid))
}

#[cfg(any(feature = "wrath", feature = "tbc", feature = "vanilla"))]
fn vector3d_to_packed(v: &Vector3d) -> u32 {
    let mut packed = 0;

    packed |= (v.x / 0.25) as u32 & 0x7FF;
    packed |= ((v.y / 0.25) as u32 & 0x7FF) << 11;
    packed |= ((v.z / 0.25) as u32 & 0x3FF) << 22;

    packed
}

#[cfg(any(feature = "wrath", feature = "tbc", feature = "vanilla"))]
const fn packed_to_vector3d(p: u32) -> Vector3d {
    let x = ((p & 0x7FF) / 4) as f32;
    let y = (((p >> 11) & 0x7FF) / 4) as f32;
    let z = (((p >> 22) & 0x3FF) / 4) as f32;

    Vector3d { x, y, z }
}

#[cfg(any(feature = "wrath", feature = "tbc", feature = "vanilla"))]
pub(crate) fn read_monster_move_spline(
    mut r: &mut impl Read,
) -> Result<Vec<Vector3d>, crate::errors::ParseError> {
    let amount_of_splines = read_u32_le(&mut r)?;
    let mut splines = Vec::with_capacity(amount_of_splines.try_into().unwrap());

    for i in 0..amount_of_splines {
        if i == 0 {
            let vec = Vector3d::read(&mut r)?;
            splines.push(vec);
        } else {
            let packed = read_u32_le(&mut r)?;
            splines.push(packed_to_vector3d(packed));
        }
    }

    Ok(splines)
}

#[cfg(any(feature = "wrath", feature = "tbc", feature = "vanilla"))]
pub(crate) fn write_monster_move_spline(
    splines: &[Vector3d],
    mut v: impl Write,
) -> Result<(), std::io::Error> {
    let amount_of_splines: u32 = splines.len().try_into().unwrap();

    v.write_all(&amount_of_splines.to_le_bytes())?;

    let mut splines = splines.iter();
    if let Some(first) = splines.next() {
        first.write_into_vec(&mut v)?;
    }

    for spline in splines {
        v.write_all(&vector3d_to_packed(spline).to_le_bytes())?;
    }

    Ok(())
}

#[cfg(any(feature = "wrath", feature = "tbc", feature = "vanilla"))]
pub(crate) fn monster_move_spline_size(splines: &[Vector3d]) -> usize {
    let mut splines = splines.iter();

    let mut size = core::mem::size_of::<u32>(); // amount_of_splines: u32
    if splines.next().is_some() {
        size += 3 * core::mem::size_of::<f32>(); // First member is Vector3d
    }
    for _ in splines {
        size += core::mem::size_of::<u32>(); // All other members are packed u32
    }

    size
}

#[cfg(feature = "wrath")]
const ACHIEVEMENT_SENTINEL_VALUE: u32 = u32::from_le_bytes((-1_i32).to_le_bytes());

#[cfg(feature = "wrath")]
pub(crate) fn read_achievement_done(
    r: &mut impl Read,
) -> Result<Vec<AchievementDone>, crate::errors::ParseError> {
    let mut first = read_u32_le(r)?;

    let mut done = Vec::new();

    while first != ACHIEVEMENT_SENTINEL_VALUE {
        let time = crate::DateTime::try_from(read_u32_le(r)?)?;

        done.push(AchievementDone {
            achievement: first,
            time,
        });

        first = read_u32_le(r)?;
    }

    Ok(done)
}

#[cfg(feature = "wrath")]
pub(crate) fn write_achievement_done(
    done: &[AchievementDone],
    mut v: impl Write,
) -> Result<(), std::io::Error> {
    for d in done {
        d.write_into_vec(&mut v)?;
    }

    v.write_all(ACHIEVEMENT_SENTINEL_VALUE.to_le_bytes().as_slice())?;

    Ok(())
}

#[cfg(feature = "wrath")]
pub(crate) fn read_achievement_in_progress(
    r: &mut impl Read,
) -> Result<Vec<AchievementInProgress>, crate::errors::ParseError> {
    let mut first = read_u32_le(r)?;

    let mut in_progress = Vec::new();

    while first != ACHIEVEMENT_SENTINEL_VALUE {
        let counter = crate::util::read_packed_guid(r)?;
        let player = crate::util::read_packed_guid(r)?;
        let timed_criteria_failed = read_u32_le(r)? != 0;
        let progress_date = crate::DateTime::try_from(read_u32_le(r)?)?;
        let time_since_progress = read_u32_le(r)?;
        let time_since_progress2 = read_u32_le(r)?;

        in_progress.push(AchievementInProgress {
            achievement: first,
            counter,
            player,
            timed_criteria_failed,
            progress_date,
            time_since_progress,
            time_since_progress2,
        });

        first = read_u32_le(r)?;
    }

    Ok(in_progress)
}

#[cfg(feature = "wrath")]
pub(crate) fn write_achievement_in_progress(
    in_progress: &[AchievementInProgress],
    mut v: impl Write,
) -> Result<(), std::io::Error> {
    for d in in_progress {
        d.write_into_vec(&mut v)?;
    }

    v.write_all(ACHIEVEMENT_SENTINEL_VALUE.to_le_bytes().as_slice())?;

    Ok(())
}

#[cfg(any(feature = "wrath", feature = "tbc"))]
pub(crate) fn write_addon_array(addons: &[Addon], mut w: impl Write) -> Result<(), std::io::Error> {
    for addon in addons {
        addon.write_into_vec(&mut w)?;
    }

    Ok(())
}
