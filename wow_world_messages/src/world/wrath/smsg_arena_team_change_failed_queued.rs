use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// This message only exists as a comment in azerothcore/trinitycore.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_change_failed_queued.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_change_failed_queued.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_TEAM_CHANGE_FAILED_QUEUED = 0x04C8 {
///     u32 unknown;
/// }
/// ```
pub struct SMSG_ARENA_TEAM_CHANGE_FAILED_QUEUED {
    pub unknown: u32,
}

impl crate::Message for SMSG_ARENA_TEAM_CHANGE_FAILED_QUEUED {
    const OPCODE: u32 = 0x04c8;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04C8, size: body_size as u32 });
        }

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_TEAM_CHANGE_FAILED_QUEUED {}

