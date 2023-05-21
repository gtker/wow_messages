use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_set_trade_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_set_trade_item.wowm#L3):
/// ```text
/// cmsg CMSG_SET_TRADE_ITEM = 0x011D {
///     u8 trade_slot;
///     u8 bag;
///     u8 slot;
/// }
/// ```
pub struct CMSG_SET_TRADE_ITEM {
    pub trade_slot: u8,
    pub bag: u8,
    pub slot: u8,
}

impl crate::private::Sealed for CMSG_SET_TRADE_ITEM {}
impl CMSG_SET_TRADE_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 3 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // trade_slot: u8
        let trade_slot = crate::util::read_u8_le(&mut r)?;

        // bag: u8
        let bag = crate::util::read_u8_le(&mut r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            trade_slot,
            bag,
            slot,
        })
    }

}

impl crate::Message for CMSG_SET_TRADE_ITEM {
    const OPCODE: u32 = 0x011d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_TRADE_ITEM {{").unwrap();
        // Members
        writeln!(s, "    trade_slot = {};", self.trade_slot).unwrap();
        writeln!(s, "    bag = {};", self.bag).unwrap();
        writeln!(s, "    slot = {};", self.slot).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 285_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "trade_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "bag", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        3
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // trade_slot: u8
        w.write_all(&self.trade_slot.to_le_bytes())?;

        // bag: u8
        w.write_all(&self.bag.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(285, "CMSG_SET_TRADE_ITEM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SET_TRADE_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_TRADE_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_TRADE_ITEM {}

