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

#[cfg(feature = "print-testcase")]
impl SMSG_ITEM_REFUND_RESULT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ITEM_REFUND_RESULT {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    result = {};", crate::wrath::ItemRefundResult::try_from(self.result.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.result {
            crate::wrath::SMSG_ITEM_REFUND_RESULT_ItemRefundResult::Success {
                arena_point_cost,
                cost,
                extra_items,
                honor_point_cost,
            } => {
                writeln!(s, "    cost = {};", cost.as_int()).unwrap();
                writeln!(s, "    honor_point_cost = {};", honor_point_cost).unwrap();
                writeln!(s, "    arena_point_cost = {};", arena_point_cost).unwrap();
                write!(s, "    extra_items = [").unwrap();
                for v in extra_items.as_slice() {
                    writeln!(s, "{{").unwrap();
                    // Members
                    writeln!(s, "    item = {};", v.item).unwrap();
                    writeln!(s, "    amount = {};", v.amount).unwrap();

                    writeln!(s, "    }},").unwrap();
                }
                writeln!(s, "];").unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1205_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "item");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

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
        let item = crate::util::read_guid(&mut r)?;

        // result: ItemRefundResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

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

impl std::fmt::Display for SMSG_ITEM_REFUND_RESULT_ItemRefundResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success{ .. } => f.write_str("Success"),
            Self::Failure => f.write_str("Failure"),
        }
    }
}

impl SMSG_ITEM_REFUND_RESULT_ItemRefundResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Success {
                ..
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

