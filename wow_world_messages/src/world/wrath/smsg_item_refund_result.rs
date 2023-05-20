use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    ItemRefundExtra, ItemRefundResult,
};
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_refund_result.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_refund_result.wowm#L8):
/// ```text
/// smsg SMSG_ITEM_REFUND_RESULT = 0x04B5 {
///     Guid item;
///     ItemRefundResult result;
///     if (result == SUCCESS) {
///         Gold cost;
///         u32 honor_point_cost;
///         u32 arena_point_cost;
///         ItemRefundExtra[5] extra_items;
///     }
/// }
/// ```
pub struct SMSG_ITEM_REFUND_RESULT {
    pub item: Guid,
    pub result: SMSG_ITEM_REFUND_RESULT_ItemRefundResult,
}

impl crate::private::Sealed for SMSG_ITEM_REFUND_RESULT {}
impl crate::Message for SMSG_ITEM_REFUND_RESULT {
    const OPCODE: u32 = 0x04b5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // result: ItemRefundResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        match &self.result {
            SMSG_ITEM_REFUND_RESULT_ItemRefundResult::Success {
                arena_point_cost,
                cost,
                extra_items,
                honor_point_cost,
            } => {
                // cost: Gold
                w.write_all((cost.as_int()).to_le_bytes().as_slice())?;

                // honor_point_cost: u32
                w.write_all(&honor_point_cost.to_le_bytes())?;

                // arena_point_cost: u32
                w.write_all(&arena_point_cost.to_le_bytes())?;

                // extra_items: ItemRefundExtra[5]
                for i in extra_items.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
            _ => {}
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=61).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04B5, size: body_size });
        }

        // item: Guid
        let item = Guid::read(&mut r)?;

        // result: ItemRefundResult
        let result: ItemRefundResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            ItemRefundResult::Success => {
                // cost: Gold
                let cost = Gold::new(crate::util::read_u32_le(&mut r)?);

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

                SMSG_ITEM_REFUND_RESULT_ItemRefundResult::Success {
                    arena_point_cost,
                    cost,
                    extra_items,
                    honor_point_cost,
                }
            }
            ItemRefundResult::Failure => SMSG_ITEM_REFUND_RESULT_ItemRefundResult::Failure,
        };

        Ok(Self {
            item,
            result: result_if,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ITEM_REFUND_RESULT {}

impl SMSG_ITEM_REFUND_RESULT {
    pub(crate) const fn size(&self) -> usize {
        8 // item: Guid
        + self.result.size() // result: SMSG_ITEM_REFUND_RESULT_ItemRefundResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_ITEM_REFUND_RESULT_ItemRefundResult {
    Success {
        arena_point_cost: u32,
        cost: Gold,
        extra_items: [ItemRefundExtra; 5],
        honor_point_cost: u32,
    },
    Failure,
}

impl Default for SMSG_ITEM_REFUND_RESULT_ItemRefundResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Failure
    }
}

impl SMSG_ITEM_REFUND_RESULT_ItemRefundResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Success { .. } => 0,
            Self::Failure => 10,
        }
    }

}

impl SMSG_ITEM_REFUND_RESULT_ItemRefundResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Success {
                arena_point_cost,
                cost,
                extra_items,
                honor_point_cost,
            } => {
                1
                + 4 // arena_point_cost: u32
                + 4 // cost: Gold
                + 5 * 8 // extra_items: ItemRefundExtra[5]
                + 4 // honor_point_cost: u32
            }
            _ => 1,
        }
    }
}

