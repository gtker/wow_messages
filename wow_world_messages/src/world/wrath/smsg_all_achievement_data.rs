use std::convert::{TryFrom, TryInto};
use crate::world::wrath::{AchievementDoneArray, AchievementInProgressArray};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm#L9):
/// ```text
/// smsg SMSG_ALL_ACHIEVEMENT_DATA = 0x047D {
///     AchievementDoneArray done;
///     AchievementInProgressArray in_progress;
/// }
/// ```
pub struct SMSG_ALL_ACHIEVEMENT_DATA {
    pub done: AchievementDoneArray,
    pub in_progress: AchievementInProgressArray,
}

impl crate::Message for SMSG_ALL_ACHIEVEMENT_DATA {
    const OPCODE: u32 = 0x047d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // done: AchievementDoneArray
        self.done.write_into_vec(w)?;

        // in_progress: AchievementInProgressArray
        self.in_progress.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size > 4294967294 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x047D, size: body_size as u32 });
        }

        // done: AchievementDoneArray
        let done = AchievementDoneArray::read(r)?;

        // in_progress: AchievementInProgressArray
        let in_progress = AchievementInProgressArray::read(r)?;

        Ok(Self {
            done,
            in_progress,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ALL_ACHIEVEMENT_DATA {}

impl SMSG_ALL_ACHIEVEMENT_DATA {
    pub(crate) fn size(&self) -> usize {
        self.done.size() // done: AchievementDoneArray
        + self.in_progress.size() // in_progress: AchievementInProgressArray
    }
}

