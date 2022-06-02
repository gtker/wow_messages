use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_removed_notification.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_removed_notification.wowm#L3):
/// ```text
/// smsg SMSG_AUCTION_REMOVED_NOTIFICATION = 0x028D {
///     u32 item_id;
///     u32 item_template;
///     u32 random_property_id;
/// }
/// ```
pub struct SMSG_AUCTION_REMOVED_NOTIFICATION {
    pub item_id: u32,
    pub item_template: u32,
    pub random_property_id: u32,
}

impl ServerMessage for SMSG_AUCTION_REMOVED_NOTIFICATION {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_template: u32
        w.write_all(&self.item_template.to_le_bytes())?;

        // random_property_id: u32
        w.write_all(&self.random_property_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x028d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_template: u32
        let item_template = crate::util::read_u32_le(r)?;

        // random_property_id: u32
        let random_property_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            item_id,
            item_template,
            random_property_id,
        })
    }

}

