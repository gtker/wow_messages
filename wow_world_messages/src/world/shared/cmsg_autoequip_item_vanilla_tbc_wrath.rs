use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm#L1):
/// ```text
/// cmsg CMSG_AUTOEQUIP_ITEM = 0x010A {
///     u8 source_bag;
///     u8 source_slot;
/// }
/// ```
pub struct CMSG_AUTOEQUIP_ITEM {
    pub source_bag: u8,
    pub source_slot: u8,
}

#[cfg(feature = "print-testcase")]
impl CMSG_AUTOEQUIP_ITEM {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUTOEQUIP_ITEM {{").unwrap();
        // Members
        writeln!(s, "    source_bag = {};", self.source_bag).unwrap();
        writeln!(s, "    source_slot = {};", self.source_slot).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 266_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "source_bag", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "source_slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_AUTOEQUIP_ITEM {}
impl crate::Message for CMSG_AUTOEQUIP_ITEM {
    const OPCODE: u32 = 0x010a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_AUTOEQUIP_ITEM::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x010A, size: body_size });
        }

        // source_bag: u8
        let source_bag = crate::util::read_u8_le(&mut r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            source_bag,
            source_slot,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

