use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_swap_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_swap_item.wowm#L3):
/// ```text
/// cmsg CMSG_SWAP_ITEM = 0x010C {
///     u8 destination_bag;
///     u8 destionation_slot;
///     u8 source_bag;
///     u8 source_slot;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SWAP_ITEM {
    pub destination_bag: u8,
    pub destionation_slot: u8,
    pub source_bag: u8,
    pub source_slot: u8,
}

impl crate::private::Sealed for CMSG_SWAP_ITEM {}
impl CMSG_SWAP_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // destination_bag: u8
        let destination_bag = crate::util::read_u8_le(&mut r)?;

        // destionation_slot: u8
        let destionation_slot = crate::util::read_u8_le(&mut r)?;

        // source_bag: u8
        let source_bag = crate::util::read_u8_le(&mut r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            destination_bag,
            destionation_slot,
            source_bag,
            source_slot,
        })
    }

}

impl crate::Message for CMSG_SWAP_ITEM {
    const OPCODE: u32 = 0x010c;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SWAP_ITEM"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SWAP_ITEM {{").unwrap();
        // Members
        writeln!(s, "    destination_bag = {};", self.destination_bag).unwrap();
        writeln!(s, "    destionation_slot = {};", self.destionation_slot).unwrap();
        writeln!(s, "    source_bag = {};", self.source_bag).unwrap();
        writeln!(s, "    source_slot = {};", self.source_slot).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 268_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "destination_bag", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "destionation_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "source_bag", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "source_slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes())?;

        // destionation_slot: u8
        w.write_all(&self.destionation_slot.to_le_bytes())?;

        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(268, "CMSG_SWAP_ITEM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SWAP_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SWAP_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SWAP_ITEM {}

