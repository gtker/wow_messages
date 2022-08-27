use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autobank_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autobank_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOBANK_ITEM = 0x0283 {
///     u8 bag_index;
///     u8 slot_index;
/// }
/// ```
pub struct CMSG_AUTOBANK_ITEM {
    pub bag_index: u8,
    pub slot_index: u8,
}

impl ClientMessage for CMSG_AUTOBANK_ITEM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // slot_index: u8
        w.write_all(&self.slot_index.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0283;

    fn client_size(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // slot_index: u8
        let slot_index = crate::util::read_u8_le(r)?;

        Ok(Self {
            bag_index,
            slot_index,
        })
    }

}

