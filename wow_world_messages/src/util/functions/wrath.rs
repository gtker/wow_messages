use crate::util::read_u32_le;
use crate::wrath::{AchievementDone, AchievementInProgress};
use std::io::{Read, Write};

const ACHIEVEMENT_SENTINEL_VALUE: u32 = u32::from_le_bytes((-1_i32).to_le_bytes());

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
