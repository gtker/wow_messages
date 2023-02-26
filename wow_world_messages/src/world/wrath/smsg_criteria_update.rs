use crate::Guid;
use crate::DateTime;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_criteria_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_criteria_update.wowm#L1):
/// ```text
/// smsg SMSG_CRITERIA_UPDATE = 0x046A {
///     u32 achievement;
///     PackedGuid progress_counter;
///     PackedGuid player;
///     u32 flags;
///     DateTime time;
///     u32 time_elapsed_in_seconds;
///     u32 unknown;
/// }
/// ```
pub struct SMSG_CRITERIA_UPDATE {
    pub achievement: u32,
    /// trinitycore/azerothcore: This is a u32 passed to the `appendPackGUID` function which promotes it to u64.
    ///
    pub progress_counter: Guid,
    pub player: Guid,
    /// trinitycore: this are some flags, 1 is for keeping the counter at 0 in client
    ///
    pub flags: u32,
    pub time: DateTime,
    pub time_elapsed_in_seconds: u32,
    pub unknown: u32,
}

impl crate::Message for SMSG_CRITERIA_UPDATE {
    const OPCODE: u32 = 0x046a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // progress_counter: PackedGuid
        self.progress_counter.write_packed_guid_into_vec(w);

        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // time: DateTime
        w.write_all(&self.time.as_int().to_le_bytes())?;

        // time_elapsed_in_seconds: u32
        w.write_all(&self.time_elapsed_in_seconds.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(24..=38).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x046A, size: body_size as u32 });
        }

        // achievement: u32
        let achievement = crate::util::read_u32_le(r)?;

        // progress_counter: PackedGuid
        let progress_counter = Guid::read_packed(r)?;

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        // time: DateTime
        let time: DateTime = crate::util::read_u32_le(r)?.try_into()?;
        // time_elapsed_in_seconds: u32
        let time_elapsed_in_seconds = crate::util::read_u32_le(r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        Ok(Self {
            achievement,
            progress_counter,
            player,
            flags,
            time,
            time_elapsed_in_seconds,
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CRITERIA_UPDATE {}

impl SMSG_CRITERIA_UPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // achievement: u32
        + self.progress_counter.size() // progress_counter: Guid
        + self.player.size() // player: Guid
        + 4 // flags: u32
        + 4 // time: DateTime
        + 4 // time_elapsed_in_seconds: u32
        + 4 // unknown: u32
    }
}

