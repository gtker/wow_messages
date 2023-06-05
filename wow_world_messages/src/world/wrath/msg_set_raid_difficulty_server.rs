use std::io::{Read, Write};

use crate::wrath::RaidDifficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_set_raid_difficulty.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_set_raid_difficulty.wowm#L7):
/// ```text
/// smsg MSG_SET_RAID_DIFFICULTY_Server = 0x04EB {
///     (u32)RaidDifficulty difficulty;
///     u32 unknown1;
///     Bool32 in_group;
/// }
/// ```
pub struct MSG_SET_RAID_DIFFICULTY_Server {
    pub difficulty: RaidDifficulty,
    /// Emus set to 1.
    ///
    pub unknown1: u32,
    pub in_group: bool,
}

impl crate::private::Sealed for MSG_SET_RAID_DIFFICULTY_Server {}
impl crate::Message for MSG_SET_RAID_DIFFICULTY_Server {
    const OPCODE: u32 = 0x04eb;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // difficulty: RaidDifficulty
        w.write_all(&u32::from(self.difficulty.as_int()).to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // in_group: Bool32
        w.write_all(u32::from(self.in_group).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04EB, size: body_size });
        }

        // difficulty: RaidDifficulty
        let difficulty = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // in_group: Bool32
        let in_group = crate::util::read_u32_le(&mut r)? != 0;

        Ok(Self {
            difficulty,
            unknown1,
            in_group,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_SET_RAID_DIFFICULTY_Server {}

