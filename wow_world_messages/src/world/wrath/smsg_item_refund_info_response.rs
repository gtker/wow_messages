use crate::Guid;
use crate::wrath::Gold;
use crate::wrath::ItemRefundExtra;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_refund_info_response.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_refund_info_response.wowm#L8):
/// ```text
/// smsg SMSG_ITEM_REFUND_INFO_RESPONSE = 0x04B2 {
///     Guid item;
///     Gold money_cost;
///     u32 honor_point_cost;
///     u32 arena_point_cost;
///     ItemRefundExtra[5] extra_items;
///     u32 unknown1;
///     u32 time_since_loss;
/// }
/// ```
pub struct SMSG_ITEM_REFUND_INFO_RESPONSE {
    pub item: Guid,
    pub money_cost: Gold,
    pub honor_point_cost: u32,
    pub arena_point_cost: u32,
    pub extra_items: [ItemRefundExtra; 5],
    /// Emus set to 0.
    ///
    pub unknown1: u32,
    pub time_since_loss: u32,
}

impl crate::Message for SMSG_ITEM_REFUND_INFO_RESPONSE {
    const OPCODE: u32 = 0x04b2;

    fn size_without_header(&self) -> u32 {
        68
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // money_cost: Gold
        w.write_all(u32::from(self.money_cost.as_int()).to_le_bytes().as_slice())?;

        // honor_point_cost: u32
        w.write_all(&self.honor_point_cost.to_le_bytes())?;

        // arena_point_cost: u32
        w.write_all(&self.arena_point_cost.to_le_bytes())?;

        // extra_items: ItemRefundExtra[5]
        for i in self.extra_items.iter() {
            i.write_into_vec(&mut w)?;
        }

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // time_since_loss: u32
        w.write_all(&self.time_since_loss.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 68 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04B2, size: body_size as u32 });
        }

        // item: Guid
        let item = Guid::read(&mut r)?;

        // money_cost: Gold
        let money_cost = Gold::new(crate::util::read_u32_le(&mut r)?);

        // honor_point_cost: u32
        let honor_point_cost = crate::util::read_u32_le(&mut r)?;

        // arena_point_cost: u32
        let arena_point_cost = crate::util::read_u32_le(&mut r)?;

        // extra_items: ItemRefundExtra[5]
        let extra_items = {
            let mut extra_items = [ItemRefundExtra::default(); 5];
            for i in extra_items.iter_mut() {
                *i = ItemRefundExtra::read(&mut r)?;
            }
            extra_items
        };

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // time_since_loss: u32
        let time_since_loss = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item,
            money_cost,
            honor_point_cost,
            arena_point_cost,
            extra_items,
            unknown1,
            time_since_loss,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ITEM_REFUND_INFO_RESPONSE {}

