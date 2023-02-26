use crate::wrath::AchievementDone;
use crate::wrath::ACHIEVEMENT_SENTINEL_VALUE;
use crate::DateTime;
use std::convert::TryFrom;
use std::io;

#[derive(Debug, Hash, Default, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct AchievementDoneArray {
    pub done: Vec<AchievementDone>,
}

impl AchievementDoneArray {
    pub(crate) fn read(r: &mut impl io::Read) -> Result<Self, crate::errors::ParseError> {
        let mut first = crate::util::read_u32_le(r)?;

        let mut done = Vec::new();

        while first != ACHIEVEMENT_SENTINEL_VALUE {
            let time = DateTime::try_from(crate::util::read_u32_le(r)?)?;

            done.push(AchievementDone {
                achievement: first,
                time,
            });

            first = crate::util::read_u32_le(r)?;
        }

        Ok(Self { done })
    }

    pub(crate) fn write_into_vec(&self, v: &mut impl std::io::Write) -> Result<(), io::Error> {
        for d in &self.done {
            v.write_all(d.achievement.to_le_bytes().as_slice())?;
            v.write_all(d.time.as_int().to_le_bytes().as_slice())?;
        }

        v.write_all(ACHIEVEMENT_SENTINEL_VALUE.to_le_bytes().as_slice())?;

        Ok(())
    }

    pub(crate) fn size(&self) -> usize {
        self.done.len() * 4 // done: AchievementDone
            + core::mem::size_of::<u32>() // Sentinel Value
    }
}
