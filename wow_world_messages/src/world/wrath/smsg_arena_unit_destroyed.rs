use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_arena_unit_destroyed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_arena_unit_destroyed.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_UNIT_DESTROYED = 0x04C7 {
///     Guid unit;
/// }
/// ```
pub struct SMSG_ARENA_UNIT_DESTROYED {
    pub unit: Guid,
}

impl crate::Message for SMSG_ARENA_UNIT_DESTROYED {
    const OPCODE: u32 = 0x04c7;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unit: Guid
        w.write_all(&self.unit.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04C7, size: body_size as u32 });
        }

        // unit: Guid
        let unit = Guid::read(r)?;

        Ok(Self {
            unit,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ARENA_UNIT_DESTROYED {}

