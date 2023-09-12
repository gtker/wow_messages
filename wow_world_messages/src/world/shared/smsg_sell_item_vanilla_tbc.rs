use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::sell_item_result_vanilla_tbc::SellItemResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_sell_item.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_sell_item.wowm#L22):
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

impl crate::private::Sealed for SMSG_SELL_ITEM {}
impl SMSG_SELL_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 17 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for SMSG_SELL_ITEM {
    const OPCODE: u32 = 0x01a1;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SELL_ITEM"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SELL_ITEM {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 19_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 417_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(417, "SMSG_SELL_ITEM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SELL_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SELL_ITEM {}

