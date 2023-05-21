use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_money_notify.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_money_notify.wowm#L1):
/// ```text
/// smsg SMSG_LOOT_MONEY_NOTIFY = 0x0163 {
///     u32 amount;
/// }
/// ```
pub struct SMSG_LOOT_MONEY_NOTIFY {
    pub amount: u32,
}

impl crate::private::Sealed for SMSG_LOOT_MONEY_NOTIFY {}
impl SMSG_LOOT_MONEY_NOTIFY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount: u32
        let amount = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            amount,
        })
    }

}

impl crate::Message for SMSG_LOOT_MONEY_NOTIFY {
    const OPCODE: u32 = 0x0163;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_MONEY_NOTIFY {{").unwrap();
        // Members
        writeln!(s, "    amount = {};", self.amount).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 355_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(355, "SMSG_LOOT_MONEY_NOTIFY", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_MONEY_NOTIFY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_MONEY_NOTIFY {}

