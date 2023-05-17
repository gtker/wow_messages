use std::io::{Read, Write};

use wow_world_base::shared::arena_type_tbc_wrath::ArenaType;

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

impl crate::private::Sealed for SMSG_ARENA_ERROR {}
impl crate::Message for SMSG_ARENA_ERROR {
    const OPCODE: u32 = 0x0376;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // arena_type: ArenaType
        w.write_all(&(self.arena_type.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0376, size: body_size });
        }

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        // arena_type: ArenaType
        let arena_type: ArenaType = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            unknown,
            arena_type,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_ERROR {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_ERROR {}

