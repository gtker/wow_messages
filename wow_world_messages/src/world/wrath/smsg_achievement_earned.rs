use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

impl crate::private::Sealed for SMSG_ACHIEVEMENT_EARNED {}
impl crate::Message for SMSG_ACHIEVEMENT_EARNED {
    const OPCODE: u32 = 0x0468;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // earn_time: DateTime
        w.write_all(&self.earn_time.as_int().to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(14..=21).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0468, size: body_size });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // achievement: u32
        let achievement = crate::util::read_u32_le(&mut r)?;

        // earn_time: DateTime
        let earn_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            achievement,
            earn_time,
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACHIEVEMENT_EARNED {}

impl SMSG_ACHIEVEMENT_EARNED {
    pub(crate) const fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
        + 4 // achievement: u32
        + 4 // earn_time: DateTime
        + 4 // unknown: u32
    }
}

