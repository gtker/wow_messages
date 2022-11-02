use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// vmangos has extra u8 with comment `0..2, read failure reason? if == 1, use next command`.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_read_item_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_read_item_failed.wowm#L3):
/// ```text
/// smsg SMSG_READ_ITEM_FAILED = 0x00AF {
///     Guid guid;
/// }
/// ```
pub struct SMSG_READ_ITEM_FAILED {
    pub guid: Guid,
}

impl crate::Message for SMSG_READ_ITEM_FAILED {
    const OPCODE: u32 = 0x00af;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00AF, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_READ_ITEM_FAILED {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_READ_ITEM_FAILED {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_READ_ITEM_FAILED {}

