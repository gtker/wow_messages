use std::io::{Read, Write};

use crate::wrath::RaidDifficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_set_raid_difficulty.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_set_raid_difficulty.wowm#L1):
/// ```text
/// cmsg MSG_SET_RAID_DIFFICULTY_Client = 0x04EB {
///     (u32)RaidDifficulty difficulty;
/// }
/// ```
pub struct MSG_SET_RAID_DIFFICULTY_Client {
    pub difficulty: RaidDifficulty,
}

impl crate::private::Sealed for MSG_SET_RAID_DIFFICULTY_Client {}
impl crate::Message for MSG_SET_RAID_DIFFICULTY_Client {
    const OPCODE: u32 = 0x04eb;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // difficulty: RaidDifficulty
        w.write_all(&u32::from(self.difficulty.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04EB, size: body_size as u32 });
        }

        // difficulty: RaidDifficulty
        let difficulty: RaidDifficulty = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            difficulty,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_SET_RAID_DIFFICULTY_Client {}

