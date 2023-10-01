use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm#L1):
/// ```text
/// cmsg CMSG_BUY_ITEM_IN_SLOT = 0x01A3 {
///     Guid vendor;
///     Item item;
///     Guid bag;
///     u8 bag_slot;
///     u8 amount;
/// }
/// ```
pub struct CMSG_BUY_ITEM_IN_SLOT {
    pub vendor: Guid,
    pub item: u32,
    pub bag: Guid,
    pub bag_slot: u8,
    pub amount: u8,
}

impl crate::private::Sealed for CMSG_BUY_ITEM_IN_SLOT {}
impl CMSG_BUY_ITEM_IN_SLOT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 22 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // vendor: Guid
        let vendor = crate::util::read_guid(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // bag: Guid
        let bag = crate::util::read_guid(&mut r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(&mut r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vendor,
            item,
            bag,
            bag_slot,
            amount,
        })
    }

}

impl crate::Message for CMSG_BUY_ITEM_IN_SLOT {
    const OPCODE: u32 = 0x01a3;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_BUY_ITEM_IN_SLOT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BUY_ITEM_IN_SLOT {{").unwrap();
        // Members
        writeln!(s, "    vendor = {};", self.vendor.guid()).unwrap();
        writeln!(s, "    item = {};", self.item).unwrap();
        writeln!(s, "    bag = {};", self.bag.guid()).unwrap();
        writeln!(s, "    bag_slot = {};", self.bag_slot).unwrap();
        writeln!(s, "    amount = {};", self.amount).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 26_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 419_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "vendor", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "bag", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        22
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // bag: Guid
        w.write_all(&self.bag.guid().to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(419, "CMSG_BUY_ITEM_IN_SLOT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BUY_ITEM_IN_SLOT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BUY_ITEM_IN_SLOT {}

