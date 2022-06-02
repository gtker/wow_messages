use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm#L3):
/// ```text
/// cmsg CMSG_BUY_ITEM_IN_SLOT = 0x01A3 {
///     Guid vendor_guid;
///     u32 item_id;
///     Guid bag_guid;
///     u8 bag_slot;
///     u8 amount;
/// }
/// ```
pub struct CMSG_BUY_ITEM_IN_SLOT {
    pub vendor_guid: Guid,
    pub item_id: u32,
    pub bag_guid: Guid,
    pub bag_slot: u8,
    pub amount: u8,
}

impl ClientMessage for CMSG_BUY_ITEM_IN_SLOT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // vendor_guid: Guid
        w.write_all(&self.vendor_guid.guid().to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // bag_guid: Guid
        w.write_all(&self.bag_guid.guid().to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01a3;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        22
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // vendor_guid: Guid
        let vendor_guid = Guid::read(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // bag_guid: Guid
        let bag_guid = Guid::read(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_id,
            bag_guid,
            bag_slot,
            amount,
        })
    }

}

