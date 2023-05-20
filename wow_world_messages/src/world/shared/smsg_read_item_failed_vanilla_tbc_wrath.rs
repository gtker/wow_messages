use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

impl crate::private::Sealed for SMSG_READ_ITEM_FAILED {}
impl crate::Message for SMSG_READ_ITEM_FAILED {
    const OPCODE: u32 = 0x00af;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00AF, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_READ_ITEM_FAILED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_READ_ITEM_FAILED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_READ_ITEM_FAILED {}

