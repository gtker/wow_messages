use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_item_query_single.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_item_query_single.wowm#L1):
/// ```text
/// cmsg CMSG_ITEM_QUERY_SINGLE = 0x0056 {
///     Item item;
///     Guid guid;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_ITEM_QUERY_SINGLE {
    pub item: u32,
    pub guid: Guid,
}

impl crate::private::Sealed for CMSG_ITEM_QUERY_SINGLE {}
impl CMSG_ITEM_QUERY_SINGLE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            item,
            guid,
        })
    }

}

impl crate::Message for CMSG_ITEM_QUERY_SINGLE {
    const OPCODE: u32 = 0x0056;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_ITEM_QUERY_SINGLE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ITEM_QUERY_SINGLE {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 86_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(86, "CMSG_ITEM_QUERY_SINGLE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ITEM_QUERY_SINGLE {}

