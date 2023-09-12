use std::io::{Read, Write};

use crate::Guid;
use crate::shared::gold_vanilla_tbc_wrath::Gold;
use crate::wrath::ItemRefundExtra;

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
    pub unknown1: u32,
    pub time_since_loss: u32,
}

impl crate::private::Sealed for SMSG_ITEM_REFUND_INFO_RESPONSE {}
impl SMSG_ITEM_REFUND_INFO_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 68 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

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

impl crate::Message for SMSG_ITEM_REFUND_INFO_RESPONSE {
    const OPCODE: u32 = 0x04b2;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ITEM_REFUND_INFO_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ITEM_REFUND_INFO_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    money_cost = {};", self.money_cost.as_int()).unwrap();
        writeln!(s, "    honor_point_cost = {};", self.honor_point_cost).unwrap();
        writeln!(s, "    arena_point_cost = {};", self.arena_point_cost).unwrap();
        write!(s, "    extra_items = [").unwrap();
        for v in self.extra_items.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        amount = {};", v.amount).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    time_since_loss = {};", self.time_since_loss).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 70_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1202_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_cost", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_point_cost", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "arena_point_cost", "    ");
        writeln!(s, "    /* extra_items: ItemRefundExtra[5] start */").unwrap();
        for (i, v) in self.extra_items.iter().enumerate() {
            writeln!(s, "    /* extra_items: ItemRefundExtra[5] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "amount", "        ");
            writeln!(s, "    /* extra_items: ItemRefundExtra[5] {i} end */").unwrap();
        }
        writeln!(s, "    /* extra_items: ItemRefundExtra[5] end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_since_loss", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        68
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // money_cost: Gold
        w.write_all((self.money_cost.as_int()).to_le_bytes().as_slice())?;

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1202, "SMSG_ITEM_REFUND_INFO_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ITEM_REFUND_INFO_RESPONSE {}

