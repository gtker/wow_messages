use std::io::{Read, Write};

use crate::DateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Used in the `AchievementDoneArray` built-in type.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm#L17):
/// ```text
/// struct AchievementDone {
///     u32 achievement;
///     DateTime time;
/// }
/// ```
pub struct AchievementDone {
    pub achievement: u32,
    pub time: DateTime,
}

impl AchievementDone {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // time: DateTime
        w.write_all(&self.time.as_int().to_le_bytes())?;

        Ok(())
    }
}

impl AchievementDone {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // achievement: u32
        let achievement = crate::util::read_u32_le(&mut r)?;

        // time: DateTime
        let time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        Ok(Self {
            achievement,
            time,
        })
    }

}

