use std::convert::{TryFrom, TryInto};
use crate::world::shared::arena_type_tbc_wrath::ArenaType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_error.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_error.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_ERROR = 0x0376 {
///     u32 unknown;
///     ArenaType arena_type;
/// }
/// ```
pub struct SMSG_ARENA_ERROR {
    pub unknown: u32,
    pub arena_type: ArenaType,
}

impl crate::Message for SMSG_ARENA_ERROR {
    const OPCODE: u32 = 0x0376;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // arena_type: ArenaType
        w.write_all(&(self.arena_type.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0376, size: body_size as u32 });
        }

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        // arena_type: ArenaType
        let arena_type: ArenaType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            unknown,
            arena_type,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_ARENA_ERROR {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ARENA_ERROR {}

