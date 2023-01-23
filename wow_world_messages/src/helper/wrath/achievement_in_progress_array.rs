use crate::wrath::AchievementInProgress;
use crate::wrath::ACHIEVEMENT_SENTINEL_VALUE;
use crate::{DateTime, Guid};
use std::convert::TryFrom;
use std::io;

#[derive(Debug, Hash, Default, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct AchievementInProgressArray {
    pub in_progress: Vec<AchievementInProgress>,
}

impl AchievementInProgressArray {
    pub(crate) fn read(r: &mut impl io::Read) -> Result<Self, crate::errors::ParseError> {
        let mut first = crate::util::read_u32_le(r)?;

        let mut in_progress = Vec::new();

        while first != ACHIEVEMENT_SENTINEL_VALUE {
            let counter = Guid::read_packed(r)?;
            let player = Guid::read_packed(r)?;
            let timed_criteria_failed = crate::util::read_u32_le(r)? != 0;
            let progress_date = DateTime::try_from(crate::util::read_u32_le(r)?)?;
            let time_since_progress = crate::util::read_u32_le(r)?;
            let time_since_progress2 = crate::util::read_u32_le(r)?;

            in_progress.push(AchievementInProgress {
                achievement: first,
                counter,
                player,
                timed_criteria_failed,
                progress_date,
                time_since_progress,
                time_since_progress2,
            });

            first = crate::util::read_u32_le(r)?;
        }

        Ok(Self { in_progress })
    }

    pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), io::Error> {
        use std::io::Write;

        for d in &self.in_progress {
            d.write_into_vec(v)?;
        }

        v.write_all(ACHIEVEMENT_SENTINEL_VALUE.to_le_bytes().as_slice())?;

        Ok(())
    }

    pub(crate) fn size(&self) -> usize {
        self.in_progress.iter().fold(0, |acc, x| acc + x.size())
    }
}
