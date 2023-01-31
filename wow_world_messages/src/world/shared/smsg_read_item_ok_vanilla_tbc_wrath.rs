use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_read_item_ok.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_read_item_ok.wowm#L3):
/// ```text
/// smsg SMSG_READ_ITEM_OK = 0x00AE {
///     Guid guid;
/// }
/// ```
pub struct SMSG_READ_ITEM_OK {
    pub guid: Guid,
}

impl crate::Message for SMSG_READ_ITEM_OK {
    const OPCODE: u32 = 0x00ae;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00AE, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_READ_ITEM_OK {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_READ_ITEM_OK {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_READ_ITEM_OK {}

