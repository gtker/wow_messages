use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::ItemRefundExtra;
use crate::world::wrath::ItemRefundResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_refund_result.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_refund_result.wowm#L8):
/// ```text
/// smsg SMSG_ITEM_REFUND_RESULT = 0x04B5 {
///     Guid item;
///     ItemRefundResult result;
///     if (result == SUCCESS) {
///         u32 money_cost;
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

impl crate::Message for SMSG_ITEM_REFUND_RESULT {
    const OPCODE: u32 = 0x04b5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // result: ItemRefundResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            SMSG_ITEM_REFUND_RESULT_ItemRefundResult::Success {
                arena_point_cost,
                extra_items,
                honor_point_cost,
                money_cost,
            } => {
                // money_cost: u32
                w.write_all(&money_cost.to_le_bytes())?;

                // honor_point_cost: u32
                w.write_all(&honor_point_cost.to_le_bytes())?;

                // arena_point_cost: u32
                w.write_all(&arena_point_cost.to_le_bytes())?;

                // extra_items: ItemRefundExtra[5]
                for i in extra_items.iter() {
                    i.write_into_vec(w)?;
                }

            }
            SMSG_ITEM_REFUND_RESULT_ItemRefundResult::Failure => {}
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=61).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04B5, size: body_size as u32 });
        }

        // item: Guid
        let item = Guid::read(r)?;

        // result: ItemRefundResult
        let result: ItemRefundResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            ItemRefundResult::Success => {
                // money_cost: u32
                let money_cost = crate::util::read_u32_le(r)?;

                // honor_point_cost: u32
                let honor_point_cost = crate::util::read_u32_le(r)?;

                // arena_point_cost: u32
                let arena_point_cost = crate::util::read_u32_le(r)?;

                // extra_items: ItemRefundExtra[5]
                let mut extra_items = [ItemRefundExtra::default(); 5];
                for i in extra_items.iter_mut() {
                    *i = ItemRefundExtra::read(r)?;
                }

                SMSG_ITEM_REFUND_RESULT_ItemRefundResult::Success {
                    arena_point_cost,
                    extra_items,
                    honor_point_cost,
                    money_cost,
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
impl crate::world::wrath::ServerMessage for SMSG_ITEM_REFUND_RESULT {}

impl SMSG_ITEM_REFUND_RESULT {
    pub(crate) fn size(&self) -> usize {
        8 // item: Guid
        + self.result.size() // result: SMSG_ITEM_REFUND_RESULT_ItemRefundResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_ITEM_REFUND_RESULT_ItemRefundResult {
    Success {
        arena_point_cost: u32,
        extra_items: [ItemRefundExtra; 5],
        honor_point_cost: u32,
        money_cost: u32,
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
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Success {
                arena_point_cost,
                extra_items,
                honor_point_cost,
                money_cost,
            } => {
                1
                + 4 // arena_point_cost: u32
                + 5 * 8 // extra_items: ItemRefundExtra[5]
                + 4 // honor_point_cost: u32
                + 4 // money_cost: u32
            }
            Self::Failure => {
                1
            }
        }
    }
}

