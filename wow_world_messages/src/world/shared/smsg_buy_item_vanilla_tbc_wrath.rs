use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_buy_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_buy_item.wowm#L3):
/// ```text
/// smsg SMSG_BUY_ITEM = 0x01A4 {
///     Guid guid;
///     u32 vendor_slot;
///     u32 amount_for_sale;
///     u32 amount_bought;
/// }
/// ```
pub struct SMSG_BUY_ITEM {
    pub guid: Guid,
    /// Starts at index 1.
    /// arcemu has this field as milliseconds since something instead.
    pub vendor_slot: u32,
    pub amount_for_sale: u32,
    pub amount_bought: u32,
}

impl crate::private::Sealed for SMSG_BUY_ITEM {}
impl crate::Message for SMSG_BUY_ITEM {
    const OPCODE: u32 = 0x01a4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BUY_ITEM {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    vendor_slot = {};", self.vendor_slot).unwrap();
        writeln!(s, "    amount_for_sale = {};", self.amount_for_sale).unwrap();
        writeln!(s, "    amount_bought = {};", self.amount_bought).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 22_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 420_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "vendor_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_for_sale", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_bought", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // vendor_slot: u32
        w.write_all(&self.vendor_slot.to_le_bytes())?;

        // amount_for_sale: u32
        w.write_all(&self.amount_for_sale.to_le_bytes())?;

        // amount_bought: u32
        w.write_all(&self.amount_bought.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A4, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // vendor_slot: u32
        let vendor_slot = crate::util::read_u32_le(&mut r)?;

        // amount_for_sale: u32
        let amount_for_sale = crate::util::read_u32_le(&mut r)?;

        // amount_bought: u32
        let amount_bought = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            vendor_slot,
            amount_for_sale,
            amount_bought,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BUY_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BUY_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BUY_ITEM {}

