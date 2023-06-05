use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Used in the `AchievementInProgressArray` built-in type.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm#L24):
/// ```text
/// struct AchievementInProgress {
///     u32 achievement;
///     PackedGuid counter;
///     PackedGuid player;
///     Bool32 timed_criteria_failed;
///     DateTime progress_date;
///     u32 time_since_progress;
///     u32 time_since_progress2;
/// }
/// ```
pub struct AchievementInProgress {
    pub achievement: u32,
    pub counter: Guid,
    pub player: Guid,
    pub timed_criteria_failed: bool,
    pub progress_date: DateTime,
    pub time_since_progress: u32,
    pub time_since_progress2: u32,
}

impl AchievementInProgress {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // counter: PackedGuid
        crate::util::write_packed_guid(&self.counter, &mut w)?;

        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // timed_criteria_failed: Bool32
        w.write_all(u32::from(self.timed_criteria_failed).to_le_bytes().as_slice())?;

        // progress_date: DateTime
        w.write_all(&self.progress_date.as_int().to_le_bytes())?;

        // time_since_progress: u32
        w.write_all(&self.time_since_progress.to_le_bytes())?;

        // time_since_progress2: u32
        w.write_all(&self.time_since_progress2.to_le_bytes())?;

        Ok(())
    }
}

impl AchievementInProgress {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // achievement: u32
        let achievement = crate::util::read_u32_le(&mut r)?;

        // counter: PackedGuid
        let counter = crate::util::read_packed_guid(&mut r)?;

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // timed_criteria_failed: Bool32
        let timed_criteria_failed = crate::util::read_u32_le(&mut r)? != 0;

        // progress_date: DateTime
        let progress_date = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // time_since_progress: u32
        let time_since_progress = crate::util::read_u32_le(&mut r)?;

        // time_since_progress2: u32
        let time_since_progress2 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            achievement,
            counter,
            player,
            timed_criteria_failed,
            progress_date,
            time_since_progress,
            time_since_progress2,
        })
    }

}

impl AchievementInProgress {
    pub(crate) const fn size(&self) -> usize {
        4 // achievement: u32
        + crate::util::packed_guid_size(&self.counter) // counter: PackedGuid
        + crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + 4 // timed_criteria_failed: Bool32
        + 4 // progress_date: DateTime
        + 4 // time_since_progress: u32
        + 4 // time_since_progress2: u32
    }
}

