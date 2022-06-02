use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_time_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_time_update.wowm#L3):
/// ```text
/// smsg SMSG_ITEM_TIME_UPDATE = 0x01EA {
///     Guid guid;
///     u32 duration;
/// }
/// ```
pub struct SMSG_ITEM_TIME_UPDATE {
    pub guid: Guid,
    pub duration: u32,
}

impl ServerMessage for SMSG_ITEM_TIME_UPDATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01ea;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            duration,
        })
    }

}

