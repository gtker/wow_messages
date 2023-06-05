use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_sell_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_sell_item.wowm#L3):
/// ```text
/// cmsg CMSG_SELL_ITEM = 0x01A0 {
///     Guid vendor;
///     Guid item;
///     u8 amount;
/// }
/// ```
pub struct CMSG_SELL_ITEM {
    pub vendor: Guid,
    pub item: Guid,
    pub amount: u8,
}

#[cfg(feature = "print-testcase")]
impl CMSG_SELL_ITEM {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SELL_ITEM {{").unwrap();
        // Members
        writeln!(s, "    vendor = {};", self.vendor.guid()).unwrap();
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    amount = {};", self.amount).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 23_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 416_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "vendor");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_SELL_ITEM {}
impl crate::Message for CMSG_SELL_ITEM {
    const OPCODE: u32 = 0x01a0;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A0, size: body_size });
        }

        // vendor: Guid
        let vendor = crate::util::read_guid(&mut r)?;

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vendor,
            item,
            amount,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SELL_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SELL_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SELL_ITEM {}

