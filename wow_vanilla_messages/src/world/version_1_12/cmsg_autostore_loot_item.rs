use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_autostore_loot_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_autostore_loot_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOSTORE_LOOT_ITEM = 0x0108 {
///     u8 item_slot;
/// }
/// ```
pub struct CMSG_AUTOSTORE_LOOT_ITEM {
    pub item_slot: u8,
}

impl ClientMessage for CMSG_AUTOSTORE_LOOT_ITEM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_slot: u8
        w.write_all(&self.item_slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0108;

    fn client_size(&self) -> u16 {
        7
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // item_slot: u8
        let item_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            item_slot,
        })
    }

}

