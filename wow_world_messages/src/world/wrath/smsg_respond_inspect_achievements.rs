use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::{AchievementDoneArray, AchievementInProgressArray};
use std::io::{Write, Read};

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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // done: AchievementDoneArray
        self.done.write_into_vec(w)?;

        // in_progress: AchievementInProgressArray
        self.in_progress.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x046C, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // done: AchievementDoneArray
        let done = AchievementDoneArray::read(r)?;

        // in_progress: AchievementInProgressArray
        let in_progress = AchievementInProgressArray::read(r)?;

        Ok(Self {
            player,
            done,
            in_progress,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_RESPOND_INSPECT_ACHIEVEMENTS {}

impl SMSG_RESPOND_INSPECT_ACHIEVEMENTS {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.done.size() // done: AchievementDoneArray
        + self.in_progress.size() // in_progress: AchievementInProgressArray
    }
}

