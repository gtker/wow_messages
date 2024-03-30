use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autobank_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autobank_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOBANK_ITEM = 0x0283 {
///     u8 bag_index;
///     u8 slot_index;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_AUTOBANK_ITEM {
    pub bag_index: u8,
    pub slot_index: u8,
}

impl crate::private::Sealed for CMSG_AUTOBANK_ITEM {}
impl CMSG_AUTOBANK_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 2 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // bag_index: u8
        let bag_index = crate::util::read_u8_le(&mut r)?;

        // slot_index: u8
        let slot_index = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            bag_index,
            slot_index,
        })
    }

}

impl crate::Message for CMSG_AUTOBANK_ITEM {
    const OPCODE: u32 = 0x0283;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_AUTOBANK_ITEM"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUTOBANK_ITEM {{").unwrap();
        // Members
        writeln!(s, "    bag_index = {};", self.bag_index).unwrap();
        writeln!(s, "    slot_index = {};", self.slot_index).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 643_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_index", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot_index", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // slot_index: u8
        w.write_all(&self.slot_index.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(643, "CMSG_AUTOBANK_ITEM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUTOBANK_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUTOBANK_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUTOBANK_ITEM {}

