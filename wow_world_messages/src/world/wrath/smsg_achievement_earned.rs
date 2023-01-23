use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::DateTime;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_achievement_earned.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_achievement_earned.wowm#L1):
/// ```text
/// smsg SMSG_ACHIEVEMENT_EARNED = 0x0468 {
///     PackedGuid player;
///     u32 achievement;
///     DateTime earn_time;
///     u32 unknown;
/// }
/// ```
pub struct SMSG_ACHIEVEMENT_EARNED {
    pub player: Guid,
    pub achievement: u32,
    pub earn_time: DateTime,
    /// All emus set to 0.
    ///
    pub unknown: u32,
}

impl crate::Message for SMSG_ACHIEVEMENT_EARNED {
    const OPCODE: u32 = 0x0468;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // earn_time: DateTime
        w.write_all(&self.earn_time.as_int().to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(14..=21).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0468, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // achievement: u32
        let achievement = crate::util::read_u32_le(r)?;

        // earn_time: DateTime
        let earn_time: DateTime = crate::util::read_u32_le(r)?.try_into()?;
        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        Ok(Self {
            player,
            achievement,
            earn_time,
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ACHIEVEMENT_EARNED {}

impl SMSG_ACHIEVEMENT_EARNED {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + 4 // achievement: u32
        + 4 // earn_time: DateTime
        + 4 // unknown: u32
    }
}

