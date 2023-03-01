use crate:: {
    Guid,
};
use crate::wrath:: {
    AchievementDoneArray,
    AchievementInProgressArray,
};
use std::io::{Read, Write};

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
    pub done: AchievementDoneArray,
    pub in_progress: AchievementInProgressArray,
}

impl crate::Message for SMSG_RESPOND_INSPECT_ACHIEVEMENTS {
    const OPCODE: u32 = 0x046c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // done: AchievementDoneArray
        self.done.write_into_vec(&mut w)?;

        // in_progress: AchievementInProgressArray
        self.in_progress.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x046C, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // done: AchievementDoneArray
        let done = AchievementDoneArray::read(&mut r)?;

        // in_progress: AchievementInProgressArray
        let in_progress = AchievementInProgressArray::read(&mut r)?;

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
        + self.done.size() // done: AchievementDoneArray
        + self.in_progress.size() // in_progress: AchievementInProgressArray
    }
}

