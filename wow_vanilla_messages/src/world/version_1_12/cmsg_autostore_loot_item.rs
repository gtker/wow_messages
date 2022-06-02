use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

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

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_slot: u8
        let item_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            item_slot,
        })
    }

}

