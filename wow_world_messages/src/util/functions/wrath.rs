use std::io::{self, Read, Write};

use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

use crate::util::read_u32_le;
use crate::wrath::{AchievementDone, AchievementInProgress, SplineFlag};

const WRATH_MONSTER_MOVE_SPLINE_FLAGS_MASK: u32 = SplineFlag::DONE
    | SplineFlag::FINAL_POINT
    | SplineFlag::FINAL_TARGET
    | SplineFlag::FINAL_ANGLE
    | 0xFF;

pub(crate) const fn wrath_monster_move_spline_flags_for_wire(flags: u32) -> u32 {
    flags & !WRATH_MONSTER_MOVE_SPLINE_FLAGS_MASK
}

pub(crate) fn wrath_vector3d_to_packed(v: &Vector3d) -> Result<u32, io::Error> {
    let x = packed_component(v.x, 11)?;
    let y = packed_component(v.y, 11)?;
    let z = packed_component(v.z, 10)?;

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

const fn sign_extend_packed_component(value: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}

pub(crate) const fn wrath_packed_to_vector3d(packed: u32) -> Vector3d {
    let x = sign_extend_packed_component(packed & 0x7FF, 11) as f32 * 0.25;
    let y = sign_extend_packed_component((packed >> 11) & 0x7FF, 11) as f32 * 0.25;
    let z = sign_extend_packed_component((packed >> 22) & 0x3FF, 10) as f32 * 0.25;

    Vector3d { x, y, z }
}

pub(crate) fn read_wrath_monster_move_spline(
    r: &mut impl Read,
    max_allocation_size: u64,
) -> Result<Vec<Vector3d>, crate::errors::ParseErrorKind> {
    let point_count = read_u32_le(&mut *r)?;
    let allocation_size = u64::from(point_count) * core::mem::size_of::<Vector3d>() as u64;
    if allocation_size > max_allocation_size {
        return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(
            allocation_size,
        ));
    }

    let capacity = usize::try_from(point_count)
        .map_err(|_| crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size))?;
    let mut splines = Vec::with_capacity(capacity);

    for index in 0..point_count {
        if index == 0 {
            splines.push(crate::util::vanilla_tbc_wrath_vector3d_read(&mut *r)?);
        } else {
            let packed = read_u32_le(&mut *r)?;
            splines.push(wrath_packed_to_vector3d(packed));
        }
    }

    Ok(splines)
}

pub(crate) fn write_wrath_monster_move_spline(
    splines: &[Vector3d],
    mut w: impl Write,
) -> Result<(), std::io::Error> {
    let point_count = u32::try_from(splines.len()).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "too many spline points")
    })?;
    w.write_all(&point_count.to_le_bytes())?;

    let mut splines = splines.iter();
    if let Some(destination) = splines.next() {
        crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(destination, &mut w)?;
    }

    for spline in splines {
        w.write_all(&wrath_vector3d_to_packed(spline)?.to_le_bytes())?;
    }

    Ok(())
}

const ACHIEVEMENT_SENTINEL_VALUE: u32 = u32::from_le_bytes((-1_i32).to_le_bytes());

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
