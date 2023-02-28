use crate::DateTime;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Used in the `AchievementDoneArray` built-in type.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm#L16):
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
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // time: DateTime
        w.write_all(&self.time.as_int().to_le_bytes())?;

        Ok(())
    }
}

impl AchievementDone {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // achievement: u32
        let achievement = crate::util::read_u32_le(r)?;

        // time: DateTime
        let time: DateTime = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            achievement,
            time,
        })
    }

}

