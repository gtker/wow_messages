use crate::errors::{MAX_ALLOCATION_SIZE_WRATH, ParseErrorKind};
use crate::util::{
    read_u32_le, vanilla_tbc_wrath_vector3d_read, vanilla_tbc_wrath_vector3d_write_into_vec,
};
use crate::wrath::{AchievementDone, AchievementInProgress, Vector3d};
use std::io::{self, Read, Write};

#[cfg(test)]
mod tests;

const ACHIEVEMENT_SENTINEL_VALUE: u32 = u32::from_le_bytes((-1_i32).to_le_bytes());
const VECTOR3D_SIZE: usize = 3 * core::mem::size_of::<f32>();
const PACKED_VECTOR_SIZE: usize = core::mem::size_of::<u32>();
const SPLINE_COUNT_SIZE: usize = core::mem::size_of::<u32>();

pub(crate) fn read_wrath_monster_move_spline(
    r: &mut impl Read,
    smooth: bool,
) -> Result<Vec<Vector3d>, ParseErrorKind> {
    let count = read_u32_le(r)?;
    let allocation_size = u64::from(count)
        .checked_mul(VECTOR3D_SIZE as u64)
        .ok_or(ParseErrorKind::AllocationTooLargeError(u64::MAX))?;
    if allocation_size > MAX_ALLOCATION_SIZE_WRATH {
        return Err(ParseErrorKind::AllocationTooLargeError(allocation_size));
    }
    let count = usize::try_from(count)
        .map_err(|_| ParseErrorKind::AllocationTooLargeError(allocation_size))?;
    let wire_size = if smooth {
        count
            .checked_mul(VECTOR3D_SIZE)
            .ok_or(ParseErrorKind::AllocationTooLargeError(allocation_size))?
    } else if count == 0 {
        0
    } else {
        VECTOR3D_SIZE + (count - 1) * PACKED_VECTOR_SIZE
    };
    let wire_size = u64::try_from(wire_size)
        .map_err(|_| ParseErrorKind::AllocationTooLargeError(allocation_size))?;
    let mut bounded = (&mut *r).take(wire_size);

    let mut splines = Vec::new();
    if smooth {
        for _ in 0..count {
            let spline = vanilla_tbc_wrath_vector3d_read(&mut bounded)?;
            push_spline(&mut splines, spline, allocation_size)?;
        }
    } else if count != 0 {
        let destination = vanilla_tbc_wrath_vector3d_read(&mut bounded)?;
        push_spline(&mut splines, destination, allocation_size)?;
        for _ in 1..count {
            let spline = packed_to_vector3d(read_u32_le(&mut bounded)?);
            push_spline(&mut splines, spline, allocation_size)?;
        }
    }

    Ok(splines)
}

fn push_spline(
    splines: &mut Vec<Vector3d>,
    spline: Vector3d,
    allocation_size: u64,
) -> Result<(), ParseErrorKind> {
    splines
        .try_reserve(1)
        .map_err(|_| ParseErrorKind::AllocationTooLargeError(allocation_size))?;
    splines.push(spline);
    Ok(())
}

pub(crate) fn write_wrath_monster_move_spline(
    splines: &[Vector3d],
    smooth: bool,
    mut w: impl Write,
) -> Result<(), io::Error> {
    let count = u32::try_from(splines.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "too many spline points"))?;
    w.write_all(&count.to_le_bytes())?;
    if smooth {
        for point in splines {
            vanilla_tbc_wrath_vector3d_write_into_vec(point, &mut w)?;
        }
    } else if let Some((destination, offsets)) = splines.split_first() {
        vanilla_tbc_wrath_vector3d_write_into_vec(destination, &mut w)?;
        for offset in offsets {
            w.write_all(&vector3d_to_wrath_packed(offset)?.to_le_bytes())?;
        }
    }

    Ok(())
}

pub(crate) const fn wrath_monster_move_spline_size(splines: &[Vector3d], smooth: bool) -> usize {
    if smooth {
        SPLINE_COUNT_SIZE + splines.len() * VECTOR3D_SIZE
    } else if splines.is_empty() {
        SPLINE_COUNT_SIZE
    } else {
        SPLINE_COUNT_SIZE + VECTOR3D_SIZE + (splines.len() - 1) * PACKED_VECTOR_SIZE
    }
}

/// Encode offsets as signed quarter-unit components packed into 11/11/10 bits.
fn vector3d_to_wrath_packed(point: &Vector3d) -> Result<u32, io::Error> {
    let x = packed_component(point.x, 11)?;
    let y = packed_component(point.y, 11)?;
    let z = packed_component(point.z, 10)?;
    Ok(x | (y << 11) | (z << 22))
}

fn packed_component(value: f32, bits: u32) -> Result<u32, io::Error> {
    if !value.is_finite() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "packed spline offset must be finite",
        ));
    }
    let quantized = (value / 0.25).trunc();
    let minimum = -(1_i32 << (bits - 1));
    let maximum = (1_i32 << (bits - 1)) - 1;
    if quantized < minimum as f32 || quantized > maximum as f32 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "packed spline offset exceeds representable range",
        ));
    }
    Ok((quantized as i32 as u32) & ((1_u32 << bits) - 1))
}

fn packed_to_vector3d(packed: u32) -> Vector3d {
    let x = sign_extend(packed & 0x7FF, 11) as f32 * 0.25;
    let y = sign_extend((packed >> 11) & 0x7FF, 11) as f32 * 0.25;
    let z = sign_extend((packed >> 22) & 0x3FF, 10) as f32 * 0.25;
    Vector3d { x, y, z }
}

const fn sign_extend(value: u32, bits: u32) -> i32 {
    let sign_bit = 1_u32 << (bits - 1);
    ((value ^ sign_bit).wrapping_sub(sign_bit)) as i32
}

pub(crate) fn read_achievement_done(
    r: &mut impl Read,
) -> Result<Vec<AchievementDone>, crate::errors::ParseErrorKind> {
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

pub(crate) fn read_achievement_in_progress(
    r: &mut impl Read,
) -> Result<Vec<AchievementInProgress>, crate::errors::ParseErrorKind> {
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
