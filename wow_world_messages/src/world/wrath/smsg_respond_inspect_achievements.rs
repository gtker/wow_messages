use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    AchievementDone, AchievementInProgress,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm#L1):
/// ```text
/// smsg SMSG_RESPOND_INSPECT_ACHIEVEMENTS = 0x046C {
///     PackedGuid player;
///     AchievementDoneArray done;
///     AchievementInProgressArray in_progress;
/// }
/// ```
pub struct SMSG_RESPOND_INSPECT_ACHIEVEMENTS {
    pub player: Guid,
    pub done: Vec<AchievementDone>,
    pub in_progress: Vec<AchievementInProgress>,
}

impl crate::private::Sealed for SMSG_RESPOND_INSPECT_ACHIEVEMENTS {}
impl crate::Message for SMSG_RESPOND_INSPECT_ACHIEVEMENTS {
    const OPCODE: u32 = 0x046c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // done: AchievementDoneArray
        crate::util::write_achievement_done(self.done.as_slice(), &mut w)?;

        // in_progress: AchievementInProgressArray
        crate::util::write_achievement_in_progress(self.in_progress.as_slice(), &mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x046C, size: body_size });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // done: AchievementDoneArray
        let done = crate::util::read_achievement_done(&mut r)?;

        // in_progress: AchievementInProgressArray
        let in_progress = crate::util::read_achievement_in_progress(&mut r)?;

        Ok(Self {
            player,
            done,
            in_progress,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_RESPOND_INSPECT_ACHIEVEMENTS {}

impl SMSG_RESPOND_INSPECT_ACHIEVEMENTS {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
        + self.done.len() * 4 // done: AchievementDoneArray
        + self.in_progress.iter().fold(0, |acc, x| acc + x.size()) // in_progress: AchievementInProgressArray
    }
}

