use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_autostore_loot_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_autostore_loot_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOSTORE_LOOT_ITEM = 0x0108 {
///     u8 item_slot;
/// }
/// ```
pub struct CMSG_AUTOSTORE_LOOT_ITEM {
    pub item_slot: u8,
}

#[cfg(feature = "print-testcase")]
impl CMSG_AUTOSTORE_LOOT_ITEM {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUTOSTORE_LOOT_ITEM {{").unwrap();
        // Members
        writeln!(s, "    item_slot = {};", self.item_slot).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 264_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "item_slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_AUTOSTORE_LOOT_ITEM {}
impl crate::Message for CMSG_AUTOSTORE_LOOT_ITEM {
    const OPCODE: u32 = 0x0108;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_AUTOSTORE_LOOT_ITEM::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item_slot: u8
        w.write_all(&self.item_slot.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0108, size: body_size });
        }

        // item_slot: u8
        let item_slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            item_slot,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUTOSTORE_LOOT_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUTOSTORE_LOOT_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUTOSTORE_LOOT_ITEM {}

