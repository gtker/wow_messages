use std::io::{Read, Write};

use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_set_trade_gold.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_set_trade_gold.wowm#L3):
/// ```text
/// cmsg CMSG_SET_TRADE_GOLD = 0x011F {
///     Gold gold;
/// }
/// ```
pub struct CMSG_SET_TRADE_GOLD {
    pub gold: Gold,
}

#[cfg(feature = "print-testcase")]
impl CMSG_SET_TRADE_GOLD {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_TRADE_GOLD {{").unwrap();
        // Members
        writeln!(s, "    gold = {};", self.gold.as_int()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 287_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "gold", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_SET_TRADE_GOLD {}
impl crate::Message for CMSG_SET_TRADE_GOLD {
    const OPCODE: u32 = 0x011f;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_SET_TRADE_GOLD::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // gold: Gold
        w.write_all((self.gold.as_int()).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x011F, size: body_size });
        }

        // gold: Gold
        let gold = Gold::new(crate::util::read_u32_le(&mut r)?);

        Ok(Self {
            gold,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SET_TRADE_GOLD {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_TRADE_GOLD {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_TRADE_GOLD {}

