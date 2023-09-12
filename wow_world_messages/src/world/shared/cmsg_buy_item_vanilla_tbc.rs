use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item.wowm#L1):
/// ```text
/// cmsg CMSG_BUY_ITEM = 0x01A2 {
///     Guid vendor;
///     u32 item;
///     u8 amount;
///     u8 unknown1;
/// }
/// ```
pub struct CMSG_BUY_ITEM {
    pub vendor: Guid,
    pub item: u32,
    pub amount: u8,
    /// cmangos says this is hardcoded to 1 in the TBC client.
    pub unknown1: u8,
}

impl crate::private::Sealed for CMSG_BUY_ITEM {}
impl CMSG_BUY_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 14 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // vendor: Guid
        let vendor = crate::util::read_guid(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vendor,
            item,
            amount,
            unknown1,
        })
    }

}

impl crate::Message for CMSG_BUY_ITEM {
    const OPCODE: u32 = 0x01a2;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_BUY_ITEM"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BUY_ITEM {{").unwrap();
        // Members
        writeln!(s, "    vendor = {};", self.vendor.guid()).unwrap();
        writeln!(s, "    item = {};", self.item).unwrap();
        writeln!(s, "    amount = {};", self.amount).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 418_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "vendor", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        14
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(418, "CMSG_BUY_ITEM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BUY_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BUY_ITEM {}

