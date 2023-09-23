use std::io::{Read, Write};

use crate::Guid;
use crate::shared::gold_vanilla_tbc_wrath::Gold;
use crate::shared::list_inventory_item_tbc_wrath::ListInventoryItem;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// if `amount_of_items` is 0 it is supposedly followed by a single u8 with 0 to say that vendor has no items.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_list_inventory.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_list_inventory.wowm#L39):
/// ```text
/// smsg SMSG_LIST_INVENTORY = 0x019F {
///     Guid vendor;
///     u8 amount_of_items;
///     ListInventoryItem[amount_of_items] items;
/// }
/// ```
pub struct SMSG_LIST_INVENTORY {
    pub vendor: Guid,
    pub items: Vec<ListInventoryItem>,
}

impl crate::private::Sealed for SMSG_LIST_INVENTORY {}
impl SMSG_LIST_INVENTORY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=8201).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // vendor: Guid
        let vendor = crate::util::read_guid(&mut r)?;

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(&mut r)?;

        // items: ListInventoryItem[amount_of_items]
        let items = {
            let mut items = Vec::with_capacity(amount_of_items as usize);
            for _ in 0..amount_of_items {
                items.push(ListInventoryItem::read(&mut r)?);
            }
            items
        };

        Ok(Self {
            vendor,
            items,
        })
    }

}

impl crate::Message for SMSG_LIST_INVENTORY {
    const OPCODE: u32 = 0x019f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LIST_INVENTORY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LIST_INVENTORY {{").unwrap();
        // Members
        writeln!(s, "    vendor = {};", self.vendor.guid()).unwrap();
        writeln!(s, "    amount_of_items = {};", self.items.len()).unwrap();
        writeln!(s, "    items = [").unwrap();
        for v in self.items.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            item_stack_count = {};", v.item_stack_count).unwrap();
            writeln!(s, "            item = {};", v.item).unwrap();
            writeln!(s, "            item_display_id = {};", v.item_display_id).unwrap();
            writeln!(s, "            max_items = {};", v.max_items).unwrap();
            writeln!(s, "            price = {};", v.price.as_int()).unwrap();
            writeln!(s, "            max_durability = {};", v.max_durability).unwrap();
            writeln!(s, "            durability = {};", v.durability).unwrap();
            writeln!(s, "            extended_cost = {};", v.extended_cost).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 415_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "vendor", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_items", "    ");
        if !self.items.is_empty() {
            writeln!(s, "    /* items: ListInventoryItem[amount_of_items] start */").unwrap();
            for (i, v) in self.items.iter().enumerate() {
                writeln!(s, "    /* items: ListInventoryItem[amount_of_items] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_stack_count", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_display_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "max_items", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "price", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "max_durability", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "durability", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "extended_cost", "        ");
                writeln!(s, "    /* items: ListInventoryItem[amount_of_items] {i} end */").unwrap();
            }
            writeln!(s, "    /* items: ListInventoryItem[amount_of_items] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: ListInventoryItem[amount_of_items]
        for i in self.items.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(415, "SMSG_LIST_INVENTORY", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LIST_INVENTORY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LIST_INVENTORY {}

impl SMSG_LIST_INVENTORY {
    pub(crate) fn size(&self) -> usize {
        8 // vendor: Guid
        + 1 // amount_of_items: u8
        + self.items.len() * 32 // items: ListInventoryItem[amount_of_items]
    }
}

