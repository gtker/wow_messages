use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_item_refund.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_item_refund.wowm#L1):
/// ```text
/// cmsg CMSG_ITEM_REFUND = 0x04B4 {
///     Guid item;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_ITEM_REFUND {
    pub item: Guid,
}

impl crate::private::Sealed for CMSG_ITEM_REFUND {}
impl CMSG_ITEM_REFUND {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        Ok(Self {
            item,
        })
    }

}

impl crate::Message for CMSG_ITEM_REFUND {
    const OPCODE: u32 = 0x04b4;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_ITEM_REFUND"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ITEM_REFUND {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1204_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1204, "CMSG_ITEM_REFUND", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ITEM_REFUND {}

