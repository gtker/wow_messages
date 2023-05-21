use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_split_item.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_split_item.wowm#L13):
/// ```text
/// cmsg CMSG_SPLIT_ITEM = 0x010E {
///     u8 source_bag;
///     u8 source_slot;
///     u8 destination_bag;
///     u8 destination_slot;
///     u32 amount;
/// }
/// ```
pub struct CMSG_SPLIT_ITEM {
    pub source_bag: u8,
    pub source_slot: u8,
    pub destination_bag: u8,
    pub destination_slot: u8,
    pub amount: u32,
}

impl crate::private::Sealed for CMSG_SPLIT_ITEM {}
impl CMSG_SPLIT_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // source_bag: u8
        let source_bag = crate::util::read_u8_le(&mut r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(&mut r)?;

        // destination_bag: u8
        let destination_bag = crate::util::read_u8_le(&mut r)?;

        // destination_slot: u8
        let destination_slot = crate::util::read_u8_le(&mut r)?;

        // amount: u32
        let amount = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            source_bag,
            source_slot,
            destination_bag,
            destination_slot,
            amount,
        })
    }

}

impl crate::Message for CMSG_SPLIT_ITEM {
    const OPCODE: u32 = 0x010e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SPLIT_ITEM {{").unwrap();
        // Members
        writeln!(s, "    source_bag = {};", self.source_bag).unwrap();
        writeln!(s, "    source_slot = {};", self.source_slot).unwrap();
        writeln!(s, "    destination_bag = {};", self.destination_bag).unwrap();
        writeln!(s, "    destination_slot = {};", self.destination_slot).unwrap();
        writeln!(s, "    amount = {};", self.amount).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 270_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "source_bag", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "source_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "destination_bag", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "destination_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes())?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes())?;

        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(270, "CMSG_SPLIT_ITEM", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SPLIT_ITEM {}

