use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_unit_destroyed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_unit_destroyed.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_UNIT_DESTROYED = 0x04C7 {
///     Guid unit;
/// }
/// ```
pub struct SMSG_ARENA_UNIT_DESTROYED {
    pub unit: Guid,
}

impl crate::private::Sealed for SMSG_ARENA_UNIT_DESTROYED {}
impl crate::Message for SMSG_ARENA_UNIT_DESTROYED {
    const OPCODE: u32 = 0x04c7;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: Guid
        w.write_all(&self.unit.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04C7, size: body_size });
        }

        // unit: Guid
        let unit = crate::util::read_guid(&mut r)?;

        Ok(Self {
            unit,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_UNIT_DESTROYED {}

