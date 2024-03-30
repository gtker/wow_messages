use std::io::{Read, Write};

use crate::tbc::ItemSlot;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm#L1):
/// ```text
/// cmsg CMSG_SWAP_INV_ITEM = 0x010D {
///     ItemSlot source_slot;
///     ItemSlot destination_slot;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SWAP_INV_ITEM {
    pub source_slot: ItemSlot,
    pub destination_slot: ItemSlot,
}

impl crate::private::Sealed for CMSG_SWAP_INV_ITEM {}
impl CMSG_SWAP_INV_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 2 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // source_slot: ItemSlot
        let source_slot = crate::util::read_u8_le(&mut r)?.try_into()?;

        // destination_slot: ItemSlot
        let destination_slot = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            source_slot,
            destination_slot,
        })
    }

}

impl crate::Message for CMSG_SWAP_INV_ITEM {
    const OPCODE: u32 = 0x010d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SWAP_INV_ITEM"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SWAP_INV_ITEM {{").unwrap();
        // Members
        writeln!(s, "    source_slot = {};", self.source_slot.as_test_case_value()).unwrap();
        writeln!(s, "    destination_slot = {};", self.destination_slot.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 269_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "source_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "destination_slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // source_slot: ItemSlot
        w.write_all(&(self.source_slot.as_int().to_le_bytes()))?;

        // destination_slot: ItemSlot
        w.write_all(&(self.destination_slot.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(269, "CMSG_SWAP_INV_ITEM", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SWAP_INV_ITEM {}

