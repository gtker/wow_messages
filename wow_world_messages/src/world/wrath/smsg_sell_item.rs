use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::SellItemResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_sell_item.wowm:62`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_sell_item.wowm#L62):
/// ```text
/// smsg SMSG_SELL_ITEM = 0x01A1 {
///     Guid guid;
///     Guid item;
///     SellItemResult result;
/// }
/// ```
pub struct SMSG_SELL_ITEM {
    pub guid: Guid,
    pub item: Guid,
    pub result: SellItemResult,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SELL_ITEM {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SELL_ITEM {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 21_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 417_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid");
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

impl crate::private::Sealed for SMSG_SELL_ITEM {}
impl crate::Message for SMSG_SELL_ITEM {
    const OPCODE: u32 = 0x01a1;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // result: SellItemResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A1, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // result: SellItemResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            item,
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SELL_ITEM {}

